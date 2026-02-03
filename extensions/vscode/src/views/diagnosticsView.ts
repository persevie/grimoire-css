import * as vscode from 'vscode';

import type { LspManager } from '../lsp/lspManager';
import type { Logger } from '../utils/log';

type RootKind = 'lintErrors' | 'lintWarnings' | 'lintNotes';

type Node =
  | { kind: 'info'; message: string }
  | { kind: 'root'; rootKind: RootKind; folder: vscode.WorkspaceFolder }
  | {
      kind: 'lint';
      folder: vscode.WorkspaceFolder;
      level: string;
      code: string;
      message: string;
      occurrence?: { file: string; line: number; column: number };
    };

type CommandIds = {
  openLocation: string;
};

export class DiagnosticsView implements vscode.TreeDataProvider<Node>, vscode.Disposable {
  private readonly onDidChangeTreeDataEmitter = new vscode.EventEmitter<Node | undefined>();
  readonly onDidChangeTreeData = this.onDidChangeTreeDataEmitter.event;

  private readonly tree: vscode.TreeView<Node>;
  private visibilityPoll: NodeJS.Timeout | undefined;

  constructor(
    private readonly lsp: LspManager,
    _logger: Logger,
    private readonly commands: CommandIds
  ) {
    this.tree = vscode.window.createTreeView('grimoirecss.diagnostics', { treeDataProvider: this });

    this.updateMessage();

    // If the view was collapsed during startup, VS Code may show a blank cached state
    // until the next explicit refresh. Force-refresh on visibility.
    this.tree.onDidChangeVisibility((e) => {
      if (!e.visible) return;
      this.refresh();

      // Poll briefly after showing: if cached data becomes available shortly after,
      // this makes the view “snap” into place without waiting for the next user action.
      this.startVisibilityPoll();
    });
  }

  private startVisibilityPoll(): void {
    if (this.visibilityPoll) clearInterval(this.visibilityPoll);
    const startedAt = Date.now();
    this.visibilityPoll = setInterval(() => {
      this.updateMessage();
      this.onDidChangeTreeDataEmitter.fire(undefined);

      // Stop after a short window.
      if (Date.now() - startedAt > 1500) {
        if (this.visibilityPoll) clearInterval(this.visibilityPoll);
        this.visibilityPoll = undefined;
        this.updateMessage();
      }
    }, 150);
  }

  private updateMessage(): void {
    const folders = this.lsp.getFolders();
    if (folders.length === 0) {
      this.tree.message = 'Starting…';
      return;
    }

    const folder = this.lsp.getActiveOrFirstFolder() ?? folders[0];
    const cache = this.lsp.getCache(folder);
    const st = this.lsp.getStatus(folder);

    if (st.checkRunning || st.lintRunning) {
      this.tree.message = 'Checking…';
      return;
    }

    if (!cache.lint) {
      this.tree.message = 'Run Check (Lint)';
      return;
    }

    this.tree.message = undefined;
  }

  dispose(): void {
    if (this.visibilityPoll) clearInterval(this.visibilityPoll);
    this.tree.dispose();
    this.onDidChangeTreeDataEmitter.dispose();
  }

  refresh(): void {
    this.updateMessage();
    this.onDidChangeTreeDataEmitter.fire(undefined);
  }

  getTreeItem(element: Node): vscode.TreeItem {
    if (element.kind === 'info') {
      const item = new vscode.TreeItem(element.message, vscode.TreeItemCollapsibleState.None);
      item.description = 'info';
      return item;
    }

    if (element.kind === 'root') {
      const label =
        element.rootKind === 'lintErrors'
          ? 'Lint errors'
          : element.rootKind === 'lintWarnings'
            ? 'Lint warnings'
            : 'Lint notes';

      return new vscode.TreeItem(label, vscode.TreeItemCollapsibleState.Expanded);
    }

    if (element.kind === 'lint') {
      const msg = element.message.replace(/\s+/g, ' ').trim();
      const item = new vscode.TreeItem(element.code, vscode.TreeItemCollapsibleState.None);

      // Prefer showing the message in the description column (gives it more room).
      item.description = msg;

      const loc = element.occurrence;
      if (loc) {
        item.command = {
          command: this.commands.openLocation,
          title: 'Open lint issue',
          arguments: [
            {
              uri: toUri(element.folder, loc.file),
              line: Math.max(0, loc.line),
              character: Math.max(0, loc.column),
            },
          ],
        };
        item.contextValue = 'lintWithOccurrence';
      } else {
        // No exact location → keep as informational only.
        item.contextValue = 'lintNoOccurrence';
      }

      const tooltipLines = [
        `**${element.code}** (${element.level})`,
        '',
        msg,
        element.occurrence
          ? `\n\nSource: ${element.occurrence.file}:${element.occurrence.line + 1}:${element.occurrence.column + 1}`
          : '',
      ].join('');
      item.tooltip = new vscode.MarkdownString(tooltipLines);
      return item;
    }

    // unreachable
    return new vscode.TreeItem('');
  }

  async getChildren(element?: Node): Promise<Node[]> {
    const folders = this.lsp.getFolders();
    if (folders.length === 0) {
      return [{ kind: 'info', message: 'Starting GrimoireCSS…' }];
    }

    const folder = this.lsp.getActiveOrFirstFolder() ?? folders[0];
    const cache = this.lsp.getCache(folder);

    if (!element) {
      return [
        { kind: 'root', rootKind: 'lintErrors', folder },
        { kind: 'root', rootKind: 'lintWarnings', folder },
        { kind: 'root', rootKind: 'lintNotes', folder },
      ];
    }

    if (element.kind === 'root' && element.rootKind.startsWith('lint')) {
      const lint = cache.lint;
      if (!lint) {
        const st = this.lsp.getStatus(folder);
        return [
          {
            kind: 'lint',
            folder,
            level: 'info',
            code: st.checkRunning || st.lintRunning ? 'loading' : 'run-check',
            message: st.checkRunning || st.lintRunning ? 'Loading…' : 'Run Check (Lint)',
          },
        ];
      }

      const list =
        element.rootKind === 'lintErrors'
          ? lint.errors
          : element.rootKind === 'lintWarnings'
            ? lint.warnings
            : lint.notes;

      return list.map((m) => ({
        kind: 'lint',
        folder,
        level: m.level,
        code: m.code,
        message: m.message,
        occurrence: m.occurrence
          ? { file: m.occurrence.file, line: m.occurrence.line, column: m.occurrence.column }
          : undefined,
      }));
    }

    return [];
  }
}

function toUri(folder: vscode.WorkspaceFolder, file: string): vscode.Uri {
  if (file.startsWith('file:')) return vscode.Uri.parse(file);
  if (file.startsWith('/')) return vscode.Uri.file(file);
  return vscode.Uri.joinPath(folder.uri, file);
}
