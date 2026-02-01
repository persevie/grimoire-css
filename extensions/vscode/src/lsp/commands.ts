import type { LanguageClient } from 'vscode-languageclient/node';

export type StatsArgs = {
  group?: 'spells' | 'scrolls' | 'vars';
  top?: number;
  token?: string;
  silent?: boolean;
};

export type LintMessage = {
  level: string;
  code: string;
  message: string;
  occurrence?: {
    token: string;
    file: string;
    byte_offset: number;
    byte_len: number;
    line: number;
    column: number;
  };
};
export type LintResult = { errors: LintMessage[]; warnings: LintMessage[]; notes: LintMessage[] };

export type DryCandidateOccurrence = {
  file: string;
  line: number;
  column: number;
  tokens?: string[];
};
export type DryCandidate = {
  tokens: string[];
  support: number;
  occurrences: DryCandidateOccurrence[];
};
export type DryCandidatesResult = {
  files_scanned: number;
  class_occurrences: number;
  candidates: DryCandidate[];
};

export type ConfigSummary = {
  config_path: string;
  projects: Array<{
    name: string;
    input_paths: string[];
    output_dir_path?: string | null;
    single_output_file_name?: string | null;
  }>;
  scrolls: string[];
  variables: Array<{ name: string; value: string }>;
  shared_spells: string[];
  custom_animations: string[];
  css_custom_properties: string[];
  external_scroll_files: string[];
  external_variable_files: string[];
};

export type ScrollSummary = {
  name: string;
  spells: number;
  spellsPreview: string[];
  overloads: string[];
  overloadSpellsPreview: Record<string, string[]> | null;
  location: { uri: string; line: number; character: number } | null;
};

export type ListScrollsResult = { scrolls: ScrollSummary[] };

export type ExplainClassTokenResult = {
  class_token: string;
  expanded_spells: string[];
  css: string;
};

export type RefsResult = {
  query: string;
  results: Array<{ kind: 'scroll' | 'var' | 'spell'; name: string; refs: unknown[] }>;
};

export type LspPosition = { line: number; character: number };
export type LspRange = { start: LspPosition; end: LspPosition };
export type LspLocation = { uri: string; range: LspRange };

export type LspTextEdit = { range: LspRange; newText: string };
export type LspWorkspaceEdit = { changes?: Record<string, LspTextEdit[]> };

export type DocumentSpellsArgs = { uri: string };
export type DocumentSpellsResult = LspRange[];

export type EntityDetailsArgs = { kind: 'scroll' | 'var' | 'function' | 'animation'; name: string };
export type EntityDetailsResult = {
  kind: 'scroll' | 'var' | 'function' | 'animation' | string;
  name: string;
  definitions: LspLocation[];
  definition_conflict: boolean;
  refs: {
    project: LspLocation[];
    config: LspLocation[];
  };
  usage_count: {
    project: number;
    config: number;
    total: number;
  };
  css?: string | null;
  value?: string | null;
  usage?: string | null;
  example?: string | null;
  source_path?: string | null;
  animation_kind?: string | null;
  group?: string | null;
};

export type ExplorerFunctionInfo = {
  name: string;
  group: string;
  usage: string;
};

export type ExplorerAnimationInfo = {
  name: string;
  kind: 'built-in' | 'custom' | string;
  source_path?: string | null;
};

export type ExplorerIndexResult = {
  scrolls: string[];
  variables: Array<{ name: string; value: string }>;
  functions: ExplorerFunctionInfo[];
  animations: ExplorerAnimationInfo[];
};

export type DryCreateScrollArgs = { scrollName: string; tokens: string[] };

export async function executeCommand<T>(
  client: LanguageClient,
  command: string,
  args: unknown[],
  timeoutMs: number
): Promise<T> {
  // vscode-languageclient v9 doesn't expose a typed ExecuteCommand request helper.
  // Use method string + a hard timeout.
  return await promiseWithTimeout(
    client.sendRequest<T>('workspace/executeCommand', { command, arguments: args }),
    timeoutMs,
    `workspace/executeCommand timed out after ${timeoutMs}ms (${command})`
  );
}

function promiseWithTimeout<T>(
  promise: Promise<T>,
  timeoutMs: number,
  message: string
): Promise<T> {
  let timer: NodeJS.Timeout | undefined;
  const timeout = new Promise<never>((_, reject) => {
    timer = setTimeout(() => reject(new Error(message)), timeoutMs);
  });

  return Promise.race([promise, timeout]).finally(() => {
    if (timer) clearTimeout(timer);
  });
}
