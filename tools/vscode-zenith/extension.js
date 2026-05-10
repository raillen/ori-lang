const cp = require('child_process');
const fs = require('fs');
const path = require('path');
const vscode = require('vscode');

const LANGUAGE = 'zenith';
const LSP_BINARY = process.platform === 'win32' ? 'zt-lsp.exe' : 'zt-lsp';
const CLI_BINARY = process.platform === 'win32' ? 'zt.exe' : 'zt';
const semanticTokenLegend = new vscode.SemanticTokensLegend(
  ['namespace', 'type', 'function', 'variable', 'property', 'keyword', 'modifier', 'string', 'number'],
  ['declaration', 'readonly', 'public'],
);

function pathEntries() {
  return (process.env.PATH || '')
    .split(path.delimiter)
    .filter((entry) => entry && entry.trim().length > 0);
}

function executableCandidates(name) {
  if (process.platform !== 'win32' || path.extname(name)) return [name];
  const extensions = (process.env.PATHEXT || '.EXE;.CMD;.BAT;.COM')
    .split(';')
    .filter(Boolean);
  return [name, ...extensions.map((ext) => `${name}${ext.toLowerCase()}`), ...extensions.map((ext) => `${name}${ext.toUpperCase()}`)];
}

function findOnPath(name) {
  for (const dir of pathEntries()) {
    for (const candidate of executableCandidates(name)) {
      const filePath = path.join(dir, candidate);
      if (fs.existsSync(filePath)) return filePath;
    }
  }
  return null;
}

function looksLikePath(value) {
  return value.includes('/') || value.includes('\\') || path.isAbsolute(value);
}

function resolveToolPath(context, settingKey, binaryName) {
  const configured = vscode.workspace.getConfiguration('zenith').get(settingKey);
  if (configured && configured.trim().length > 0) {
    const value = configured.trim();
    if (looksLikePath(value)) return fs.existsSync(value) ? value : null;
    return findOnPath(value);
  }

  const repoRoot = path.resolve(context.extensionPath, '..', '..');
  const localCandidates = [
    path.join(context.extensionPath, binaryName),
    path.join(context.extensionPath, 'bin', binaryName),
    path.join(repoRoot, binaryName),
  ];
  for (const local of localCandidates) {
    if (fs.existsSync(local)) return local;
  }
  return findOnPath(binaryName);
}

function showMissingToolMessage(tool, settingKey, buildHint) {
  const message = `Zenith: nao encontrei ${tool}. Configure zenith.${settingKey} ou rode ${buildHint}.`;
  vscode.window.showErrorMessage(message, 'Abrir configuracoes').then((choice) => {
    if (choice === 'Abrir configuracoes') {
      vscode.commands.executeCommand('workbench.action.openSettings', `zenith.${settingKey}`);
    }
  });
}

class CompassClient {
  constructor(context, diagnostics) {
    this.context = context;
    this.diagnostics = diagnostics;
    this.proc = null;
    this.buffer = Buffer.alloc(0);
    this.nextId = 1;
    this.pending = new Map();
    this.started = false;
    this.workspaceIndexed = false;
    this.documentVersions = new Map();
  }

  repoRoot() {
    return path.resolve(this.context.extensionPath, '..', '..');
  }

  resolveLspPath() {
    return resolveToolPath(this.context, 'lsp.path', LSP_BINARY);
  }

  resolveLspCwd(exe) {
    const folder = vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders[0];
    const candidates = [
      this.context.extensionPath,
      path.join(this.context.extensionPath, 'runtime'),
      this.repoRoot(),
      folder ? folder.uri.fsPath : undefined,
      path.dirname(exe),
      process.cwd(),
    ].filter(Boolean);
    for (const candidate of candidates) {
      if (fs.existsSync(path.join(candidate, 'stdlib', 'std', 'io.zt'))) return candidate;
    }
    return path.dirname(exe);
  }

