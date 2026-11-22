# JOSS for Visual Studio Code

Editor support for [JOSS](https://github.com/chrishayuk/joss), the JOHNNIAC Open-Shop System: the conversational language that RAND ran on the JOHNNIAC from 1963 to 1965, here as it stood in January 1965. A `.joss` file is the lines typed at a JOSS console, and the editor reads them as JOSS would.

Nothing else needs installing: the language server and the interpreter are built into the extension as WebAssembly.

## What you get

- **Syntax highlighting** for step numbers, verbs, nouns, `if` and `for`, functions, letters, numbers, text in quotes and notes (lines that begin or end with `*`). The line after `Form n:` is shown as a picture, with its blanks picked out.
- **Hover** on any word, function, operator or special character to see what it does, with an example. Hover on `part 1`, `step 1.2` or `form 1` to see the steps or the form. Hover on a letter to see where it is given a value, and on a blank of a form to see its width and decimals.
- **Problems as you type:** a line that is not JOSS (from the column where it stops being JOSS), a `Do` of a part with no steps, a step or form that is not there, a form with too few blanks for its values, blanks that run together, a step that a later line replaces, an instruction that cannot be a step or cannot be given directly, and a letter that is never given a value.
- **Go to definition** and **find all references** for steps, parts, forms and letters.
- **Rename** a letter. JOSS has 52 names, a to z and A to Z, so the new name must be one letter that is not in use.
- **Completion:** verbs at the start of a line, nouns after `Do`, `Type`, `Delete` and `To`, the numbers of the steps, parts and forms that exist, and letters and functions in an expression.
- **Signature help** inside the brackets of a function, an **outline** of the parts with their steps and of the forms, and **folding** for parts, forms and runs of notes.
- **The characters `·` `≠` `≤` `≥`.** Type them with Alt+8, Alt+=, Alt+, and Alt+. (Option on a Mac). Or type the stand-ins `@`, `!=`, `<=`, `>=` and accept the suggestion, or run **Format Document** to change every stand-in in the file. JOSS accepts both.
- **Run Session:** the play button at the top of the editor, or Cmd+Enter (Ctrl+Enter on Windows and Linux). The lines of the file are typed at a JOSS console, and the page appears in a terminal: each line as it was typed, and what JOSS typed in answer.

A file may also hold lines that are not typed but stand for a key or a setting: `! joss 1964` for the wording of the 1964 film, `! interrupt after 4 lines` for the INTERRUPT key, `! users 6` and `! time 1230`.

## Settings

| Setting | What it does |
| --- | --- |
| `joss.server.path` | Use a `joss-lsp` program you built yourself as the language server. |
| `joss.interpreter.path` | Run sessions with a `joss` program you built yourself. |
| `joss.trace.server` | Log the messages exchanged with the language server. |

## Building it

From this folder, with Rust (and its `wasm32-wasip1` target) and Node installed:

```sh
npm install
npm run build        # compiles the server and interpreter to WebAssembly, bundles the extension
npm test             # grammar, language server and runner tests
npm run test:vscode  # opens a real VS Code and drives the extension
npm run package      # writes joss-language-0.1.2.vsix
code --install-extension joss-language-0.1.2.vsix
```

To try it without installing, open this folder in VS Code and press F5.

## How it fits together

| File | What it is |
| --- | --- |
| `syntaxes/joss.tmLanguage.json` | The highlighting grammar. |
| `language-configuration.json` | The note character, brackets and auto-closing pairs. |
| `snippets/joss.code-snippets` | `times`, `ne`, `le` and `ge` for the four special characters. |
| `src/extension.js` | Starts the language server and provides Run Session. |
| `server/lsp.js`, `server/joss-lsp.wasm` | The language server from `../../lsp`, compiled for WASI. |
| `server/run.js`, `server/joss.wasm` | The interpreter, compiled for WASI. |
| `server/wasi.js` | A small WASI host for Node, with no dependencies. |

## Licence

Apache License 2.0. See [LICENSE](LICENSE).
