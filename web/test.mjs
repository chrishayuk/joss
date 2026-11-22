// Replays every example and every original session through the WebAssembly
// build and the page's own code, and compares with the .out files and with
// the native build's report. Then asks the language server, through the
// worker the page's editor uses, about the same examples.
//   node web/test.mjs
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { Sitting, paper, replay, verify } from "./session.js";

const root = new URL("..", import.meta.url).pathname;
const module = new WebAssembly.Module(readFileSync(root + "web/joss.wasm"));
let failed = 0;
function check(what, ok, detail = "") {
  if (ok) return;
  failed++;
  console.log(`FAIL ${what} ${detail}`);
}

// Every example: the page the browser would show is the .out file.
let examples = 0;
for (const file of readdirSync(root + "examples").filter((f) => f.endsWith(".joss")).sort()) {
  const name = file.slice(0, -5);
  const all = readFileSync(`${root}examples/${file}`, "utf8").replace(/\n$/, "").split("\n");
  const year = all.some((line) => line.trim() === "! joss 1964") ? "1964" : "1965";
  const lines = all.filter((line) => !/^! joss 196[45]$/.test(line.trim()));
  const result = replay(module, { lines, year });
  const expected = readFileSync(`${root}examples/${name}.out`, "utf8").replace(/\n$/, "");
  check(`example ${name}`, paper(result) === expected, `\n--- page\n${paper(result)}\n--- .out\n${expected}`);
  examples++;
}

// Every original session: the wasm build's report is the native build's.
let sessions = 0;
for (const file of readdirSync(root + "transcripts").filter((f) => f.endsWith(".txt")).sort()) {
  const text = readFileSync(`${root}transcripts/${file}`, "utf8");
  const report = verify(module, text);
  check(`transcript ${file}`, !report.includes("DIFFERS") && /match the page/.test(report), report);
  const native = `${root}target/release/joss`;
  if (existsSync(native)) {
    const there = execFileSync(native, ["--verify", "-"], { input: text, encoding: "utf8" }).replace(/^-\n/, "");
    check(`transcript ${file} native`, there === report);
  }
  sessions++;
}

// A sitting, a line at a time, as the page does it: an error, a correction, Go.
const sitting = new Sitting((request) => replay(module, request));
await sitting.typeAll(["1.1 Type 1/a.", "Do part 1."]);
check("error at a step", sitting.result.page.at(-1).text === "Error at step 1.1:  a is undefined." && sitting.result.suspended);
await sitting.type("Set a = 4.");
await sitting.type("Go.");
check("Go", sitting.result.page.at(-1).text === "     1/a =        .25" && !sitting.result.suspended, JSON.stringify(sitting.result.page.at(-1)));
await sitting.typeAll(["2.1 Demand x.", "2.2 Type x·2.", "Do part 2."]);
check("Demand waits", sitting.result.waiting === "value" && sitting.result.page.at(-1).asked === "       x = ");
await sitting.type("21");
check("Demand answered", sitting.result.waiting === "instruction" && sitting.result.page.at(-1).text.endsWith("42"));
check("the answer is on JOSS's line", sitting.result.page.at(-2).asked === "       x = " && sitting.result.page.at(-2).text === "21");
await sitting.typeAll(["3.1 Type i.", "Do part 3 for i = 1(1)1000."]);
await sitting.interrupt(3);
check("INTERRUPT", sitting.result.page.at(-1).text === "I'm at step 3.1." && sitting.result.suspended, sitting.result.page.at(-1).text);
// A loop that never ends: the page presses INTERRUPT for the reader.
const endless = new Sitting((request) => replay(module, request), { patience: 5000 });
await endless.typeAll(["1.1 To step 1.1.", "Do part 1."]);
check("patience", endless.result.page.at(-1).text === "I'm at step 1.1." && endless.result.suspended);
const film = new Sitting((request) => replay(module, request), { year: "1964" });
await film.type("Type 7/0.");
check("1964", film.result.page.at(-1).text === "Error above:  Zero divisor.");

// The language server, as the editor asks it.
let asked = 0;
if (existsSync(root + "web/lsp-worker.js") && existsSync(root + "web/joss-lsp.wasm")) {
  const { answer } = await import("./lsp-worker.js");
  const server = new WebAssembly.Module(readFileSync(root + "web/joss-lsp.wasm"));
  for (const file of readdirSync(root + "examples").filter((f) => f.endsWith(".joss")).sort()) {
    const text = readFileSync(`${root}examples/${file}`, "utf8");
    const reply = answer(server, text, [{ method: "textDocument/semanticTokens/full" }]);
    check(`lsp ${file}`, reply.results[0]?.data?.length > 0);
    // The sessions from the papers have their mistakes in them, on purpose.
    const meant = /^19\d\d-|mistake|error|eh/.test(file);
    const errors = reply.diagnostics.filter((d) => d.severity === 1);
    check(`lsp ${file} has no errors`, meant || errors.length === 0, JSON.stringify(errors));
    asked++;
  }
  const hover = answer(server, "1.1 Type sqrt(x).\n", [{ method: "textDocument/hover", params: { position: { line: 0, character: 10 } } }]);
  check("lsp hover", /square root/i.test(JSON.stringify(hover.results[0])));
}

console.log(`${examples} examples, ${sessions} original sessions, ${asked} files through the language server, ${failed} failed`);
process.exit(failed ? 1 : 0);
