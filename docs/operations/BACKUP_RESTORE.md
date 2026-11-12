# Belegwerk – Backup & Restore (Stub)

Stand: 12. November 2026 (Europe/Berlin)  
Bezug: D-08, D-20, REQ-BU-001/002, Product Description §24

**Status:** Stub — Messziele gesetzt; konkrete Medien vom Nutzer vor Produktivbetrieb zu bestätigen.

---

## 1. Goals (not an SLA)

| Metric | Target | Meaning |
|---|---|---|
| RPO | ≤ 24 hours | Local backup at least daily or after critical batch completion |
| RTO | ≤ 4 hours | Documented restore on same Mac performance class |

## 2. What a backup must include

1. Primary SQLite database file(s) for the BusinessProfile.  
2. Content-addressed object store (`objects/` + optional `derived/`).  
3. App config and credential **references** (not Keychain secrets themselves).  
4. Plugin/manifest versions and tax-rule pack versions in use.  
5. Migration version / schema stamp.

## 3. Key strategy (draft)

- DB encryption key lives in macOS Keychain (target).  
- Restore documentation must include how to recover/re-enter key material.  
- Backups without recoverable key material are useless for encrypted DBs — call this out in UI later.  
- Exact SQLCipher crate pin: open (`OPEN_QUESTIONS.md`).

## 4. Restore drill (planned M14)

1. Restore DB + objects to a throwaway data root.  
2. Verify schema version and sample synthetic records.  
3. Verify object hash integrity for a fixture set.  
4. Measure elapsed time vs. RTO goal.  
5. Record results; never claim success without a run.

## 5. Media (deferred)

Disk image, Time Machine volume, encrypted external drive, etc. — **user confirms before production**. Tracked as deferred item.

## 6. Non-goals for this stub

- No automatic cloud backup in v1 default.  
- No claim of GoBD-certified archiving.  
- No productive Belege in test restores — synthetic fixtures only.
