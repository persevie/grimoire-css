import * as path from 'node:path';
import * as vscode from 'vscode';
import { registerEditorHighlights } from './features/editorHighlights';
import { registerShowDetailsCodeActions } from './features/showDetailsCodeActions';
import { LspManager } from './lsp/lspManager';
import { openVirtualJson, openVirtualText } from './preview/virtualDocuments';
import { Logger } from './utils/log';
import { ActionsViewProvider } from './views/actionsView';
import { ConfigViewProvider } from './views/configView';
import { DetailsViewProvider } from './views/detailsView';
import { DiagnosticsView } from './views/diagnosticsView';
import { DryView } from './views/dryView';
import { ExplorerViewProvider } from './views/explorerView';
import { findWorkspaceFoldersWithConfig } from './workspace/workspaceRoots';

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  const getTraceLevel = () =>
    (vscode.workspace.getConfiguration('grimoireCss').get('trace') as
      | 'off'
      | 'messages'
      | 'verbose'
      | undefined) ?? 'off';

  const logger = new Logger(getTraceLevel);
  const extensionVersion = String(context.extension.packageJSON.version ?? 'unknown');
  logger.info(`GrimoireCSS extension activating (v${extensionVersion})...`);
  vscode.window.setStatusBarMessage(`GrimoireCSS extension v${extensionVersion} activated`, 5000);

  const lsp = new LspManager(context, logger);
  const editorHighlights = registerEditorHighlights(context, lsp, logger);

  // CLI helpers (build/init/shorten). Run in an integrated terminal.
  // By default this invokes `grimoire_css` from PATH, but it can be overridden via settings
  // to avoid accidentally using a stale binary during extension development.
  let cliTerminal: vscode.Terminal | undefined;
  let cliTerminalCwd: string | undefined;

  const getCliTerminal = (folder: vscode.WorkspaceFolder): vscode.Terminal => {
    const cwd = folder.uri.fsPath;
    if (cliTerminal && cliTerminalCwd === cwd) return cliTerminal;

    try {
      cliTerminal?.dispose();
    } catch {
      // ignore
    }

    cliTerminal = vscode.window.createTerminal({ name: 'GrimoireCSS', cwd });
    cliTerminalCwd = cwd;
    context.subscriptions.push({ dispose: () => cliTerminal?.dispose() });
    return cliTerminal;
  };

  const runCliInTerminal = async (
    folder: vscode.WorkspaceFolder,
    args: string[]
  ): Promise<void> => {
    const terminal = getCliTerminal(folder);
    terminal.show(true);

    const cliCommand =
      vscode.workspace.getConfiguration('grimoireCss').get<string>('cli.command') || 'grimoire_css';
    const quotedCmd = cliCommand.includes(' ')
      ? `"${cliCommand.replaceAll('"', '\\"')}"`
      : cliCommand;

    // Intentionally no fallback and no preflight PATH checks:
    // the integrated terminal environment can differ from the extension host environment,
    // especially on macOS when VS Code is launched from the Dock.
    terminal.sendText([quotedCmd, ...args].join(' '));
  };

  const diagnosticsStatus = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
  diagnosticsStatus.command = 'grimoirecss.diagnostics.focus';
  const dryStatus = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 99);
  dryStatus.command = 'grimoirecss.dry.focus';

  const getPreferredWorkspaceFolder = (): vscode.WorkspaceFolder | undefined => {
    const editor = vscode.window.activeTextEditor;
    if (editor) {
      const folder = vscode.workspace.getWorkspaceFolder(editor.document.uri);
      if (folder) return folder;
    }
    return vscode.workspace.workspaceFolders?.[0];
  };

  const updateStatusCounts = (): void => {
    const folder = lsp.getActiveOrFirstFolder();
    if (!folder) {
      diagnosticsStatus.hide();
      dryStatus.hide();
      return;
    }

    const cache = lsp.getCache(folder);
    const lint = cache.lint;
    const errors = lint?.errors?.length ?? 0;
    const warnings = lint?.warnings?.length ?? 0;
    const notes = lint?.notes?.length ?? 0;
    diagnosticsStatus.text = lint
      ? `$(error) ${errors} $(warning) ${warnings}`
      : '$(error) – $(warning) –';
    diagnosticsStatus.tooltip = lint
      ? `GrimoireCSS Diagnostics\nErrors: ${errors}\nWarnings: ${warnings}\nNotes: ${notes}\n\nClick to open Diagnostics view.`
      : 'GrimoireCSS Diagnostics\nRun Check (Lint) to populate.\n\nClick to open Diagnostics view.';
    diagnosticsStatus.show();

    const dry = cache.dry;
    const candidates = dry?.candidates?.length ?? 0;
    dryStatus.text = dry ? `DRY: ${candidates}` : 'DRY: –';
    dryStatus.tooltip = dry
      ? `GrimoireCSS DRY\nCandidates: ${candidates}\n\nClick to open DRY view.`
      : 'GrimoireCSS DRY\nRun Check DRY to populate.\n\nClick to open DRY view.';
    dryStatus.show();
  };

  updateStatusCounts();
  context.subscriptions.push(diagnosticsStatus, dryStatus);
  context.subscriptions.push(vscode.window.onDidChangeActiveTextEditor(updateStatusCounts));

  const folders = await findWorkspaceFoldersWithConfig();
  if (folders.length === 0) {
    logger.info(
      'No workspace folder with grimoire/config/grimoire.config.json found. Views will be empty until a config is present.'
    );
  }

  for (const folder of folders) {
    try {
      await lsp.ensureStarted(folder);
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      logger.info(`Failed to start LSP for ${folder.name}: ${msg}`);
      void vscode.window.showErrorMessage(
        `GrimoireCSS: failed to start LSP for ${folder.name}: ${msg}`
      );
    }
  }

  // Apply editor highlights immediately after startup.
  // Without this, highlights may only appear after the first editor/theme/config event.
  editorHighlights.refreshVisibleEditors();

  const commands = {
    refresh: 'persevie.grimoirecss-vscode.refresh',
    check: 'persevie.grimoirecss-vscode.check',
    checkDry: 'persevie.grimoirecss-vscode.checkDry',
    checkLint: 'persevie.grimoirecss-vscode.checkLint',
    dryCreateScroll: 'persevie.grimoirecss-vscode.dryCreateScroll',
    showDetails: 'persevie.grimoirecss-vscode.showDetails',
    openConfig: 'persevie.grimoirecss-vscode.openConfig',
    restartLsp: 'persevie.grimoirecss-vscode.restartLsp',
    explainToken: 'persevie.grimoirecss-vscode.explainToken',
    findRefs: 'persevie.grimoirecss-vscode.findRefs',
    stats: 'persevie.grimoirecss-vscode.stats',
    build: 'persevie.grimoirecss-vscode.build',
    init: 'persevie.grimoirecss-vscode.init',
    shorten: 'persevie.grimoirecss-vscode.shorten',
    toggleEditorHighlights: 'persevie.grimoirecss-vscode.toggleEditorHighlights',
    openSettings: 'persevie.grimoirecss-vscode.openSettings',
    openLocation: 'persevie.grimoirecss-vscode._openLocation',
    selectEntity: 'persevie.grimoirecss-vscode._selectEntity',
  };

  // Keep a context key for future menu gating (not currently used).
  const setHasConfigContextForFolder = async (
    folder: vscode.WorkspaceFolder | undefined
  ): Promise<void> => {
    if (!folder) {
      await vscode.commands.executeCommand('setContext', 'grimoirecss.hasConfig', false);
      return;
    }

    const configUri = vscode.Uri.joinPath(folder.uri, 'grimoire', 'config', 'grimoire.config.json');
    let hasConfig = false;
    try {
      const st = await vscode.workspace.fs.stat(configUri);
      hasConfig = st.type === vscode.FileType.File;
    } catch {
      hasConfig = false;
    }
    await vscode.commands.executeCommand('setContext', 'grimoirecss.hasConfig', hasConfig);
  };

  const refreshHasConfigContext = async (): Promise<void> => {
    await setHasConfigContextForFolder(getPreferredWorkspaceFolder());
  };

  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor(() => void refreshHasConfigContext())
  );
  await refreshHasConfigContext();

  const actionsProvider = new ActionsViewProvider(context, logger);
  const explorerProvider = new ExplorerViewProvider(context, lsp, logger, {
    selectEntity: commands.selectEntity,
  });
  const diagnosticsView = new DiagnosticsView(lsp, logger, {
    openLocation: commands.openLocation,
  });
  const dryView = new DryView(lsp, logger, {
    openLocation: commands.openLocation,
  });

  const detailsProvider = new DetailsViewProvider(context, lsp, logger, {
    openLocation: commands.openLocation,
  });
  const configProvider = new ConfigViewProvider(context, lsp, logger, {
    selectEntity: commands.selectEntity,
    openLocation: commands.openLocation,
  });

  context.subscriptions.push(
    diagnosticsView,
    dryView,
    vscode.window.registerWebviewViewProvider('grimoirecss.tools', actionsProvider),
    vscode.window.registerWebviewViewProvider('grimoirecss.explorer', explorerProvider),
    vscode.window.registerWebviewViewProvider('grimoirecss.details', detailsProvider),
    vscode.window.registerWebviewViewProvider('grimoirecss.config', configProvider)
  );

  context.subscriptions.push(
    vscode.window.onDidChangeActiveColorTheme(async () => {
      await actionsProvider.render();
      await configProvider.render();
      await detailsProvider.render();
      await explorerProvider.render();
    })
  );

  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration('grimoireCss.editorHighlights')) {
        editorHighlights.refreshVisibleEditors();
      }
    })
  );

  safeRegisterCommand(context, logger, commands.toggleEditorHighlights, async () => {
    const cfg = vscode.workspace.getConfiguration('grimoireCss');
    const current = cfg.get<boolean>('editorHighlights.enable', true);
    await cfg.update('editorHighlights.enable', !current, true);
    editorHighlights.refreshVisibleEditors();
  });

  safeRegisterCommand(context, logger, commands.openSettings, async () => {
    await vscode.commands.executeCommand(
      'workbench.action.openSettings',
      '@ext:persevie.grimoirecss-vscode'
    );
  });

  safeRegisterCommand(context, logger, commands.build, async () => {
    const folder = getPreferredWorkspaceFolder();
    if (!folder) {
      void vscode.window.showWarningMessage('No GrimoireCSS workspace folder found.');
      return;
    }

    await runCliInTerminal(folder, ['build']);
  });

  safeRegisterCommand(context, logger, commands.init, async () => {
    const folder = getPreferredWorkspaceFolder();
    if (!folder) {
      void vscode.window.showWarningMessage('No GrimoireCSS workspace folder found.');
      return;
    }

    await runCliInTerminal(folder, ['init']);
  });

  safeRegisterCommand(context, logger, commands.shorten, async () => {
    const folder = getPreferredWorkspaceFolder();
    if (!folder) {
      void vscode.window.showWarningMessage('No GrimoireCSS workspace folder found.');
      return;
    }

    await runCliInTerminal(folder, ['shorten']);
  });

  safeRegisterCommand(
    context,
    logger,
    commands.openLocation,
    async (loc: { uri: vscode.Uri; line?: number; character?: number }) => {
      try {
        const st = await vscode.workspace.fs.stat(loc.uri);
        if (st.type === vscode.FileType.Directory) {
          await vscode.commands.executeCommand('revealInExplorer', loc.uri);
          return;
        }
      } catch {
        // ignore; we'll attempt to open as a document below.
      }

      const doc = await vscode.workspace.openTextDocument(loc.uri);
      const editor = await vscode.window.showTextDocument(doc, {
        preview: true,
      });
      if (typeof loc.line === 'number' && typeof loc.character === 'number') {
        const pos = new vscode.Position(loc.line, loc.character);
        editor.selection = new vscode.Selection(pos, pos);
        editor.revealRange(new vscode.Range(pos, pos), vscode.TextEditorRevealType.InCenter);
      }
    }
  );

  safeRegisterCommand(
    context,
    logger,
    commands.selectEntity,
    async (arg: { kind: 'scroll' | 'var' | 'function' | 'animation'; name: string }) => {
      const folder = lsp.getActiveOrFirstFolder();
      if (!folder) {
        void vscode.window.showWarningMessage('No GrimoireCSS workspace folder found.');
        return;
      }

      const kind = arg?.kind;
      const name = (arg?.name ?? '').trim();
      if (
        (kind !== 'scroll' && kind !== 'var' && kind !== 'function' && kind !== 'animation') ||
        name.length === 0
      )
        return;

      // Ensure the Details view is revealed even if it hasn't been opened before.
      try {
        await vscode.commands.executeCommand('grimoirecss.details.focus');
      } catch {
        // ignore
      }

      await detailsProvider.setSelection({ folder, kind, name });
    }
  );

  safeRegisterCommand(context, logger, commands.refresh, async () => {
    const startedAt = Date.now();
    logger.info('Refresh started...');
    await lsp.refreshAll();
    await explorerProvider.render();
    diagnosticsView.refresh();
    dryView.refresh();
    await configProvider.render();
    await detailsProvider.render();
    editorHighlights.refreshVisibleEditors();
    updateStatusCounts();
    await refreshHasConfigContext();
    logger.info(`Refresh finished in ${Date.now() - startedAt}ms`);
  });

  safeRegisterCommand(context, logger, commands.check, async () => {
    const startedAt = Date.now();
    logger.info('Check started...');
    await lsp.checkAll();
    diagnosticsView.refresh();
    dryView.refresh();
    updateStatusCounts();
    logger.info(`Check finished in ${Date.now() - startedAt}ms`);
  });

  safeRegisterCommand(context, logger, commands.checkDry, async () => {
    const folder = lsp.getActiveOrFirstFolder();
    if (!folder) {
      void vscode.window.showWarningMessage('No GrimoireCSS workspace folder found.');
      return;
    }

    logger.info('DRY check started...');
    await lsp.runDry(folder);
    dryView.refresh();
    updateStatusCounts();
    logger.info('DRY check finished.');
  });

  safeRegisterCommand(context, logger, commands.checkLint, async () => {
    const folder = lsp.getActiveOrFirstFolder();
    if (!folder) {
      void vscode.window.showWarningMessage('No GrimoireCSS workspace folder found.');
      return;
    }

    logger.info('Lint check started...');
    await lsp.runLint(folder);
    diagnosticsView.refresh();
    updateStatusCounts();
    logger.info('Lint check finished.');
  });

  safeRegisterCommand(context, logger, commands.dryCreateScroll, async (node: unknown) => {
    const folder = lsp.getActiveOrFirstFolder();
    if (!folder) {
      void vscode.window.showWarningMessage('No GrimoireCSS workspace folder found.');
      return;
    }

    const tokensRaw =
      node && typeof node === 'object' ? (node as Record<string, unknown>).tokens : undefined;
    const tokens: string[] = Array.isArray(tokensRaw)
      ? tokensRaw.filter((t): t is string => typeof t === 'string')
      : [];
    if (tokens.length === 0) {
      void vscode.window.showWarningMessage('Select a DRY candidate first.');
      return;
    }

    const scrollName = await vscode.window.showInputBox({
      title: 'Create scroll from DRY candidate',
      prompt: 'New scroll name (will be added to grimoire.config.json)',
      placeHolder: 'e.g. shared-card',
      validateInput: (v) => {
        const s = (v ?? '').trim();
        if (!s) return 'Scroll name is required';
        if (!/^[A-Za-z0-9_.-]+$/.test(s)) return 'Allowed: A-Z a-z 0-9 _ . -';
        return undefined;
      },
    });

    if (!scrollName) return;

    const edit = await lsp.dryCreateScroll(folder, {
      scrollName: scrollName.trim(),
      tokens,
    });
    const changes = edit?.changes;

    const wsEdit = new vscode.WorkspaceEdit();
    for (const [uriStr, textEdits] of Object.entries(changes ?? {})) {
      let uri: vscode.Uri;
      try {
        uri = vscode.Uri.parse(uriStr);
      } catch {
        continue;
      }
      const edits = Array.isArray(textEdits) ? textEdits : [];
      for (const te of edits) {
        const r = te?.range;
        if (!r) continue;
        const start = new vscode.Position(r.start.line, r.start.character);
        const end = new vscode.Position(r.end.line, r.end.character);
        wsEdit.replace(uri, new vscode.Range(start, end), te.newText ?? '');
      }
    }

    const applied = await vscode.workspace.applyEdit(wsEdit);
    if (!applied) {
      void vscode.window.showErrorMessage('Failed to apply edits for DRY scroll extraction.');
      return;
    }

    // Rebuild caches and refresh views/highlights.
    await vscode.commands.executeCommand(commands.refresh);
  });

  safeRegisterCommand(context, logger, commands.openConfig, async () => {
    const folder = lsp.getActiveOrFirstFolder();
    if (!folder) {
      void vscode.window.showWarningMessage('No GrimoireCSS workspace folder found.');
      return;
    }

    try {
      await vscode.commands.executeCommand('grimoirecss.config.focus');
    } catch {
      // ignore
    }
    await configProvider.render();
    configProvider.show();
  });

  safeRegisterCommand(context, logger, commands.restartLsp, async () => {
    logger.info('Restart LSP requested...');
    await lsp.restartAll();
    await lsp.refreshAll();
    await explorerProvider.render();
    diagnosticsView.refresh();
    dryView.refresh();
    await configProvider.render();
    await detailsProvider.render();
    editorHighlights.refreshVisibleEditors();
  });

  safeRegisterCommand(context, logger, commands.showDetails, async (tokenArg?: string) => {
    const folder = lsp.getActiveOrFirstFolder();
    if (!folder) {
      void vscode.window.showWarningMessage('No GrimoireCSS workspace folder found.');
      return;
    }

    const token = tokenArg ?? (await promptForToken('Show details'));
    if (!token) return;

    const parsed = parseEntityToken(token);
    if (!parsed) {
      void vscode.window.showWarningMessage('Expected a scroll name or $variable.');
      return;
    }

    try {
      await vscode.commands.executeCommand('grimoirecss.details.focus');
    } catch {
      // ignore
    }

    await detailsProvider.setSelection({
      folder,
      kind: parsed.kind,
      name: parsed.name,
    });
  });

  safeRegisterCommand(context, logger, commands.explainToken, async (tokenArg?: string) => {
    const folder = lsp.getActiveOrFirstFolder();
    if (!folder) {
      void vscode.window.showWarningMessage('No GrimoireCSS workspace folder found.');
      return;
    }

    const token = tokenArg ?? (await promptForToken('Explain token'));
    if (!token) return;

    const result = await lsp.explain(folder, token);
    await openVirtualText(
      `grimoirecss://preview/${encodeURIComponent(token)}.css`,
      result.css,
      'css'
    );
  });

  safeRegisterCommand(context, logger, commands.findRefs, async (tokenArg?: string) => {
    const folder = lsp.getActiveOrFirstFolder();
    if (!folder) {
      void vscode.window.showWarningMessage('No GrimoireCSS workspace folder found.');
      return;
    }

    const token = tokenArg ?? (await promptForToken('Find refs'));
    if (!token) return;

    const refs = await lsp.refs(folder, token);
    const pretty = JSON.stringify(refs, null, 2);
    await openVirtualJson(`grimoirecss://refs/${encodeURIComponent(token)}.json`, refs);
    logger.trace(pretty);
  });

  const statsHandler = async (tokenArg?: string) => {
    const folder = lsp.getActiveOrFirstFolder();
    if (!folder) {
      void vscode.window.showWarningMessage('No GrimoireCSS workspace folder found.');
      return;
    }

    const mode = await vscode.window.showQuickPick(
      [
        { label: 'Top spells', value: { group: 'spells' as const } },
        { label: 'Top scrolls', value: { group: 'scrolls' as const } },
        { label: 'Top vars', value: { group: 'vars' as const } },
        { label: 'Token stats', value: { token: tokenArg ?? '' } },
      ],
      { title: 'GrimoireCSS Stats', ignoreFocusOut: true }
    );

    if (!mode) return;

    if ('token' in mode.value) {
      const token = mode.value.token || (await promptForToken('Stats for token'));
      if (!token) return;
      const res = await lsp.stats(folder, { token });
      await openVirtualJson(`grimoirecss://stats/token/${encodeURIComponent(token)}.json`, res);
      return;
    }

    const topStr = await vscode.window.showInputBox({
      title: 'Top N',
      prompt: 'How many items?',
      value: '50',
      validateInput: (v) => {
        const n = Number(v);
        return Number.isFinite(n) && n > 0 ? undefined : 'Enter a positive number';
      },
    });
    if (!topStr) return;

    const top = Number(topStr);
    const res = await lsp.stats(folder, { group: mode.value.group, top });
    await openVirtualJson(`grimoirecss://stats/top/${mode.value.group}.json`, res);
  };

  safeRegisterCommand(context, logger, commands.stats, statsHandler);

  // Warm cache on first activation if at least one LSP is running.
  if (lsp.getFolders().length > 0) {
    await lsp.refreshAll();
    await lsp.checkAll();
    await explorerProvider.render();
    diagnosticsView.refresh();
    dryView.refresh();
    await configProvider.render();
    await detailsProvider.render();
    updateStatusCounts();
    await refreshHasConfigContext();

    // Auto-run lint + DRY after changes in project inputs and grimoire/ configs.
    setupAutoChecks(context, logger, lsp, {
      explorerProvider,
      diagnosticsView,
      dryView,
      configProvider,
      detailsProvider,
      updateStatusCounts,
      refreshHasConfigContext,
    });
  }

  // Provide a single extension-side Show Details quick action. LSP actions are separately labeled.
  context.subscriptions.push(registerShowDetailsCodeActions({ showDetails: commands.showDetails }));

  logger.info('GrimoireCSS extension activated.');
}

