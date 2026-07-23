import * as path from 'node:path';

import * as vscode from 'vscode';

export type GutterMarkerStyle = 'dot' | 'count' | 'bar';

export function getConfiguredMarkerStyle(): GutterMarkerStyle {
  const configured = vscode.workspace
    .getConfiguration('frilvault')
    .get<string>('gutterMarkerStyle', 'dot');

  if (configured === 'count' || configured === 'bar') {
    return configured;
  }

  return 'dot';
}

export function createMarkerDecorationType(
  extensionPath: string,
  style: GutterMarkerStyle,
): vscode.TextEditorDecorationType {
  if (style === 'count') {
    return vscode.window.createTextEditorDecorationType({});
  }

  const iconName =
    style === 'bar' ? 'frilvault-marker-bar' : 'frilvault-marker-dot';

  return vscode.window.createTextEditorDecorationType({
    gutterIconPath: markerIconUri(extensionPath, iconName, 'light'),
    gutterIconSize: 'contain',
    light: {
      gutterIconPath: markerIconUri(extensionPath, iconName, 'light'),
    },
    dark: {
      gutterIconPath: markerIconUri(extensionPath, iconName, 'dark'),
    },
  });
}

export function markerRenderOptions(
  style: GutterMarkerStyle,
  noteCount: number,
): vscode.DecorationInstanceRenderOptions | undefined {
  if (style !== 'count') {
    return undefined;
  }

  const label = noteCount > 9 ? '9+' : String(noteCount);

  return {
    before: {
      contentText: label,
      color: new vscode.ThemeColor('editorInfo.foreground'),
      fontWeight: 'bold',
      margin: '0 1ch 0 0',
    },
  };
}

function markerIconUri(
  extensionPath: string,
  iconName: string,
  variant: 'light' | 'dark',
): vscode.Uri {
  return vscode.Uri.file(
    path.join(extensionPath, 'media', `${iconName}-${variant}.svg`),
  );
}
