import * as vscode from 'vscode';
import type { LspManager } from '../lsp/lspManager';
import { extractTokenAtPosition } from './tokenExtract';

export type CommandIds = {
  explainToken: string;
  findRefs: string;
  stats: string;
};

export function registerHover(_lsp: LspManager, commands?: CommandIds): vscode.Disposable {
  const cmd = commands ?? {
    explainToken: 'grimoirecss.explainToken',
  };

  return vscode.languages.registerHoverProvider(
    { scheme: 'file' },
    {
      provideHover(document, position) {
        const token = extractTokenAtPosition(document, position);
        if (!token) return;

        const md = new vscode.MarkdownString(undefined, true);
        md.isTrusted = true;

        md.appendMarkdown(
          `[Explain](command:${cmd.explainToken}?${encodeURIComponent(JSON.stringify([token]))})  `
        );

        return new vscode.Hover(md);
      },
    }
  );
}
