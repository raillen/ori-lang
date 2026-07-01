import * as fs from "fs";
import * as path from "path";
import { execFile } from "child_process";
import { promisify } from "util";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  Trace,
} from "vscode-languageclient/node";

const execFileAsync = promisify(execFile);

let client: LanguageClient | undefined;
let doctorChannel: vscode.OutputChannel | undefined;

export function activate(context: vscode.ExtensionContext): void {
  doctorChannel = vscode.window.createOutputChannel("Ori Doctor");
  context.subscriptions.push(doctorChannel);

  context.subscriptions.push(
    vscode.commands.registerCommand("ori.runDoctor", () => runDoctor(context)),
    vscode.commands.registerCommand("ori.checkFile", () => runOriOnActive(context, "check")),
    vscode.commands.registerCommand("ori.runFile", () => runOriOnActive(context, "run")),
    vscode.commands.registerCommand("ori.testFile", () => runOriOnActive(context, "test")),
    vscode.commands.registerCommand("ori.summaryProject", () => runSummary(context)),
    vscode.commands.registerCommand("ori.formatFile", () =>
      vscode.commands.executeCommand("editor.action.formatDocument")
    )
  );

  void suggestWorkspaceBinaries(context);

  startLanguageClient(context).catch((err) => {
    void vscode.window.showErrorMessage(
      `Ori LSP failed to start: ${err instanceof Error ? err.message : String(err)}`
    );
  });
}

async function startLanguageClient(context: vscode.ExtensionContext): Promise<void> {
  const config = () => vscode.workspace.getConfiguration("ori");

  const lspPath =
    resolveBinary(context, "lsp.path", process.platform === "win32" ? ["ori-lsp.exe", "ori-lsp"] : ["ori-lsp"]) ??
    "ori-lsp";

  const serverOptions: ServerOptions = {
    command: lspPath,
    args: [],
    options: { env: buildOriEnv() },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "ori" }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher("**/*.orl"),
    },
  };

  client = new LanguageClient("oriLanguageServer", "Ori Language Server", serverOptions, clientOptions);

  const trace = config().get<string>("trace.server") ?? "off";
  client.setTrace(trace === "verbose" ? Trace.Verbose : trace === "messages" ? Trace.Messages : Trace.Off);

  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration("ori.trace.server")) {
        const t = config().get<string>("trace.server") ?? "off";
        client?.setTrace(t === "verbose" ? Trace.Verbose : t === "messages" ? Trace.Messages : Trace.Off);
      }
    })
  );

  await client.start();
}

export async function deactivate(): Promise<void> {
  if (client) {
    await client.stop();
  }
}

function buildOriEnv(): NodeJS.ProcessEnv {
  const env = { ...process.env };
  const cfg = vscode.workspace.getConfiguration("ori");
  const setIf = (key: string, value: string | undefined) => {
    if (value) {
      env[key] = value;
    }
  };
  setIf("ORI_STDLIB_ROOT", cfg.get<string>("stdlib.root"));
  setIf("ORI_RUNTIME_LIB", cfg.get<string>("runtime.lib"));
  setIf("ORI_RUNTIME_CDYLIB", cfg.get<string>("runtime.cdylib"));
  if (cfg.get<boolean>("useJit")) {
    env["ORI_USE_JIT"] = "1";
  }
  return env;
}

async function runDoctor(context: vscode.ExtensionContext): Promise<void> {
  const oriPath = resolveOriCompiler(context) ?? "ori";
  const channel = doctorChannel ?? vscode.window.createOutputChannel("Ori Doctor");
  channel.clear();
  channel.show(true);
  channel.appendLine(`Running: ${quote(oriPath)} doctor\n`);

  try {
    const { stdout, stderr } = await execFileAsync(oriPath, ["doctor"], {
      env: buildOriEnv(),
      maxBuffer: 1024 * 256,
    });
    if (stdout.trim()) {
      channel.appendLine(stdout.trimEnd());
    }
    if (stderr.trim()) {
      channel.appendLine(stderr.trimEnd());
    }
  } catch (err: unknown) {
    const execErr = err as { stdout?: string; stderr?: string; message?: string };
    if (execErr.stdout) {
      channel.appendLine(execErr.stdout.trimEnd());
    }
    if (execErr.stderr) {
      channel.appendLine(execErr.stderr.trimEnd());
    }
    channel.appendLine(execErr.message ?? String(err));
    void vscode.window.showErrorMessage("Ori doctor failed — see Output panel.");
  }
}

