# Synthetic fixtures only

**Policy (REQ-P-007):** Development and tests use **synthetic or explicitly anonymized** data only.

## Allowed

- Invented counterparties, IBAN-like placeholders, fake invoice numbers
- Generated PDFs/PNGs created for tests (no real receipts)
- Scrubbed dumps that cannot identify real persons or accounts

## Forbidden

- Productive Belege / invoices / bank exports from live systems
- Real credentials, API keys, Keychain dumps
- Unredacted personal data in prompts, logs, or git history

Place future fixtures in this directory (e.g. `bank/`, `documents/`, `invoices/`). Keep a short note per fixture set describing how it was generated.
