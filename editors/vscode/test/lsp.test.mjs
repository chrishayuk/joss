// Talks to the language server the way an editor does: over standard input
// and output. It tests the WebAssembly build the extension ships and, when
// it has been built, the native program other editors would use.
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import test from "node:test";

const here = new URL("..", import.meta.url).pathname;
const native = new URL("../../../target/release/joss-lsp", import.meta.url).pathname;
const servers = [["WebAssembly", process.execPath, [`${here}server/lsp.js`, "--stdio"]]];
if (existsSync(native)) servers.push(["native", native, []]);

function connect(command, args) {
  const child = spawn(command, args, { stdio: ["pipe", "pipe", "inherit"] });
  let buffer = Buffer.alloc(0);
  const waiting = [];
  const arrived = [];
  child.stdout.on("data", (chunk) => {
    buffer = Buffer.concat([buffer, chunk]);
    for (;;) {
      const header = buffer.indexOf("\r\n\r\n");
      if (header === -1) return;
      const length = Number(/Content-Length: (\d+)/i.exec(buffer.subarray(0, header).toString())[1]);
      if (buffer.length < header + 4 + length) return;
      const message = JSON.parse(buffer.subarray(header + 4, header + 4 + length).toString());
      buffer = buffer.subarray(header + 4 + length);
      const next = waiting.shift();
      if (next) next(message);
      else arrived.push(message);
    }
  });
  const send = (message) => {
    const body = JSON.stringify({ jsonrpc: "2.0", ...message });
    child.stdin.write(`Content-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`);
  };
  const receive = () => new Promise((resolve) => (arrived.length ? resolve(arrived.shift()) : waiting.push(resolve)));
  let id = 0;
  const request = async (method, params) => {
    send({ id: ++id, method, params });
    const reply = await receive();
    assert.equal(reply.id, id);
    return reply.result;
  };
  const exited = new Promise((resolve) => child.on("exit", resolve));
  return { send, receive, request, exited };
}

const uri = "file:///hypotenuse.joss";
// From P-3131 (28 April 1965), with a condition that uses the stand-ins.
const program = `*  Stored program for computing the hypotenuse:
1.1 Set c=sqrt(a*2+b*2).
1.2 Type a,b,c in form 1 if fp(c) != 0 or c >= 5.
Form 1:
a = __   b = __   c = ___.______
2.1 Do part 1 for b=1(1)a.
2.2 Line.
Do part 2 for a=1(1)4.
`;

for (const [name, command, args] of servers) {
  test(`${name} server: a session from initialize to exit`, async () => {
    const server = connect(command, args);
    const initialised = await server.request("initialize", { processId: null, rootUri: null, capabilities: {} });
    assert.equal(initialised.serverInfo.name, "joss-lsp");
    assert.equal(initialised.capabilities.hoverProvider, true);
    assert.equal(initialised.capabilities.documentFormattingProvider, true);
    server.send({ method: "initialized", params: {} });

    server.send({ method: "textDocument/didOpen", params: { textDocument: { uri, languageId: "joss", version: 1, text: program } } });
    const clean = await server.receive();
    assert.equal(clean.method, "textDocument/publishDiagnostics");
    assert.deepEqual(clean.params.diagnostics, []);

    const at = (line, character) => ({ textDocument: { uri }, position: { line, character } });
    const verb = await server.request("textDocument/hover", at(1, 5));
    assert.match(verb.contents.value, /`Set`.*gives a letter a value/);
    const power = await server.request("textDocument/hover", at(1, 16));
    assert.match(power.contents.value, /`\*`.*to the power/);
    const standIn = await server.request("textDocument/hover", at(2, 34));
    assert.match(standIn.contents.value, /`≠`.*is not equal to/);
    assert.match(standIn.contents.value, /Typed here as `!=`/);
    const part = await server.request("textDocument/hover", at(5, 12));
    assert.match(part.contents.value, /Part 1.*2 steps/);
    assert.match(part.contents.value, /1\.1 Set c=sqrt\(a\*2\+b\*2\)\./);
    const form = await server.request("textDocument/hover", at(2, 23));
    assert.match(form.contents.value, /a = __   b = __   c = ___\.______/);
    const field = await server.request("textDocument/hover", at(4, 30));
    assert.match(field.contents.value, /Fixed blank.*10 columns wide, 6 decimals/);

    const definition = await server.request("textDocument/definition", at(5, 12));
    assert.deepEqual(definition.range.start, { line: 1, character: 0 });

    const renamed = await server.request("textDocument/rename", { ...at(1, 8), newName: "h" });
    assert.equal(renamed.changes[uri].length, 4);

    const completion = await server.request("textDocument/completion", at(7, 3));
    assert.deepEqual(completion.map((item) => item.label), ["step", "part"]);

    const symbols = await server.request("textDocument/documentSymbol", { textDocument: { uri } });
    assert.deepEqual(symbols.map((s) => s.name), ["Part 1", "Form 1", "Part 2"]);
    assert.deepEqual(symbols[0].children.map((s) => s.name), ["Step 1.1", "Step 1.2"]);

    const tokens = await server.request("textDocument/semanticTokens/full", { textDocument: { uri } });
    assert.equal(tokens.data.length % 5, 0);
    assert.ok(tokens.data.length >= 5 * 40);

    const edits = await server.request("textDocument/formatting", { textDocument: { uri }, options: { tabSize: 4, insertSpaces: true } });
    assert.deepEqual(edits.map((edit) => edit.newText), ["≠", "≥"]);

    // Break it: a line that is not JOSS, and a Do of a part with no steps.
    server.send({ method: "textDocument/didChange", params: { textDocument: { uri, version: 2 }, contentChanges: [{ text: "Type 2+2+.\nDo part 3.\n" }] } });
    const broken = await server.receive();
    assert.deepEqual(broken.params.diagnostics.map((d) => [d.code, d.severity]), [["eh", 1], ["missing", 2]]);
    assert.deepEqual(broken.params.diagnostics[0].range, { start: { line: 0, character: 9 }, end: { line: 0, character: 10 } });
    assert.equal(broken.params.diagnostics[1].message, "Part 3 has no steps yet.");

    assert.equal(await server.request("shutdown", null), null);
    server.send({ method: "exit" });
    assert.equal(await server.exited, 0);
  });
}