async function runSummary(context: vscode.ExtensionContext): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  const target = editor?.document.uri.fsPath ?? workspaceRoots()[0];
  if (!target) {
    vscode.window.showWarningMessage("Open a workspace folder or an Ori file first.");
    return;
  }
  const oriPath = resolveOriCompiler(context) ?? "ori";
  const channel = doctorChannel ?? vscode.window.createOutputChannel("Ori Doctor");
  channel.clear();
  channel.show(true);
  try {
    const { stdout } = await execFileAsync(oriPath, ["summary", target], {
      env: buildOriEnv(),
      maxBuffer: 1024 * 256,
    });
    channel.appendLine(stdout.trimEnd());
  } catch (err: unknown) {
    const execErr = err as { stdout?: string; stderr?: string; message?: string };
    channel.appendLine(execErr.stderr ?? execErr.message ?? String(err));
  }
}

async function runOriOnActive(context: vscode.ExtensionContext, subcommand: string): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== "ori") {
    vscode.window.showWarningMessage("Open an Ori (.orl) file first.");
    return;
  }
  const file = editor.document.uri.fsPath;
  const oriPath = resolveOriCompiler(context) ?? "ori";
  const term = vscode.window.createTerminal({ name: `Ori ${subcommand}`, env: buildOriEnv() });
  term.show();
  term.sendText(`${quote(oriPath)} ${subcommand} ${quote(file)}`);
}

async function suggestWorkspaceBinaries(context: vscode.ExtensionContext): Promise<void> {
  const cfg = vscode.workspace.getConfiguration("ori");
  const lsp = resolveBinary(context, "lsp.path", process.platform === "win32" ? ["ori-lsp.exe", "ori-lsp"] : ["ori-lsp"]);
  const compiler = resolveOriCompiler(context);
  const stdlib = workspaceRoots()
    .map((root) => path.join(root, "stdlib"))
    .find((p) => fs.existsSync(p));

  const updates: Array<[string, string]> = [];
  if (lsp && !cfg.get<string>("lsp.path")) {
    updates.push(["lsp.path", lsp]);
  }
  if (compiler && !cfg.get<string>("compiler.path")) {
    updates.push(["compiler.path", compiler]);
  }
  if (stdlib && !cfg.get<string>("stdlib.root")) {
    updates.push(["stdlib.root", stdlib]);
  }
  if (updates.length === 0) {
    return;
  }
  const picked = await vscode.window.showInformationMessage(
    "Ori: configure toolchain paths for this workspace?",
    "Configure",
    "Dismiss"
  );
  if (picked !== "Configure") {
    return;
  }
  for (const [key, value] of updates) {
    await cfg.update(key, value, vscode.ConfigurationTarget.Workspace);
  }
}

function resolveBinary(
  context: vscode.ExtensionContext,
  settingKey: string,
  names: string[]
): string | undefined {
  const configured = vscode.workspace.getConfiguration("ori").get<string>(settingKey)?.trim();
  if (configured && fs.existsSync(configured)) {
    return configured;
  }
  for (const name of names) {
    const found = whichOnPath(name);
    if (found) {
      return found;
    }
  }
  for (const root of workspaceRoots()) {
    const dev = path.join(root, "target", "debug", names[0]);
    if (fs.existsSync(dev)) {
      return dev;
    }
  }
  return configured || undefined;
}

function resolveOriCompiler(_context: vscode.ExtensionContext): string | undefined {
  return resolveBinary(_context, "compiler.path", process.platform === "win32" ? ["ori.exe", "ori"] : ["ori"]);
}

function workspaceRoots(): string[] {
  return (vscode.workspace.workspaceFolders ?? []).map((f) => f.uri.fsPath);
}

function whichOnPath(name: string): string | undefined {
  const dirs = (process.env.PATH ?? "").split(path.delimiter);
  for (const dir of dirs) {
    const candidate = path.join(dir, name);
    if (fs.existsSync(candidate)) {
      return candidate;
    }
  }
  return undefined;
}

function quote(p: string): string {
  return p.includes(" ") ? `"${p}"` : p;
}
