import * as vscode from 'vscode';

export class InvalidFrilVaultUriError extends Error {
  public constructor(message: string) {
    super(message);
    this.name = 'InvalidFrilVaultUriError';
  }
}

export function decodeQueryParameter(
  value: string | undefined,
  name: string,
): string | undefined {
  if (value === undefined) {
    return undefined;
  }

  try {
    return decodeURIComponent(value);
  } catch {
    throw new InvalidFrilVaultUriError(
      `Invalid percent encoding in query parameter: ${name}`,
    );
  }
}

export function parseWorkspaceQuery(uri: vscode.Uri): string | undefined {
  if (!uri.query) {
    return undefined;
  }

  for (const segment of uri.query.split('&')) {
    if (!segment.startsWith('workspace=')) {
      continue;
    }

    const rawValue = segment.slice('workspace='.length);

    if (rawValue.length === 0) {
      return undefined;
    }

    return decodeQueryParameter(rawValue, 'workspace');
  }

  return undefined;
}
