import * as vscode from 'vscode';

export type TraceLevel = 'off' | 'messages' | 'verbose';

export class Logger {
  private readonly channel = vscode.window.createOutputChannel('Grimoire CSS Client');

  constructor(private readonly getTraceLevel: () => TraceLevel) {}

  info(message: string): void {
    this.channel.appendLine(message);
  }

  debug(message: string): void {
    if (this.getTraceLevel() === 'verbose') {
      this.channel.appendLine(`[debug] ${message}`);
    }
  }

  trace(message: string): void {
    const level = this.getTraceLevel();
    if (level === 'messages' || level === 'verbose') {
      this.channel.appendLine(`[trace] ${message}`);
    }
  }

  show(): void {
    this.channel.show(true);
  }
}
