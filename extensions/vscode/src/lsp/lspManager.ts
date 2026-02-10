import { spawn } from 'node:child_process';
import { constants as fsConstants } from 'node:fs';
import * as fs from 'node:fs/promises';
import { access } from 'node:fs/promises';
import * as path from 'node:path';
import semver from 'semver';
import * as vscode from 'vscode';
import {
  LanguageClient,
  type LanguageClientOptions,
  RevealOutputChannelOn,
  type ServerOptions,
} from 'vscode-languageclient/node';
import type { Logger } from '../utils/log';
import {
  type ConfigSummary,
  type DocumentSpellsArgs,
  type DocumentSpellsResult,
  type DryCandidatesResult,
  type DryCreateScrollArgs,
  type EntityDetailsArgs,
  type EntityDetailsResult,
  type ExplainClassTokenResult,
  type ExplorerIndexResult,
  executeCommand,
  type LintResult,
  type ListScrollsResult,
  type LspWorkspaceEdit,
  type RefsResult,
  type StatsArgs,
} from './commands';

type FolderState = {
  folder: vscode.WorkspaceFolder;
  client: LanguageClient;
  entityDetailsCache: Map<string, EntityDetailsResult>;
  entityDetailsInflight: Map<string, Promise<EntityDetailsResult>>;
  status: {
    checkRunning: boolean;
    lintRunning: boolean;
    dryRunning: boolean;
  };
  cache: {
    configSummary?: ConfigSummary;
    listScrolls?: ListScrollsResult;
    listVars?: Array<{ name: string; value: string }>;
    explorerIndex?: ExplorerIndexResult;
    lint?: LintResult;
    dry?: DryCandidatesResult;
    lastRefs?: RefsResult;
  };
};

export class LspManager implements vscode.Disposable {
  private readonly states = new Map<string, FolderState>();

  constructor(
    private readonly context: vscode.ExtensionContext,
    private readonly logger: Logger
  ) {}

  dispose(): void {
    for (const state of this.states.values()) {
      void state.client.stop();
      state.client.dispose();
    }
    this.states.clear();
  }

  getFolders(): vscode.WorkspaceFolder[] {
    return Array.from(this.states.values()).map((s) => s.folder);
  }

  getActiveOrFirstFolder(): vscode.WorkspaceFolder | undefined {
    const editor = vscode.window.activeTextEditor;
    if (editor) {
      const folder = vscode.workspace.getWorkspaceFolder(editor.document.uri);
      if (folder && this.states.has(folder.uri.toString())) return folder;
    }
    return this.getFolders()[0];
  }

  async ensureStarted(folder: vscode.WorkspaceFolder): Promise<void> {
    const key = folder.uri.toString();
    if (this.states.has(key)) return;

    const serverPath = await this.resolveServerPath(folder);
    if (!serverPath) {
      throw new Error('Failed to resolve grimoire_css_lsp server path');
    }

    const serverOptions: ServerOptions = {
      command: serverPath,
      args: [],
      options: { cwd: folder.uri.fsPath },
    };

    const clientOptions: LanguageClientOptions = {
      workspaceFolder: folder,
      documentSelector: [{ scheme: 'file' }],
      outputChannel: vscode.window.createOutputChannel(`Grimoire CSS Server (${folder.name})`),
      revealOutputChannelOn: RevealOutputChannelOn.Never,
    };

    const client = new LanguageClient('grimoirecss', 'GrimoireCSS', serverOptions, clientOptions);

    this.logger.info(`Starting LSP for ${folder.name} using ${serverPath}`);
    this.context.subscriptions.push(client);

    await withTimeout(
      client.start(),
      30_000,
      `Timed out starting grimoire_css_lsp for ${folder.name}. Check network/LSP logs or set grimoireCss.lsp.path.`
    );

    this.states.set(key, {
      folder,
      client,
      entityDetailsCache: new Map(),
      entityDetailsInflight: new Map(),
      status: { checkRunning: false, lintRunning: false, dryRunning: false },
      cache: {},
    });
  }

