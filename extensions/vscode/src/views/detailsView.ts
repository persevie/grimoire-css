import * as vscode from 'vscode';
import * as shiki from 'shiki';
import type { EntityDetailsResult, LspLocation } from '../lsp/commands';
import type { LspManager } from '../lsp/lspManager';
import { openVirtualText } from '../preview/virtualDocuments';
import type { Logger } from '../utils/log';

type CommandIds = {
  openLocation: string;
};

type SelectedEntity = {
  folder: vscode.WorkspaceFolder;
  kind: 'scroll' | 'var' | 'function' | 'animation';
  name: string; // normalized name: scroll without '.', var without '$'
};

export class DetailsViewProvider implements vscode.WebviewViewProvider {
  private view: vscode.WebviewView | undefined;
  private selection: SelectedEntity | undefined;
  private inflight: Promise<void> | undefined;
  private lastCssPreview: { title: string; css: string } | undefined;
  private lastRenderedCss: { mode: 'light' | 'dark'; css: string; html: string } | undefined;

  constructor(
    private readonly context: vscode.ExtensionContext,
    private readonly lsp: LspManager,
    private readonly logger: Logger,
    private readonly commands: CommandIds
  ) {}

  resolveWebviewView(view: vscode.WebviewView): void {
    this.view = view;
    view.webview.options = {
      enableScripts: true,
      localResourceRoots: [this.context.extensionUri],
    };

    view.webview.onDidReceiveMessage((msg: unknown) => {
      const type = getStringProp(msg, 'type');
      if (!type) return;

      if (type === 'openLocation') {
        const uriStr = getStringProp(msg, 'uri');
        const uri = typeof uriStr === 'string' ? vscode.Uri.parse(uriStr) : undefined;
        if (!uri) return;
        const line = getNumberProp(msg, 'line');
        const character = getNumberProp(msg, 'character');
        void vscode.commands.executeCommand(this.commands.openLocation, {
          uri,
          line,
          character,
        });
      }

      if (type === 'openCssPreview') {
        const payload = this.lastCssPreview;
        if (!payload) return;
        void openVirtualText(
          `grimoirecss://details/${encodeURIComponent(payload.title)}.css`,
          payload.css,
          'css'
        );
      }

      if (type === 'copyToClipboard') {
        const text = getStringProp(msg, 'text') ?? '';
        if (!text) return;
        void vscode.env.clipboard.writeText(text);
      }
    });

    void this.render();
  }