type AutoCheckViews = {
  explorerProvider: ExplorerViewProvider;
  diagnosticsView: DiagnosticsView;
  dryView: DryView;
  sharedView?: { refresh: () => void };
  configProvider: ConfigViewProvider;
  detailsProvider: DetailsViewProvider;
  updateStatusCounts: () => void;
  refreshHasConfigContext: () => Promise<void>;
};

function setupAutoChecks(
  context: vscode.ExtensionContext,
  logger: Logger,
  lsp: LspManager,
  views: AutoCheckViews
): void {
  const perFolder = new Map<
    string,
    {
      watchers: vscode.Disposable[];
      timer?: NodeJS.Timeout;
      running?: boolean;
      pending?: boolean;
      needsRefresh?: boolean;
      needsCheck?: boolean;
      rebuildWatchers?: boolean;
    }
  >();

  const buildWatchersForFolder = async (folder: vscode.WorkspaceFolder): Promise<void> => {
    const key = folder.uri.toString();

    // Dispose existing watchers.
    const existing = perFolder.get(key);
    if (existing) {
      for (const w of existing.watchers) w.dispose();
      existing.watchers = [];
      existing.rebuildWatchers = false;
    }

    const entry = existing ?? { watchers: [] as vscode.Disposable[] };
    perFolder.set(key, entry);

    let patterns: string[] = ['grimoire/**'];
    try {
      const summary = await lsp.getConfigSummary(folder);
      const inputPaths = (summary.projects ?? []).flatMap(
        (project: { input_paths?: string[] }) => project.input_paths ?? []
      );
      patterns.push(
        ...inputPaths.flatMap((inputPath: string) => deriveWatchPatternsFromInputPath(inputPath))
      );
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      logger.info(`Auto-check: failed to read configSummary for ${folder.name}: ${msg}`);
    }

    // De-dup patterns.
    patterns = Array.from(
      new Set(patterns.map((p) => p.replace(/\\/g, '/').replace(/^\.(\/|\\)/, '')))
    );

    for (const pattern of patterns) {
      const isAbsolute =
        pattern.startsWith('/') || /^[A-Za-z]:[\\/]/.test(pattern) || pattern.startsWith('file:');
      const watcher = isAbsolute
        ? vscode.workspace.createFileSystemWatcher(pattern)
        : vscode.workspace.createFileSystemWatcher(new vscode.RelativePattern(folder, pattern));
      const handler = (uri: vscode.Uri) => {
        const st = perFolder.get(key);
        if (!st) return;
        const fsPath = uri.fsPath.replace(/\\/g, '/');

        // Project inputs only need lint/DRY; full refresh is only needed when config / shared inputs change.
        if (isConfigFile(fsPath)) {
          st.needsRefresh = true;
          st.needsCheck = true;
          st.rebuildWatchers = true;
        } else if (isSharedConfigInput(fsPath)) {
          st.needsRefresh = true;
          st.needsCheck = true;
        } else if (isCustomAnimationFile(fsPath)) {
          // Custom animations are discovered from grimoire/animations/*.css on config load,
          // so we need a refresh to re-read them.
          st.needsRefresh = true;
          st.needsCheck = true;
        } else {
          st.needsCheck = true;
        }

        schedule(folder);
      };
      watcher.onDidChange(handler);
      watcher.onDidCreate(handler);
      watcher.onDidDelete(handler);
      entry.watchers.push(watcher);
      context.subscriptions.push(watcher);
    }

    logger.trace(`Auto-check watching ${patterns.length} pattern(s) for ${folder.name}`);
  };

  const schedule = (folder: vscode.WorkspaceFolder): void => {
    const key = folder.uri.toString();
    const state = perFolder.get(key);
    if (!state) return;

    // Coalesce triggers while a run is in-flight.
    if (state.running) {
      state.pending = true;
      return;
    }

    if (state.timer) clearTimeout(state.timer);
    state.timer = setTimeout(() => {
      void run(folder);
    }, 500);
  };

  const run = async (folder: vscode.WorkspaceFolder): Promise<void> => {
    const key = folder.uri.toString();
    const state = perFolder.get(key);
    if (!state) return;

    if (state.running) {
      state.pending = true;
      return;
    }

    const shouldRebuild = Boolean(state.rebuildWatchers);
    const shouldRefresh = Boolean(state.needsRefresh || shouldRebuild);
    const shouldCheck = Boolean(state.needsCheck || shouldRefresh);

    state.running = true;
    state.pending = false;
    state.needsRefresh = false;
    state.needsCheck = false;
    state.rebuildWatchers = false;

    try {
      if (shouldRefresh) {
        await lsp.refreshFolder(folder);
      }
      if (shouldCheck) {
        await lsp.checkFolder(folder);
      }

      await views.explorerProvider.render();
      views.diagnosticsView.refresh();
      views.dryView.refresh();
      views.sharedView?.refresh();
      await views.configProvider.render();
      await views.detailsProvider.render();

      views.updateStatusCounts();
      await views.refreshHasConfigContext();

      if (shouldRebuild) {
        await buildWatchersForFolder(folder);
      }
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      logger.info(`Auto-check failed for ${folder.name}: ${msg}`);
    } finally {
      const st = perFolder.get(key);
      if (st) {
        st.running = false;
        if (st.pending) {
          // Run one more time to catch changes that happened during the in-flight run.
          st.pending = false;
          schedule(folder);
        }
      }
    }
  };

  for (const folder of lsp.getFolders()) {
    void buildWatchersForFolder(folder);
  }

  context.subscriptions.push({
    dispose() {
      for (const entry of perFolder.values()) {
        if (entry.timer) clearTimeout(entry.timer);
        for (const w of entry.watchers) w.dispose();
      }
      perFolder.clear();
    },
  });
}

