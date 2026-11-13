//! Capability probes for macOS live connectors (feature-flagged; fixture remains default).

use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProbeResult {
    pub id: String,
    pub available: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacosProbeReport {
    pub platform: String,
    pub probes: Vec<ProbeResult>,
    pub live_enabled: bool,
}

/// `BELEGWERK_LIVE_CONNECTORS=1` enables live paths; otherwise report-only.
pub fn live_connectors_enabled() -> bool {
    std::env::var("BELEGWERK_LIVE_CONNECTORS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn run_probes() -> MacosProbeReport {
    let live_enabled = live_connectors_enabled();
    #[cfg(target_os = "macos")]
    {
        MacosProbeReport {
            platform: "macos".into(),
            probes: vec![
                probe_moneymoney(),
                probe_keychain_slot(),
                probe_mail(),
                probe_tauri_runtime(),
            ],
            live_enabled,
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        MacosProbeReport {
            platform: std::env::consts::OS.into(),
            probes: vec![ProbeResult {
                id: "macos_only".into(),
                available: false,
                detail: "Live connectors require macOS".into(),
            }],
            live_enabled,
        }
    }
}

#[cfg(target_os = "macos")]
fn probe_moneymoney() -> ProbeResult {
    let output = Command::new("osascript")
        .args([
            "-e",
            "tell application \"System Events\" to (name of processes) contains \"MoneyMoney\"",
        ])
        .output();
    match output {
        Ok(o) if o.status.success() => {
            let stdout = String::from_utf8_lossy(&o.stdout).trim().to_string();
            let running = stdout.eq_ignore_ascii_case("true");
            ProbeResult {
                id: "moneymoney".into(),
                available: running,
                detail: if running {
                    "MoneyMoney process detected (read-only AppleScript path gated by BELEGWERK_LIVE_CONNECTORS)".into()
                } else {
                    "MoneyMoney not running — fixture replay remains default".into()
                },
            }
        }
        Ok(o) => ProbeResult {
            id: "moneymoney".into(),
            available: false,
            detail: format!(
                "osascript exit {:?} — {}",
                o.status.code(),
                String::from_utf8_lossy(&o.stderr)
            ),
        },
        Err(e) => ProbeResult {
            id: "moneymoney".into(),
            available: false,
            detail: format!("osascript unavailable: {e}"),
        },
    }
}

#[cfg(target_os = "macos")]
fn probe_keychain_slot() -> ProbeResult {
    // Do not write secrets — only check `security` binary exists.
    let ok = Command::new("security")
        .arg("help")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    ProbeResult {
        id: "keychain".into(),
        available: ok,
        detail: if ok {
            "security CLI present — credential slot can use Keychain when live flag is on".into()
        } else {
            "security CLI missing".into()
        },
    }
}

#[cfg(target_os = "macos")]
fn probe_mail() -> ProbeResult {
    let output = Command::new("osascript")
        .args([
            "-e",
            "tell application \"System Events\" to (name of processes) contains \"Mail\"",
        ])
        .output();
    match output {
        Ok(o) if o.status.success() => {
            let running = String::from_utf8_lossy(&o.stdout)
                .trim()
                .eq_ignore_ascii_case("true");
            ProbeResult {
                id: "apple_mail".into(),
                available: running,
                detail: if running {
                    "Apple Mail running — search/draft live gated; send remains approval-gated".into()
                } else {
                    "Apple Mail not running — stub remains default".into()
                },
            }
        }
        _ => ProbeResult {
            id: "apple_mail".into(),
            available: false,
            detail: "Could not probe Mail via osascript".into(),
        },
    }
}

#[cfg(target_os = "macos")]
fn probe_tauri_runtime() -> ProbeResult {
    ProbeResult {
        id: "tauri_gui".into(),
        available: true,
        detail: "Running inside desktop process — use docs/acceptance/MACOS_SMOKE.md".into(),
    }
}