  async restartAll(): Promise<void> {
    const folders = this.getFolders();

    // Stop+dispose existing clients first.
    for (const state of this.states.values()) {
      try {
        await state.client.stop();
      } catch {
        // ignore
      }
      try {
        state.client.dispose();
      } catch {
        // ignore
      }
    }
    this.states.clear();

    // Re-resolve server path and recreate clients.
    for (const folder of folders) {
      await this.ensureStarted(folder);
    }
  }

  async refreshAll(): Promise<void> {
    for (const state of this.states.values()) {
      await this.refreshFolder(state.folder);
    }
  }

  async checkAll(): Promise<void> {
    for (const state of this.states.values()) {
      await this.checkFolder(state.folder);
    }
  }

  async refreshFolder(folder: vscode.WorkspaceFolder): Promise<void> {
    const state = this.mustGetState(folder);
    state.entityDetailsCache.clear();
    state.entityDetailsInflight.clear();
    state.cache.configSummary = await this.configSummary(folder);
    state.cache.listScrolls = await this.listScrolls(folder);
    state.cache.listVars = await this.listVars(folder);
    state.cache.explorerIndex = await this.explorerIndex(folder);
  }

  async checkFolder(folder: vscode.WorkspaceFolder): Promise<void> {
    const state = this.mustGetState(folder);
    state.entityDetailsCache.clear();
    state.entityDetailsInflight.clear();
    state.status.checkRunning = true;
    try {
      state.cache.lint = await this.lint(folder);
      state.cache.dry = await this.dryCandidates(folder);
    } finally {
      state.status.checkRunning = false;
    }
  }

  async runDry(folder: vscode.WorkspaceFolder): Promise<DryCandidatesResult> {
    const state = this.mustGetState(folder);
    state.status.dryRunning = true;
    try {
      state.cache.dry = await this.dryCandidates(folder);
      return state.cache.dry;
    } finally {
      state.status.dryRunning = false;
    }
  }

  async runLint(folder: vscode.WorkspaceFolder): Promise<LintResult> {
    const state = this.mustGetState(folder);
    state.status.lintRunning = true;
    try {
      state.cache.lint = await this.lint(folder);
      return state.cache.lint;
    } finally {
      state.status.lintRunning = false;
    }
  }

  getStatus(folder: vscode.WorkspaceFolder): FolderState['status'] {
    return this.mustGetState(folder).status;
  }

  getCache(folder: vscode.WorkspaceFolder): FolderState['cache'] {
    return this.mustGetState(folder).cache;
  }

  async getConfigSummary(folder: vscode.WorkspaceFolder): Promise<ConfigSummary> {
    const state = this.mustGetState(folder);
    if (!state.cache.configSummary) state.cache.configSummary = await this.configSummary(folder);
    return state.cache.configSummary;
  }

  async explain(folder: vscode.WorkspaceFolder, token: string): Promise<ExplainClassTokenResult> {
    return this.execute<ExplainClassTokenResult>(folder, 'grimoirecss.explain', [token, true]);
  }

  async refs(folder: vscode.WorkspaceFolder, query: string): Promise<RefsResult> {
    return this.execute<RefsResult>(folder, 'grimoirecss.refs', [query, true]);
  }

  setLastRefs(folder: vscode.WorkspaceFolder, refs: RefsResult): void {
    const state = this.mustGetState(folder);
    state.cache.lastRefs = refs;
  }

  async stats(folder: vscode.WorkspaceFolder, args: StatsArgs): Promise<unknown> {
    return this.execute<unknown>(folder, 'grimoirecss.stats', [args]);
  }

  async entityDetails(
    folder: vscode.WorkspaceFolder,
    args: EntityDetailsArgs
  ): Promise<EntityDetailsResult> {
    const state = this.mustGetState(folder);
    const key = `${args.kind}:${args.name}`;

    const cached = state.entityDetailsCache.get(key);
    if (cached) return cached;

    const inflight = state.entityDetailsInflight.get(key);
    if (inflight) return await inflight;

    const p = this.execute<EntityDetailsResult>(folder, 'grimoirecss.entityDetails', [args])
      .then((res) => {
        state.entityDetailsCache.set(key, res);
        return res;
      })
      .finally(() => {
        state.entityDetailsInflight.delete(key);
      });

    state.entityDetailsInflight.set(key, p);
    return await p;
  }

