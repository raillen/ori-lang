import { invoke } from "@tauri-apps/api/core";
import { autocompletion, CompletionContext, CompletionResult } from "@codemirror/autocomplete";
import { StreamLanguage } from "@codemirror/language";
import { basicSetup, EditorView } from "codemirror";
import "./styles.css";

type FilePayload = {
  path: string;
  text: string;
};

type WorkspacePayload = {
  root: string;
  files: string[];
};

type CommandOutput = {
  command: string;
  status: number | null;
  output: string;
};

type Problem = {
  severity: "error" | "warning" | "info";
  file: string;
  line: number;
  column: number;
  message: string;
};

type CommandItem = {
  label: string;
  hint: string;
  run: () => void | Promise<void>;
};

const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("Missing app root");
}

app.innerHTML = `
  <header class="topbar">
    <strong>Zenith IDE</strong>
    <nav aria-label="Primary actions">
      <button id="open-folder">Open Folder</button>
      <button id="open-file">Open File</button>
      <button id="save-file">Save</button>
      <button id="run-check">Check</button>
      <button id="run-run">Run</button>
      <button id="run-build">Build</button>
      <button id="run-test">Test</button>
      <button id="run-format">Format</button>
    </nav>
    <label class="theme-control">
      Theme
      <select id="theme-select">
        <option value="dark">Dark</option>
        <option value="light">Light</option>
        <option value="high-contrast">High contrast</option>
      </select>
    </label>
    <button id="focus-mode">Focus</button>
    <span id="status">No workspace</span>
  </header>
  <main class="shell">
    <aside id="sidebar" class="sidebar">
      <label for="path-input">Path</label>
      <input id="path-input" spellcheck="false" placeholder="C:\\\\path\\\\to\\\\project-or-file" />
      <section>
        <h2>Project</h2>
        <ul id="file-list"></ul>
      </section>
    </aside>
    <section class="editor-column">
      <div id="tabs" class="tabs"></div>
      <div id="editor"></div>
      <section id="problems-panel" class="bottom-panel">
        <h2>Problems</h2>
        <ul id="problems"></ul>
      </section>
      <section id="output-panel" class="bottom-panel">
        <h2>Output</h2>
        <pre id="output"></pre>
      </section>
    </section>
  </main>
  <div id="palette" class="palette hidden" role="dialog" aria-label="Command palette">
    <div class="palette-box">
      <label for="palette-input">Command</label>
      <input id="palette-input" spellcheck="false" placeholder="Type a command" />
      <ul id="palette-list"></ul>
    </div>
  </div>
`;

const statusEl = document.querySelector<HTMLSpanElement>("#status")!;
const pathInput = document.querySelector<HTMLInputElement>("#path-input")!;
const fileList = document.querySelector<HTMLUListElement>("#file-list")!;
const tabs = document.querySelector<HTMLDivElement>("#tabs")!;
const output = document.querySelector<HTMLPreElement>("#output")!;
const problemsList = document.querySelector<HTMLUListElement>("#problems")!;
const sidebar = document.querySelector<HTMLElement>("#sidebar")!;
const palette = document.querySelector<HTMLDivElement>("#palette")!;
const paletteInput = document.querySelector<HTMLInputElement>("#palette-input")!;
const paletteList = document.querySelector<HTMLUListElement>("#palette-list")!;
const themeSelect = document.querySelector<HTMLSelectElement>("#theme-select")!;

const openFiles = new Map<string, FilePayload>();
let currentFile: string | null = null;
let workspaceRoot: string | null = null;
let commandItems: CommandItem[] = [];
let problems: Problem[] = [];

const zenithKeywords = new Set([
  "namespace",
  "using",
  "extern",
  "func",
  "return",
  "if",
  "else",
  "while",
  "for",
  "match",
  "struct",
  "enum",
  "trait",
  "apply",
  "const",
  "let",
  "mut",
  "pub",
  "true",
  "false",
  "none",
]);

const zenithTypes = new Set(["int", "float", "bool", "text", "void", "bytes", "Result", "Option"]);

const zenithLanguage = StreamLanguage.define({
  token(stream) {
    if (stream.eatSpace()) {
      return null;
    }
    if (stream.match("//")) {
      stream.skipToEnd();
      return "comment";
    }
    if (stream.match(/"(?:[^"\\]|\\.)*"?/)) {
      return "string";
    }
    if (stream.match(/[0-9]+(?:\.[0-9]+)?/)) {
      return "number";
    }
    if (stream.match(/[A-Za-z_][A-Za-z0-9_]*/)) {
      const value = stream.current();
      if (zenithKeywords.has(value)) {
        return "keyword";
      }
      if (zenithTypes.has(value)) {
        return "type";
      }
      return "variableName";
    }
    stream.next();
    return null;
  },
});

