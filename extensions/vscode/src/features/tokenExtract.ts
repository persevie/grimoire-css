import type * as vscode from 'vscode';

// Best-effort token extraction. Grimoire tokens can include chars like '=', ':', '-', '_', '$'.
// We stop on whitespace and common quote/angle delimiters.
const STOP_CHARS = new Set([
  ' ',
  '\t',
  '\r',
  '\n',
  '"',
  "'",
  '`',
  '<',
  '>',
  '(',
  ')',
  '{',
  '}',
  '[',
  ']',
  ',',
]);

export function extractTokenAtPosition(
  document: vscode.TextDocument,
  position: vscode.Position
): string | undefined {
  const line = document.lineAt(position.line).text;
  if (line.length === 0) return undefined;

  let start = position.character;
  let end = position.character;

  if (start >= line.length) start = line.length - 1;

  // If cursor is on a stop char, try left.
  if (STOP_CHARS.has(line[start])) {
    if (start === 0) return undefined;
    start -= 1;
    end = start;
  }

  while (start > 0 && !STOP_CHARS.has(line[start - 1])) start -= 1;
  while (end < line.length && !STOP_CHARS.has(line[end])) end += 1;

  const token = line.slice(start, end).trim();
  if (token.length === 0) return undefined;

  // Very light validation: must have at least one non-punctuation char.
  if (!/[a-zA-Z0-9_$]/.test(token)) return undefined;

  return token;
}
