// Tokenises JOSS with the same TextMate engine VS Code uses and checks the
// scopes the grammar assigns.
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { createRequire } from "node:module";
import test from "node:test";
import oniguruma from "vscode-oniguruma";
import textmate from "vscode-textmate";

const require = createRequire(import.meta.url);
const grammarPath = new URL("../syntaxes/joss.tmLanguage.json", import.meta.url).pathname;

const wasm = readFileSync(require.resolve("vscode-oniguruma/release/onig.wasm"));
await oniguruma.loadWASM(wasm.buffer.slice(wasm.byteOffset, wasm.byteOffset + wasm.byteLength));
const registry = new textmate.Registry({
  onigLib: Promise.resolve({
    createOnigScanner: (patterns) => new oniguruma.OnigScanner(patterns),
    createOnigString: (text) => new oniguruma.OnigString(text),
  }),
  loadGrammar: async () => textmate.parseRawGrammar(readFileSync(grammarPath, "utf8"), grammarPath),
});
const grammar = await registry.loadGrammar("source.joss");

/** Each non-blank piece of text with its most specific scope. */
function scopes(source) {
  const out = [];
  let state = textmate.INITIAL;
  for (const line of source.split("\n")) {
    const result = grammar.tokenizeLine(line, state);
    for (const token of result.tokens) {
      const text = line.slice(token.startIndex, token.endIndex);
      if (text.trim()) out.push([text, token.scopes.at(-1)]);
    }
    state = result.ruleStack;
  }
  return out;
}

test("a step: its number, verb, letters, functions, operators and the condition", () => {
  assert.deepEqual(scopes("1.1 Set c=sqrt(a*2+b*2) if a ≠ 0 and b >= 2@a."), [
    ["1.1", "entity.name.section.step.joss"],
    ["Set", "keyword.control.joss"],
    ["c", "variable.other.letter.joss"],
    ["=", "keyword.operator.equals.joss"],
    ["sqrt", "support.function.joss"],
    ["(", "punctuation.section.brackets.joss"],
    ["a", "variable.other.letter.joss"],
    ["*", "keyword.operator.power.joss"],
    ["2", "constant.numeric.joss"],
    ["+", "keyword.operator.arithmetic.joss"],
    ["b", "variable.other.letter.joss"],
    ["*", "keyword.operator.power.joss"],
    ["2", "constant.numeric.joss"],
    [")", "punctuation.section.brackets.joss"],
    ["if", "keyword.control.conditional.joss"],
    ["a", "variable.other.letter.joss"],
    ["≠", "keyword.operator.comparison.joss"],
    ["0", "constant.numeric.joss"],
    ["and", "keyword.operator.logical.joss"],
    ["b", "variable.other.letter.joss"],
    [">=", "keyword.operator.comparison.joss"],
    ["2", "constant.numeric.joss"],
    ["@", "keyword.operator.times.joss"],
    ["a", "variable.other.letter.joss"],
    [".", "punctuation.terminator.joss"],
  ]);
});

test("direct instructions: nouns, the numbers they name, for, and text", () => {
  assert.deepEqual(scopes('Do part 2 for x = .5(.5)3, 4.\nType "a*b = c".\nDelete all values.\nDo step 1.1.'), [
    ["Do", "keyword.control.joss"],
    ["part", "storage.type.noun.joss"],
    ["2", "entity.name.section.reference.joss"],
    ["for", "keyword.control.loop.joss"],
    ["x", "variable.other.letter.joss"],
    ["=", "keyword.operator.equals.joss"],
    [".5", "constant.numeric.joss"],
    ["(", "punctuation.section.brackets.joss"],
    [".5", "constant.numeric.joss"],
    [")", "punctuation.section.brackets.joss"],
    ["3", "constant.numeric.joss"],
    [",", "punctuation.separator.comma.joss"],
    ["4", "constant.numeric.joss"],
    [".", "punctuation.terminator.joss"],
    ["Type", "keyword.control.joss"],
    ['"', "punctuation.definition.string.begin.joss"],
    ["a*b = c", "string.quoted.double.joss"],
    ['"', "punctuation.definition.string.end.joss"],
    [".", "punctuation.terminator.joss"],
    ["Delete", "keyword.control.joss"],
    ["all", "storage.type.noun.joss"],
    ["values", "storage.type.noun.joss"],
    [".", "punctuation.terminator.joss"],
    ["Do", "keyword.control.joss"],
    ["step", "storage.type.noun.joss"],
    ["1.1", "entity.name.section.reference.joss"],
    [".", "punctuation.terminator.joss"],
  ]);
});

