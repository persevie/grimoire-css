import * as vscode from 'vscode';

import type { LspManager } from '../lsp/lspManager';
import type { Logger } from '../utils/log';

type Highlighter = {
  refreshVisibleEditors: () => void;
  dispose: () => void;
};

export function registerEditorHighlights(
  context: vscode.ExtensionContext,
  lsp: LspManager,
  logger: Logger
): Highlighter {
  let entityDecoration = vscode.window.createTextEditorDecorationType({});
  context.subscriptions.push(entityDecoration);

  let lastDecorationKey: string | undefined;

  const cfg = () => vscode.workspace.getConfiguration('grimoireCss');
  const getEnabled = () => cfg().get<boolean>('editorHighlights.enable', true);

  const ensureDecorationType = (): vscode.TextEditorDecorationType => {
    const colorRaw = String(cfg().get<string>('editorHighlights.color', '#639') ?? '#639').trim();
    const color = normalizeHexColor(colorRaw) ?? '#663399';
    const style = (cfg().get<string>('editorHighlights.style', 'background') ?? 'background') as
      | 'background'
      | 'backgroundSoft'
      | 'backgroundStrong'
      | 'wholeLineBackground'
      | 'underlineSolid'
      | 'underlineDotted'
      | 'underlineDashed'
      | 'underlineWavy'
      | 'outline'
      | 'border'
      | 'borderBottomSolid'
      | 'borderBottomDotted'
      | 'leftBar'
      | 'prefixTag'
      | 'suffixTag';
    const overviewRuler = cfg().get<boolean>('editorHighlights.overviewRuler', true) ?? true;

    const key = `${color}|${style}|${overviewRuler}`;
    if (lastDecorationKey === key) return entityDecoration;
    lastDecorationKey = key;

    // Swap decoration type (VS Code doesn't let us mutate an existing type).
    entityDecoration.dispose();

    const base: vscode.DecorationRenderOptions = {
      overviewRulerLane: overviewRuler ? vscode.OverviewRulerLane.Right : undefined,
      overviewRulerColor: overviewRuler ? color : undefined,
    };

    const mkBackground = (alpha: number, wholeLine: boolean): vscode.TextEditorDecorationType =>
      vscode.window.createTextEditorDecorationType({
        ...base,
        isWholeLine: wholeLine,
        backgroundColor: hexToRgba(color, alpha),
        borderRadius: wholeLine ? undefined : '2px',
      });

    switch (style) {
      case 'background':
        entityDecoration = mkBackground(0.18, false);
        break;
      case 'backgroundSoft':
        entityDecoration = mkBackground(0.12, false);
        break;
      case 'backgroundStrong':
        entityDecoration = mkBackground(0.28, false);
        break;
      case 'wholeLineBackground':
        entityDecoration = mkBackground(0.14, true);
        break;

      case 'underlineSolid':
        entityDecoration = vscode.window.createTextEditorDecorationType({
          ...base,
          textDecoration: `underline solid ${color}`,
        });
        break;
      case 'underlineDotted':
        entityDecoration = vscode.window.createTextEditorDecorationType({
          ...base,
          textDecoration: `underline dotted ${color}`,
        });
        break;
      case 'underlineDashed':
        entityDecoration = vscode.window.createTextEditorDecorationType({
          ...base,
          textDecoration: `underline dashed ${color}`,
        });
        break;
      case 'underlineWavy':
        entityDecoration = vscode.window.createTextEditorDecorationType({
          ...base,
          textDecoration: `underline wavy ${color}`,
        });
        break;

      case 'outline':
        entityDecoration = vscode.window.createTextEditorDecorationType({
          ...base,
          outline: `1px solid ${color}`,
          borderRadius: '2px',
        });
        break;

      case 'border':
        entityDecoration = vscode.window.createTextEditorDecorationType({
          ...base,
          border: `1px solid ${color}`,
          borderRadius: '2px',
        });
        break;
      case 'borderBottomSolid':
        entityDecoration = vscode.window.createTextEditorDecorationType({
          ...base,
          borderWidth: '0 0 1px 0',
          borderStyle: 'solid',
          borderColor: color,
        });
        break;
      case 'borderBottomDotted':
        entityDecoration = vscode.window.createTextEditorDecorationType({
          ...base,
          borderWidth: '0 0 1px 0',
          borderStyle: 'dotted',
          borderColor: color,
        });
        break;
      case 'leftBar':
        entityDecoration = vscode.window.createTextEditorDecorationType({
          ...base,
          borderWidth: '0 0 0 3px',
          borderStyle: 'solid',
          borderColor: color,
        });
        break;

      case 'prefixTag':
        entityDecoration = vscode.window.createTextEditorDecorationType({
          ...base,
          before: {
            contentText: 'grimoire',
            color,
            margin: '0 0.35em 0 0',
            fontStyle: 'normal',
          },
        });
        break;
      case 'suffixTag':
        entityDecoration = vscode.window.createTextEditorDecorationType({
          ...base,
          after: {
            contentText: 'grimoire',
            color,
            margin: '0 0 0 0.35em',
            fontStyle: 'normal',
          },
        });
        break;
    }

    context.subscriptions.push(entityDecoration);
    return entityDecoration;
  };

  let timer: NodeJS.Timeout | undefined;
  let running = false;
  let pending = false;

  const schedule = (): void => {
    if (!getEnabled()) return;
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => void refreshVisibleEditors(), 200);
  };

  const clearEditor = (editor: vscode.TextEditor): void => {
    ensureDecorationType();
    editor.setDecorations(entityDecoration, []);
  };

  const refreshVisibleEditors = async (): Promise<void> => {
    if (!getEnabled()) {
      ensureDecorationType();
      for (const editor of vscode.window.visibleTextEditors) clearEditor(editor);
      return;
    }

    ensureDecorationType();

    if (running) {
      pending = true;
      return;
    }
    running = true;
    pending = false;

    try {
      const active = vscode.window.activeTextEditor;
      if (!active) return;

      // Safety: avoid scanning huge files.
      if (active.document.getText().length > 1_000_000) {
        clearEditor(active);
        return;
      }

      const folder = vscode.workspace.getWorkspaceFolder(active.document.uri);
      if (!folder) {
        clearEditor(active);
        return;
      }

      // Only highlight in folders where GrimoireCSS LSP is running.
      const hasClient = lsp.getFolders().some((f) => f.uri.toString() === folder.uri.toString());
      if (!hasClient) {
        clearEditor(active);
        return;
      }

      const ranges = await lsp.documentSpells(folder, { uri: active.document.uri.toString() });
      const decoRanges = (ranges ?? []).map(
        (r) =>
          new vscode.Range(
            new vscode.Position(r.start.line, r.start.character),
            new vscode.Position(r.end.line, r.end.character)
          )
      );

      active.setDecorations(entityDecoration, decoRanges);
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      logger.trace(`editorHighlights refresh failed: ${msg}`);
    } finally {
      running = false;
      if (pending) {
        pending = false;
        schedule();
      }
    }
  };

  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor(() => schedule()),
    vscode.window.onDidChangeVisibleTextEditors(() => schedule()),
    vscode.workspace.onDidChangeTextDocument((e) => {
      const active = vscode.window.activeTextEditor;
      if (!active) return;
      if (e.document.uri.toString() !== active.document.uri.toString()) return;
      schedule();
    }),
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration('grimoireCss.editorHighlights')) {
        schedule();
      }
    })
  );

  // Initial run.
  schedule();

  return {
    refreshVisibleEditors,
    dispose() {
      if (timer) clearTimeout(timer);
      entityDecoration.dispose();
    },
  };
}

function normalizeHexColor(input: string): string | undefined {
  const s = (input ?? '').trim();
  if (!s) return undefined;
  const m3 = s.match(/^#([0-9a-fA-F]{3})$/);
  if (m3) {
    const [r, g, b] = m3[1].split('');
    return `#${r}${r}${g}${g}${b}${b}`.toLowerCase();
  }
  const m6 = s.match(/^#([0-9a-fA-F]{6})$/);
  if (m6) return `#${m6[1]}`.toLowerCase();
  return undefined;
}

function hexToRgba(hex: string, alpha: number): string {
  const h = normalizeHexColor(hex) ?? '#663399';
  const r = parseInt(h.slice(1, 3), 16);
  const g = parseInt(h.slice(3, 5), 16);
  const b = parseInt(h.slice(5, 7), 16);
  const a = Math.min(1, Math.max(0, alpha));
  return `rgba(${r}, ${g}, ${b}, ${a})`;
}