function deriveWatchPatternsFromInputPath(inputPath: string): string[] {
  const raw = (inputPath ?? '').trim();
  if (!raw) return [];

  const p = raw.replace(/\\/g, '/').replace(/^\.(\/|\\)/, '');
  const hasGlob = /[*?[]/.test(p);
  const firstGlobIdx = hasGlob ? p.search(/[*?[]/) : -1;

  // If the pattern has no fixed prefix, keep the glob as-is (avoid broad '**/*' watchers).
  if (hasGlob && firstGlobIdx === 0) return [p];

  // Coarsen to a directory watch to avoid creating many fine-grained watchers.
  const prefix = hasGlob && firstGlobIdx > 0 ? p.slice(0, firstGlobIdx) : p;
  let dir = prefix;

  if (dir.endsWith('/')) {
    dir = dir.replace(/\/+$/, '');
  } else {
    const ext = path.posix.extname(dir);
    dir = ext ? path.posix.dirname(dir) : dir;
  }

  if (!dir || dir === '.') {
    // Fallback to original string if we can't infer a better prefix.
    return [p];
  }

  return [`${dir}/**/*`];
}

function isConfigFile(fsPath: string): boolean {
  return fsPath.endsWith('/grimoire/config/grimoire.config.json');
}

function isSharedConfigInput(fsPath: string): boolean {
  return /\/grimoire\/(config\/)?grimoire\..*\.(scrolls|variables)\.json$/.test(fsPath);
}

function isCustomAnimationFile(fsPath: string): boolean {
  return /\/grimoire\/animations\/.*\.css$/.test(fsPath);
}

export async function deactivate(): Promise<void> {
  // LanguageClient instances are disposed via subscriptions.
}

async function promptForToken(title: string): Promise<string | undefined> {
  const editor = vscode.window.activeTextEditor;
  const selection = editor?.selection;

  if (editor && selection && !selection.isEmpty) {
    const selected = editor.document.getText(selection).trim();
    if (selected.length > 0) return selected;
  }

  return vscode.window.showInputBox({
    title,
    prompt: 'Enter full Grimoire token (scroll / $var / spell)',
  });
}

function parseEntityToken(token: string): { kind: 'scroll' | 'var'; name: string } | undefined {
  const t = token.trim();
  if (t.length === 0) return undefined;
  if (t.startsWith('$')) {
    const name = t.slice(1).trim();
    if (name.length === 0) return undefined;
    return { kind: 'var', name };
  }

  // Templated scroll invocation: g!scroll-name;
  const templated = t.match(/^g!([A-Za-z0-9_\-.]+);$/);
  if (templated) {
    return { kind: 'scroll', name: templated[1] };
  }

  // Scroll names are plain tokens (no leading '.')
  const name = t.startsWith('.') ? t.slice(1) : t;
  if (name.length === 0) return undefined;
  return { kind: 'scroll', name };
}

function safeRegisterCommand<TArgs extends unknown[], TResult>(
  context: vscode.ExtensionContext,
  logger: Logger,
  id: string,
  // VS Code command handlers are dynamically invoked; keep args flexible, but allow typed handlers.
  handler: (...args: TArgs) => TResult
): boolean {
  try {
    const wrapped = (...args: unknown[]) => handler(...(args as TArgs));
    context.subscriptions.push(vscode.commands.registerCommand(id, wrapped));
    return true;
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    logger.info(`Failed to register command '${id}': ${msg}`);
    return false;
  }
}
