# Fixture-Policy

Stand: 12. November 2026  
Anforderung: REQ-P-007

1. Nur synthetische oder ausdrücklich anonymisierte Entwicklungsdaten.
2. Keine echten Zugangsdaten oder produktiven Belege in Repository, Fixtures, Logs oder Prompts.
3. Ablage: `tests/fixtures/synthetic/`.
4. Wenn reale Schnittstellen getestet werden (M04+), nur mit freigegebenen Testdaten und dokumentierter Herkunft — niemals ungefragt aus Produktivsystemen.

Siehe `tests/fixtures/synthetic/README.md`.
