// The JOSS extension: starts the language server and adds a command that
// replays the lines in the editor at a JOSS console and shows the page.
"use strict";

const vscode = require("vscode");
const fs = require("fs");
const os = require("os");
const path = require("path");
const { LanguageClient, TransportKind } = require("vscode-languageclient/node");

/** @type {LanguageClient | undefined} */
let client;

function setting(name) {
  return (vscode.workspace.getConfiguration("joss").get(name) || "").trim();
}

async function startServer(context) {
  const custom = setting("server.path");
  // Built in: the server compiled to WebAssembly, run by the editor's own
  // Node, so nothing has to be installed.
  const builtIn = { module: context.asAbsolutePath(path.join("server", "lsp.js")), transport: TransportKind.stdio };
  const serverOptions = custom ? { command: custom, args: [] } : { run: builtIn, debug: builtIn };
  client = new LanguageClient("joss", "JOSS", serverOptions, {
    documentSelector: [{ language: "joss" }],
  });
  await client.start();
}

async function restartServer(context) {
  if (client) await client.stop().catch(() => {});
  client = undefined;
  await startServer(context);
}

async function runProgram(context) {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== "joss") {
    vscode.window.showInformationMessage("Open a JOSS file to run it.");
    return;
  }
  const document = editor.document;
  let file = document.uri.fsPath;
  if (document.isUntitled || document.uri.scheme !== "file") {
    // Not on disk yet: run a copy.
    file = path.join(os.tmpdir(), "untitled.joss");
    fs.writeFileSync(file, document.getText());
  } else if (document.isDirty) {
    await document.save();
  }

  const custom = setting("interpreter.path");
  const execution = custom
    ? new vscode.ProcessExecution(custom, [file])
    : new vscode.ProcessExecution(process.execPath, [context.asAbsolutePath(path.join("server", "run.js")), file], {
        env: { ELECTRON_RUN_AS_NODE: "1" },
      });
  const task = new vscode.Task(
    { type: "joss" },
    vscode.TaskScope.Workspace,
    `Run ${path.basename(file)}`,
    "joss",
    execution,
    []
  );
  task.presentationOptions = {
    reveal: vscode.TaskRevealKind.Always,
    panel: vscode.TaskPanelKind.Dedicated,
    focus: true,
    clear: true,
    // Without this the terminal first prints the whole command line.
    echo: false,
    showReuseMessage: false,
  };
  await vscode.tasks.executeTask(task);
}

async function activate(context) {
  context.subscriptions.push(
    vscode.commands.registerCommand("joss.run", () => runProgram(context)),
    vscode.commands.registerCommand("joss.restartServer", () => restartServer(context)),
    // The server's formatter does one thing: it writes the characters of the
    // JOSS typewriter where a stand-in was typed.
    vscode.commands.registerCommand("joss.realCharacters", () => vscode.commands.executeCommand("editor.action.formatDocument")),
    vscode.workspace.onDidChangeConfiguration((event) => {
      if (event.affectsConfiguration("joss.server.path")) restartServer(context);
    })
  );
  await startServer(context);
}

function deactivate() {
  return client ? client.stop() : undefined;
}

module.exports = { activate, deactivate };
