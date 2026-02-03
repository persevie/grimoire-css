import * as vscode from 'vscode';

import type { Logger } from '../utils/log';

type CommandId =
  | 'persevie.grimoirecss-vscode.init'
  | 'persevie.grimoirecss-vscode.build'
  | 'persevie.grimoirecss-vscode.shorten'
  | 'persevie.grimoirecss-vscode.refresh'
  | 'persevie.grimoirecss-vscode.openSettings'
  | 'persevie.grimoirecss-vscode.openConfig'
  | 'persevie.grimoirecss-vscode.restartLsp';

export class ActionsViewProvider implements vscode.WebviewViewProvider {
  private view: vscode.WebviewView | undefined;

  constructor(
    private readonly context: vscode.ExtensionContext,
    _logger: Logger
  ) {}

  resolveWebviewView(view: vscode.WebviewView): void {
    this.view = view;
    view.webview.options = {
      enableScripts: true,
      localResourceRoots: [this.context.extensionUri],
    };

    view.webview.onDidReceiveMessage((msg: unknown) => {
      const type = getStringProp(msg, 'type');
      if (type !== 'command') return;

      const command = getStringProp(msg, 'command') ?? '';
      if (!isAllowedCommand(command)) return;

      void vscode.commands.executeCommand(command);
    });

    // Re-render when config appears/disappears.
    for (const folder of vscode.workspace.workspaceFolders ?? []) {
      const watcher = vscode.workspace.createFileSystemWatcher(
        new vscode.RelativePattern(folder, 'grimoire/config/grimoire.config.json')
      );
      watcher.onDidCreate(() => void this.render());
      watcher.onDidDelete(() => void this.render());
      watcher.onDidChange(() => void this.render());
      this.context.subscriptions.push(watcher);
    }

    void this.render();
  }

