import * as vscode from 'vscode';
import type { ConfigSummary } from '../lsp/commands';
import type { LspManager } from '../lsp/lspManager';
import type { Logger } from '../utils/log';

type CommandIds = {
  selectEntity: string;
  openLocation: string;
};

export class ConfigViewProvider implements vscode.WebviewViewProvider {
  private view: vscode.WebviewView | undefined;

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
      if (msg.type === 'selectEntity') {
        const kind =
          msg.kind === 'scroll' || msg.kind === 'var' || msg.kind === 'animation'
            ? msg.kind
            : undefined;
        const name = typeof msg.name === 'string' ? msg.name : undefined;
        if (!kind || !name) return;
        void vscode.commands.executeCommand(this.commands.selectEntity, {
          kind,
          name,
        });
      }

      if (msg.type === 'openPath') {
        const uriStr = typeof msg.uri === 'string' ? msg.uri : undefined;
        if (!uriStr) return;
        try {
          const uri = vscode.Uri.parse(uriStr);
          void vscode.commands.executeCommand(this.commands.openLocation, {
            uri,
          });
        } catch {
          // ignore
        }
      }
    });

    void this.render();
  }

  show(): void {
    this.view?.show?.(true);
  }

  async render(): Promise<void> {
    if (!this.view) return;

    const folder = this.lsp.getActiveOrFirstFolder();
    if (!folder) {
      this.view.webview.html = this.wrapHtml(
        `<h2>Config</h2><p class="muted">No workspace folder.</p>`
      );
      return;
    }

    let summary: ConfigSummary;
    try {
      summary = await this.lsp.getConfigSummary(folder);
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      this.view.webview.html = this.wrapHtml(
        `<h2>Config</h2><p class="error">Failed to load config: ${escapeHtml(msg)}</p>`
      );
      return;
    }

    this.view.webview.html = this.wrapHtml(this.renderSummary(folder, summary));
  }

  private renderSummary(folder: vscode.WorkspaceFolder, s: ConfigSummary): string {
    const configUri = toUriString(folder, s.config_path);

    const projects = (s.projects ?? [])
      .map((p) => {
        const inputs = (p.input_paths ?? [])
          .map((rawInputPath) => {
            const openPath = getOpenablePathForMaybeGlob(rawInputPath);
            const uri = toUriString(folder, openPath);
            return `<li><a href="#" class="path" data-uri="${escapeHtml(uri)}">${escapeHtml(rawInputPath)}</a></li>`;
          })
          .join('');

        const out = p.output_dir_path
          ? `<a href="#" class="path" data-uri="${escapeHtml(toUriString(folder, p.output_dir_path))}">${escapeHtml(p.output_dir_path)}</a>`
          : '<span class="muted">(none)</span>';
        const single = (() => {
          const fileName = (p.single_output_file_name ?? '').trim();
          if (!fileName) return '<span class="muted">(none)</span>';

          const outDir = (p.output_dir_path ?? '').trim();
          if (!outDir) return `<span class="mono">${escapeHtml(fileName)}</span>`;

          const fullPath = joinPathStrings(outDir, fileName);
          const uri = toUriString(folder, fullPath);
          return `<a href="#" class="path mono" data-uri="${escapeHtml(uri)}">${escapeHtml(fileName)}</a>`;
        })();

        return `
            <div class="pad">
              <details open>
                <summary><b>${escapeHtml(p.name)}</b></summary>

                <div class="rows">
                  <div class="kv"><span class="k">Output</span><span class="v">${out}</span></div>
                  <div class="kv"><span class="k">Single file</span><span class="v">${single}</span></div>
                  <div class="kv"><span class="k">Inputs</span>
                    <span class="v">
                      <ul class="list">${inputs || '<li class="muted">(none)</li>'}</ul>
                    </span>
                  </div>
                </div>
              </details>
            </div>
          `;
      })
      .join('');

    const scrolls = (s.scrolls ?? [])
      .map(
        (name) =>
          `<li><a href="#" class="entity" data-kind="scroll" data-name="${escapeHtml(name)}">${escapeHtml(name)}</a></li>`
      )
      .join('');

    const vars = (s.variables ?? [])
      .map(
        (v) =>
          `<li><a href="#" class="entity" data-kind="var" data-name="${escapeHtml(v.name)}">$${escapeHtml(v.name)}</a> <span class="muted">=</span> <span class="mono">${escapeHtml(v.value)}</span></li>`
      )
      .join('');

    const sharedSpells = (s.shared_spells ?? [])
      .map((raw) => {
        const t = (raw ?? '').trim();
        if (!t) return '';

        if (!looksLikePath(t)) {
          return `<li><span class="mono">${escapeHtml(t)}</span></li>`;
        }

        const openPath = getOpenablePathForMaybeGlob(t);
        const uri = toUriString(folder, openPath);
        return `<li><a href="#" class="path mono" data-uri="${escapeHtml(uri)}">${escapeHtml(t)}</a></li>`;
      })
      .join('');

    const animations = (s.custom_animations ?? [])
      .map(
        (name) =>
          `<li><a href="#" class="entity" data-kind="animation" data-name="${escapeHtml(name)}">${escapeHtml(name)}</a></li>`
      )
      .join('');

    const cssProps = (s.css_custom_properties ?? [])
      .map((name) => `<li><span class="mono">${escapeHtml(name)}</span></li>`)
      .join('');

    const externalScrollFiles = (s.external_scroll_files ?? [])
      .map((p) => {
        const uri = toUriString(folder, p);
        return `<li><a href="#" class="path" data-uri="${escapeHtml(uri)}">${escapeHtml(p)}</a></li>`;
      })
      .join('');

    const externalVarFiles = (s.external_variable_files ?? [])
      .map((p) => {
        const uri = toUriString(folder, p);
        return `<li><a href="#" class="path" data-uri="${escapeHtml(uri)}">${escapeHtml(p)}</a></li>`;
      })
      .join('');

    return `
      <h2>Config</h2>
      <div class="kv"><span class="k">Path</span><span class="v-mono "><a href="#" class="path" data-uri="${escapeHtml(configUri)}">${escapeHtml(s.config_path)}</a></span></div>

      <h3>Projects</h3>
      ${projects || '<p class="muted">(none)</p>'}

      <h3>Scrolls</h3>
      <ul class="list">${scrolls || '<li class="muted">(none)</li>'}</ul>

      <h3>Variables</h3>
      <ul class="list">${vars || '<li class="muted">(none)</li>'}</ul>

      <h3>Shared</h3>

      <div class="rows">
        <details open>
          <summary><b>Shared spells</b></summary>
          <div class="pad"><ul class="list">${sharedSpells || '<li class="muted">(none)</li>'}</ul></div>
        </details>

        <details open>
          <summary><b>Custom animations</b></summary>
          <div class="pad"><ul class="list">${animations || '<li class="muted">(none)</li>'}</ul></div>
        </details>

        <details open>
          <summary><b>CSS custom properties</b></summary>
          <div class="pad"><ul class="list">${cssProps || '<li class="muted">(none)</li>'}</ul></div>
        </details>

        <details open>
          <summary><b>External scroll files</b></summary>
          <div class="pad"><ul class="list">${externalScrollFiles || '<li class="muted">(none)</li>'}</ul></div>
        </details>

        <details open>
          <summary><b>External variable files</b></summary>
          <div class="pad"><ul class="list">${externalVarFiles || '<li class="muted">(none)</li>'}</ul></div>
        </details>
      </div>
    `;
  }

  private wrapHtml(body: string): string {
    const nonce = String(Date.now());
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
            vscode.Uri.joinPath(this.context.extensionUri, 'resources', 'webview', 'config.css')
          )
          .toString()
      : '';
    return `<!doctype html>
<html lang="en">
<head>
<meta charset="UTF-8" />
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}';" />
<meta name="viewport" content="width=device-width, initial-scale=1.0" />
<link rel="stylesheet" href="${resetCssHref}" />
<link rel="stylesheet" href="${cssHref}" />
</head>
<body>
${body}
<script nonce="${nonce}">
  const vscode = acquireVsCodeApi();
  document.addEventListener('click', (ev) => {
    const a = ev.target && ev.target.closest ? ev.target.closest('a.entity') : null;
    if (!a) return;
    ev.preventDefault();
    vscode.postMessage({ type: 'selectEntity', kind: a.dataset.kind, name: a.dataset.name });
  });

  document.addEventListener('click', (ev) => {
    const a = ev.target && ev.target.closest ? ev.target.closest('a.path') : null;
    if (!a) return;
    ev.preventDefault();
    vscode.postMessage({ type: 'openPath', uri: a.dataset.uri });
  });
</script>
</body>
</html>`;
  }
}