  async setSelection(selection: SelectedEntity | undefined): Promise<void> {
    this.selection = selection;
    await this.render();
    this.view?.show?.(true);
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

    if (!this.selection) {
      this.view.webview.html = this.wrapHtml(`
  <h2>Details</h2>
  <p>Select an entity in Explorer (or Config) to see details.</p>
`);
      return;
    }

    const { folder, kind, name } = this.selection;

    let details: EntityDetailsResult | undefined;
    try {
      details = await this.lsp.entityDetails(folder, { kind, name });
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      if (/method not found/i.test(msg)) {
        this.logger.info(
          `entityDetails is not supported by the running LSP server for ${folder.name}.`
        );
        void vscode.window
          .showErrorMessage(
            "GrimoireCSS: LSP server doesn't support 'entityDetails' yet. Rebuild the server and restart LSP.",
            'Restart LSP'
          )
          .then((picked) => {
            if (picked === 'Restart LSP') {
              void vscode.commands.executeCommand('persevie.grimoirecss-vscode.restartLsp');
            }
          });
      }
      this.view.webview.html = this.wrapHtml(`
  <h2>Details</h2>
  <p class="error">Failed to load details: ${escapeHtml(msg)}</p>
`);
      return;
    }

    const title = kind === 'var' ? `$${name}` : name;

    // Store CSS for the "Open in editor" action.
    if (kind === 'scroll' || kind === 'animation') {
      this.lastCssPreview = { title, css: details.css ?? '' };
    } else {
      this.lastCssPreview = undefined;
    }

    const defsBlock = this.renderLocations(details.definitions);
    const projRefsBlock = this.renderLocations(details.refs.project);
    const cfgRefsBlock = this.renderLocations(details.refs.config);

    const conflict = details.definition_conflict
      ? `<p class="warn">Multiple definitions found (name conflict).</p>`
      : '';

    const mode =
      vscode.window.activeColorTheme.kind === vscode.ColorThemeKind.Light ||
      vscode.window.activeColorTheme.kind === vscode.ColorThemeKind.HighContrastLight
        ? 'light'
        : 'dark';

    const cssBlock =
      kind === 'scroll' || kind === 'animation'
        ? await this.renderCodeBlock(details.css ?? '', 'css', mode)
        : '';

    const groupBlock = details.group
      ? `<div class="kv"><span class="k">Group</span><span class="v">${escapeHtml(details.group)}</span></div>`
      : '';

    const sourceUri = details.source_path
      ? details.source_path.startsWith('file:')
        ? details.source_path
        : vscode.Uri.file(details.source_path).toString()
      : undefined;

    const openCssAction =
      kind === 'scroll' || kind === 'animation'
        ? `<a href="#" class="action" data-action="openCss">Open in editor</a>`
        : '';
    const openSourceAction =
      kind === 'animation' && sourceUri
        ? `<a href="#" class="action loc" data-uri="${escapeHtml(sourceUri)}" data-line="0" data-character="0">Open source</a>`
        : '';
    const actions =
      openCssAction || openSourceAction
        ? `<div class="actions">${[openCssAction, openSourceAction].filter(Boolean).join(' ')}</div>`
        : '';

    const usageBlock = details.usage
      ? `<div class="kv kv-copy"><span class="k">Usage</span><span class="v-mono " id="usageValue">${escapeHtml(
          details.usage
        )}</span><a href="#" class="action-sm" data-action="copyUsage" data-copy="${escapeHtml(
          details.usage
        )}">Copy</a></div>`
      : '';
    const exampleBlock = details.example
      ? `<div class="kv"><span class="k">Example</span><span class="v-mono ">${escapeHtml(details.example)}</span></div>`
      : '';
    const animationKindBlock = details.animation_kind
      ? `<div class="kv"><span class="k">Kind</span><span class="v">${escapeHtml(details.animation_kind)}</span></div>`
      : '';

    const previewCss = details.css ? escapeStyleTagContents(details.css) : '';
    const previewBlock =
      kind === 'animation'
        ? `
 <div class="preview-area">
<div id="preview-element" class="preview-element" data-anim-name="${escapeHtml(name)}"></div>
 </div>
 <style>${previewCss}</style>
  `
        : '';

    const primaryBlock =
      kind === 'var'
        ? `<div class="kv"><span class="k">Value</span><span class="v">${escapeHtml(details.value ?? '')}</span></div>`
        : kind === 'function'
          ? `${groupBlock}${usageBlock}${exampleBlock || ''}`
          : kind === 'animation'
            ? `${actions}${groupBlock}${animationKindBlock}${usageBlock}${cssBlock}`
            : `${actions}${cssBlock}`;

    const configRefsSection =
      details.refs.config.length > 0
        ? `
<details>
  <summary><b>References (config)</b> <span class="muted">(${details.refs.config.length})</span></summary>
  <div class="pad">${cfgRefsBlock}</div>
</details>`
        : '';

    const statsSection = `
<div class="stats margin-bottom=10px">
  <div><span class="k">Usages (project)</span> <span class="v">${details.usage_count.project}</span></div>
  <div><span class="k">Usages (config)</span> <span class="v">${details.usage_count.config}</span></div>
  <div><span class="k">Total</span> <span class="v">${details.usage_count.total}</span></div>
</div>`;

    const showDefs = details.definitions.length > 0 || details.definition_conflict;
    const showRefs =
      kind === 'scroll' || kind === 'var' || kind === 'function' || kind === 'animation';

    this.view.webview.html = this.wrapHtml(`
<div class="titlebar">
  <h2 id="entityTitle">${escapeHtml(title)}</h2>
  <a href="#" class="action-sm" data-action="copyTitle" data-copy="${escapeHtml(title)}">Copy</a>
</div>

<div class="rows">
${statsSection}

<details open>
  <summary><b>${
    kind === 'scroll'
      ? 'CSS output'
      : kind === 'var'
        ? 'Variable'
        : kind === 'function'
          ? 'Function'
          : 'Animation'
  }</b></summary>
  <div class="pad rows">${primaryBlock}</div>
</details>

${
  kind === 'animation'
    ? `
<details open>
  <summary><b>Preview</b></summary>
  <div class="pad">${previewBlock}</div>
</details>`
    : ''
}

${
  showDefs
    ? `
<details open>
  <summary><b>Definition</b> <span class="muted">(${details.definitions.length})</span></summary>
  <div class="pad">${conflict}${defsBlock}</div>
</details>`
    : ''
}

${
  showRefs
    ? `
<details open>
  <summary><b>References (project files)</b> <span class="muted">(${details.refs.project.length})</span></summary>
  <div class="pad">${projRefsBlock}</div>
</details>`
    : ''
}

${configRefsSection}
 </div>`);
  }

