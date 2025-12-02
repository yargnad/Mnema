#!/usr/bin/env node
import { cpSync, existsSync, mkdirSync, statSync } from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), "..");
const srcTauriDir = path.join(repoRoot, "src-tauri");
const canonicalDll = path.join(srcTauriDir, "resources", "onnxruntime", "onnxruntime.dll");

if (!existsSync(canonicalDll)) {
  console.error("[sync-onnxruntime] Missing canonical DLL at", canonicalDll);
  process.exit(1);
}

const targets = [
  path.join(srcTauriDir, "onnxruntime.dll"),
  path.join(srcTauriDir, "target", "debug", "onnxruntime.dll"),
  path.join(srcTauriDir, "target", "release", "onnxruntime.dll"),
  path.join(srcTauriDir, "target", "debug", "resources", "onnxruntime", "onnxruntime.dll"),
  path.join(srcTauriDir, "target", "release", "resources", "onnxruntime", "onnxruntime.dll"),
];

const formatRelative = (absPath) => path.relative(repoRoot, absPath) || absPath;

const filesDiffer = (dest) => {
  if (!existsSync(dest)) {
    return true;
  }

  try {
    const srcStats = statSync(canonicalDll);
    const destStats = statSync(dest);
    return srcStats.size !== destStats.size || srcStats.mtimeMs > destStats.mtimeMs;
  } catch (err) {
    return true;
  }
};

const ensureDir = (dir) => {
  if (!existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
  }
};

let copies = 0;
for (const target of targets) {
  const targetDir = path.dirname(target);
  ensureDir(targetDir);

  if (filesDiffer(target)) {
    cpSync(canonicalDll, target);
    console.log(`[sync-onnxruntime] Updated ${formatRelative(target)}`);
    copies += 1;
  }
}

if (copies === 0) {
  console.log("[sync-onnxruntime] DLL already in sync");
}
