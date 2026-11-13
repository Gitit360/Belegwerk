#!/usr/bin/env bash
# Fail fast if this Mac is building Belegwerk under Rosetta / x86_64 Rust.
# Non-Darwin hosts (Linux CI) skip — crates still build there; Tauri is macOS-primary.
set -euo pipefail

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "OK: non-macOS ($(uname -s)/$(uname -m)); Silicon check skipped"
  exit 0
fi

machine="$(uname -m)"
if [[ "$machine" != "arm64" ]]; then
  echo "Belegwerk erwartet Apple Silicon (arm64). Aktuell: $machine" >&2
  exit 1
fi


if [[ "$(sysctl -n sysctl.proc_translated 2>/dev/null || echo 0)" == "1" ]]; then
  echo "Diese Shell läuft unter Rosetta. Bitte natives Terminal (arm64) nutzen." >&2
  exit 1
fi

host="$(rustc -vV | awk '/^host:/{print $2}')"
if [[ "$host" != "aarch64-apple-darwin" ]]; then
  echo "rustc-Host ist '$host', erwartet aarch64-apple-darwin." >&2
  echo "Fix: rustup set default-host aarch64-apple-darwin && rustup toolchain install stable" >&2
  exit 1
fi

rustc_bin="$(rustup which rustc)"
arch_line="$(file "$rustc_bin")"
if [[ "$arch_line" != *"arm64"* ]]; then
  echo "rustc ist nicht arm64: $arch_line" >&2
  exit 1
fi

echo "OK: Apple Silicon native (rustc host=$host)"
