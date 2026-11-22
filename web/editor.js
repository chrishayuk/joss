// Turns the program box into an editor that understands JOSS: colours,
// problems as you type, hover help and completion, all from the language
// server in lsp-worker.js.
//
// The editor itself is CodeMirror, loaded from a CDN because this page has
// no build step. If that cannot be loaded (no network, say), upgrade()
// resolves to null and the plain text box carries on working.

const CDN = "https://esm.sh/";
const PIN = "?deps=@codemirror/state@6.7.6,@codemirror/view@6.43.15";
// The order of the server's semantic token legend.
const TOKEN_TYPES = ["comment", "string", "number", "variable", "function", "parameter", "operator", "keyword", "type", "label"];

let worker = null;
let nextId = 0;
const waiting = new Map();

/** Opens `text` in the language server and asks it the given requests. */
function ask(text, requests) {
  if (!worker) {
    worker = new Worker(new URL("lsp-worker.js", import.meta.url), { type: "module" });
    worker.onmessage = ({ data }) => {
      waiting.get(data.id)?.(data.error ? null : data);
      waiting.delete(data.id);
    };
    worker.onerror = () => {
      for (const resolve of waiting.values()) resolve(null);
      waiting.clear();
    };
  }
  const id = ++nextId;
  return new Promise((resolve) => {
    waiting.set(id, resolve);
    worker.postMessage({ id, wasmUrl: new URL("joss-lsp.wasm", import.meta.url).href, text, requests });
  });
}

const offsetOf = (doc, position) => {
  const line = doc.line(Math.min(position.line + 1, doc.lines));
  return Math.min(line.from + position.character, line.to);
};
const positionOf = (doc, offset) => {
  const line = doc.lineAt(offset);
  return { line: line.number - 1, character: offset - line.from };
};

/** The server's small Markdown: paragraphs, bold, code and code blocks. */
function render(markdown) {
  const element = (tag, text) => {
    const made = document.createElement(tag);
    if (text !== undefined) made.textContent = text;
    return made;
  };
  const box = element("div");
  box.className = "joss-hover";
  const fill = (parent, text) => {
    // Code may hold a star (`x*2`), so it is looked for inside bold too.
    for (const piece of text.split(/(\*\*(?:`[^`]+`|[^*`])+\*\*|`[^`]+`)/)) {
      if (piece.startsWith("**") && piece.length > 4) fill(parent.appendChild(element("strong")), piece.slice(2, -2));
      else if (piece.startsWith("`") && piece.length > 2) parent.appendChild(element("code", piece.slice(1, -1)));
      else if (piece) parent.appendChild(document.createTextNode(piece));
    }
  };
  for (const block of markdown.split(/\n{2,}/)) {
    const fence = /^```\w*\n([\s\S]*?)\n?```$/.exec(block.trim());
    if (fence) box.appendChild(element("pre", fence[1]));
    else fill(box.appendChild(element("p")), block);
  }
  return box;
}

/**
 * Replaces a <textarea> with the editor. The textarea stays in the page,
 * hidden, and always holds the current lines.
 *
 * @param {HTMLTextAreaElement} textarea
 * @param {{onRun?: () => void}} [options]  onRun is called for Cmd/Ctrl+Enter
 * @returns {Promise<(EditorView & {set(text: string): void}) | null>} the
 *          CodeMirror view, with a set() that replaces the whole text; or
 *          null if the editor could not be loaded
 */
