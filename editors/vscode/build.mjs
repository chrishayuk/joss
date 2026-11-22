// Builds everything the extension ships:  node build.mjs
//   server/joss-lsp.wasm  the language server, compiled for WASI
//   server/joss.wasm      the interpreter, compiled for WASI
//   dist/extension.js      the extension and its one dependency, bundled
import { execSync } from "node:child_process";
import { copyFileSync } from "node:fs";
import { build } from "esbuild";

const here = new URL(".", import.meta.url).pathname;
const root = new URL("../..", import.meta.url).pathname;
const release = `${root}target/wasm32-wasip1/release`;

execSync("cargo build --release --target wasm32-wasip1 -p joss -p joss-lsp", { cwd: root, stdio: "inherit" });
copyFileSync(`${release}/joss-lsp.wasm`, `${here}server/joss-lsp.wasm`);
copyFileSync(`${release}/joss.wasm`, `${here}server/joss.wasm`);

await build({
  entryPoints: [`${here}src/extension.js`],
  outfile: `${here}dist/extension.js`,
  bundle: true,
  platform: "node",
  format: "cjs",
  target: "node18",
  external: ["vscode"],
  minify: true,
  logLevel: "warning",
});
console.log("server/joss-lsp.wasm, server/joss.wasm and dist/extension.js written");
