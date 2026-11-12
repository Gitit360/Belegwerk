# Module surface checklist

| Modul | Code surface | Runnable on this Linux VM |
|---|---|---|
| M00 | Docs | yes |
| M01 | Shell + tokens + CI | Vite yes; Tauri window no |
| M02 | CommandBus + PermissionGate on all commands | unit/integration tests yes |
| M03 | ImportPipeline (hash → blob → DB rows) | unit tests yes |
| M04 | BankImportService + synthetic upsert | Linux yes; AppleScript no |
| M05 | MatchingEngine + three-pane fixture UI | types/tests/UI yes |
| M06 | ReminderService (cases/placeholders/escalation) | yes |
| M07 | extraction stub | yes |
| M08 | AiProvider + NullAiProvider | yes (offline) |
| M09 | GrandTotal file stub | yes |
| M10 | mail/portal slots | stubs only |
| M11 | Balanced journal drafts; blocked unverified tax | yes |
| M12 | TaxReturnDraft shells | yes |
| M13 | OpenAPI + CLI + loopback HTTP stub | yes |
| M14 | backup copy helper + docs | partial |
| M15 | extension hook docs | hooks only |
