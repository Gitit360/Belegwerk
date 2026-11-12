# Belegwerk – Plugin-/SDK-Vertragsentwurf v1.1

Stand: 12. November 2026 (Europe/Berlin)  
SDK-Protokollversion: **`sdkVersion: "1"`**  
Lizenzhinweis: Anwendungsprodukt proprietär/privat (D-09); dieser Vertrags-/SDK-Text ist **Apache-2.0-orientiert reserviert** — finale Lizenzentscheidung deferred. Vertrag bleibt versioniert dokumentiert.

---

## 1. Zweck

Plugins erweitern Belegwerk (Bank, Rechnung, Dokument, Export, Portal, Mail, OCR, KI, Reporting, Übergabe), **ohne** den Buchhaltungskern zu ändern. Steuerregelpakete sind ein besonders geschützter Plugin-Typ mit zusätzlicher Fachfreigabe.

---

## 2. Begriffe

| Begriff | Bedeutung |
|---|---|
| Host | Belegwerk Plugin-Host-Prozess |
| Plugin | Isolierte Erweiterung mit Manifest |
| Capability | Benannte, prüfbare Fähigkeit |
| Permission | Konkret gewährtes Recht (Domain, FS-Pfad, Credential-Scope) |
| Contract Message | Versionierte JSON-Nachricht Host ↔ Plugin |

---

## 3. Manifest (Schema-Skizze)

```json
{
  "id": "belegwerk.connector.example",
  "version": "0.1.0",
  "sdkVersion": "1",
  "kind": "document-source",
  "displayName": "Example Connector",
  "description": "Synthetisches Beispiel",
  "capabilities": ["documents.list", "documents.download"],
  "permissions": {
    "networkDomains": ["billing.example.invalid"],
    "filesystem": [],
    "credentialScopes": ["example.billing.read"],
    "maxRuntimeMs": 120000,
    "maxMemoryMb": 512
  },
  "entrypoint": "plugin-host",
  "schemas": {
    "configuration": "config.schema.json",
    "messages": "messages.schema.json"
  },
  "signature": null,
  "development": true
}
```

### 3.1 `kind`-Werte (v1)

`bank-import` · `invoice-import` · `document-source` · `export` · `portal` · `mail` · `ocr` · `ai-provider` · `reporting` · `delivery` · `tax-rules` (geschützt)

### 3.2 SemVer

- Manifest `version`: Plugin-Release.
- `sdkVersion`: Host-Kompatibilität. Host lehnt inkompatible Major ab.
- Breaking Message-Changes → neue `sdkVersion`.

---

## 4. Capability-Katalog (Auszug sdkVersion 1)

| Capability | Erlaubt | Explizit verboten |
|---|---|---|
| `documents.list` | Metadaten listen | Löschen fremder Portale |
| `documents.download` | Download in Staging | Original im Kern überschreiben |
| `bank.transactions.read` | Lesen via Adaptervertrag | `bank.payment.create` (existiert nicht) |
| `mail.search` | Suche nach Regel | Mail löschen/verschieben o. Freigabe |
| `mail.draft.create` | Entwurf | `mail.send` ohne Approval |
| `ai.complete` | Vorschlag | Rechte erweitern / Shell |
| `export.build` | Paket vorbereiten | Stiller Upload |
| `delivery.submit` | Nur nach ActionApproval | Blind retry als Erfolg |
| `tax.rules.evaluate` | Nur freigegebene Regelpakete | Ungeprüfte Web-Rules laden |

Capability Negotiation: Plugin deklariert → Host filtert auf gewährte Permissions → Runtime enforced.

---

## 5. Message-Protokoll (Skizze)

Transport: stdin/stdout oder lokaler Socket; **JSON Lines**; jede Message:

```json
{
  "v": 1,
  "id": "msg_01H...",
  "type": "request|response|event|error",
  "method": "documents.list",
  "correlationId": "job_...",
  "payload": {},
  "error": null
}
```

### 5.1 Pflichtmethoden Host → Plugin

| Method | Zweck |
|---|---|
| `plugin.handshake` | sdkVersion, capabilities, limits |
| `plugin.configure` | Validierte Config laut Schema |
| `plugin.shutdown` | Graceful stop |
| `plugin.cancel` | Job cancellation |

### 5.2 Beispiel Document Source

| Method | Richtung | Ergebnis |
|---|---|---|
| `documents.list` | Host→Plugin | Liste Referenzen + Cursor |
| `documents.fetch` | Host→Plugin | Bytes in Staging-Pfad (Host-kontrolliert) |
| `documents.progress` | Plugin→Host | Event |

Host kopiert Staging → content-addressed store; Plugin erhält keinen direkten DB-Handle.

---

## 6. Adaptervertrag Bank (Minimum)

Plugin/`bank-import` liefert Records:

```text
providerId
sourceAccountId
sourceTransactionRef
bookingDate
valueDate
amountMinor
currency
counterparty
purpose
status
importedAt
rawRef
```

Mapping auf `SourceTransaction` erfolgt im Application-Layer. Unsicheres Mergen zu `EconomicTransaction` erfordert User-Review.

---

## 7. Sicherheitsregeln

1. Kein Direct SQL / Journal-Write aus Plugins.
2. Network nur Allowlist-Domains aus Manifest + User-Confirm bei Erweiterung.
3. Filesystem nur deklarierte Pfade (meist leer; Host liefert Staging).
4. Credentials nur über Broker-Scopes.
5. Unsignierte Plugins: nur **Development Mode**, UI-Warnung.
6. Signatur = Herkunft, nicht fachliche Vertrauenswürdigkeit.
7. Rechtentzug zur Laufzeit: Host bricht Capability sofort ab (Abnahmetest M02).
8. Crashisolation: Plugin-Abort darf Archive/Journal nicht beschädigen.
9. Steuerregel-Plugins: zusätzliche Fachfreigabe + Testmatrix vor Aktivierung.
10. Prompt-/Dokumentinhalt aus Plugins darf keine Gate-Bypass-Anweisungen auslösen.

---

## 8. SDK-Paket (geplant)

```text
packages/plugin-sdk/
  README.md
  schemas/manifest.schema.json
  schemas/messages.schema.json
  src/ (TypeScript und/oder Rust client helpers)
plugins/examples/hello-document-source/
```

Abnahme (M02): Beispielplugin bauen, installieren, deaktivieren, updaten — **ohne Kerncode-Änderung**.

---

## 9. Kompatibilitätsmatrix (lebendiges Dokument)

| sdkVersion | Host min | Status |
|---|---|---|
| 1 | Belegwerk ≥ 0.1 (geplant) | Entwurf v1.1 |

Breaking changes werden in `PLUGIN_SPEC.md` im Repo geführt.

---

## 10. Was dieser Entwurf bewusst nicht finalisiert

- Exakte IPC-Transportwahl (stdio vs. UDS) — M02 Spike.
- Signatur-PKI-Details — vor Prod-Plugin-Distribution (M14).
- Finale OSI-Lizenzzeile — D-09 deferred.

---

*Ende Plugin-Contract v1.1.*
