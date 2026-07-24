import { promises as fs, constants as fsConstants } from 'node:fs';
import path from 'node:path';
import { execFile as execFileCallback } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';

const execFile = promisify(execFileCallback);
const SUPPORTED_TARGETS = new Set([
  'darwin-arm64',
  'darwin-x64',
  'linux-x64',
  'win32-x64',
]);
const VERSION_PATTERN = /\b(\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?)\b/;

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const extensionDir = path.resolve(scriptDir, '..');
const repoRoot = path.resolve(extensionDir, '..', '..');

const packageJson = JSON.parse(
  await fs.readFile(path.join(extensionDir, 'package.json'), 'utf8'),
);

const target = readArg('--target') ?? inferTarget(process.platform, process.arch);

if (!SUPPORTED_TARGETS.has(target)) {
  throw new Error(`Unsupported VSIX target: ${target}`);
}

const cliBinary =
  readArg('--cli')
  ?? process.env.FRILVAULT_CLI_BINARY
  ?? defaultCliBinaryPath(target);

await prepareBundledCli({
  cliBinary,
  expectedVersion: packageJson.frilvaultBundledCliVersion ?? packageJson.version,
  extensionDir,
  target,
});

async function prepareBundledCli({
  cliBinary,
  expectedVersion,
  extensionDir,
  target,
}) {
  const destinationDir = path.join(extensionDir, 'bin');
  const destinationPath = path.join(
    destinationDir,
    target.startsWith('win32-') ? 'flvt.exe' : 'flvt',
  );

  await fs.rm(path.join(destinationDir, 'flvt'), { force: true });
  await fs.rm(path.join(destinationDir, 'flvt.exe'), { force: true });

  await fs.mkdir(destinationDir, { recursive: true });
  await fs.copyFile(cliBinary, destinationPath);

  if (!target.startsWith('win32-')) {
    await fs.chmod(destinationPath, 0o755);
    await fs.access(destinationPath, fsConstants.X_OK);
  }

  const version = await readCliVersion(destinationPath);

  if (expectedVersion && version !== expectedVersion) {
    throw new Error(
      `Bundled CLI version mismatch. Expected ${expectedVersion}, found ${version}.`,
    );
  }

  console.log(`Bundled ${destinationPath} for ${target} (${version})`);
}

function defaultCliBinaryPath(target) {
  const hostTarget = inferTarget(process.platform, process.arch);

  if (target !== hostTarget) {
    throw new Error(
      `Target ${target} does not match the current host ${hostTarget}. Set FRILVAULT_CLI_BINARY to a prebuilt CLI for cross-target packaging.`,
    );
  }

  return path.join(
    repoRoot,
    'target',
    'release',
    target.startsWith('win32-') ? 'flvt.exe' : 'flvt',
  );
}

async function readCliVersion(cliBinary) {
  const { stdout } = await execFile(cliBinary, ['--version'], {
    cwd: repoRoot,
  });
  const version = stdout.match(VERSION_PATTERN)?.[1];

  if (!version) {
    throw new Error(`Could not parse CLI version from: ${stdout.trim()}`);
  }

  return version;
}

function inferTarget(platform, arch) {
  if (platform === 'darwin' && arch === 'arm64') {
    return 'darwin-arm64';
  }

  if (platform === 'darwin' && arch === 'x64') {
    return 'darwin-x64';
  }

  if (platform === 'linux' && arch === 'x64') {
    return 'linux-x64';
  }

  if (platform === 'win32' && arch === 'x64') {
    return 'win32-x64';
  }

  throw new Error(`Unsupported host platform for VSIX packaging: ${platform}-${arch}`);
}

function readArg(flag) {
  const index = process.argv.indexOf(flag);
  return index >= 0 ? process.argv[index + 1] : undefined;
}
