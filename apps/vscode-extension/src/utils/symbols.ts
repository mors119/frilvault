import * as vscode from 'vscode';

export async function findSymbolAtPosition(
  document: vscode.TextDocument,
  position: vscode.Position,
): Promise<vscode.DocumentSymbol | undefined> {
  const symbols = await vscode.commands.executeCommand<
    vscode.DocumentSymbol[] | undefined
  >('vscode.executeDocumentSymbolProvider', document.uri);

  if (!symbols || symbols.length === 0) {
    return undefined;
  }

  return findInnermostSymbol(symbols, position);
}

function findInnermostSymbol(
  symbols: readonly vscode.DocumentSymbol[],
  position: vscode.Position,
): vscode.DocumentSymbol | undefined {
  for (const symbol of symbols) {
    if (!symbol.range.contains(position)) {
      continue;
    }

    const nested = findInnermostSymbol(symbol.children, position);
    return nested ?? symbol;
  }

  return undefined;
}

export function mapDocumentSymbolKind(kind: vscode.SymbolKind): string {
  switch (kind) {
    case vscode.SymbolKind.Function:
      return 'function';
    case vscode.SymbolKind.Method:
      return 'method';
    case vscode.SymbolKind.Class:
    case vscode.SymbolKind.Struct:
      return 'struct';
    case vscode.SymbolKind.Enum:
      return 'enum';
    case vscode.SymbolKind.Interface:
      return 'trait';
    default:
      return 'unknown';
  }
}

export function readSymbolSignature(
  document: vscode.TextDocument,
  symbol: vscode.DocumentSymbol,
): string | undefined {
  const line = document.lineAt(symbol.range.start.line).text.trim();
  return line.length > 0 ? line : undefined;
}