const editor = new EditorView({
  doc: "",
  extensions: [
    basicSetup,
    EditorView.lineWrapping,
    zenithLanguage,
    autocompletion({ override: [zenithCompletionSource] }),
  ],
  parent: document.querySelector<HTMLDivElement>("#editor")!,
});

function zenithCompletionSource(context: CompletionContext): CompletionResult | null {
  const word = context.matchBefore(/[A-Za-z_][A-Za-z0-9_]*/);
  if (!word || (word.from === word.to && !context.explicit)) {
    return null;
  }
  const options = [...zenithKeywords].map((label) => ({ label, type: "keyword" }));
  options.push(...[...zenithTypes].map((label) => ({ label, type: "type" })));
  options.push(
    { label: "func main() -> int", type: "snippet" },
    { label: "extern c", type: "snippet" },
  );
  return { from: word.from, options };
}

function setStatus(text: string) {
  statusEl.textContent = text;
}

function setOutput(text: string) {
  output.textContent = text;
}

function captureCurrentFileText() {
  if (!currentFile) {
    return;
  }
  openFiles.set(currentFile, {
    path: currentFile,
    text: editor.state.doc.toString(),
  });
}

function setFile(payload: FilePayload) {
  captureCurrentFileText();
  currentFile = payload.path;
  openFiles.set(payload.path, payload);
  editor.dispatch({
    changes: { from: 0, to: editor.state.doc.length, insert: payload.text },
  });
  renderTabs();
  setStatus(payload.path);
}

function renderTabs() {
  tabs.innerHTML = "";
  for (const file of openFiles.values()) {
    const button = document.createElement("button");
    button.textContent = file.path.split(/[\\/]/).pop() ?? file.path;
    button.className = file.path === currentFile ? "active" : "";
    button.title = file.path;
    button.addEventListener("click", () => {
      const payload = openFiles.get(file.path);
      if (payload) {
        setFile(payload);
      }
    });
    tabs.append(button);
  }
}

function setWorkspace(payload: WorkspacePayload) {
  workspaceRoot = payload.root;
  fileList.innerHTML = "";
  for (const file of payload.files) {
    const item = document.createElement("li");
    const button = document.createElement("button");
    button.textContent = file;
    button.addEventListener("click", () => openFile(joinWorkspacePath(file)));
    item.append(button);
    fileList.append(item);
  }
  setStatus(`${payload.root} - ${payload.files.length} Zenith files`);
}

function joinWorkspacePath(relative: string) {
  if (!workspaceRoot) {
    return relative;
  }
  const separator = workspaceRoot.includes("\\") ? "\\" : "/";
  if (workspaceRoot.endsWith("\\") || workspaceRoot.endsWith("/")) {
    return `${workspaceRoot}${relative}`;
  }
  return `${workspaceRoot}${separator}${relative}`;
}

async function openFolder(path = pathInput.value.trim()) {
  const payload = await invoke<WorkspacePayload>("open_folder", { path });
  setWorkspace(payload);
}

async function openFile(path = pathInput.value.trim()) {
  const payload = await invoke<FilePayload>("open_file", { path });
  setFile(payload);
}

async function saveFile() {
  if (!currentFile) {
    setStatus("No file selected");
    return;
  }
  await invoke("save_file", {
    path: currentFile,
    text: editor.state.doc.toString(),
  });
  captureCurrentFileText();
  setStatus(`Saved ${currentFile}`);
}

async function runZt(command: string) {
  const root = workspaceRoot ?? currentFile ?? pathInput.value.trim();
  if (!root) {
    setStatus("Open a folder or file first");
    return;
  }
  const result = await invoke<CommandOutput>("run_zt", { root, command });
  setOutput(`${result.command} exit=${result.status ?? "none"}\n\n${result.output}`);
  problems = parseProblems(result.output);
  if (problems.length === 0 && result.status && result.status !== 0) {
    problems = [
      {
        severity: "error",
        file: currentFile ?? root,
        line: 1,
        column: 1,
        message: "Command failed. See Output.",
      },
    ];
  }
  renderProblems();
}

function parseProblems(text: string): Problem[] {
  return text
    .split(/\r?\n/)
    .map((line) => {
      const match = line.match(/^(.*?):(\d+):(\d+):\s*(error|warning|info)\b:?\s*(.*)$/i);
      if (!match) {
        return null;
      }
      return {
        file: match[1],
        line: Number(match[2]),
        column: Number(match[3]),
        severity: match[4].toLowerCase() as Problem["severity"],
        message: match[5] || line,
      };
    })
    .filter((problem): problem is Problem => problem !== null);
}

