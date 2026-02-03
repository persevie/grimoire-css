import * as vscode from 'vscode';
import type { ExplorerIndexResult } from '../lsp/commands';
import type { LspManager } from '../lsp/lspManager';
import type { Logger } from '../utils/log';

type CommandIds = {
  selectEntity: string;
};

export class ExplorerViewProvider implements vscode.WebviewViewProvider {
  private view: vscode.WebviewView | undefined;
  private inflight: Promise<void> | undefined;
  private index: ExplorerIndexResult | undefined;

  constructor(
    private readonly context: vscode.ExtensionContext,
    private readonly lsp: LspManager,
    _logger: Logger,
    private readonly commands: CommandIds
  ) {}

  resolveWebviewView(view: vscode.WebviewView): void {
    this.view = view;
    view.webview.options = {
      enableScripts: true,
      localResourceRoots: [this.context.extensionUri],
    };

    view.webview.onDidReceiveMessage((msg) => {
      if (!msg || typeof msg !== 'object') return;
      if (msg.type === 'select') {
        const kind =
          msg.kind === 'scroll' ||
          msg.kind === 'var' ||
          msg.kind === 'function' ||
          msg.kind === 'animation'
            ? msg.kind
            : undefined;
        const name = typeof msg.name === 'string' ? msg.name : undefined;
        if (!kind || !name) return;

        void vscode.commands.executeCommand(this.commands.selectEntity, {
          kind,
          name,
        });
      }
    });

    void this.render();
  }

  async render(): Promise<void> {
    if (!this.view) return;
    if (this.inflight) return;
    this.inflight = this.renderImpl().finally(() => {
      this.inflight = undefined;
    });
    await this.inflight;
  }

