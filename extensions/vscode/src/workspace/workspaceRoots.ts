import * as vscode from 'vscode';

const CONFIG_RELATIVE_PATH = 'grimoire/config/grimoire.config.json';

export async function findWorkspaceFoldersWithConfig(): Promise<vscode.WorkspaceFolder[]> {
  const folders = vscode.workspace.workspaceFolders ?? [];
  const results: vscode.WorkspaceFolder[] = [];

  for (const folder of folders) {
    if (await hasConfigAtWorkspaceRoot(folder)) {
      results.push(folder);
    }
  }

  // If the extension activated but we didn't find root-level config, it's likely nested.
  if (results.length === 0) {
    const nested = await vscode.workspace.findFiles(
      `**/${CONFIG_RELATIVE_PATH}`,
      '**/node_modules/**',
      5
    );
    if (nested.length > 0) {
      const sample = nested
        .slice(0, 5)
        .map((u) => u.fsPath)
        .join('\n');
      void vscode.window.showWarningMessage(
        'GrimoireCSS: found grimoire.config.json, but not at a workspace folder root. ' +
          'Open the folder that contains `grimoire/` as the workspace root.\n\n' +
          sample
      );
    }
  }

  return results;
}

export async function hasConfigAtWorkspaceRoot(folder: vscode.WorkspaceFolder): Promise<boolean> {
  const uri = vscode.Uri.joinPath(folder.uri, CONFIG_RELATIVE_PATH);
  try {
    const stat = await vscode.workspace.fs.stat(uri);
    return stat.type === vscode.FileType.File;
  } catch {
    return false;
  }
}

export function configPathUri(folder: vscode.WorkspaceFolder): vscode.Uri {
  return vscode.Uri.joinPath(folder.uri, CONFIG_RELATIVE_PATH);
}
