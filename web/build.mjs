// Builds the files the playground needs:  node web/build.mjs
//   web/joss.wasm      the interpreter, compiled for WASI
//   web/joss-lsp.wasm  the language server, compiled for WASI
//   web/examples.json  the example sessions and the original transcripts
import { execSync } from "node:child_process";
import { copyFileSync, readFileSync, readdirSync, writeFileSync } from "node:fs";

const root = new URL("..", import.meta.url).pathname;
execSync("cargo build --release --target wasm32-wasip1 -p joss -p joss-lsp", { cwd: root, stdio: "inherit" });
copyFileSync(`${root}target/wasm32-wasip1/release/joss.wasm`, `${root}web/joss.wasm`);
copyFileSync(`${root}target/wasm32-wasip1/release/joss-lsp.wasm`, `${root}web/joss-lsp.wasm`);

// Shown first, in this order; the rest follow alphabetically.
const first = [
  "hello",
  "1965-quadratic-with-mistakes",
  "1965-primes",
  "1965-hypotenuse",
  "1965-root-finding",
  "1965-matrix-inversion",
  "1965-table",
  "1964-film-table",
];

const names = readdirSync(`${root}examples`).filter((f) => f.endsWith(".joss")).map((f) => f.slice(0, -5));
names.sort((a, b) => {
  const [i, j] = [first.indexOf(a), first.indexOf(b)];
  if (i !== -1 || j !== -1) return (i === -1 ? 99 : i) - (j === -1 ? 99 : j);
  return a.localeCompare(b);
});
const examples = names.map((name) => {
  const lines = readFileSync(`${root}examples/${name}.joss`, "utf8").replace(/\n$/, "").split("\n");
  // A file that begins `! joss 1964` is a sitting at the JOSS of the film.
  const year = lines.some((line) => line.trim() === "! joss 1964") ? "1964" : "1965";
  return { name, year, lines: lines.filter((line) => !/^! joss 196[45]$/.test(line.trim())) };
});
writeFileSync(`${root}web/examples.json`, JSON.stringify(examples));

// The original sessions, as transcribed: the page shows them beside its own.
const transcripts = readdirSync(`${root}transcripts`)
  .filter((f) => f.endsWith(".txt"))
  .sort()
  .map((file) => ({ name: file.slice(0, -4), text: readFileSync(`${root}transcripts/${file}`, "utf8") }));
writeFileSync(`${root}web/transcripts.json`, JSON.stringify(transcripts));
console.log(`web/joss.wasm, web/joss-lsp.wasm, web/examples.json (${examples.length} sittings) and web/transcripts.json (${transcripts.length}) written`);
