// Runs every example through the runner the "Run Session" command uses, and
// compares the page it prints with the expected one.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";

const runner = new URL("../server/run.js", import.meta.url).pathname;
const examples = new URL("../../../examples/", import.meta.url).pathname;

function run(text, options = [], name = "session.joss") {
  const file = join(mkdtempSync(join(tmpdir(), "joss-")), name);
  writeFileSync(file, text);
  return spawnSync(process.execPath, [runner, ...options, file]);
}

test("the built-in interpreter replays every example", () => {
  const names = readdirSync(examples).filter((f) => f.endsWith(".joss"));
  assert.ok(names.length >= 20);
  for (const file of names) {
    const result = spawnSync(process.execPath, [runner, examples + file]);
    assert.equal(result.status, 0, `${file}: ${result.stderr}`);
    assert.equal(result.stdout.toString(), readFileSync(`${examples}${file.slice(0, -5)}.out`, "utf8"), file);
  }
});

test("the page has the user's lines, with the typewriter's characters, and JOSS's answers", () => {
  const result = run("Set x = 3.\nType 2@x, x*2.\nType x if x != 0.\n");
  assert.equal(result.status, 0, result.stderr.toString());
  assert.equal(
    result.stdout.toString(),
    "Set x = 3.\nType 2·x, x*2.\n     2·x =       6\n     x*2 =       9\nType x if x ≠ 0.\n       x =       3\n"
  );
});

test("JOSS says what is wrong, on the page, and the replay goes on", () => {
  const result = run("Type 2+2+.\nType 1/0.\n1.1 Type sqrt(-1).\nDo part 1.\nType 2+2.\n");
  assert.equal(result.status, 0);
  assert.deepEqual(result.stdout.toString().split("\n").filter((line) => /^(Eh|Error)/.test(line)), [
    "Eh?",
    "Error above:  I have a zero divisor.",
    "Error at step 1.1:  I have a negative argument for sqrt.",
  ]);
  assert.match(result.stdout.toString(), / {5}2\+2 = {7}4\n$/);
});

test("the answers to a Demand are lines of the file", () => {
  const result = run("1.1 Demand x.\n1.2 Type x·2.\nDo part 1.\n21\n", ["--quiet"]);
  assert.equal(result.stdout.toString(), "       x = \n     x·2 =      42\n");
});

test("options go through to the interpreter", () => {
  const marked = run("Type 7/0.\n", ["--marked", "--year", "1964"]);
  assert.equal(marked.stdout.toString(), "U: Type 7/0.\nJ: Error above:  Zero divisor.\n");
});

test("a file that is not there is reported", () => {
  const result = spawnSync(process.execPath, [runner, "/no/such/session.joss"]);
  assert.equal(result.status, 2);
  assert.match(result.stderr.toString(), /cannot read/);
});