  async ensureStarted() {
    if (this.started && this.proc && !this.proc.killed) return;
    const exe = this.resolveLspPath();
    if (!exe) {
      showMissingToolMessage('zt-lsp', 'lsp.path', 'python tools/build_lsp.py');
      throw new Error('zt-lsp not found');
    }
    this.proc = cp.spawn(exe, [], { cwd: this.resolveLspCwd(exe), stdio: ['pipe', 'pipe', 'pipe'] });
    this.started = true;
    this.proc.stdout.on('data', (chunk) => this.onData(chunk));
    this.proc.stderr.on('data', (chunk) => console.error(`[zenith-lsp] ${chunk.toString()}`));
    this.proc.on('exit', () => {
      this.started = false;
      for (const [, pending] of this.pending) pending.reject(new Error('Compass LSP exited'));
      this.pending.clear();
    });
    this.proc.on('error', (err) => {
      this.started = false;
      showMissingToolMessage('zt-lsp', 'lsp.path', 'python tools/build_lsp.py');
      for (const [, pending] of this.pending) pending.reject(err);
      this.pending.clear();
    });
    const folder = vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders[0];
    await this.request('initialize', {
      capabilities: {},
      locale: vscode.env.language || 'en',
      processId: process.pid,
      rootUri: folder ? folder.uri.toString() : null,
    });
    this.notify('initialized', {});
  }

  stop() {
    if (!this.proc) return;
    try {
      this.request('shutdown', {}).finally(() => this.notify('exit', {}));
    } catch (_) {
      this.proc.kill();
    }
  }

  onData(chunk) {
    this.buffer = Buffer.concat([this.buffer, chunk]);
    while (true) {
      const headerEnd = this.buffer.indexOf('\r\n\r\n');
      if (headerEnd < 0) return;
      const header = this.buffer.slice(0, headerEnd).toString('ascii');
      const match = /content-length:\s*(\d+)/i.exec(header);
      if (!match) {
        this.buffer = this.buffer.slice(headerEnd + 4);
        continue;
      }
      const length = Number(match[1]);
      const bodyStart = headerEnd + 4;
      const bodyEnd = bodyStart + length;
      if (this.buffer.length < bodyEnd) return;
      const body = this.buffer.slice(bodyStart, bodyEnd).toString('utf8');
      this.buffer = this.buffer.slice(bodyEnd);
      try {
        this.handleMessage(JSON.parse(body));
      } catch (err) {
        console.error(`[zenith-lsp] invalid JSON-RPC payload: ${err.message || err}`);
      }
    }
  }

  handleMessage(message) {
    if (message.id !== undefined && this.pending.has(message.id)) {
      const pending = this.pending.get(message.id);
      this.pending.delete(message.id);
      if (message.error) pending.reject(new Error(message.error.message || 'LSP request failed'));
      else pending.resolve(message.result);
      return;
    }
    if (message.method === 'textDocument/publishDiagnostics') {
      this.applyDiagnostics(message.params);
    }
  }

  applyDiagnostics(params) {
    if (!params || !params.uri) return;
    const knownVersion = this.documentVersions.get(params.uri);
    if (typeof params.version === 'number' && typeof knownVersion === 'number' && params.version < knownVersion) {
      return;
    }
    const uri = vscode.Uri.parse(params.uri);
    const items = (params.diagnostics || []).map((diag) => {
      const range = new vscode.Range(
        diag.range.start.line,
        diag.range.start.character,
        diag.range.end.line,
        diag.range.end.character,
      );
      const severity = diag.severity === 2
        ? vscode.DiagnosticSeverity.Warning
        : diag.severity === 3
          ? vscode.DiagnosticSeverity.Information
          : diag.severity === 4
            ? vscode.DiagnosticSeverity.Hint
            : vscode.DiagnosticSeverity.Error;
      const item = new vscode.Diagnostic(range, diag.message, severity);
      item.source = diag.source || 'zenith';
      item.code = diag.code;
      return item;
    });
    this.diagnostics.set(uri, items);
  }

  send(payload) {
    if (!this.proc || !this.proc.stdin.writable) return false;
    const body = Buffer.from(JSON.stringify(payload), 'utf8');
    const header = Buffer.from(`Content-Length: ${body.length}\r\n\r\n`, 'ascii');
    this.proc.stdin.write(Buffer.concat([header, body]));
    return true;
  }

  request(method, params) {
    const id = this.nextId++;
    const payload = { jsonrpc: '2.0', id, method, params };
    return new Promise((resolve, reject) => {
      if (!this.send(payload)) {
        reject(new Error(`Compass LSP is not running: ${method}`));
        return;
      }
      this.pending.set(id, { resolve, reject });
      setTimeout(() => {
        if (!this.pending.has(id)) return;
        this.pending.delete(id);
        reject(new Error(`Compass LSP timeout: ${method}`));
      }, 5000);
    });
  }