  async getExplorerIndex(folder: vscode.WorkspaceFolder): Promise<ExplorerIndexResult> {
    const state = this.mustGetState(folder);
    if (!state.cache.explorerIndex) state.cache.explorerIndex = await this.explorerIndex(folder);
    return state.cache.explorerIndex;
  }

  async documentSpells(
    folder: vscode.WorkspaceFolder,
    args: DocumentSpellsArgs
  ): Promise<DocumentSpellsResult> {
    return this.execute<DocumentSpellsResult>(folder, 'grimoirecss.documentSpells', [args]);
  }

  async dryCreateScroll(
    folder: vscode.WorkspaceFolder,
    args: DryCreateScrollArgs
  ): Promise<LspWorkspaceEdit> {
    // Returned edit is generated by the LSP (authoritative ranges).
    return this.execute<LspWorkspaceEdit>(folder, 'grimoirecss.dryCreateScroll', [args]);
  }

  private async listScrolls(folder: vscode.WorkspaceFolder): Promise<ListScrollsResult> {
    return this.execute<ListScrollsResult>(folder, 'grimoirecss.listScrolls', [true]);
  }

  private async listVars(
    folder: vscode.WorkspaceFolder
  ): Promise<Array<{ name: string; value: string }>> {
    return this.execute<Array<{ name: string; value: string }>>(folder, 'grimoirecss.listVars', [
      true,
    ]);
  }

  private async configSummary(folder: vscode.WorkspaceFolder): Promise<ConfigSummary> {
    return this.execute<ConfigSummary>(folder, 'grimoirecss.configSummary', [true]);
  }

  private async explorerIndex(folder: vscode.WorkspaceFolder): Promise<ExplorerIndexResult> {
    return this.execute<ExplorerIndexResult>(folder, 'grimoirecss.explorerIndex', []);
  }

  private async lint(folder: vscode.WorkspaceFolder): Promise<LintResult> {
    return this.execute<LintResult>(folder, 'grimoirecss.lint', [true]);
  }

  private async dryCandidates(folder: vscode.WorkspaceFolder): Promise<DryCandidatesResult> {
    return this.execute<DryCandidatesResult>(folder, 'grimoirecss.dryCandidates', [
      undefined,
      undefined,
      true,
    ]);
  }

  private async execute<T>(
    folder: vscode.WorkspaceFolder,
    command: string,
    args: unknown[]
  ): Promise<T> {
    const state = this.mustGetState(folder);

    const timeoutMs =
      vscode.workspace
        .getConfiguration('grimoireCss')
        .get<number>('timeouts.executeCommandMs', 15000) ?? 15000;

    return executeCommand<T>(state.client, command, args, timeoutMs);
  }

  private mustGetState(folder: vscode.WorkspaceFolder): FolderState {
    const key = folder.uri.toString();
    const state = this.states.get(key);
    if (!state) {
      throw new Error(`No LSP client for folder: ${folder.name}`);
    }
    return state;
  }