function toUriString(folder: vscode.WorkspaceFolder, p: string): string {
  const raw = (p ?? '').trim();
  if (!raw) return folder.uri.toString();
  if (raw.startsWith('file:')) return raw;
  if (raw.startsWith('/')) return vscode.Uri.file(raw).toString();
  return vscode.Uri.joinPath(folder.uri, raw).toString();
}

function looksLikePath(s: string): boolean {
  const t = (s ?? '').trim();
  if (!t) return false;
  if (t.startsWith('file:') || t.startsWith('/') || t.startsWith('./') || t.startsWith('../'))
    return true;

  // Heuristics: allow links for common file extensions and path-looking strings.
  if (/(^|\/)[^\s/]+\.[a-z0-9]{1,6}$/i.test(t)) return true;
  if (t.includes('/')) return true;

  return false;
}

function isGlobLike(s: string): boolean {
  return /[*?[\]{}]/.test(s);
}

function getOpenablePathForMaybeGlob(rawPath: string): string {
  const p = (rawPath ?? '').trim();
  if (!p) return p;
  if (!isGlobLike(p)) return p;

  const wildcardIndex = p.search(/[*?[\]{}]/);
  const prefix = wildcardIndex >= 0 ? p.slice(0, wildcardIndex) : p;

  // Use the nearest directory to the glob prefix.
  const trimmed = prefix.endsWith('/') ? prefix.slice(0, -1) : prefix;
  const lastSlash = trimmed.lastIndexOf('/');
  if (lastSlash >= 0) {
    const dir = trimmed.slice(0, lastSlash + 1);
    return dir || '.';
  }
  return '.';
}

function joinPathStrings(dir: string, name: string): string {
  const d = (dir ?? '').trim();
  const n = (name ?? '').trim();
  if (!d) return n;
  if (!n) return d;
  if (d.endsWith('/')) return `${d}${n}`;
  return `${d}/${n}`;
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');
}
