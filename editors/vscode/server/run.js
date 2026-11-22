// Runs a JOSS session:  node run.js [options] session.joss
// The file is the lines typed at the console. The interpreter is given them
// on standard input, so it needs no access to files, and it prints the page:
// each line as it was typed, and what JOSS typed in answer. Options are those
// of `joss` (--marked, --quiet, --year 1964, --patience N).
"use strict";

const fs = require("fs");
const path = require("path");
const { run } = require("./wasi.js");

const args = process.argv.slice(2);
const file = args.pop();
if (!file || file.startsWith("--")) {
  console.error("usage: run.js [options] session.joss");
  process.exit(2);
}
let lines;
try {
  lines = fs.readFileSync(file);
} catch (error) {
  console.error(`joss: cannot read ${file}: ${error.message}`);
  process.exit(2);
}
process.exitCode = run(path.join(__dirname, "joss.wasm"), { args, prefix: lines, keyboard: false });