  notify(method, params) {
    this.send({ jsonrpc: '2.0', method, params });
  }

  async syncDocument(document) {
    if (document.languageId !== LANGUAGE) return;
    await this.ensureStarted();
    const uriText = document.uri.toString();
    const wasOpen = this.documentVersions.has(uriText);
    this.documentVersions.set(uriText, document.version);
    if (wasOpen) {
      this.notify('textDocument/didChange', {
        textDocument: { uri: uriText, version: document.version },
        contentChanges: [{ text: document.getText() }],
      });
      return;
    }
    this.notify('textDocument/didOpen', {
      textDocument: {
        uri: uriText,
        languageId: LANGUAGE,
        version: document.version,
        text: document.getText(),
      },
    });
  }

  async syncUri(uri) {
    if (uri.scheme !== 'file') return;
    const document = await vscode.workspace.openTextDocument(uri);
    await this.syncDocument(document);
  }

  async indexWorkspaceDocuments() {
    if (this.workspaceIndexed) return;
    await this.ensureStarted();
    const files = await vscode.workspace.findFiles(
      '**/*.zt',
      '**/{.git,node_modules,target,build,.release-install,.playwright-mcp}/**',
      5000,
    );
    for (const uri of files) {
      await this.syncUri(uri);
    }
    this.workspaceIndexed = true;
  }

  closeUri(uri) {
    const uriText = uri.toString();
    this.documentVersions.delete(uriText);
    this.workspaceIndexed = false;
    this.notify('textDocument/didClose', { textDocument: { uri: uriText } });
  }
}

let activeClient = null;

function documentSelector() {
  return [{ language: LANGUAGE, scheme: 'file' }, { language: LANGUAGE, scheme: 'untitled' }];
}

function resolveCliPath(context) {
  return resolveToolPath(context, 'cli.path', CLI_BINARY);
}

function resolveZenithHome(context, cliPath) {
  const candidates = [
    context.extensionPath,
    path.resolve(context.extensionPath, '..', '..'),
    path.dirname(cliPath),
    path.resolve(path.dirname(cliPath), '..'),
  ];
  for (const candidate of candidates) {
    if (fs.existsSync(path.join(candidate, 'stdlib', 'std', 'io.zt'))) return candidate;
  }
  return undefined;
}

function projectTarget() {
  const editor = vscode.window.activeTextEditor;
  if (editor) {
    const folder = vscode.workspace.getWorkspaceFolder(editor.document.uri);
    if (folder) return folder.uri.fsPath;
    if (editor.document.uri.scheme === 'file') return editor.document.uri.fsPath;
  }
  const folder = vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders[0];
  return folder ? folder.uri.fsPath : '.';
}

function runTerminalCommand(context, command) {
  const cli = resolveCliPath(context);
  if (!cli) {
    showMissingToolMessage('zt', 'cli.path', 'python build.py');
    return;
  }
  const env = {};
  const zenithHome = resolveZenithHome(context, cli);
  if (zenithHome) env.ZENITH_HOME = zenithHome;
  const terminal = vscode.window.createTerminal({ name: 'Zenith', env });
  terminal.show(true);
  terminal.sendText(`"${cli}" ${command} "${projectTarget()}"`);
}

function autoNamespaceOnCreateEnabled() {
  return vscode.workspace.getConfiguration('zenith').get('files.autoNamespaceOnCreate') !== false;
}

function pathIsDirectory(filePath) {
  try {
    return fs.statSync(filePath).isDirectory();
  } catch (_) {
    return false;
  }
}

function fileExists(filePath) {
  try {
    return fs.existsSync(filePath);
  } catch (_) {
    return false;
  }
}

function isPathInside(basePath, candidatePath) {
  const relative = path.relative(path.resolve(basePath), path.resolve(candidatePath));
  return relative === '' || (!relative.startsWith('..') && !path.isAbsolute(relative));
}