  private async resolveServerPath(folder: vscode.WorkspaceFolder): Promise<string | undefined> {
    const cfg = vscode.workspace.getConfiguration('grimoireCss');

    const explicit = cfg.get<string | null>('lsp.path', null);
    if (explicit) {
      const ok = await isExecutable(explicit);
      if (!ok) {
        void vscode.window.showErrorMessage(
          `grimoireCss.lsp.path is set but not executable: ${explicit}`
        );
        return undefined;
      }
      return explicit;
    }

    const candidates: string[] = [];
    const exe = process.platform === 'win32' ? 'grimoire_css_lsp.exe' : 'grimoire_css_lsp';

    // In extension development, `cargo build` produces debug binaries by default.
    // Prefer debug so the user doesn't have to build `--release` to get new LSP commands.
    const preferDebug = this.context.extensionMode === vscode.ExtensionMode.Development;
    if (preferDebug) {
      candidates.push(path.join(folder.uri.fsPath, 'target', 'debug', exe));
      candidates.push(path.join(folder.uri.fsPath, 'target', 'release', exe));
    } else {
      candidates.push(path.join(folder.uri.fsPath, 'target', 'release', exe));
      candidates.push(path.join(folder.uri.fsPath, 'target', 'debug', exe));
    }

    // Dev ergonomics: when running the extension from this repo, the LSP is usually built at
    // <repoRoot>/target/(debug|release)/grimoire_css_lsp while the workspace might be a nested folder.
    // Check relative to extension development path.
    //
    // Note: this repo places the extension at <repoRoot>/extensions/vscode, so repoRoot is two levels up.
    const repoRootFromExtension = path.resolve(this.context.extensionPath, '..', '..');
    if (preferDebug) {
      candidates.push(path.join(repoRootFromExtension, 'target', 'debug', exe));
      candidates.push(path.join(repoRootFromExtension, 'target', 'release', exe));
    } else {
      candidates.push(path.join(repoRootFromExtension, 'target', 'release', exe));
      candidates.push(path.join(repoRootFromExtension, 'target', 'debug', exe));
    }

    // Also try walking up from workspace folder to find a parent target/ (useful when user opens
    // a nested folder inside a mono-repo).
    for (const p of await findUpTargetCandidates(folder.uri.fsPath, exe, 8)) {
      candidates.push(p);
    }

    for (const p of candidates) {
      if (await isExecutable(p)) {
        await this.maybeWarnForOldLocalVersion(p);
        return p;
      }
    }

    // PATH
    const pathCandidate = await canRunFromPath('grimoire_css_lsp');
    if (pathCandidate) {
      await this.maybeWarnForOldLocalVersion('grimoire_css_lsp');
      return 'grimoire_css_lsp';
    }

    // Previously downloaded server.
    const downloadedPath = this.context.globalState.get<string>('grimoireCss.lsp.downloadedPath');
    if (downloadedPath && (await isExecutable(downloadedPath))) {
      return downloadedPath;
    }

    const autoDownload = cfg.get<boolean>('lsp.autoDownload', true);
    if (!autoDownload) {
      void vscode.window.showErrorMessage(
        'No grimoire_css_lsp found. Build it with `cargo build --features lsp --bin grimoire_css_lsp` or set grimoireCss.lsp.path.'
      );
      return undefined;
    }

    const downloaded = await this.tryDownloadLatest(folder);
    if (downloaded) return downloaded;

    void vscode.window.showErrorMessage(
      'Unable to auto-download grimoire_css_lsp (no compatible release asset found). Build it locally or set grimoireCss.lsp.path.'
    );

    return undefined;
  }

  private async maybeWarnForOldLocalVersion(commandOrPath: string): Promise<void> {
    const prefer = vscode.workspace
      .getConfiguration('grimoireCss')
      .get<string>('lsp.prefer', 'auto');
    if (prefer === 'local') return;

    const local = await tryGetVersion(commandOrPath);
    const latest = await tryGetLatestReleaseVersion();

    if (!local || !latest) return;

    if (semver.valid(local) && semver.valid(latest) && semver.lt(local, latest)) {
      const picked = await vscode.window.showInformationMessage(
        `Local grimoire_css_lsp v${local} is older than latest release v${latest}.`,
        'Use local anyway',
        'Download latest',
        'Always prefer local'
      );

      if (picked === 'Always prefer local') {
        await vscode.workspace.getConfiguration('grimoireCss').update('lsp.prefer', 'local', true);
      }

      if (picked === 'Download latest') {
        // Download is performed later if local missing; here we just set preference.
        await vscode.workspace
          .getConfiguration('grimoireCss')
          .update('lsp.prefer', 'downloaded', true);
      }
    }
  }

  private async tryDownloadLatest(_folder: vscode.WorkspaceFolder): Promise<string | undefined> {
    const prefer = vscode.workspace
      .getConfiguration('grimoireCss')
      .get<string>('lsp.prefer', 'auto');
    if (prefer === 'local') return undefined;

    // Minimal first implementation: download a raw executable asset (or a .zip/.tar.gz we can extract via system tools).
    const release = await fetchLatestRelease();
    if (!release) return undefined;

    const platformArch = platformArchKey();
    const asset = release.assets.find(
      (a) => a.name.includes('grimoire_css_lsp') && a.name.includes(platformArch)
    );
    if (!asset) return undefined;

    const storageDir = this.context.globalStorageUri;
    await vscode.workspace.fs.createDirectory(storageDir);

    const downloadUri = vscode.Uri.joinPath(storageDir, asset.name);
    await downloadFile(asset.browser_download_url, downloadUri);

    const outPath = await materializeExecutable(downloadUri, storageDir);
    if (!outPath) return undefined;

    await this.context.globalState.update('grimoireCss.lsp.downloadedPath', outPath);
    await this.context.globalState.update('grimoireCss.lsp.downloadedVersion', release.tag_name);

    return outPath;
  }
}