  async render(): Promise<void> {
    if (!this.view) return;

    const folder = getPreferredWorkspaceFolder();
    if (!folder) {
      this.view.webview.html = this.wrapHtml({
        hasConfig: false,
        note: 'No workspace folder open.',
      });
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

    this.view.webview.html = this.wrapHtml({
      hasConfig,
      note: hasConfig
        ? undefined
        : 'No config found. Run Init to create grimoire/config/grimoire.config.json.',
    });
  }

  private wrapHtml(state: { hasConfig: boolean; note?: string }): string {
    const nonce = String(Date.now());
    const { hasConfig, note } = state;

    const icons = this.getIconUris();
    const cspSource = this.view?.webview.cspSource ?? 'vscode-webview:';
    const resetCssHref = this.view
      ? this.view.webview
          .asWebviewUri(
            vscode.Uri.joinPath(this.context.extensionUri, 'resources', 'webview', 'reset.css')
          )
          .toString()
      : '';
    const cssHref = this.view
      ? this.view.webview
          .asWebviewUri(
            vscode.Uri.joinPath(this.context.extensionUri, 'resources', 'webview', 'actions.css')
          )
          .toString()
      : '';

    const primaryButtons = [
      { label: 'Build', command: 'persevie.grimoirecss-vscode.build' as const },
    ];

    const buildRowSecondaryButtons = [
      {
        label: 'Shorten',
        command: 'persevie.grimoirecss-vscode.shorten' as const,
      },
    ];

    const secondaryButtons = [
      // Init is only meaningful before config exists.
      ...(hasConfig
        ? []
        : [
            {
              label: 'Init',
              command: 'persevie.grimoirecss-vscode.init' as const,
            },
          ]),
      {
        label: 'Refresh',
        command: 'persevie.grimoirecss-vscode.refresh' as const,
      },
      {
        label: 'Settings',
        command: 'persevie.grimoirecss-vscode.openSettings' as const,
      },
    ];

    const dangerButtons = [
      {
        label: 'Restart LSP',
        command: 'persevie.grimoirecss-vscode.restartLsp' as const,
      },
    ];

    const renderButtons = (
      btns: Array<{ label: string; command: CommandId }>,
      kind: 'primary' | 'secondary' | 'danger'
    ) =>
      btns
        .map(
          (b) =>
            `<button data-btn-type="${kind}" class="btn" type="button" data-command="${escapeHtml(b.command)}">${renderIcon(
              icons,
              b.command
            )}<span class="lbl">${escapeHtml(b.label)}</span></button>`
        )
        .join('');

    // Keep this view compact: it lives above other views.
    return `<!doctype html>
<html lang="en">
<head>
<meta charset="UTF-8" />
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src ${cspSource}; style-src ${cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}';" />
<meta name="viewport" content="width=device-width, initial-scale=1.0" />
<link rel="stylesheet" href="${resetCssHref}" />
<link rel="stylesheet" href="${cssHref}" />
</head>
<body>
  <div class="row-base">
    ${renderButtons(primaryButtons, 'primary')}
    ${renderButtons(buildRowSecondaryButtons, 'secondary')}
  </div>
  <div class="sep"></div>
  <div class="row-base">
    ${renderButtons(secondaryButtons, 'secondary')}
    ${renderButtons(dangerButtons, 'danger')}
  </div>
  ${note ? `<p class="note">${escapeHtml(note)}</p>` : ''}

<script nonce="${nonce}">
  const vscode = acquireVsCodeApi();
  document.addEventListener('click', (ev) => {
    const btn = ev.target && ev.target.closest ? ev.target.closest('button[data-command]') : null;
    if (!btn) return;
    const command = btn.getAttribute('data-command') || '';
    vscode.postMessage({ type: 'command', command });
  });
</script>
</body>
</html>`;
  }

  private getIconUris(): Record<string, string> {
    const view = this.view;
    if (!view) return {};
    const icon = (name: string) =>
      view.webview
        .asWebviewUri(vscode.Uri.joinPath(this.context.extensionUri, 'resources', 'icons', name))
        .toString();

    return {
      build: icon('build.svg'),
      shorten: icon('shorten.svg'),
      init: icon('init.svg'),
      refresh: icon('refresh.svg'),
      settings: icon('settings.svg'),
      restart: icon('restart.svg'),
    };
  }
}

function getStringProp(obj: unknown, key: string): string | undefined {
  if (!obj || typeof obj !== 'object') return undefined;
  const v = (obj as Record<string, unknown>)[key];
  return typeof v === 'string' ? v : undefined;
}

function isAllowedCommand(command: string): command is CommandId {
  return (
    command === 'persevie.grimoirecss-vscode.init' ||
    command === 'persevie.grimoirecss-vscode.build' ||
    command === 'persevie.grimoirecss-vscode.shorten' ||
    command === 'persevie.grimoirecss-vscode.refresh' ||
    command === 'persevie.grimoirecss-vscode.openSettings' ||
    command === 'persevie.grimoirecss-vscode.openConfig' ||
    command === 'persevie.grimoirecss-vscode.restartLsp'
  );
}

function getPreferredWorkspaceFolder(): vscode.WorkspaceFolder | undefined {
  const editor = vscode.window.activeTextEditor;
  if (editor) {
    const folder = vscode.workspace.getWorkspaceFolder(editor.document.uri);
    if (folder) return folder;
  }
  return vscode.workspace.workspaceFolders?.[0];
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');
}

function renderIcon(icons: Record<string, string>, command: CommandId): string {
  const src = iconForCommand(icons, command);
  if (!src) return '';
  // Use CSS mask so the icon color follows `currentColor`.
  // The `.ico` scroll defines: mask: var(--icon) ...; background: currentColor.
  return `<span class="ico" aria-hidden="true" style="--icon:url('${escapeHtml(src)}')"></span>`;
}

function iconForCommand(icons: Record<string, string>, command: CommandId): string | undefined {
  switch (command) {
    case 'persevie.grimoirecss-vscode.build':
      return icons.build;
    case 'persevie.grimoirecss-vscode.shorten':
      return icons.shorten;
    case 'persevie.grimoirecss-vscode.init':
      return icons.init;
    case 'persevie.grimoirecss-vscode.refresh':
      return icons.refresh;
    case 'persevie.grimoirecss-vscode.openSettings':
      return icons.settings;
    case 'persevie.grimoirecss-vscode.restartLsp':
      return icons.restart;
    default:
      return undefined;
  }
}