  private renderLocations(locs: LspLocation[]): string {
    if (!locs || locs.length === 0) {
      return `<p class="muted">(none)</p>`;
    }

    const items = locs
      .map((l) => {
        const uri = l.uri;
        const line = l.range?.start?.line ?? 0;
        const character = l.range?.start?.character ?? 0;
        const label = `${pathTail(uri)}:${line + 1}:${character + 1}`;
        return `<li><a href="#" class="loc" data-uri="${escapeHtml(uri)}" data-line="${line}" data-character="${character}">${escapeHtml(label)}</a></li>`;
      })
      .join('');

    return `<ul class="list">${items}</ul>`;
  }

  private async renderCodeBlock(
    code: string,
    _language: 'css',
    mode: 'light' | 'dark'
  ): Promise<string> {
    const css = code || '';

    if (
      this.lastRenderedCss &&
      this.lastRenderedCss.mode === mode &&
      this.lastRenderedCss.css === css
    ) {
      return this.lastRenderedCss.html;
    }

    const shikiHtml = await highlightCssWithShiki(css, mode);
    if (shikiHtml) {
      const html = `<div class="code code-shiki">${shikiHtml}</div>`;
      this.lastRenderedCss = { mode, css, html };
      return html;
    }

    const html = `<pre class="code code-plain"><code class="lang-css language-css" data-lang="css">${escapeHtml(css)}</code></pre>`;
    this.lastRenderedCss = { mode, css, html };
    return html;
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
            vscode.Uri.joinPath(this.context.extensionUri, 'resources', 'webview', 'details.css')
          )
          .toString()
      : '';
    const themeKind = vscode.window.activeColorTheme.kind;
    const mode =
      themeKind === vscode.ColorThemeKind.Light ||
      themeKind === vscode.ColorThemeKind.HighContrastLight
        ? 'light'
        : 'dark';

    const editorFontWeightRaw = vscode.workspace
      .getConfiguration('editor')
      .get<string>('fontWeight', 'normal');
    const editorFontWeight = sanitizeFontWeight(editorFontWeightRaw);
    return `<!doctype html>
<html lang="en">
<head>
<meta charset="UTF-8" />
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}';" />
<meta name="viewport" content="width=device-width, initial-scale=1.0" />
<link rel="stylesheet" href="${resetCssHref}" />
<link rel="stylesheet" href="${cssHref}" />
</head>
<body data-mode="${mode}" style="--grimoire-editor-font-weight: ${editorFontWeight};">
${body}
<script nonce="${nonce}">
  const vscode = acquireVsCodeApi();

  // Animation preview: apply selected animation name.
  (function () {
 const el = document.getElementById('preview-element');
 if (!el) return;
 const anim = el.getAttribute('data-anim-name') || '';
 if (anim) el.style.animationName = anim;
  })();

  document.addEventListener('click', (ev) => {
 const a = ev.target && ev.target.closest ? ev.target.closest('a.loc') : null;
 if (!a) return;
 ev.preventDefault();
 vscode.postMessage({
type: 'openLocation',
uri: a.dataset.uri,
line: Number(a.dataset.line || '0'),
character: Number(a.dataset.character || '0'),
 });
  });

  document.addEventListener('click', (ev) => {
 const a = ev.target && ev.target.closest ? ev.target.closest('a.action') : null;
 if (!a) return;
 ev.preventDefault();
 if (a.dataset.action === 'openCss') {
vscode.postMessage({ type: 'openCssPreview' });
 }
 if (a.dataset.action === 'copyTitle' || a.dataset.action === 'copyUsage') {
const text = a.dataset.copy || '';
if (!text) return;

const original = a.textContent;
const done = () => {
  a.textContent = 'Copied';
  setTimeout(() => {
 a.textContent = original;
  }, 800);
};

if (navigator.clipboard && typeof navigator.clipboard.writeText === 'function') {
  navigator.clipboard.writeText(text).then(done, () => {
 vscode.postMessage({ type: 'copyToClipboard', text });
 done();
  });
} else {
  vscode.postMessage({ type: 'copyToClipboard', text });
  done();
}
 }
  });
</script>
</body>
</html>`;
  }
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');
}