function renderProblems() {
  problemsList.innerHTML = "";
  if (problems.length === 0) {
    const item = document.createElement("li");
    item.textContent = "No problems";
    problemsList.append(item);
    return;
  }
  for (const problem of problems) {
    const item = document.createElement("li");
    item.className = problem.severity;
    item.textContent = `${problem.severity} ${problem.file}:${problem.line}:${problem.column} ${problem.message}`;
    problemsList.append(item);
  }
}

function setupCommands() {
  commandItems = [
    { label: "Open folder", hint: "Project", run: () => openFolder() },
    { label: "Open file", hint: "File", run: () => openFile() },
    { label: "Save file", hint: "File", run: () => saveFile() },
    { label: "Check", hint: "zt", run: () => runZt("check") },
    { label: "Run", hint: "zt", run: () => runZt("run") },
    { label: "Build", hint: "zt", run: () => runZt("build") },
    { label: "Test", hint: "zt", run: () => runZt("test") },
    { label: "Format", hint: "zt", run: () => runZt("format") },
    { label: "Toggle project panel", hint: "Layout", run: () => sidebar.classList.toggle("hidden-panel") },
    { label: "Cycle theme", hint: "Appearance", run: () => cycleTheme() },
    { label: "Toggle focus mode", hint: "Appearance", run: () => toggleFocusMode() },
  ];
}

function openPalette() {
  palette.classList.remove("hidden");
  paletteInput.value = "";
  renderPalette();
  paletteInput.focus();
}

function closePalette() {
  palette.classList.add("hidden");
  editor.focus();
}

function renderPalette() {
  const query = paletteInput.value.trim().toLowerCase();
  paletteList.innerHTML = "";
  for (const item of commandItems.filter((command) => command.label.toLowerCase().includes(query))) {
    const row = document.createElement("li");
    const button = document.createElement("button");
    button.innerHTML = `<span>${item.label}</span><small>${item.hint}</small>`;
    button.addEventListener("click", () => runPaletteCommand(item));
    row.append(button);
    paletteList.append(row);
  }
}

async function runPaletteCommand(item: CommandItem) {
  try {
    await item.run();
    closePalette();
  } catch (error) {
    setOutput(String(error));
  }
}

function applyTheme(theme: string) {
  document.body.dataset.theme = theme;
  themeSelect.value = theme;
  localStorage.setItem("zenith-ide.theme", theme);
}

function cycleTheme() {
  const values = ["dark", "light", "high-contrast"];
  const current = values.indexOf(themeSelect.value);
  applyTheme(values[(current + 1) % values.length]);
}

function toggleFocusMode() {
  document.body.classList.toggle("focus-mode");
  localStorage.setItem("zenith-ide.focus", document.body.classList.contains("focus-mode") ? "on" : "off");
}

function restorePreferences() {
  applyTheme(localStorage.getItem("zenith-ide.theme") ?? "dark");
  if (localStorage.getItem("zenith-ide.focus") === "on") {
    document.body.classList.add("focus-mode");
  }
}

document.querySelector("#open-folder")?.addEventListener("click", () => {
  openFolder().catch((error) => setOutput(String(error)));
});
document.querySelector("#open-file")?.addEventListener("click", () => {
  openFile().catch((error) => setOutput(String(error)));
});
document.querySelector("#save-file")?.addEventListener("click", () => {
  saveFile().catch((error) => setOutput(String(error)));
});
document.querySelector("#run-check")?.addEventListener("click", () => {
  runZt("check").catch((error) => setOutput(String(error)));
});
document.querySelector("#run-run")?.addEventListener("click", () => {
  runZt("run").catch((error) => setOutput(String(error)));
});
document.querySelector("#run-build")?.addEventListener("click", () => {
  runZt("build").catch((error) => setOutput(String(error)));
});
document.querySelector("#run-test")?.addEventListener("click", () => {
  runZt("test").catch((error) => setOutput(String(error)));
});
document.querySelector("#run-format")?.addEventListener("click", () => {
  runZt("format").catch((error) => setOutput(String(error)));
});
document.querySelector("#focus-mode")?.addEventListener("click", () => toggleFocusMode());
themeSelect.addEventListener("change", () => applyTheme(themeSelect.value));
paletteInput.addEventListener("input", () => renderPalette());
paletteInput.addEventListener("keydown", (event) => {
  if (event.key === "Escape") {
    closePalette();
  }
  if (event.key === "Enter") {
    const first = paletteList.querySelector<HTMLButtonElement>("button");
    first?.click();
  }
});
document.addEventListener("keydown", (event) => {
  if (event.ctrlKey && event.key.toLowerCase() === "s") {
    event.preventDefault();
    saveFile().catch((error) => setOutput(String(error)));
  }
  if (event.ctrlKey && event.key.toLowerCase() === "p") {
    event.preventDefault();
    openPalette();
  }
  if (event.key === "Escape" && !palette.classList.contains("hidden")) {
    closePalette();
  }
});

setupCommands();
restorePreferences();
renderProblems();
