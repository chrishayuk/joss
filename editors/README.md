# JOSS in your editor

Two pieces give an editor an understanding of JOSS:

- **`joss-lsp`**, a language server in [`../lsp`](../lsp). It works with any editor that speaks the Language Server Protocol.
- **A Visual Studio Code extension** in [`vscode`](vscode), which bundles the server and adds syntax highlighting and a run button. Its [README](vscode/README.md) lists the features.

Both are built on [`src/analysis.rs`](../src/analysis.rs), the same code behind `joss --check`. It asks the interpreter's own lexer and parser, so the editor and JOSS cannot disagree about what is JOSS. JOSS itself said only "Eh?" about a line it could not read. The editor says where.

A `.joss` file is the lines typed at a JOSS console, one after another: steps (`1.1 Type x.`), direct instructions (`Do part 1.`), the heading of a form and the form on the next line, notes (lines that begin or end with `*`), and the answers to a `Demand`.

## The language server

```sh
cargo install --git https://github.com/chrishayuk/joss joss-lsp
```

or, in a clone of this repository, `cargo build --release -p joss-lsp`.

It talks on standard input and output and takes no arguments. It provides diagnostics, hover, go to definition, references, document highlights, rename, completion, signature help, document symbols, folding ranges, semantic tokens, formatting and code actions. Semantic tokens mean an editor with no JOSS grammar still gets highlighting.

Formatting does one thing. It writes the characters of the JOSS typewriter, `·` `≠` `≤` `≥`, where their stand-ins `@` `!=` `<=` `>=` were typed. Text in quotes, notes and forms are left alone. The code actions do the same for one stand-in or for all of them.

### Neovim (0.11 or later)

```lua
vim.filetype.add({ extension = { joss = "joss" } })
vim.lsp.config("joss", {
  cmd = { "/path/to/joss-lsp" },
  filetypes = { "joss" },
})
vim.lsp.enable("joss")
```

### Helix

In `languages.toml`:

```toml
[language-server.joss-lsp]
command = "/path/to/joss-lsp"

[[language]]
name = "joss"
scope = "source.joss"
file-types = ["joss"]
comment-token = "*"
language-servers = ["joss-lsp"]
```

### Any other LSP client

Start `joss-lsp` with no arguments for files that end in `.joss`, with the language id `joss`. The server wants the whole text on every change (full document sync) and counts positions in UTF-16 code units.

These configurations follow each editor's documented format but have not been run here; the server itself is tested over standard input and output by `vscode/test/lsp.test.mjs`.

## Typing `·` `≠` `≤` `≥`

The JOSS typewriter had keys for these. JOSS here accepts `@`, `!=`, `<=` and `>=` in their place, and always types the real character back. On a Mac keyboard they are Option+Shift+9, Option+=, Option+, and Option+. The star `*` is not times: it is "to the power".

## Checking a file from the command line

```sh
joss --check session.joss
```

prints one line per problem, as `file:line:column: severity: message`, and exits with status 1 if any is an error.