test("a star at either end kills the line, and elsewhere it is a power", () => {
  assert.deepEqual(scopes("*  PRIME NUMBERS\n  * indented\nDo part 12 for *\nType x*2."), [
    ["*", "punctuation.definition.comment.joss"],
    ["  PRIME NUMBERS", "comment.line.asterisk.joss"],
    ["*", "punctuation.definition.comment.joss"],
    [" indented", "comment.line.asterisk.joss"],
    ["Do part 12 for ", "comment.line.asterisk.joss"],
    ["*", "punctuation.definition.comment.joss"],
    ["Type", "keyword.control.joss"],
    ["x", "variable.other.letter.joss"],
    ["*", "keyword.operator.power.joss"],
    ["2", "constant.numeric.joss"],
    [".", "punctuation.terminator.joss"],
  ]);
});

test("the line after Form n: is a picture, with its blanks", () => {
  assert.deepEqual(scopes("Form 1:\na = __   c = -___.______  e = ........... *\nType a, c, e in form 1."), [
    ["Form", "keyword.control.joss"],
    ["1", "entity.name.type.form.joss"],
    [":", "punctuation.separator.form.joss"],
    ["a = ", "string.unquoted.form.joss"],
    ["__", "variable.parameter.field.fixed.joss"],
    ["   c = ", "string.unquoted.form.joss"],
    ["-___.______", "variable.parameter.field.fixed.joss"],
    ["  e = ", "string.unquoted.form.joss"],
    ["...........", "variable.parameter.field.scientific.joss"],
    [" *", "string.unquoted.form.joss"],
    ["Type", "keyword.control.joss"],
    ["a", "variable.other.letter.joss"],
    [",", "punctuation.separator.comma.joss"],
    ["c", "variable.other.letter.joss"],
    [",", "punctuation.separator.comma.joss"],
    ["e", "variable.other.letter.joss"],
    ["in", "keyword.other.in.joss"],
    ["form", "storage.type.noun.joss"],
    ["1", "entity.name.section.reference.joss"],
    [".", "punctuation.terminator.joss"],
  ]);
});

test("a word that is not JOSS, a blank line in a list, the line number and a key pressed", () => {
  assert.deepEqual(scopes("Let x = |y|.\nType a, _, $.\n! interrupt after 4 lines"), [
    ["Let", "invalid.illegal.word.joss"],
    ["x", "variable.other.letter.joss"],
    ["=", "keyword.operator.equals.joss"],
    ["|", "keyword.operator.absolute.joss"],
    ["y", "variable.other.letter.joss"],
    ["|", "keyword.operator.absolute.joss"],
    [".", "punctuation.terminator.joss"],
    ["Type", "keyword.control.joss"],
    ["a", "variable.other.letter.joss"],
    [",", "punctuation.separator.comma.joss"],
    ["_", "constant.language.blank.joss"],
    [",", "punctuation.separator.comma.joss"],
    ["$", "variable.language.line.joss"],
    [".", "punctuation.terminator.joss"],
    ["!", "punctuation.definition.directive.joss"],
    ["interrupt after 4 lines", "keyword.control.directive.joss"],
  ]);
});

// The grammar's lists of words are the interpreter's.
test("the grammar knows exactly the words in src/lex.rs", () => {
  const lex = readFileSync(new URL("../../../src/lex.rs", import.meta.url), "utf8");
  const words = (name) => [...new RegExp(`pub const ${name}: &\\[&str\\] = &\\[([^\\]]*)\\]`).exec(lex)[1].matchAll(/"(\w+)"/g)].map((m) => m[1]);
  const expected = { VERBS: "keyword.control.joss", FUNCTIONS: "support.function.joss", NOUNS: "storage.type.noun.joss" };
  for (const [list, scope] of Object.entries(expected)) {
    for (const word of words(list)) assert.deepEqual(scopes(word), [[word, scope]], word);
  }
  for (const word of words("CONNECTIVES")) assert.match(scopes(word)[0][1], /^keyword\./, word);
});

test("every example tokenises with no word that is not JOSS and no text left unscoped", () => {
  const examples = new URL("../../../examples/", import.meta.url).pathname;
  const files = readdirSync(examples).filter((f) => f.endsWith(".joss"));
  assert.ok(files.length >= 20);
  for (const file of files) {
    const lines = readFileSync(examples + file, "utf8").split("\n");
    for (const [text, scope] of scopes(lines.join("\n"))) {
      assert.notEqual(scope, "source.joss", `${file}: unscoped "${text}"`);
      assert.notEqual(scope, "invalid.illegal.word.joss", `${file}: ${text}`);
    }
  }
});
