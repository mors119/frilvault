import * as fs from 'node:fs';
import * as path from 'node:path';

import * as vscode from 'vscode';

export type CliSource = 'configured' | 'bundled' | 'missing';

export interface CliResolution {
  cliPath?: string;
  source: CliSource;
}

export interface ResolveCliPathInput {
  configuredCliPath: string;
  extensionPath?: string;
  platform?: NodeJS.Platform;
  existsSync?: (filePath: string) => boolean;
}

export function getConfiguredCliPath(): string {
  return vscode.workspace
    .getConfiguration('frilvault')
    .get<string>('cliPath', '')
    .trim();
}

export function bundledCliFileName(platform = process.platform): string {
  return platform === 'win32' ? 'flvt.exe' : 'flvt';
}

export function resolveBundledCliPath(
  extensionPath: string,
  platform = process.platform,
): string {
  return path.join(extensionPath, 'bin', bundledCliFileName(platform));
}

export function resolveCliPath(input: ResolveCliPathInput): CliResolution {
  if (input.configuredCliPath.length > 0) {
    return {
      cliPath: input.configuredCliPath,
      source: 'configured',
    };
  }

  if (!input.extensionPath) {
    return { source: 'missing' };
  }

  const existsSync = input.existsSync ?? fs.existsSync;
  const bundledCliPath = resolveBundledCliPath(
    input.extensionPath,
    input.platform ?? process.platform,
  );

  if (existsSync(bundledCliPath)) {
    return {
      cliPath: bundledCliPath,
      source: 'bundled',
    };
  }

  return { source: 'missing' };
}
