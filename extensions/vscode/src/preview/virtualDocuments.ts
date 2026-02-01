import * as vscode from 'vscode';

const providers = new Map<string, vscode.Disposable>();

export async function openVirtualText(
  uriString: string,
  content: string,
  languageId?: string
): Promise<void> {
  registerProviderIfNeeded(uriString, content);

  const uri = vscode.Uri.parse(uriString);
  const doc = await vscode.workspace.openTextDocument(uri);
  const editor = await vscode.window.showTextDocument(doc, { preview: true });

  if (languageId) {
    await vscode.languages.setTextDocumentLanguage(editor.document, languageId);
  }
}

export async function openVirtualJson(uriString: string, value: unknown): Promise<void> {
  const content = JSON.stringify(value, null, 2);
  await openVirtualText(uriString, content, 'json');
}

function registerProviderIfNeeded(uriString: string, content: string): void {
  const uri = vscode.Uri.parse(uriString);
  const scheme = uri.scheme;

  if (providers.has(scheme)) {
    // Provider already exists; content is keyed by full URI.
    VirtualContentProvider.set(uri.toString(), content);
    return;
  }

  const provider = new VirtualContentProvider();
  provider.set(uri.toString(), content);

  const disposable = vscode.workspace.registerTextDocumentContentProvider(scheme, provider);
  providers.set(scheme, disposable);
}

class VirtualContentProvider implements vscode.TextDocumentContentProvider {
  private static readonly contents = new Map<string, string>();

  static set(uri: string, content: string): void {
    VirtualContentProvider.contents.set(uri, content);
  }

  private readonly onDidChangeEmitter = new vscode.EventEmitter<vscode.Uri>();
  readonly onDidChange = this.onDidChangeEmitter.event;

  set(uri: string, content: string): void {
    VirtualContentProvider.contents.set(uri, content);
    this.onDidChangeEmitter.fire(vscode.Uri.parse(uri));
  }

  provideTextDocumentContent(uri: vscode.Uri): string {
    return VirtualContentProvider.contents.get(uri.toString()) ?? '';
  }
}