async function withTimeout<T>(p: Promise<T>, timeoutMs: number, message: string): Promise<T> {
  let timeoutId: NodeJS.Timeout | undefined;
  const timeoutPromise = new Promise<never>((_, reject) => {
    timeoutId = setTimeout(() => reject(new Error(message)), timeoutMs);
  });

  try {
    return await Promise.race([p, timeoutPromise]);
  } finally {
    if (timeoutId) clearTimeout(timeoutId);
  }
}

async function isExecutable(p: string): Promise<boolean> {
  try {
    await access(p, fsConstants.X_OK);
    return true;
  } catch {
    return false;
  }
}

async function findUpTargetCandidates(
  startDir: string,
  exeName: string,
  maxDepth: number
): Promise<string[]> {
  const results: string[] = [];
  let current = startDir;

  for (let depth = 0; depth <= maxDepth; depth++) {
    results.push(path.join(current, 'target', 'release', exeName));
    results.push(path.join(current, 'target', 'debug', exeName));

    const parent = path.dirname(current);
    if (parent === current) break;
    current = parent;
  }

  // Deduplicate while preserving order.
  return Array.from(new Set(results));
}

async function canRunFromPath(cmd: string): Promise<boolean> {
  const v = await tryGetVersion(cmd);
  return v !== undefined;
}

