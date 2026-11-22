// Runs inside VS Code. Exercises the extension the way a person would: open
// a program, hover, jump, rename, see problems, and run it.
"use strict";

const assert = require("node:assert/strict");
const path = require("node:path");
const vscode = require("vscode");

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function until(what, probe, seconds = 20) {
  for (let i = 0; i < seconds * 10; i++) {
    const value = await probe();
    if (value) return value;
    await sleep(100);
  }
  throw new Error(`timed out waiting for ${what}`);
}

const hoverText = (hovers) =>
  hovers.flatMap((h) => h.contents).map((c) => (typeof c === "string" ? c : c.value)).join("\n");

exports.run = async function run() {
  const folder = vscode.workspace.workspaceFolders[0].uri.fsPath;
  const document = await vscode.workspace.openTextDocument(path.join(folder, "1965-hypotenuse.joss"));
  await vscode.window.showTextDocument(document);
  assert.equal(document.languageId, "joss", "the .joss extension selects the language");

  const part = new vscode.Position(7, 12); // the 1 of "part 1"
  assert.equal(document.lineAt(7).text, "2.1 Do part 1 for b=1(1)a.");
  assert.equal(document.lineAt(3).text, "1.1 Set c=sqrt(a*2+b*2).");

  // The server is ready when it answers a hover.
  const hovers = await until("a hover from the language server", async () => {
    const found = await vscode.commands.executeCommand("vscode.executeHoverProvider", document.uri, part);
    return found.length ? found : undefined;
  });
  assert.match(hoverText(hovers), /Part 1/);
  assert.match(hoverText(hovers), /1\.1 Set c=sqrt\(a\*2\+b\*2\)\./);
  const word = await vscode.commands.executeCommand("vscode.executeHoverProvider", document.uri, new vscode.Position(3, 5));
  assert.match(hoverText(word), /`Set`.*gives a letter a value/);
  const power = await vscode.commands.executeCommand("vscode.executeHoverProvider", document.uri, new vscode.Position(3, 16));
  assert.match(hoverText(power), /`\*`.*to the power/);
  const field = await vscode.commands.executeCommand("vscode.executeHoverProvider", document.uri, new vscode.Position(6, 30));
  assert.match(hoverText(field), /Fixed blank.*10 columns wide, 6 decimals/);
  console.log("hover ok");

  const definitions = await vscode.commands.executeCommand("vscode.executeDefinitionProvider", document.uri, part);
  assert.equal(definitions.length, 1);
  assert.equal((definitions[0].range || definitions[0].targetRange).start.line, 3);
  console.log("definition ok");

  const references = await vscode.commands.executeCommand("vscode.executeReferenceProvider", document.uri, part);
  assert.equal(references.length, 4, "three lines give steps of part 1, and one Do names it");

  const letter = new vscode.Position(3, 8); // the c of "Set c"
  const edit = await vscode.commands.executeCommand("vscode.executeDocumentRenameProvider", document.uri, letter, "h");
  assert.equal(edit.entries()[0][1].length, 4, "renaming a letter changes every place it is written");
  await assert.rejects(
    vscode.commands.executeCommand("vscode.executeDocumentRenameProvider", document.uri, letter, "a"),
    /the letter a is already used/
  );
  await assert.rejects(
    vscode.commands.executeCommand("vscode.executeDocumentRenameProvider", document.uri, letter, "side"),
    /not a JOSS name/
  );
  console.log("references and rename ok");

  const symbols = await vscode.commands.executeCommand("vscode.executeDocumentSymbolProvider", document.uri);
  assert.deepEqual(symbols.map((s) => s.name), ["Part 1", "Form 1", "Part 2"]);
  assert.deepEqual(symbols[0].children.map((s) => s.name), ["Step 1.1", "Step 1.2"]);

  const legend = await vscode.commands.executeCommand("vscode.provideDocumentSemanticTokensLegend", document.uri);
  assert.ok(legend.tokenTypes.includes("label"));
  const tokens = await vscode.commands.executeCommand("vscode.provideDocumentSemanticTokens", document.uri);
  assert.ok(tokens.data.length >= 5 * 40);
  console.log("symbols and semantic tokens ok");

  const completions = await vscode.commands.executeCommand("vscode.executeCompletionItemProvider", document.uri, new vscode.Position(12, 3));
  const labels = completions.items.map((i) => (typeof i.label === "string" ? i.label : i.label.label));
  for (const wanted of ["step", "part"]) assert.ok(labels.includes(wanted), wanted);

  const help = await vscode.commands.executeCommand("vscode.executeSignatureHelpProvider", document.uri, new vscode.Position(3, 15), "(");
  assert.equal(help.signatures[0].label, "sqrt(x)");
  console.log("completion and signature help ok");

  // Step 1.2 is typed twice in this session; the first is shown as replaced.
  assert.deepEqual(vscode.languages.getDiagnostics(document.uri).map((p) => p.code), ["replaced"]);

  // A file with mistakes shows them.
  const broken = await vscode.workspace.openTextDocument({ language: "joss", content: "Type 2+2+.\nDo part 3.\nSet x = 2@3 if 1 <= 2.\n" });
  await vscode.window.showTextDocument(broken);
  const problems = await until("diagnostics", () => {
    const found = vscode.languages.getDiagnostics(broken.uri);
    return found.length ? found : undefined;
  });
  assert.deepEqual(problems.map((p) => p.code), ["eh", "missing"]);
  assert.equal(problems[0].severity, vscode.DiagnosticSeverity.Error);
  assert.equal(problems[0].source, "joss");
  assert.equal(problems[0].range.start.character, 9);
  console.log("diagnostics ok");

  // The formatter writes the typewriter's characters for the stand-ins.
  const edits = await vscode.commands.executeCommand("vscode.executeFormatDocumentProvider", broken.uri, { tabSize: 4, insertSpaces: true });
  assert.deepEqual(edits.map((e) => e.newText), ["·", "≤"]);
  console.log("formatting ok");

  // Run Session: a task that ends with exit code 0.
  await vscode.window.showTextDocument(document);
  const finished = new Promise((resolve) => {
    const listener = vscode.tasks.onDidEndTaskProcess((event) => {
      if (event.execution.task.definition.type === "joss") {
        listener.dispose();
        resolve(event.exitCode);
      }
    });
  });
  await vscode.commands.executeCommand("joss.run");
  assert.equal(await Promise.race([finished, sleep(20000).then(() => "timed out")]), 0);
  console.log("run ok");
};
