import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import fs from 'node:fs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const extensionDir = path.resolve(scriptDir, '..');
const packageJson = JSON.parse(
  fs.readFileSync(path.join(extensionDir, 'package.json'), 'utf8'),
);

const target = readArg('--target');

if (!target) {
  throw new Error('Missing required --target argument.');
}

const cliBinary = readArg('--cli') ?? process.env.FRILVAULT_CLI_BINARY;
const nodeBin = process.execPath;
const prepareScript = path.join(scriptDir, 'prepare-bundled-cli.mjs');
const vsceBinary = path.join(
  extensionDir,
  'node_modules',
  '.bin',
  process.platform === 'win32' ? 'vsce.cmd' : 'vsce',
);

const prepareArgs = [prepareScript, '--target', target];
if (cliBinary) {
  prepareArgs.push('--cli', cliBinary);
}

execFileSync(nodeBin, prepareArgs, {
  cwd: extensionDir,
  stdio: 'inherit',
});

const listedFiles = execFileSync(vsceBinary, ['ls'], {
  cwd: extensionDir,
  encoding: 'utf8',
});

const expectedBundledPath = target.startsWith('win32-') ? 'bin/flvt.exe' : 'bin/flvt';
if (!listedFiles.includes(expectedBundledPath)) {
  throw new Error(`VSIX contents are missing ${expectedBundledPath}.`);
}

const outputName = `frilvault-${packageJson.version}-${target}.vsix`;

execFileSync(
  vsceBinary,
  ['package', '--target', target, '--out', outputName],
  {
    cwd: extensionDir,
    stdio: 'inherit',
  },
);

console.log(`Created ${outputName}`);

function readArg(flag) {
  const index = process.argv.indexOf(flag);
  return index >= 0 ? process.argv[index + 1] : undefined;
}
