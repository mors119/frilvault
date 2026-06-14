const fs = require('fs');
const path = require('path');
const childProcess = require('child_process');

const extensionRoot = path.resolve(__dirname, '..');
const workspaceRoot = path.resolve(extensionRoot, '..', '..');
const profile = process.argv.includes('--production') ? 'release' : 'debug';
const crateName = 'frilvault-node';
const dylibName = {
  darwin: `libfrilvault_node.dylib`,
  linux: `libfrilvault_node.so`,
  win32: `frilvault_node.dll`,
}[process.platform];

if (!dylibName) {
  throw new Error(`Unsupported platform: ${process.platform}`);
}

childProcess.execFileSync(
  'cargo',
  ['build', '-p', crateName, ...(profile === 'release' ? ['--release'] : [])],
  {
    cwd: workspaceRoot,
    stdio: 'inherit',
  },
);

const source = path.join(workspaceRoot, 'target', profile, dylibName);
const destinationDir = path.join(extensionRoot, 'dist');
const destination = path.join(destinationDir, 'frilvault.node');

fs.mkdirSync(destinationDir, { recursive: true });
fs.copyFileSync(source, destination);