async function tryGetVersion(cmdOrPath: string): Promise<string | undefined> {
  return new Promise((resolve) => {
    const child = spawn(cmdOrPath, ['--version'], {
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    let out = '';
    child.stdout.on('data', (d) => {
      out += String(d);
    });
    child.on('error', () => resolve(undefined));
    child.on('close', (code) => {
      if (code !== 0) return resolve(undefined);
      // Expect: "grimoire_css_lsp 1.2.3" or just "1.2.3"
      const m = out.trim().match(/(\d+\.\d+\.\d+)/);
      resolve(m?.[1]);
    });
  });
}

async function tryGetLatestReleaseVersion(): Promise<string | undefined> {
  const rel = await fetchLatestRelease();
  if (!rel) return undefined;
  const m = rel.tag_name.match(/v?(\d+\.\d+\.\d+)/);
  return m?.[1];
}

type GithubRelease = {
  tag_name: string;
  assets: Array<{ name: string; browser_download_url: string }>;
};

async function fetchLatestRelease(): Promise<GithubRelease | undefined> {
  try {
    const url = 'https://api.github.com/repos/persevie/grimoire-css/releases/latest';
    const res = await fetchWithTimeout(
      url,
      {
        headers: {
          'User-Agent': 'grimoirecss-vscode',
        },
      },
      15_000
    );
    if (!res.ok) return undefined;
    const json = (await res.json()) as GithubRelease;
    if (!json?.tag_name) return undefined;
    return json;
  } catch {
    return undefined;
  }
}

function platformArchKey(): string {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === 'darwin' && arch === 'arm64') return 'darwin-arm64';
  if (platform === 'darwin' && arch === 'x64') return 'darwin-x64';
  if (platform === 'linux' && arch === 'x64') return 'linux-x64';
  if (platform === 'win32' && arch === 'x64') return 'win32-x64';

  return `${platform}-${arch}`;
}

async function downloadFile(url: string, dest: vscode.Uri): Promise<void> {
  const res = await fetchWithTimeout(
    url,
    {
      headers: {
        'User-Agent': 'grimoirecss-vscode',
      },
    },
    60_000
  );
  if (!res.ok) throw new Error(`Download failed: ${res.status} ${res.statusText}`);

  const buffer = Buffer.from(await res.arrayBuffer());
  await vscode.workspace.fs.writeFile(dest, buffer);
}

async function fetchWithTimeout(
  url: string,
  init: RequestInit,
  timeoutMs: number
): Promise<Response> {
  if (typeof fetch !== 'function') {
    throw new Error('fetch() is not available in this VS Code runtime');
  }

  const controller = new AbortController();
  const id = setTimeout(() => controller.abort(), timeoutMs);
  try {
    return await fetch(url, {
      ...init,
      signal: controller.signal,
    });
  } finally {
    clearTimeout(id);
  }
}

async function materializeExecutable(
  downloaded: vscode.Uri,
  storageDir: vscode.Uri
): Promise<string | undefined> {
  const name = path.basename(downloaded.fsPath);

  // If it's a raw executable.
  const isRawExe =
    name === 'grimoire_css_lsp' ||
    name === 'grimoire_css_lsp.exe' ||
    name.startsWith('grimoire_css_lsp-') ||
    name.endsWith('.exe');

  if (isRawExe) {
    const finalUri = vscode.Uri.joinPath(
      storageDir,
      process.platform === 'win32' ? 'grimoire_css_lsp.exe' : 'grimoire_css_lsp'
    );
    await vscode.workspace.fs.copy(downloaded, finalUri, { overwrite: true });
    if (process.platform !== 'win32') {
      await fs.chmod(finalUri.fsPath, 0o755);
    }
    return finalUri.fsPath;
  }

  // Attempt to extract archives via system tools.
  if (name.endsWith('.zip')) {
    return extractZip(downloaded, storageDir);
  }

  if (name.endsWith('.tar.gz') || name.endsWith('.tgz')) {
    return extractTarGz(downloaded, storageDir);
  }

  return undefined;
}

async function extractZip(zipPath: vscode.Uri, outDir: vscode.Uri): Promise<string | undefined> {
  const _target = vscode.Uri.joinPath(outDir, 'grimoire_css_lsp');

  if (process.platform === 'win32') {
    // Best-effort: PowerShell Expand-Archive
    await execProcess('powershell', [
      '-NoProfile',
      '-NonInteractive',
      '-Command',
      `Expand-Archive -Force -Path "${zipPath.fsPath}" -DestinationPath "${outDir.fsPath}"`,
    ]);
  } else {
    await execProcess('unzip', ['-o', zipPath.fsPath, '-d', outDir.fsPath]);
  }

  const exe = process.platform === 'win32' ? 'grimoire_css_lsp.exe' : 'grimoire_css_lsp';
  const found = await findFileRecursively(outDir.fsPath, exe);
  if (!found) return undefined;

  const finalUri = vscode.Uri.joinPath(outDir, exe);
  await vscode.workspace.fs.copy(vscode.Uri.file(found), finalUri, {
    overwrite: true,
  });
  if (process.platform !== 'win32') {
    await fs.chmod(finalUri.fsPath, 0o755);
  }
  return finalUri.fsPath;
}

async function extractTarGz(tarPath: vscode.Uri, outDir: vscode.Uri): Promise<string | undefined> {
  await execProcess('tar', ['-xzf', tarPath.fsPath, '-C', outDir.fsPath]);
  const exe = process.platform === 'win32' ? 'grimoire_css_lsp.exe' : 'grimoire_css_lsp';
  const found = await findFileRecursively(outDir.fsPath, exe);
  if (!found) return undefined;

  const finalUri = vscode.Uri.joinPath(outDir, exe);
  await vscode.workspace.fs.copy(vscode.Uri.file(found), finalUri, {
    overwrite: true,
  });
  if (process.platform !== 'win32') {
    await fs.chmod(finalUri.fsPath, 0o755);
  }
  return finalUri.fsPath;
}

async function findFileRecursively(root: string, fileName: string): Promise<string | undefined> {
  const entries = await fs.readdir(root, { withFileTypes: true });
  for (const e of entries) {
    const full = path.join(root, e.name);
    if (e.isFile() && e.name === fileName) return full;
    if (e.isDirectory()) {
      const nested = await findFileRecursively(full, fileName);
      if (nested) return nested;
    }
  }
  return undefined;
}

async function execProcess(cmd: string, args: string[]): Promise<void> {
  await new Promise<void>((resolve, reject) => {
    const child = spawn(cmd, args, { stdio: 'ignore' });
    child.on('error', reject);
    child.on('close', (code) =>
      code === 0 ? resolve() : reject(new Error(`${cmd} exited with ${code}`))
    );
  });
}
