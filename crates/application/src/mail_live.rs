//! Read-only Apple Mail search (macOS + BELEGWERK_LIVE_CONNECTORS). Send stays approval-gated elsewhere.

use crate::macos_probes::live_connectors_enabled;
use belegwerk_domain::mail::MailHit;
#[cfg(target_os = "macos")]
use serde::Deserialize;
#[cfg(target_os = "macos")]
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MailLiveError {
    #[error("live connectors disabled — set BELEGWERK_LIVE_CONNECTORS=1")]
    LiveDisabled,
    #[error("Apple Mail live search requires macOS")]
    NotMacos,
    #[error("{0}")]
    Osascript(String),
    #[error("json: {0}")]
    Json(String),
}

#[cfg(target_os = "macos")]
#[derive(Debug, Deserialize)]
struct MailRow {
    id: String,
    subject: String,
    from: String,
    received_at: String,
}

/// Search recent inbox messages whose subject contains `needle` (case-insensitive via AppleScript).
pub fn search_inbox_subject(needle: &str, limit: usize) -> Result<Vec<MailHit>, MailLiveError> {
    if !live_connectors_enabled() {
        return Err(MailLiveError::LiveDisabled);
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (needle, limit);
        Err(MailLiveError::NotMacos)
    }
    #[cfg(target_os = "macos")]
    {
        let lim = limit.clamp(1, 50);
        // Build a small JSON array in AppleScript to avoid brittle line parsing.
        let escaped = needle.replace('\\', "\\\\").replace('"', "\\\"");
        let script = format!(
            r#"
set needle to "{escaped}"
set lim to {lim}
set out to "{{ \"hits\": ["
set firstItem to true
tell application "Mail"
  set msgs to (messages of inbox whose subject contains needle)
  set n to 0
  repeat with m in msgs
    if n ≥ lim then exit repeat
    set n to n + 1
    set subj to my esc(subject of m)
    set frm to my esc(sender of m)
    set rid to id of m as string
    set rcv to (date received of m) as string
    set rcvEsc to my esc(rcv)
    if firstItem is false then set out to out & ","
    set firstItem to false
    set out to out & "{{ \"id\": \"" & rid & "\", \"subject\": \"" & subj & "\", \"from\": \"" & frm & "\", \"received_at\": \"" & rcvEsc & "\" }}"
  end repeat
end tell
set out to out & "] }}"
return out

on esc(t)
  set t to my replaceText(t, "\\", "\\\\")
  set t to my replaceText(t, "\"", "\\\"")
  set t to my replaceText(t, return, " ")
  set t to my replaceText(t, linefeed, " ")
  return t
end esc

on replaceText(theText, oldString, newString)
  set AppleScript's text item delimiters to oldString
  set theItems to every text item of theText
  set AppleScript's text item delimiters to newString
  set theText to theItems as string
  set AppleScript's text item delimiters to ""
  return theText
end replaceText
"#
        );
        let output = Command::new("osascript")
            .args(["-e", &script])
            .output()
            .map_err(|e| MailLiveError::Osascript(e.to_string()))?;
        if !output.status.success() {
            return Err(MailLiveError::Osascript(format!(
                "exit {:?} stderr={}",
                output.status.code(),
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        #[derive(Deserialize)]
        struct Wrap {
            hits: Vec<MailRow>,
        }
        let wrap: Wrap =
            serde_json::from_str(stdout.trim()).map_err(|e| MailLiveError::Json(e.to_string()))?;
        Ok(wrap
            .hits
            .into_iter()
            .map(|r| MailHit {
                id: r.id,
                subject: r.subject,
                from: r.from,
                received_at: r.received_at,
            })
            .collect())
    }
}

/// Ensure a Belegwerk Keychain slot exists (generic password, no real secret payload logged).
pub fn ensure_keychain_slot() -> Result<String, MailLiveError> {
    if !live_connectors_enabled() {
        return Err(MailLiveError::LiveDisabled);
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err(MailLiveError::NotMacos)
    }
    #[cfg(target_os = "macos")]
    {
        const SERVICE: &str = "Belegwerk";
        const ACCOUNT: &str = "live-connector-slot";
        let find = Command::new("security")
            .args(["find-generic-password", "-s", SERVICE, "-a", ACCOUNT])
            .output()
            .map_err(|e| MailLiveError::Osascript(e.to_string()))?;
        if find.status.success() {
            return Ok("keychain-slot:exists".into());
        }
        let add = Command::new("security")
            .args([
                "add-generic-password",
                "-U",
                "-s",
                SERVICE,
                "-a",
                ACCOUNT,
                "-w",
                "belegwerk-slot-marker",
            ])
            .output()
            .map_err(|e| MailLiveError::Osascript(e.to_string()))?;
        if !add.status.success() {
            return Err(MailLiveError::Osascript(format!(
                "security add failed: {}",
                String::from_utf8_lossy(&add.stderr)
            )));
        }
        Ok("keychain-slot:created".into())
    }
}
