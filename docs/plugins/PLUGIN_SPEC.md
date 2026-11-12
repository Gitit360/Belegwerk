# Belegwerk – Plugin Specification (Repo)

Stand: 12. November 2026 (Europe/Berlin)  
Canonical draft: `belegwerk-plugin-contract-v1.1.md`  
`sdkVersion`: **"1"**

License note (D-09): App proprietary/private for now; this SDK contract text is Apache-2.0-oriented reserved — final license deferred.

---

## 1. Purpose

Plugins extend Belegwerk (bank, invoice, document, export, portal, mail, OCR, AI, reporting, delivery) **without** changing the accounting core. Tax-rule plugins are a protected kind with extra review.

## 2. Manifest (sdkVersion 1)

Required fields: `id`, `version`, `sdkVersion`, `kind`, `displayName`, `capabilities`, `permissions`, `entrypoint`.

`kind` values: `bank-import` · `invoice-import` · `document-source` · `export` · `portal` · `mail` · `ocr` · `ai-provider` · `reporting` · `delivery` · `tax-rules` (protected).

See TypeScript types: `packages/plugin-sdk/src/manifest.ts`.

## 3. Capabilities (excerpt)

| Capability | Allowed | Forbidden |
|---|---|---|
| `documents.list` / `documents.download` | List/download to host staging | Overwrite core originals |
| `bank.transactions.read` | Read via adapter contract | Any payment creation |
| `mail.draft.create` | Draft | `mail.send` without approval |
| `export.build` / `delivery.submit` | Prepare / submit after approval | Silent upload / blind retry-as-success |
| `tax.rules.evaluate` | Approved rule packs only | Unreviewed web rules |

## 4. Message envelope

JSON Lines; fields `v`, `id`, `type`, `method`, `correlationId`, `payload`, `error`.

Host→Plugin required: `plugin.handshake`, `plugin.configure`, `plugin.shutdown`, `plugin.cancel`.

## 5. Bank adapter minimum fields

`providerId`, `sourceAccountId`, `sourceTransactionRef`, `bookingDate`, `valueDate`, `amountMinor`, `currency`, `counterparty`, `purpose`, `status`, `importedAt`, `rawRef`.

Mapping to `SourceTransaction` happens in application layer. Merging into `EconomicTransaction` requires user review when uncertain.

## 6. Security rules (summary)

1. No direct SQL / journal write from plugins.  
2. Network allowlist only.  
3. FS mostly host staging.  
4. Credentials via broker scopes.  
5. Unsigned = Development Mode.  
6. Signature ≠ domain trust.  
7. Runtime capability revoke must abort.  
8. Plugin crash must not corrupt journal/object store.  
9. Tax-rule plugins need extra gate.  
10. Prompt/document content cannot escalate permissions.

## 7. Compatibility

| sdkVersion | Host min | Status |
|---|---|---|
| 1 | Belegwerk ≥ 0.1 (planned) | Draft landed in scaffold |

Breaking changes are recorded in this file.

## 8. Stubs in repo (M01+)

- `packages/plugin-sdk/` — types + schema stubs  
- `plugins/examples/hello-document-source/` — synthetic example  
- `plugins/connectors/moneymoney/` — empty bank-import stub  
- `plugins/connectors/grandtotal/` — empty invoice-import stub  

No live MoneyMoney / GrandTotal calls until M04 / M09 with verified APIs.