function parseSourceRoot(manifestText) {
  let section = '';
  for (const rawLine of manifestText.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (!line || line.startsWith('#') || line.startsWith('--')) continue;

    const sectionMatch = /^\[([^\]]+)\]$/.exec(line);
    if (sectionMatch) {
      section = sectionMatch[1].trim();
      continue;
    }

    if (section !== 'source') continue;
    const rootMatch = /^root\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s#;]+))/.exec(line);
    if (rootMatch) return rootMatch[1] || rootMatch[2] || rootMatch[3] || 'src';
  }
  return 'src';
}

async function findProjectInfo(startPath) {
  let current = pathIsDirectory(startPath) ? startPath : path.dirname(startPath);
  while (current && current !== path.dirname(current)) {
    const manifestPath = path.join(current, 'zenith.ztproj');
    if (fileExists(manifestPath)) {
      const manifestText = await fs.promises.readFile(manifestPath, 'utf8');
      return {
        manifestPath,
        projectRoot: current,
        sourceRoot: parseSourceRoot(manifestText),
      };
    }
    current = path.dirname(current);
  }

  const folder = vscode.workspace.getWorkspaceFolder(vscode.Uri.file(startPath));
  if (folder) {
    return {
      manifestPath: null,
      projectRoot: folder.uri.fsPath,
      sourceRoot: 'src',
    };
  }

  return null;
}