  private async renderImpl(): Promise<void> {
    if (!this.view) return;
    const folder = this.lsp.getActiveOrFirstFolder();
    if (!folder) {
      const wsFolder = getPreferredWorkspaceFolder();
      if (wsFolder) {
        const configUri = vscode.Uri.joinPath(
          wsFolder.uri,
          'grimoire',
          'config',
          'grimoire.config.json'
        );
        let hasConfig = false;
        try {
          const st = await vscode.workspace.fs.stat(configUri);
          hasConfig = st.type === vscode.FileType.File;
        } catch {
          hasConfig = false;
        }

        this.view.webview.html = this.wrapHtml(`
          <div class="app">
            <div class="header">
              <input id="q" type="search" placeholder="Search" disabled />
              <div class="header-r"><span class="muted">${hasConfig ? 'Starting…' : 'No config'}</span></div>
            </div>
            <div class="body">
              <p class="muted">
                ${
                  hasConfig
                    ? 'GrimoireCSS config exists, but the LSP is not running yet. Try running Refresh or restarting the extension.'
                    : 'No grimoire/config/grimoire.config.json found. Run Init from the view toolbar to create it.'
                }
              </p>
            </div>
          </div>
        `);
        return;
      }

      this.view.webview.html = this.wrapHtml(`
        <div class="app">
          <div class="header">
            <input id="q" type="search" placeholder="Search" disabled />
            <div class="header-r"><span class="muted">No workspace</span></div>
          </div>
          <div class="body"><p class="muted">No GrimoireCSS workspace folder found.</p></div>
        </div>
      `);
      return;
    }

    try {
      this.index = await this.lsp.getExplorerIndex(folder);
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      this.view.webview.html = this.wrapHtml(`
        <div class="app">
          <div class="header">
            <input id="q" type="search" class="search" placeholder="Search" disabled />
            <div class="header-r"><span class="error">Failed</span></div>
          </div>
          <div class="body"><p class="error">Failed to load Explorer index: ${escapeHtml(msg)}</p></div>
        </div>
      `);
      return;
    }

    const payload = safeJsonForScript(this.index);

    this.view.webview.html = this.wrapHtml(`
      <div class="app">
        <div class="header">
          <input id="q" type="search" class="search" placeholder="Search scrolls, vars, functions, animations" />
          <div class="header-r">
            <span id="count" class="muted"></span>
            <button id="clear" class="btn" type="button">Clear</button>
          </div>
        </div>

        <div>
          <details class="group" id="gScrolls" open>
            <summary>Scrolls</summary>
            <div id="scrolls"></div>
          </details>
          <details class="group" id="gVars" open>
            <summary>Variables</summary>
            <div id="vars"></div>
          </details>
          <details class="group" id="gFns" open>
            <summary>Functions</summary>
            <div id="functions"></div>
          </details>
          <details class="group" id="gAnims" open>
            <summary>Animations</summary>
            <div id="animations"></div>
          </details>
        </div>
      </div>

      <script>
        const vscode = acquireVsCodeApi();
        const index = ${payload};

        const q = document.getElementById('q');
        const clearBtn = document.getElementById('clear');
        const countEl = document.getElementById('count');

        const scrollsRoot = document.getElementById('scrolls');
        const varsRoot = document.getElementById('vars');
        const fnsRoot = document.getElementById('functions');
        const animRoot = document.getElementById('animations');

        const gScrolls = document.getElementById('gScrolls');
        const gVars = document.getElementById('gVars');
        const gFns = document.getElementById('gFns');
        const gAnims = document.getElementById('gAnims');

        function esc(s) {
          return String(s)
            .replaceAll('&', '&amp;')
            .replaceAll('<', '&lt;')
            .replaceAll('>', '&gt;')
            .replaceAll('"', '&quot;')
            .replaceAll("'", '&#39;');
        }

        function matches(hay, filter) {
          if (!filter) return true;
          return String(hay).toLowerCase().includes(filter);
        }

        function renderRows(rows) {
          return rows.join('') || '<p class="muted">(none)</p>';
        }

        function refresh() {
          const filter = (q.value || '').toLowerCase().trim();
          let total = 0;

          clearBtn.disabled = filter.length === 0;

          // Scrolls
          const sRows = (index.scrolls || [])
            .filter((name) => matches(name, filter))
            .map((name) => {
              total++;
              return '<div class="row" data-kind="scroll" data-name="' + esc(name) + '">' +
                '<div class="name">' + esc(name) + '</div>' +
                '<div class="meta">scroll</div>' +
              '</div>';
            });
          scrollsRoot.innerHTML = renderRows(sRows);
          gScrolls.style.display = sRows.length ? '' : 'none';

          // Vars
          const vRows = (index.variables || [])
            .filter((v) => matches('$' + v.name + ' ' + v.value, filter))
            .map((v) => {
              total++;
              return '<div class="row" data-kind="var" data-name="' + esc(v.name) + '">' +
                '<div class="name">$' + esc(v.name) + '</div>' +
                '<div class="meta">' + esc(v.value || '') + '</div>' +
              '</div>';
            });
          varsRoot.innerHTML = renderRows(vRows);
          gVars.style.display = vRows.length ? '' : 'none';

          // Functions
          const fRows = (index.functions || [])
            .filter((f) => matches(f.name + ' ' + f.group + ' ' + f.usage, filter))
            .map((f) => {
              total++;
              return '<div class="row" data-kind="function" data-name="' + esc(f.name) + '">' +
                '<div class="name">' + esc(f.name) + '</div>' +
                '<div class="meta">' + esc(f.group) + (f.usage ? ' · ' + esc(f.usage) : '') + '</div>' +
              '</div>';
            });
          fnsRoot.innerHTML = renderRows(fRows);
          gFns.style.display = fRows.length ? '' : 'none';

          // Animations
          const aRows = (index.animations || [])
            .filter((a) => matches(a.name + ' ' + a.kind, filter))
            .map((a) => {
              total++;
              return '<div class="row" data-kind="animation" data-name="' + esc(a.name) + '">' +
                '<div class="name">' + esc(a.name) + '</div>' +
                '<div class="meta">' + esc(a.kind || '') + '</div>' +
              '</div>';
            });
          animRoot.innerHTML = renderRows(aRows);
          gAnims.style.display = aRows.length ? '' : 'none';

          countEl.textContent = total ? (total + ' matches') : '';
        }

        q.addEventListener('input', refresh);
        clearBtn.addEventListener('click', () => {
          q.value = '';
          q.focus();
          refresh();
        });

        document.addEventListener('click', (e) => {
          const row = e.target.closest('.row');
          if (!row) return;
          const kind = row.getAttribute('data-kind');
          const name = row.getAttribute('data-name');
          if (kind && name) {
            vscode.postMessage({ type: 'select', kind, name });
          }
        });

        refresh();
      </script>
    `);
  }

  private wrapHtml(body: string): string {
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
            vscode.Uri.joinPath(this.context.extensionUri, 'resources', 'webview', 'explorer.css')
          )
          .toString()
      : '';

    return `<!doctype html><html><head><meta charset="utf-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${cspSource} 'unsafe-inline'; script-src 'unsafe-inline';" />
      <link rel="stylesheet" href="${resetCssHref}" />
      <link rel="stylesheet" href="${cssHref}" />
    </head><body>${body}</body></html>`;
  }
}

function escapeHtml(v: string): string {
  return v
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

function safeJsonForScript(v: unknown): string {
  return JSON.stringify(v).replace(/</g, '\\u003c');
}

function getPreferredWorkspaceFolder(): vscode.WorkspaceFolder | undefined {
  const editor = vscode.window.activeTextEditor;
  if (editor) {
    const folder = vscode.workspace.getWorkspaceFolder(editor.document.uri);
    if (folder) return folder;
  }
  return vscode.workspace.workspaceFolders?.[0];
}
