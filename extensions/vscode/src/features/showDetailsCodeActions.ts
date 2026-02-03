import * as vscode from 'vscode';

import { extractTokenAtPosition } from './tokenExtract';

type CommandIds = {
  showDetails: string;
};

export function registerShowDetailsCodeActions(commands: CommandIds): vscode.Disposable {
  return vscode.languages.registerCodeActionsProvider(
    { scheme: 'file' },
    {
      provideCodeActions(document, range) {
        const token = extractTokenAtPosition(document, range.start);
        if (!token) return;

        const entityToken = pickEntityToken(token);
        if (!entityToken) return;

        const action = new vscode.CodeAction(
          'GrimoireCSS: Show details',
          vscode.CodeActionKind.Refactor
        );
        action.command = {
          command: commands.showDetails,
          title: 'Show details',
          arguments: [entityToken],
        };
        return [action];
      },
    },
    { providedCodeActionKinds: [vscode.CodeActionKind.Refactor] }
  );
}

function pickEntityToken(token: string): string | undefined {
  const t = token.trim();
  if (t.length === 0) return undefined;

  // Templated scroll invocation: g!scroll-name;
  const templated = t.match(/^g!([A-Za-z0-9_.-]+);$/);
  if (templated) return templated[1];

  // Prefer $var inside a larger token.
  const m = t.match(/\$[a-zA-Z0-9_-]+/);
  if (m) return m[0];

  // Scroll names are plain tokens; allow leading '.' as in CSS selectors.
  if (/^[A-Za-z0-9_.-]+$/.test(t)) return t;

  return undefined;
}