function normalizeInputPath(input) {
  return input
    .trim()
    .replace(/^["']|["']$/g, '')
    .replace(/[\\/]+/g, path.sep);
}

function namespaceFromFilePath(filePath, projectInfo) {
  if (!projectInfo || path.extname(filePath).toLowerCase() !== '.zt') return null;

  const sourceDir = path.resolve(projectInfo.projectRoot, projectInfo.sourceRoot || 'src');
  if (!isPathInside(sourceDir, filePath)) return null;

  const relativePath = path.relative(sourceDir, filePath);
  const withoutExtension = relativePath.slice(0, -'.zt'.length);
  const parts = withoutExtension.split(/[\\/]+/).filter(Boolean);
  if (parts.length === 0) return null;

  for (const part of parts) {
    if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(part)) return null;
  }

  return parts.join('.');
}

function sourceRootLabel(projectInfo) {
  const sourceDir = path.resolve(projectInfo.projectRoot, projectInfo.sourceRoot || 'src');
  return path.relative(projectInfo.projectRoot, sourceDir) || '.';
}

function resolveNewZenithFilePath(projectInfo, rawInput) {
  const sourceDir = path.resolve(projectInfo.projectRoot, projectInfo.sourceRoot || 'src');
  const sourceRootInput = normalizeInputPath(projectInfo.sourceRoot || 'src');
  const input = normalizeInputPath(rawInput);
  if (!input) return null;

  let filePath;
  if (path.isAbsolute(input)) {
    filePath = input;
  } else if (input === sourceRootInput || input.startsWith(`${sourceRootInput}${path.sep}`)) {
    filePath = path.resolve(projectInfo.projectRoot, input);
  } else {
    filePath = path.resolve(sourceDir, input);
  }

  if (path.extname(filePath).toLowerCase() !== '.zt') {
    filePath = `${filePath}.zt`;
  }

  if (!isPathInside(sourceDir, filePath)) return null;
  return filePath;
}

async function readUriText(uri) {
  const bytes = await vscode.workspace.fs.readFile(uri);
  return Buffer.from(bytes).toString('utf8');
}

function syncUriBestEffort(client, uri) {
  if (!client) return;
  client.syncUri(uri).catch((err) => {
    console.error(`[zenith] failed to sync ${uri.fsPath}: ${err.message || err}`);
  });
}

async function writeNamespaceTemplate(uri, namespaceName, client) {
  await vscode.workspace.fs.writeFile(uri, Buffer.from(`namespace ${namespaceName}\n\n`, 'utf8'));
  syncUriBestEffort(client, uri);
}

async function fillNamespaceIfEmpty(uri, client, showFeedback) {
  if (!uri || uri.scheme !== 'file' || path.extname(uri.fsPath).toLowerCase() !== '.zt') return false;

  const text = await readUriText(uri);
  if (text.trim().length > 0) return false;

  const projectInfo = await findProjectInfo(uri.fsPath);
  const namespaceName = namespaceFromFilePath(uri.fsPath, projectInfo);
  if (!namespaceName) {
    if (showFeedback) {
      vscode.window.showWarningMessage('Nao consegui calcular o namespace. Crie o arquivo dentro de source.root usando nomes validos.');
    }
    return false;
  }

  await writeNamespaceTemplate(uri, namespaceName, client);
  return true;
}

async function selectProjectInfo() {
  const editor = vscode.window.activeTextEditor;
  if (editor && editor.document.uri.scheme === 'file') {
    const projectInfo = await findProjectInfo(editor.document.uri.fsPath);
    if (projectInfo) return projectInfo;
  }

  const folders = vscode.workspace.workspaceFolders || [];
  if (folders.length === 0) return null;

  let folder = folders[0];
  if (folders.length > 1) {
    folder = await vscode.window.showWorkspaceFolderPick({
      placeHolder: 'Escolha o workspace do projeto Zenith',
    });
    if (!folder) return null;
  }

  return findProjectInfo(folder.uri.fsPath);
}

async function createZenithFile(client) {
  const projectInfo = await selectProjectInfo();
  if (!projectInfo) {
    vscode.window.showErrorMessage('Abra uma pasta de projeto para criar um arquivo Zenith.');
    return;
  }

  const relativePath = await vscode.window.showInputBox({
    prompt: `Caminho do arquivo .zt relativo a ${sourceRootLabel(projectInfo)}`,
    placeHolder: 'app/main.zt',
    value: 'app/main.zt',
    validateInput: (value) => {
      if (!value || !value.trim()) return 'Informe um caminho de arquivo.';
      const filePath = resolveNewZenithFilePath(projectInfo, value);
      if (!filePath) return 'O arquivo precisa ficar dentro de source.root.';
      if (!namespaceFromFilePath(filePath, projectInfo)) return 'Use nomes validos para namespace, como app/main.zt.';
      return null;
    },
  });
  if (relativePath === undefined) return;

  const filePath = resolveNewZenithFilePath(projectInfo, relativePath);
  const namespaceName = filePath ? namespaceFromFilePath(filePath, projectInfo) : null;
  if (!filePath || !namespaceName) {
    vscode.window.showErrorMessage('Nao consegui criar esse arquivo com namespace valido.');
    return;
  }

  const uri = vscode.Uri.file(filePath);
  if (!fileExists(filePath)) {
    await vscode.workspace.fs.createDirectory(vscode.Uri.file(path.dirname(filePath)));
    await writeNamespaceTemplate(uri, namespaceName, client);
  } else {
    await fillNamespaceIfEmpty(uri, client, true);
  }

  const document = await vscode.workspace.openTextDocument(uri);
  await vscode.window.showTextDocument(document);
}

function toRange(range) {
  if (!range) return new vscode.Range(0, 0, 0, 1);
  return new vscode.Range(
    range.start.line,
    range.start.character,
    range.end.line,
    range.end.character,
  );
}

function toSymbolKind(kind) {
  return kind ? Math.max(0, kind - 1) : vscode.SymbolKind.Variable;
}

function toDocumentSymbol(item) {
  const symbol = new vscode.DocumentSymbol(
    item.name,
    item.detail || '',
    toSymbolKind(item.kind),
    toRange(item.range),
    toRange(item.selectionRange || item.range),
  );
  for (const child of item.children || []) {
    symbol.children.push(toDocumentSymbol(child));
  }
  return symbol;
}

function toSymbolInformation(item) {
  return new vscode.SymbolInformation(
    item.name,
    toSymbolKind(item.kind),
    item.containerName || '',
    new vscode.Location(vscode.Uri.parse(item.location.uri), toRange(item.location.range)),
  );
}

function toLocation(item) {
  if (!item || !item.uri || !item.range) return undefined;
  return new vscode.Location(vscode.Uri.parse(item.uri), toRange(item.range));
}

function toDocumentation(value) {
  if (!value) return undefined;
  const text = typeof value === 'string' ? value : value.value;
  if (!text) return undefined;
  const markdown = new vscode.MarkdownString(text);
  markdown.isTrusted = false;
  markdown.supportHtml = false;
  return markdown;
}

function toSignatureHelp(result) {
  if (!result || !Array.isArray(result.signatures) || result.signatures.length === 0) return undefined;
  const help = new vscode.SignatureHelp();
  help.activeSignature = result.activeSignature || 0;
  help.activeParameter = result.activeParameter || 0;
  help.signatures = result.signatures.map((item) => {
    const signature = new vscode.SignatureInformation(item.label || '');
    signature.documentation = toDocumentation(item.documentation);
    signature.parameters = (item.parameters || []).map(
      (param) => {
        const parameter = new vscode.ParameterInformation(param.label || '');
        parameter.documentation = toDocumentation(param.documentation);
        return parameter;
      },
    );
    return signature;
  });
  return help;
}

function toWorkspaceEdit(result) {
  if (!result || !result.changes) return undefined;
  const workspaceEdit = new vscode.WorkspaceEdit();
  for (const [uriText, edits] of Object.entries(result.changes)) {
    const uri = vscode.Uri.parse(uriText);
    for (const edit of edits || []) {
      workspaceEdit.replace(uri, toRange(edit.range), edit.newText || '');
    }
  }
  return workspaceEdit;
}

function activate(context) {
  const diagnostics = vscode.languages.createDiagnosticCollection('zenith');
  const client = new CompassClient(context, diagnostics);
  activeClient = client;

  context.subscriptions.push(diagnostics);
  context.subscriptions.push({ dispose: () => client.stop() });

  context.subscriptions.push(vscode.workspace.onDidOpenTextDocument((doc) => client.syncDocument(doc)));
  context.subscriptions.push(vscode.workspace.onDidChangeTextDocument((event) => client.syncDocument(event.document)));
  context.subscriptions.push(vscode.workspace.onDidCloseTextDocument((doc) => {
    if (doc.languageId !== LANGUAGE) return;
    client.closeUri(doc.uri);
    diagnostics.delete(doc.uri);
  }));

  const watcher = vscode.workspace.createFileSystemWatcher('**/*.zt');
  context.subscriptions.push(watcher);
  context.subscriptions.push(watcher.onDidCreate((uri) => {
    client.workspaceIndexed = false;
    if (autoNamespaceOnCreateEnabled()) {
      fillNamespaceIfEmpty(uri, client, false)
        .then((filled) => {
          if (!filled) syncUriBestEffort(client, uri);
        })
        .catch((err) => {
          console.error(`[zenith] failed to initialize namespace for ${uri.fsPath}: ${err.message || err}`);
          syncUriBestEffort(client, uri);
        });
      return;
    }
    syncUriBestEffort(client, uri);
  }));
  context.subscriptions.push(watcher.onDidChange((uri) => {
    client.workspaceIndexed = false;
    client.syncUri(uri);
  }));
  context.subscriptions.push(watcher.onDidDelete((uri) => {
    client.closeUri(uri);
    diagnostics.delete(uri);
  }));

  for (const doc of vscode.workspace.textDocuments) client.syncDocument(doc);
  client.indexWorkspaceDocuments();

  context.subscriptions.push(vscode.languages.registerHoverProvider(documentSelector(), {
    async provideHover(document, position) {
      await client.syncDocument(document);
      const result = await client.request('textDocument/hover', {
        textDocument: { uri: document.uri.toString() },
        position,
      });
      if (!result || !result.contents) return undefined;
      const value = typeof result.contents === 'string' ? result.contents : result.contents.value;
      return new vscode.Hover(new vscode.MarkdownString(value || ''));
    },
  }));

  context.subscriptions.push(vscode.languages.registerDefinitionProvider(documentSelector(), {
    async provideDefinition(document, position) {
      await client.syncDocument(document);
      const result = await client.request('textDocument/definition', {
        textDocument: { uri: document.uri.toString() },
        position,
      });
      if (!result || !result.uri || !result.range) return undefined;
      return new vscode.Location(vscode.Uri.parse(result.uri), new vscode.Range(
        result.range.start.line,
        result.range.start.character,
        result.range.end.line,
        result.range.end.character,
      ));
    },
  }));

  context.subscriptions.push(vscode.languages.registerReferenceProvider(documentSelector(), {
    async provideReferences(document, position, context) {
      await client.indexWorkspaceDocuments();
      await client.syncDocument(document);
      const result = await client.request('textDocument/references', {
        textDocument: { uri: document.uri.toString() },
        position,
        context: { includeDeclaration: Boolean(context && context.includeDeclaration) },
      });
      return (result || []).map(toLocation).filter(Boolean);
    },
  }));

  context.subscriptions.push(vscode.languages.registerSignatureHelpProvider(documentSelector(), {
    async provideSignatureHelp(document, position) {
      await client.indexWorkspaceDocuments();
      await client.syncDocument(document);
      const result = await client.request('textDocument/signatureHelp', {
        textDocument: { uri: document.uri.toString() },
        position,
      });
      return toSignatureHelp(result);
    },
  }, '(', ','));

  context.subscriptions.push(vscode.languages.registerRenameProvider(documentSelector(), {
    async prepareRename(document, position) {
      await client.indexWorkspaceDocuments();
      await client.syncDocument(document);
      const result = await client.request('textDocument/prepareRename', {
        textDocument: { uri: document.uri.toString() },
        position,
      });
      if (!result || !result.range) return undefined;
      return { range: toRange(result.range), placeholder: result.placeholder || document.getText(toRange(result.range)) };
    },
    async provideRenameEdits(document, position, newName) {
      await client.indexWorkspaceDocuments();
      await client.syncDocument(document);
      const result = await client.request('textDocument/rename', {
        textDocument: { uri: document.uri.toString() },
        position,
        newName,
      });
      return toWorkspaceEdit(result);
    },
  }));

  context.subscriptions.push(vscode.languages.registerDocumentSymbolProvider(documentSelector(), {
    async provideDocumentSymbols(document) {
      await client.syncDocument(document);
      const result = await client.request('textDocument/documentSymbol', {
        textDocument: { uri: document.uri.toString() },
      });
      return (result || []).map(toDocumentSymbol);
    },
  }));

  context.subscriptions.push(vscode.languages.registerDocumentSemanticTokensProvider(documentSelector(), {
    async provideDocumentSemanticTokens(document) {
      await client.syncDocument(document);
      const result = await client.request('textDocument/semanticTokens/full', {
        textDocument: { uri: document.uri.toString() },
      });
      return new vscode.SemanticTokens(new Uint32Array((result && result.data) || []));
    },
  }, semanticTokenLegend));

  context.subscriptions.push(vscode.languages.registerWorkspaceSymbolProvider({
    async provideWorkspaceSymbols(query) {
      await client.indexWorkspaceDocuments();
      const result = await client.request('workspace/symbol', { query });
      return (result || []).map(toSymbolInformation);
    },
  }));

  context.subscriptions.push(vscode.languages.registerDocumentFormattingEditProvider(documentSelector(), {
    async provideDocumentFormattingEdits(document, options) {
      await client.syncDocument(document);
      const result = await client.request('textDocument/formatting', {
        textDocument: { uri: document.uri.toString() },
        options,
      });
      return (result || []).map((edit) => new vscode.TextEdit(
        new vscode.Range(
          edit.range.start.line,
          edit.range.start.character,
          edit.range.end.line,
          edit.range.end.character,
        ),
        edit.newText,
      ));
    },
  }));

  context.subscriptions.push(vscode.languages.registerCompletionItemProvider(documentSelector(), {
    async provideCompletionItems(document, position) {
      await client.indexWorkspaceDocuments();
      await client.syncDocument(document);
      const result = await client.request('textDocument/completion', {
        textDocument: { uri: document.uri.toString() },
        position,
      });
      return (result || []).map((item) => {
        const completion = new vscode.CompletionItem(
          item.label,
          item.kind ? item.kind - 1 : vscode.CompletionItemKind.Text,
        );
        if (item.detail) completion.detail = item.detail;
        if (item.documentation) completion.documentation = toDocumentation(item.documentation);
        if (item.insertText) {
          completion.insertText = item.insertTextFormat === 2
            ? new vscode.SnippetString(item.insertText)
            : item.insertText;
        }
        return completion;
      });
    },
  }, '.', ':', '<', ' '));

  context.subscriptions.push(vscode.commands.registerCommand('zenith.check', () => runTerminalCommand(context, 'check')));
  context.subscriptions.push(vscode.commands.registerCommand('zenith.build', () => runTerminalCommand(context, 'build')));
  context.subscriptions.push(vscode.commands.registerCommand('zenith.run', () => runTerminalCommand(context, 'run')));
  context.subscriptions.push(vscode.commands.registerCommand('zenith.newFile', () => createZenithFile(client)));
}

function deactivate() {
  if (activeClient) activeClient.stop();
  activeClient = null;
}

module.exports = { activate, deactivate };