function escapeStyleTagContents(s: string): string {
  // Prevent accidental </style> termination.
  return String(s).replace(/<\s*\/\s*style/gi, '<\\/style');
}

let shikiHighlighterPromise: Promise<unknown> | undefined;

async function highlightCssWithShiki(
  text: string,
  mode: 'light' | 'dark'
): Promise<string | undefined> {
  try {
    const highlighter = await getShikiHighlighter();
    if (!highlighter) return undefined;
    const theme = mode === 'light' ? 'github-light' : 'github-dark';
    if (!hasCodeToHtml(highlighter)) return undefined;
    const raw = String(highlighter.codeToHtml(text, { lang: 'css', theme }));
    // Shiki may insert large whitespace nodes between line spans; with `white-space: pre` this can
    // create apparent extra blank lines / artefacts. Collapse whitespace between line wrappers.
    return raw.replace(
      /<\/span><\/span>\s*<span class="line">/g,
      '</span></span><span class="line">'
    );
  } catch {
    return undefined;
  }
}

async function getShikiHighlighter(): Promise<unknown | undefined> {
  if (!shikiHighlighterPromise) {
    shikiHighlighterPromise = (async () => {
      const shikiModule: unknown = shiki;

      if (hasCreateHighlighter(shikiModule)) {
        return await shikiModule.createHighlighter({
          themes: ['github-dark', 'github-light'],
          langs: ['css'],
        });
      }

      if (hasGetHighlighter(shikiModule)) {
        try {
          return await shikiModule.getHighlighter({
            themes: ['github-dark', 'github-light'],
            langs: ['css'],
          });
        } catch {
          // Older API: single theme argument.
          return await shikiModule.getHighlighter({
            theme: 'github-dark',
            langs: ['css'],
          });
        }
      }

      return undefined;
    })();
  }

  return shikiHighlighterPromise;
}

function getStringProp(obj: unknown, key: string): string | undefined {
  if (!obj || typeof obj !== 'object') return undefined;
  const v = (obj as Record<string, unknown>)[key];
  return typeof v === 'string' ? v : undefined;
}

function getNumberProp(obj: unknown, key: string): number | undefined {
  if (!obj || typeof obj !== 'object') return undefined;
  const v = (obj as Record<string, unknown>)[key];
  return typeof v === 'number' ? v : undefined;
}

function hasCreateHighlighter(
  v: unknown
): v is { createHighlighter(opts: unknown): Promise<unknown> } {
  if (!v || typeof v !== 'object') return false;
  return typeof (v as Record<string, unknown>).createHighlighter === 'function';
}

function hasGetHighlighter(v: unknown): v is { getHighlighter(opts: unknown): Promise<unknown> } {
  if (!v || typeof v !== 'object') return false;
  return typeof (v as Record<string, unknown>).getHighlighter === 'function';
}

function hasCodeToHtml(v: unknown): v is {
  codeToHtml(code: string, opts: { lang: string; theme: string }): unknown;
} {
  if (!v || typeof v !== 'object') return false;
  return typeof (v as Record<string, unknown>).codeToHtml === 'function';
}

function pathTail(uri: string): string {
  try {
    const u = vscode.Uri.parse(uri);
    const parts = u.path.split('/');
    return parts[parts.length - 1] || uri;
  } catch {
    const parts = uri.split('/');
    return parts[parts.length - 1] || uri;
  }
}

function sanitizeFontWeight(v: string): string {
  const s = (v ?? '').toString().trim();
  if (/^(normal|bold|bolder|lighter)$/.test(s)) return s;
  if (/^\d{3}$/.test(s)) return s;
  return 'normal';
}
