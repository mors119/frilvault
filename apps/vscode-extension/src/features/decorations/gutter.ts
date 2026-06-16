import * as path from 'node:path';

import * as vscode from 'vscode';

export function createLineNoteDecorationType(extensionPath: string): vscode.TextEditorDecorationType {
  return vscode.window.createTextEditorDecorationType({
    gutterIconPath: vscode.Uri.file(path.join(extensionPath, 'media', 'frilvault-line-note.svg')),
    gutterIconSize: 'contain',
  });
}

export function createSymbolNoteDecorationType(
  extensionPath: string,
): vscode.TextEditorDecorationType {
  return vscode.window.createTextEditorDecorationType({
    gutterIconPath: vscode.Uri.file(path.join(extensionPath, 'media', 'frilvault-symbol-note.svg')),
    gutterIconSize: 'contain',
  });
}
