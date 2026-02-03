import * as vscode from 'vscode';

import type { LspManager } from '../lsp/lspManager';
import type { Logger } from '../utils/log';

type Node =
  | { kind: 'info'; message: string }
  | {
      kind: 'candidate';
      folder: vscode.WorkspaceFolder;
      support: number;
      tokens: string[];
      occurrences: Array<{ file: string; line: number; column: number }>;
    }
  | {
      kind: 'occurrence';
      folder: vscode.WorkspaceFolder;
      file: string;
      line: number;
      column: number;
    };

type CommandIds = {
  openLocation: string;
};

export class DryView implements vscode.TreeDataProvider<Node>, vscode.Disposable {
  private readonly onDidChangeTreeDataEmitter = new vscode.EventEmitter<Node | undefined>();
  readonly onDidChangeTreeData = this.onDidChangeTreeDataEmitter.event;

  private readonly tree: vscode.TreeView<Node>;
  private visibilityPoll: NodeJS.Timeout | undefined;

  constructor(
    private readonly lsp: LspManager,
    _logger: Logger,
    private readonly commands: CommandIds
  ) {
    this.tree = vscode.window.createTreeView('grimoirecss.dry', {
      treeDataProvider: this,
    });

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

    if (st.checkRunning || st.dryRunning) {
      this.tree.message = 'Checking…';
      return;
    }

    if (!cache.dry) {
      this.tree.message = 'Run Check DRY';
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

    if (element.kind === 'candidate') {
      const item = new vscode.TreeItem(
        `support=${element.support}  ${element.tokens.join(' ')}`,
        element.occurrences.length > 0
          ? vscode.TreeItemCollapsibleState.Collapsed
          : vscode.TreeItemCollapsibleState.None
      );
      item.description = `${element.occurrences.length} occurrence(s)`;
      item.contextValue = 'dryCandidate';
      return item;
    }

    const item = new vscode.TreeItem(
      `${shortFileName(element.file)}:${element.line}:${element.column}`,
      vscode.TreeItemCollapsibleState.None
    );
    item.description = 'occurrence';
    item.command = {
      command: this.commands.openLocation,
      title: 'Open occurrence',
      arguments: [
        {
          uri: toUri(element.folder, element.file),
          line: Math.max(0, element.line - 1),
          character: Math.max(0, element.column - 1),
        },
      ],
    };
    return item;
  }

  async getChildren(element?: Node): Promise<Node[]> {
    const folders = this.lsp.getFolders();
    if (folders.length === 0) return [{ kind: 'info', message: 'Starting GrimoireCSS…' }];

    const folder = this.lsp.getActiveOrFirstFolder() ?? folders[0];
    const cache = this.lsp.getCache(folder);

    if (!element) {
      const dry = cache.dry;
      if (!dry) {
        const st = this.lsp.getStatus(folder);
        return [
          {
            kind: 'candidate',
            folder,
            support: 0,
            tokens: [st.checkRunning || st.dryRunning ? '(loading…)' : '(run Check DRY)'],
            occurrences: [],
          },
        ];
      }

      if (dry.candidates.length === 0) {
        return [
          {
            kind: 'info',
            message: 'No candidates found.',
          },
        ];
      }

      return dry.candidates.map((c) => ({
        kind: 'candidate',
        folder,
        support: c.support,
        tokens: c.tokens,
        occurrences: (c.occurrences ?? []).map((o) => ({
          file: o.file,
          line: o.line,
          column: o.column,
        })),
      }));
    }

    if (element.kind === 'candidate') {
      return element.occurrences.map((o) => ({
        kind: 'occurrence',
        folder: element.folder,
        file: o.file,
        line: o.line,
        column: o.column,
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

function shortFileName(file: string): string {
  const parts = file.split('/');
  return parts[parts.length - 1] || file;
}