export async function upgrade(textarea, { onRun } = {}) {
  let cm;
  try {
    const [state, view, commands, lint, autocomplete] = await Promise.all([
      import(`${CDN}@codemirror/state@6.7.6`),
      import(`${CDN}@codemirror/view@6.43.15${PIN}`),
      import(`${CDN}@codemirror/commands@6.11.1${PIN}`),
      import(`${CDN}@codemirror/lint@6.9.8${PIN}`),
      import(`${CDN}@codemirror/autocomplete@6.20.4${PIN}`),
    ]);
    cm = { ...state, ...view, ...commands, ...lint, ...autocomplete };
  } catch {
    return null;
  }
  const { EditorState, StateEffect, StateField, Decoration, EditorView, hoverTooltip, keymap } = cm;

  const setTokens = StateEffect.define();
  const tokens = StateField.define({
    create: () => Decoration.none,
    update(value, transaction) {
      for (const effect of transaction.effects) if (effect.is(setTokens)) return effect.value;
      return value.map(transaction.changes);
    },
    provide: (field) => EditorView.decorations.from(field),
  });
  // Modifier 1 of the legend marks where a step is typed or a letter is given
  // a value, which gets a class of its own.
  const marks = TOKEN_TYPES.map((type) => Decoration.mark({ class: `joss-token-${type}` }));
  const defining = TOKEN_TYPES.map((type) => Decoration.mark({ class: `joss-token-${type} joss-token-definition` }));

  // The server's five-numbers-per-token list, as editor decorations.
  const decorate = (doc, data) => {
    const ranges = [];
    let line = 0;
    let character = 0;
    for (let i = 0; i + 4 < data.length; i += 5) {
      line += data[i];
      character = data[i] ? data[i + 1] : character + data[i + 1];
      if (line >= doc.lines) break;
      const from = offsetOf(doc, { line, character });
      const to = Math.min(from + data[i + 2], doc.line(line + 1).to);
      const mark = (data[i + 4] & 1 ? defining : marks)[data[i + 3]];
      if (to > from && mark) ranges.push(mark.range(from, to));
    }
    return Decoration.set(ranges, true);
  };

  const severities = ["error", "error", "warning", "info", "hint"];
  const problems = cm.linter(async (editor) => {
    const doc = editor.state.doc;
    const answer = await ask(doc.toString(), [{ method: "textDocument/semanticTokens/full" }]);
    if (!answer || editor.state.doc !== doc) return [];
    editor.dispatch({ effects: setTokens.of(decorate(doc, answer.results[0]?.data || [])) });
    return answer.diagnostics.map((problem) => {
      const from = offsetOf(doc, problem.range.start);
      return { from, to: Math.max(offsetOf(doc, problem.range.end), from), severity: severities[problem.severity] || "info", message: problem.message, source: "joss" };
    });
  }, { delay: 120 });

  const hover = hoverTooltip(async (editor, offset) => {
    const doc = editor.state.doc;
    const answer = await ask(doc.toString(), [{ method: "textDocument/hover", params: { position: positionOf(doc, offset) } }]);
    const found = answer?.results[0];
    if (!found || editor.state.doc !== doc) return null;
    return { pos: offsetOf(doc, found.range.start), end: offsetOf(doc, found.range.end), above: true, create: () => ({ dom: render(found.contents.value) }) };
  }, { hoverTime: 200 });

  // Verbs at the start of a line, nouns after them, letters and functions
  // in an expression: offered when asked for, once two letters of a word are
  // typed, or when a stand-in (`@`, `!=`, `<=`, `>=`) has just been typed and
  // the real character can take its place.
  const kinds = { 3: "function", 6: "variable", 14: "keyword", 15: "text", 18: "constant", 24: "keyword" };
  const complete = async (context) => {
    const word = context.matchBefore(/[A-Za-z]+/);
    const standIn = context.matchBefore(/@|[!<>]=/);
    if (!context.explicit && !standIn && !(word && word.text.length >= 2)) return null;
    const doc = context.state.doc;
    const answer = await ask(doc.toString(), [{ method: "textDocument/completion", params: { position: positionOf(doc, context.pos) } }]);
    const items = answer?.results[0] || [];
    if (!items.length) return null;
    // The server says where a stand-in begins; a word begins where it was matched.
    const edit = items[0].textEdit;
    return {
      from: edit ? offsetOf(doc, edit.range.start) : word ? word.from : context.pos,
      filter: !edit,
      validFor: edit ? undefined : /^[A-Za-z]*$/,
      options: items.map((item, n) => ({
        label: item.label,
        detail: item.detail,
        info: item.documentation || undefined,
        type: kinds[item.kind] || "text",
        // The server lists the likeliest first; keep its order among equal matches.
        boost: -n / items.length,
        // The server writes snippets the way editors expect; CodeMirror's form differs slightly.
        apply: item.insertTextFormat === 2 && item.insertText
          ? cm.snippet(item.insertText.replace(/\$0/g, "${}").replace(/\\\$/g, "$"))
          : item.textEdit?.newText || item.insertText || item.label,
      })),
    };
  };

  // The four characters of the JOSS typewriter that a keyboard lacks.
  const strike = (text) => (view) => {
    view.dispatch(view.state.replaceSelection(text), { scrollIntoView: true, userEvent: "input.type" });
    return true;
  };
  const characters = [
    { key: "Alt-8", run: strike("\u00b7") },
    { key: "Alt-=", run: strike("\u2260") },
    { key: "Alt-,", run: strike("\u2264") },
    { key: "Alt-.", run: strike("\u2265") },
  ];

  // Colours for the server's token types. A page may override them:
  // the classes are joss-token-comment, joss-token-keyword and so on.
  const colours = EditorView.baseTheme({
    ".joss-token-comment": { color: "#6a737d", fontStyle: "italic" },
    ".joss-token-string": { color: "#22863a" },
    ".joss-token-number": { color: "#005cc5" },
    ".joss-token-variable": { color: "#24292e" },
    ".joss-token-function": { color: "#6f42c1" },
    ".joss-token-parameter": { color: "#e36209", backgroundColor: "rgba(227, 98, 9, .08)" },
    ".joss-token-operator": { color: "#d73a49" },
    ".joss-token-keyword": { color: "#d73a49", fontWeight: "600" },
    ".joss-token-type": { color: "#005cc5", fontWeight: "600" },
    ".joss-token-label": { color: "#b08800", fontWeight: "600" },
    ".joss-token-variable.joss-token-definition": { textDecoration: "underline dotted" },
  });

  const host = document.createElement("div");
  host.className = "joss-editor";
  host.id = "editor";
  textarea.before(host);
  const editor = new EditorView({
    parent: host,
    state: EditorState.create({
      doc: textarea.value,
      extensions: [
        cm.history(),
        keymap.of([{ key: "Mod-Enter", run: () => { onRun?.(); return true; } }, ...characters, ...cm.defaultKeymap, ...cm.historyKeymap]),
        tokens, colours, problems, hover,
        cm.autocompletion({ override: [complete] }),
        EditorState.tabSize.of(4),
        EditorView.contentAttributes.of({ "aria-label": "JOSS lines", spellcheck: "false", autocapitalize: "off", autocorrect: "off" }),
        EditorView.updateListener.of((update) => {
          if (update.docChanged) textarea.value = update.state.doc.toString();
        }),
      ],
    }),
  });
  textarea.hidden = true;
  editor.set = (text) => {
    if (editor.state.doc.toString() !== text) editor.dispatch({ changes: { from: 0, to: editor.state.doc.length, insert: text } });
  };
  return editor;
}
