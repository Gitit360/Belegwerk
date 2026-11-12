# Belegwerk – Product Description und verbindlicher Cursor-Arbeitsplan

Version: **1.1** – 12. November 2026 (Europe/Berlin)  
Status: Verbindlicher Spezifikationsentwurf mit geschlossenen Engineering-Defaults; keine Zusicherung steuerlicher Konformität.  
Sprache der Anwendung und Dokumentation: **Deutsch**; Codebezeichner: **Englisch**.  
Produktname: **Belegwerk** (früherer Arbeitstitel: LocalLedger).  
Ziel-Repository (künftig): `Gitit360/Belegwerk`.

---

## 0. Auftrag an Cursor

Entwickle eine lokale Mac-Anwendung für die Buchhaltung eines deutschen freiberuflichen Einzelunternehmens anhand dieses Dokuments. Baue nicht die gesamte Anwendung in einem Durchgang. Arbeite in nachvollziehbaren, einzeln prüfbaren Modulschritten. Das Ziel ist das vollständige hier beschriebene Produkt, nicht nur ein MVP. Einzelne Funktionen dürfen erst nach fachlicher und technischer Abnahme produktiv maßgeblich werden.

Verbindlich:
1. Lies vor jeder Implementierung die Produktanforderungen, Architekturentscheidungen und Abnahmekriterien des betreffenden Moduls.
2. Beschreibe vor Änderungen Umfang, betroffene Dateien, Datenmodelländerungen, Risiken und Tests.
3. Implementiere jeweils ein klar begrenztes Arbeitspaket.
4. Führe die verfügbaren Tests aus und protokolliere tatsächliche Ergebnisse. Behaupte niemals erfolgreiche Tests, die nicht ausgeführt wurden.
5. Liefere einen nachvollziehbaren Änderungsbericht mit offenen Punkten und nächstem vorgeschlagenem Schritt.
6. Beende jeden Meilenstein mit einer Abnahmevorlage. Beginne den nächsten Meilenstein erst nach ausdrücklicher Projektfreigabe.
7. Verwende von Anfang an ein offenes, dokumentiertes, versioniertes Plugin-Konzept. Neue Integrationen dürfen keine Änderungen am Buchhaltungskern erfordern.
8. Implementiere keine vermuteten steuerlichen Regeln. Unklare Fälle bleiben blockiert und dokumentiert.
9. Verwende ausschließlich synthetische oder ausdrücklich anonymisierte Entwicklungsdaten. Keine echten Zugangsdaten oder produktiven Belege in Prompts, Repository, Fixtures oder Logs.
10. Dieses Dokument beschreibt Sollanforderungen. Existenz, Lizenz, Fähigkeiten und Versionen externer Schnittstellen sind vor Implementierung zu verifizieren.

### 0.1 Erste Cursor-Aktion (Status v1.1)

Produktcode bleibt untersagt, bis der Nutzer M00 freigibt. Das Spezifikationspaket v1.1 (dieses Dokument plus Traceability, Decision Register, Architekturvorschlag, Plugin-Vertrag, M00-Arbeitsplan) erfüllt die Vorbereitung aus §0.1 der Quelle. **Nächster Engineering-Schritt:** Umsetzung laut `belegwerk-m00-workplan-v1.1.md` erst nach ausdrücklicher Nutzerfreigabe. Kein Anwendungscode vor dieser Freigabe.

---

## 1. Produktvision

**Belegwerk** ist eine weitgehend lokal arbeitende Mac-Plattform, die Belegoriginale, Bankdaten, Rechnungsdaten aus GrandTotal, nachvollziehbare Zuordnungen, geprüfte Buchungsregeln, Belegbeschaffung, Erinnerungen sowie steuerliche und betriebswirtschaftliche Auswertungen verbindet.

Primärer Nutzen: Schnell und zuverlässig erkennen, welcher Nachweis zu welcher Zahlung gehört, welche Belege fehlen, wo sie beschafft werden können und welche Fälle fachlich ungeklärt sind.

Kernworkflow:  
Kontobewegung importieren → Nachweispflicht/Falltyp prüfen → Beleg zuordnen oder beschaffen → Daten und steuerliche Behandlung prüfen → Buchungsentwurf → Freigabe → Festschreibung → Auswertung/Übergabe.

KI unterstützt Suche, Extraktion, Klassifikation und Erklärung. Sie ist kein autonomer Steuerentscheider. Geldberechnungen, Validierungen, Rechte, Journal und verbindliche Aktionen sind deterministisch.

---

## 2. Bestätigtes Nutzer- und Betriebsprofil

| Thema | Festlegung (v1.1) |
|---|---|
| Unternehmen | Freiberufliches Einzelunternehmen, Deutschland |
| Gewinnermittlung | Einnahmenüberschussrechnung (EÜR) |
| Umsatzsteuer | Istversteuerung; Vorsteuerabzug |
| Voranmeldungsrhythmus | **Default: monatlich**; Wechsel auf quartalsweise über versioniertes `ReportingSchedule` mit Gültigkeitsdaten (siehe Decision Register D-04) |
| Dauerfristverlängerung | **Standard aus**; erst nach Nutzerbestätigung in Einstellungen aktiv (D-05) |
| Ausland | US-SaaS, Reverse Charge (unterstützte Fälle), Fremdwährungen **in Scope**; **EU-B2B-Verkäufe explizit außerhalb der v1-Fallmatrix** (D-06) |
| Rechnungsprogramm | GrandTotal bleibt führend; **Zieladapter: GrandTotal 10, dateibasierter Export/Import**; Versionsprobe bei Adapter-Setup; keine proprietären DB-Schreibzugriffe (D-02) |
| Bankdaten primär | MoneyMoney, ausschließlich lesender Adapter |
| Ergänzende Bankquelle | **Bank X Professional: Phase-2-Adapter nach MoneyMoney** (M15); Kern nutzt nur Adaptervertrag (D-03) |
| Auszüge | PDF ergänzend; strukturierter Import bevorzugt |
| Banken/Zahlungswege | GLS, Volksbank, Deutsche-Bank/Miles-&-More-Kreditkarte als geplante Quellen; konkrete Konten-IDs und Emittentenzuordnung **beim Setup erfassen**, nicht hartcodieren |
| Bankwechsel | Neue Konten und Adapter ohne Änderung des Buchungskerns |
| E-Mail | Apple Mail primär (M10-Prototyp prüft reale Schnittstelle); Provideradapter (IMAP) als Fallback |
| Gerät | Ein Mac; Planungsannahme M5 Pro / 48 GB RAM; reale Umgebung beim Setup erfassen |
| Hintergrundbetrieb | Bei eingeschaltetem und technisch verfügbarem Mac; kein Always-on-Server |
| KI | Ollama lokal (Standard); OpenRouter optional; **Codex nur Dev-optional, keine Produktionsabhängigkeit** (D-12) |
| Steuerziel | USt-Voranmeldungen, USt-Jahreserklärung, EÜR-Vorbereitung, ELSTER-Übergabe; direkte Übermittlung eigenes Modul (M15+) |
| Reporting | Steuerliche Auswertungen und getrennte Managementberichte |
| Kanzlei | **Steuerboard als Post-M15-Plugin**; nicht im kritischen Pfad M00–M14 (D-13) |
| App-Sprache / Locale | **Deutsche UI**, Locale `de-DE`; Währung **EUR** als Default mit Mehrwährungsfeldern (D-14) |
| RPO / RTO | **RPO ≤ 24 h** lokales Backup; **RTO ≤ 4 h** dokumentierte Wiederherstellung auf vergleichbarer Mac-Klasse — Messziele, kein SLA-Marketing (D-08) |
| Lizenz | Anwendung zunächst **proprietär/privat**; Plugin-SDK-Vertragsdokumentation **Apache-2.0-orientiert reserviert**; finale Lizenzentscheidung bewusst zurückgestellt (D-09) |

Früheres Volumen von etwa 25–30 Buchungen monatlich ist eine Planungsannahme, keine technische Obergrenze.

---

## 3. Produktgrenzen

Im Zielumfang:
- Archiv, Import, Suche, OCR, Belegabgleich und Vollständigkeitsprüfung.
- Splits, Teilzahlungen, Sammelzahlungen, Erstattungen, interne Umbuchungen.
- Erinnerungen und kontrollierte Belegbeschaffung.
- Fachlich geprüfter SKR03-orientierter Buchhaltungskern und EÜR-Abbildung.
- Zeitabhängige Steuerregeln und verifizierte Steuerformulare.
- Offene API, CLI, MCP und Plugin-SDK.
- Vollständiger Datenexport und Wiederherstellung.

Nicht automatisch im Umfang:
- Lohnbuchhaltung, Warenwirtschaft, Konzern-/Mehrmandantenbuchhaltung.
- Vollständige private Einkommensteuererklärung einschließlich Familien-/Sonderfällen.
- Jede denkbare deutsche oder internationale Steuerkonstellation.
- **EU-B2B-Verkaufsfälle in v1** (erst nach fachlicher Erweiterung / M15+).
- Automatisierte Bankzahlungen, Vertragsänderungen oder Käufe.
- Produktive Übermittlung ohne Freigabe.

Nicht unterstützte Fälle werden sichtbar zur Klärung gestellt. Niemals stillschweigend auf einen ähnlichen unterstützten Fall abbilden.

---

## 4. Verbindliche Sicherheits- und Freigabepolitik

| Aktion | Standard |
|---|---|
| Lokale Dokumente lesen | Nur in eingerichteten Datenquellen |
| Bankdaten lesen | Über freigegebene Read-only-Funktionen |
| Dokumente extrahieren/klassifizieren | Automatisch als prüfbarer Vorschlag |
| Bekannte Portalrechnungen herunterladen | Nach Einrichtung und Erlaubnis des konkreten Connectors |
| E-Mails durchsuchen | Nach konkretem Auftrag oder separat eingerichteter, begrenzter Regel |
| E-Mail-Entwurf erstellen | Erlaubt |
| E-Mail senden | Vorherige ausdrückliche Freigabe für Ziel und Inhalt |
| Bankbewegungen auslösen | Nicht implementieren; auch nicht per Plugin/API/MCP |
| Daten in fremden Portalen löschen | Standardmäßig nicht implementieren; niemals ungefragt |
| Verträge/Abos ändern oder Käufe auslösen | Nicht Bestandteil der Beschaffung |
| Buchung festschreiben | Zunächst ausdrückliche Freigabe; spätere Automationsregeln nur gesondert genehmigt |
| Steuerdaten übermitteln | Ausdrückliche Freigabe des konkreten Pakets |
| Extern hochladen | Ziel- und inhaltsbezogene Freigabe oder später ausdrücklich eingerichtete eng begrenzte Regel |

Die Erlaubnisprüfung erfolgt im Anwendungskern. UI, Plugins, CLI und MCP können sie nicht umgehen. Freigaben sind an Aktion, Ziel, Payload-Hash, Benutzer, Zeit und Gültigkeit gebunden. Nach Inhaltsänderung ist eine neue Freigabe nötig.

Geheimnisse über Credential Broker / macOS-Schlüsselbund. Keine Secrets in Modellkontext, Logs oder Dokumentendatenbank. Authentifizierte Browserprofile sind sensible Geheimnisse. Modelle erhalten keine beliebigen Shell-, AppleScript- oder Browserbefehle.

---

## 5. Fachliche Grundprinzipien

- Originaldokument, Rechnung, Zahlung, Zuordnung und Journal sind getrennte Objekte.
- Ein Match bestätigt keine steuerliche Zulässigkeit.
- Ein KI-Vorschlag ist keine Freigabe.
- Lieferantenname oder Tag löst allein keine Steuerbehandlung aus.
- Istversteuerung ist keine universelle Regel für alle steuerlichen Zeitpunkte.
- EÜR-Zeitpunkt, Umsatzsteuerzeitpunkt und Vorsteuerzeitpunkt separat bestimmen.
- Historische Regelversionen erhalten; Updates schreiben alte Buchungen nicht um.
- Festgeschriebene Daten nur über nachvollziehbare Korrekturen ändern.
- Originale nie durch OCR-Dateien, Vorschaubilder oder PDF-Konvertierungen ersetzen.
- Ein Hash oder ein normales Dateiverzeichnis allein garantiert keine Unveränderbarkeit.
- Keine pauschale Behauptung „GoBD-zertifiziert“ oder „rechtssicher“.

---

## 6. Informationsarchitektur und UX

Navigation:
1. Übersicht  
2. Eingang  
3. Konten  
4. Abgleich  
5. Fehlende Belege  
6. Archiv/Suche  
7. Buchhaltung  
8. Anlagen  
9. Berichte  
10. Übergaben  
11. Automationen/Connectoren  
12. Einstellungen  

### 6.1 Primäre Arbeitsansicht

Dreispaltig: Transaktion/Kandidaten links; Originalbeleg mit Fundstellen mittig; Daten, Prüfungen und Entwurf rechts. Details einklappbar. Chat nur ergänzend.

### 6.2 Interaktionsprinzipien

Playful bedeutet flüssig, leicht und direkt, nicht steuerliche Gamification.
- Drag-and-drop für Import, Zuordnung und Kontextverknüpfung.
- Multi-Select und kontextuelle Batch-Werkzeugleiste.
- Kontextmenüs als zusätzlicher Zugang; wichtige Aktionen bleiben sichtbar auffindbar.
- Quick Look, Tastaturbedienung, Command-Palette.
- Vorschau vor fachlichen Änderungen und externen Aktionen.
- Gespeicherte Filter, Spaltenkonfiguration und kompakte/komfortable Tabellen.
- Undo für reversible Entwurfs-/Metadatenänderungen; Korrekturworkflow statt Undo bei Festschreibung.
- Keine ausschließliche Farbcodierung; Fokus, Screenreader, Reduced Motion und Kontraste prüfen.

Batch-Vorschau zeigt Anzahl, Summenänderung, Ausnahmen, Rechte und Reversibilität. Konfliktfälle werden nicht stillschweigend übersprungen.

### 6.3 Designsystem

Vor Fachseiten implementieren: Primitive → semantische Tokens → Komponenten-Tokens → fachliche Zustände.

Beispiele:
```text
color.surface.default
color.surface.selected
color.text.primary
color.status.warning
space.1 / space.2 / space.4
size.row.compact
focus.ring.width
interaction.drop.allowed
interaction.drop.warning
interaction.drop.blocked
evidence.original
evidence.placeholder
evidence.unverified
action.requiresApproval
action.prohibited
motion.duration.short
motion.reduced
```

Komponenten: Datentabelle, Geldanzeige, Dokumentviewer, Match-Karte, Prüfpanel, Split-Editor, Statusanzeige, Verlauf, Änderungsvergleich, Freigabedialog, Batch-Vorschau, Connector-/Jobstatus, Leer-/Lade-/Fehlerzustände.

Tokenquelle versionieren; UI-Werte nicht frei pro Seite duplizieren. Hell-/Dunkelmodus und visuelle Regressionstests vorsehen.

---

## 7. Dokumentenarchiv

Jeder Import erzeugt Herkunft, Zeit, Originalreferenz, Hash, Dateityp und Import-Batch. Abgeleitete Texte, OCR, Vorschau und Extraktionen separat speichern.

**Entscheidung D-11:** Originale liegen in einem **content-addressierten Dateispeicher** (Hash als Pfadschlüssel); Metadaten und Relationen in **SQLite mit Migrationen**. Abgeleitete Artefakte referenzieren den Original-Hash, überschreiben ihn nie.

Anforderungen:
- Keine stille Originalersetzung.
- Byte-identische Dubletten erkennen; Herkunft zusätzlicher Eingänge erhalten.
- Ähnliche Dokumente nur vorschlagen, nicht automatisch als identisch behandeln.
- Dokumentversionen/Korrekturrechnungen explizit verknüpfen.
- Elektronische Originalformate und strukturierte Rechnungsbestandteile erhalten.
- Schad-/Fehlformate begrenzen und isolieren.
- Sichere Pfade, Größenlimits und Schutz gegen ZIP-Bomben/Path Traversal.
- Exportierbare Beleg-IDs statt Identität über Dateinamen.

Aufbewahrung je Dokumentklasse, Rechtsgrundlage und Gültigkeitszeitraum konfigurieren. Keine pauschale Frist für alle Objekte hart codieren. Sperren für laufende Verfahren berücksichtigen. Löschung sensibler Daten nur nach Berechtigungs- und Aufbewahrungsprüfung.

---

## 8. Bankimport und Quellenabgleich

### 8.1 MoneyMoney

Offizielle dokumentierte Exportfunktionen nutzen. Vor Implementierung aktuelle AppleScript-Dictionary und Ausgabeformate prüfen. Kein generischer AppleScript-Executor. Import muss wiederholbar und idempotent sein. Primärer Bankadapter in M04.

### 8.2 Adaptervertrag

Mindestens: Provider-ID, Quellkonto-ID, Quelltransaktionsreferenz, Buchungsdatum, Wertstellung, Betrag, Währung, Gegenpartei, Verwendungszweck, Status, Importzeitpunkt, Rohdatenreferenz.

Trenne `SourceTransaction` von `EconomicTransaction`: Derselbe reale Umsatz kann aus MoneyMoney, Bank X und einem PDF stammen. Quell-IDs nicht global als identisch behandeln. Unsichere Zusammenführungen benötigen Prüfung.

### 8.3 PDF-Kontoauszüge

Getestete Parserprofile statt generischer Erfolgsgarantie. Konto, Zeitraum, Seiten, Transaktionen und Salden prüfen, soweit vorhanden. Fundstellen speichern. Fehlende Seiten/unsichere Beträge blockieren Bestätigung. Originalauszug archivieren.

### 8.4 Sonderfälle

Vormerkung versus gebuchter Umsatz; nachträglich veränderte Quellinformationen; interne Umbuchung; Kreditkartenabrechnung; Einzelkartenumsätze; Rücklastschrift; Erstattung; Gebühren; Fremdwährung.

Kreditkartenausgleich nicht als zweite Ausgabe erfassen. Bankwechsel erhält historische Konten und Buchungen.

### 8.5 Bank X Professional (Phase 2)

**Entscheidung D-03:** Nach stabilem MoneyMoney-Adapter. Konkrete Schnittstelle (Exportformat/API) wird im Adaptermodul spezifiziert und verifiziert. Bis dahin existiert nur der Slot im Adaptervertrag; keine Kernänderung für Bank X.

---

## 9. GrandTotal-Integration

GrandTotal bleibt führend für Ausgangsrechnungen, Rechnungsnummern und Rechnungskorrekturen.

**Entscheidung D-02:** Erster Adapter zielt auf **GrandTotal 10**, **dateibasierter** Export/Import. Bei Adapter-Setup: Versionsprobe und Profilmatrix (welche Exporte verfügbar). **Keine direkten Schreibzugriffe** auf die proprietäre GrandTotal-Datenbank. Rückschreiben des Zahlungsstatus nur als spätere, gesondert geprüfte Funktion (nicht v1-kritisch).

Importziele: Originaldokumente, Rechnungsdaten, Positionen, Kundenreferenzen, Stornos/Gutschriften, bekannte Zahlungsinformationen. Importierte Zahlungsinformationen sind keine zusätzlichen Bankeingänge. Abgleich verhindert doppelte Einnahmen.

---

## 10. Matching, Splits und Vollständigkeit

Viele-zu-viele-Modell für Zahlungen und Belege.

Matching-Reihenfolge:
1. Referenz/Rechnungsnummer.
2. Betrag/Währung/Gegenpartei.
3. Datumsfenster und bestätigte Händleraliase.
4. Wiederkehrende Muster.
5. Teil-/Sammelzahlungen.
6. Volltext/semantische Ähnlichkeit.
7. KI für verbleibende Unklarheiten.

Jeder Vorschlag erklärt verwendete Merkmale und Unsicherheit. Automatisches Bestätigen erst nach expliziter Regel und getesteten Qualitätsgrenzen. Modell-Selbsteinschätzungen nicht als kalibrierte Wahrscheinlichkeiten darstellen.

### 10.1 Drei Splitarten

- `PaymentAllocation`: eine Zahlung auf mehrere Belege oder mehrere Zahlungen auf einen Beleg.
- `BookingSplit`: Positionen/Beträge auf verschiedene Konten oder Steuerfälle.
- `UsageAllocation`: betrieblicher/privater Nutzungsanteil.

Summen, Währungen, Rundungen und offene Reste abstimmen. Betrag darf nicht doppelt verbraucht werden.

### 10.2 Fehlende Nachweise

Nicht jede Kontobewegung ohne PDF ist automatisch fehlerhaft. Falltyp bestimmt benötigte Nachweise. „Nicht erforderlich“ nur begründet und nachvollziehbar.

Getrennte Statusdimensionen:
- Nachweis: fehlt / erwartet / vorhanden / anderer Nachweis / begründet nicht erforderlich.
- Zahlung: offen / teilweise / bezahlt / erstattet.
- Prüfung: ungeprüft / technisch geprüft / fachlich offen / freigegeben.
- Buchung: Entwurf / freigegeben / festgeschrieben / korrigiert.
- Übergabe: nicht vorbereitet / vorbereitet / übertragen / Empfang bestätigt / fehlgeschlagen / unklar.

---

## 11. Proxy-Belege, Tags und Meta-Nodes

### 11.1 Proxy-Beleg — verbindliche Arbeitsdefinition (D-16)

`DocumentPlaceholder` ist ein Platzhalter für erwartete Nachweise, **kein Original**, kein automatischer Eigenbeleg und **keine Grundlage alleinigen Vorsteuerabzugs**. Enthält erwarteten Lieferanten, Betrag/Währung, Zeitraum, Zahlung, Quelle und Beschaffungsstatus. Original später anhängen und erneut prüfen; Historie erhalten.

Eigenbelege sind ein eigener Falltyp und bleiben in v1 **blockiert**, bis eine geprüfte Regelversion vorliegt. Weicht die Nutzerbedeutung von dieser Definition ab, wird zuerst die Fachdefinition geändert — nicht die Implementierung „zurechtgebogen“.

### 11.2 Tags

Freie Organisationsmerkmale. Steuerlich relevante Eigenschaften als validierte Felder führen, nicht aus Tags ableiten.

### 11.3 Meta-Nodes — Entscheidung D-17

`ContextNode` / `ContextRelation` für Lieferant, Kunde, Projekt, Vertrag, Abo, Anlage, Kostenstelle und Beschaffungsfall. **Speicherung relational in SQLite.** Visuelle Graph-/Node-Ansicht **nicht in M00–M14**; Listen und Beziehungen müssen ohne Graph bedienbar sein. Graph-UI erst nach ausdrücklicher Nutzerbestätigung als optionales M15+-Feature.

---

## 12. Reminder und Belegbeschaffung

`MissingDocumentCase` enthält Transaktionen, erwartete Nachweise, Lieferant, Vertrag/Abo, Suche, Kontaktversuche, nächste Wiedervorlage und Eskalation.

Beispielhafte Fristen (frei konfigurierbar, **keine gesetzlichen Fristen**): Tag 3 lokale Suche; Tag 7 Mail-Suchauftrag; Tag 14 Portalabruf; Tag 21 Anforderungsentwurf. Versand immer gesondert freigeben.

Bei spätem Beleg: Fall erneut prüfen und Zuordnungen ergänzen. Bereits festgeschriebene oder übergebene Vorgänge nicht still verändern.

Beschaffungspriorität:
1. Lokales Archiv/erlaubte Ordner.
2. Freigegebene E-Mail-Suche.
3. Offizielle Anbieter-API/Export.
4. Begrenzte Browserautomation.

### 12.1 Portalconnectoren

Backlog: Amazon, Adobe, OpenAI, Perplexity, Cursor, Deutsche Telekom, **Slot `mobile-de`** (kein fingierter Markenname; Nutzer konfiguriert den konkreten Mobilfunkanbieter später — D-07), weitere bei Bedarf.

Je Connector: Produkt/Abrechnungskonto, Domains, Authentifizierung, MFA, erlaubte Aktionen, Dokumenttypen, Zeitraum, Fortschritt, Wiederholung, letzter Erfolg, Fehlerbild. Browserprofile isolieren. Bei CAPTCHA, Layoutänderung oder fremder Domain anhalten. Keine Sicherheitsumgehung.

### 12.2 Apple Mail

Apple-Mail-Adapter im Prototyp (M10) auf tatsächliche Möglichkeiten prüfen; alternativ Provideradapter (IMAP). Nachrichten oder Anhänge mit stabiler Herkunftsreferenz importieren. Keine Mail löschen, verschieben oder als gelesen markieren, sofern nicht ausdrücklich separat erlaubt. Automatische Hintergrundsuche nur durch eingerichtete begrenzte Regel.

---

## 13. Rechnungs-OCR und Extraktion

Pipeline: Dateierkennung → strukturierte Rechnungsdaten → PDF-Text → lokale OCR → Rechnungsfelder/Positionen → Summenprüfung → Unsicherheitsprüfung.

Extrahieren: Parteien, Rechnungsnummer, Datum, Leistung/Zeitraum, Währung, Nettobeträge, Steuergruppen, Brutto, Positionen, Referenzen, Gutschrift-/Reverse-Charge-Hinweise.

Je Feld: Wert, Seite/Fundstelle, Verfahren, Parser-/Modellversion, Prüfstatus und manuelle Korrektur. Kein Original überschreiben. Keine fehlenden Angaben erfinden. Konflikte zwischen strukturierten Daten und Darstellung anzeigen.

Auswahl der OCR-/Vision-Modelle anhand anonymisierter Referenzbelege. Messen: feldbezogene Genauigkeit, kritische Betragsfehler, Laufzeit, RAM, manueller Korrekturaufwand. Keine Modellfestlegung nur anhand allgemeiner Benchmarks.

---

## 14. KI-Provider und Token-Effizienz

### 14.1 Providervertrag

Capabilities: Text, Vision, strukturierte Ausgabe, Embeddings, Streaming; nicht jeder Provider unterstützt alles. Konfigurierbare Modellprofile statt hart codierter Namen. Modell-/Prompt-/Schema-Versionen dokumentieren.

### 14.2 Ollama

Lokaler Standardadapter. Endpoint standardmäßig lokal; externe Endpoints gesondert kennzeichnen. Modellinstallation, Lizenz, Speicherbedarf und reale Leistung prüfen. Ohne verfügbares Modell bleiben deterministische Kernfunktionen nutzbar.

### 14.3 OpenRouter

Optional, standardmäßig aus. Explizite Modell-/Provider-Allowlist, Budget, Datenminimierung und geeignete Retention-Einstellungen. Kein stiller Providerfallback. ZDR ist kein pauschaler Datenschutzkonformitätsnachweis; Anbieterbedingungen und Transfers gesondert prüfen.

### 14.4 Codex — Entscheidung D-12

Entwicklungsassistent und **optionaler Dev-only**-Adapter. **Keine Produktions-Runtime-Abhängigkeit.** Abonnementzugang nicht als frei nutzbare Produktions-API behandeln. Kein Auslesen interner Tokens und keine inoffiziellen API-Proxys. Buchhaltung funktioniert ohne Codex.

### 14.5 Effizienz

- Parser/Regeln vor LLM.
- Kandidaten über Betrag, Datum und Lieferant eingrenzen.
- Nur relevante Ausschnitte senden.
- Cache nach Dokumenthash, Verfahren, Schema- und Modellversion.
- JSON-Schema-Ausgaben validieren; formale Gültigkeit ist keine inhaltliche Korrektheit.
- Token-/Kostenlimits, Timeout, Retry-Limit, keine Endlosschleifen.
- Embeddings separat versionieren und vollständig neu aufbaubar halten.
- Keine Belege mehrfach analysieren, wenn unverändert.
- Cloudkosten und Datenübertragungen sichtbar protokollieren, ohne sensible Inhalte in technischen Logs.

---

## 15. Buchhaltung und Steuerregeln

### 15.1 Buchungsmodell

Geprüfter, ausgeglichener Journalansatz mit Soll-/Haben-Zeilen; laiengerechte Oberfläche mit Kategorien. EÜR als eigene getestete Abbildung. SKR03-Versionen, Kontenlizenzen und Steuerschlüssel vor produktivem Einsatz verifizieren (Engineering: Regelpakete versionieren und Fälle ohne Freigabe blockieren).

Geld mit expliziter Dezimal-/Minor-Unit-Strategie; keine binären Fließkommazahlen für Geldberechnungen. Wechselkurse mit notwendiger höherer Präzision, Quelle und Datum. Ursprungswährung erhalten. Rundungsregeln je fachlichem Verfahren testen.

### 15.2 Regelregister

Jede Regel: ID, Gültigkeit, Rechts-/Fachquelle, Fallvoraussetzungen, Ausnahmen, Beispiele, Tests, Freigabe, Version. Keine unkontrolliert aus dem Web geladenen produktiven Regeln.

Fallmatrix v1 mindestens:
- Inländische Einnahmen/Ausgaben mit unterstützten Steuersätzen.
- Teilzahlungen, Storno, Gutschrift, Erstattungen.
- Private Anteile und Einlagen/Entnahmen.
- Unterstützte Reverse-Charge-Fälle, insbesondere US-/Auslands-SaaS (nach geprüfter Regelversion).
- Fremdwährungen und Gebühren.
- Anlagen/GWG/Abschreibungen nach geprüften Regeln (keine hartcodierten Universalgrenzen ohne Jahresbezug).
- Periodenwechsel und besondere zeitliche Zuordnungen.
- **EU-B2B-Leistungen: außerhalb v1**, erst nach fachlicher Erweiterung.

Keine generische Regel „Ausland = Reverse Charge“. Kein Vorsteuerabzug nur wegen vorhandener Zahlung. Keine abschließende Steuerbehandlung allein durch OCR-Hinweise.

**Fachprüfung (D-18):** Primärer Fachprüfer der Regelpakete und Referenzbuchungen ist der **Nutzer**. Vor Produktivfreigabe des steuerlich maßgeblichen Kerns (M11/M14): mindestens eine vollständige Periode Parallelabstimmung. Optionale externe Steuerberater-Review ist empfohlen, aber nicht automatisiert im Produkt erzwungen.

### 15.3 Steuerprofile und Perioden

`TaxProfileVersion` und `ReportingSchedule` mit Gültigkeit. Istversteuerung, EÜR-Zuordnung und Vorsteuerzeitpunkt getrennt. Monat/Quartal nicht beliebig pro Formular umschalten — Wechsel nur über neue Schedule-Version mit `validFrom`/`validTo`. Abgegebene Meldungen und Korrekturketten erhalten. Dauerfristverlängerung standardmäßig aus (D-05).

**Steuerjahr-Default (D-19):** Beim Erstsetup wird das **aktuelle Kalenderjahr** als Startjahr des TaxProfiles vorgeschlagen; der Nutzer bestätigt oder setzt Historie/Wechseldaten. Keine stillschweigende Rückrechnung ohne bestätigte Perioden.

### 15.4 Anlagen

`Asset` mit Anschaffung, Beleg, Anschaffungsnebenkosten, Nutzungsanteil, Inbetriebnahme, Regelversion, Abschreibungsverlauf und Abgang. Steuerliche Sonderfälle nur nach geprüfter Unterstützung. Keine universellen Wertgrenzen oder Nutzungsdauern ohne Jahresbezug hart codieren. Ungeprüfte Anschaffungsfälle bleiben blockiert mit Dokumentationspfad.

---

## 16. ELSTER und steuerliche Auswertungen

### 16.1 Vorbereitung

`TaxReturnDraft` je Steuerart/Jahr/Periode. Feldmapping mit Formularversion, Beträgen, Buchungsherkunft, Korrekturen und Validierungen. USt-Voranmeldungen, USt-Jahresabstimmung und EÜR unterstützen. Kein generisches XML/CSV als garantiert importierbar bezeichnen. Formularversionen und Feldzuordnungen vor M12 gegen aktuelle ELSTER-/BMF-Quellen verifizieren — bis dahin: Entwurfsstruktur und Validierungs-Hooks, keine „rechtssicheren“ Feldwerte erfinden.

### 16.2 Direkte Übermittlung

Separates Plugin/Modul nach offizieller ERiC-Integration und Entwicklerregistrierung (**M15+**). Voraussetzungen, Plattformunterstützung und aktuelle Schemata prüfen. Zertifikate sicher verwalten. Preview und ausdrückliche Freigabe. Übermittlungsprotokoll und Antwort archivieren. Unklarer Netzwerkstatus führt zur Statusprüfung, nicht blind zur erneuten Abgabe.

Eine Einkommensteuer-Gesamterklärung ist nicht Teil dieser Erstfestlegung.

---

## 17. Betriebswirtschaftliche Berichte

Steuerberichte und Managementberichte deutlich trennen.

Management: Perioden-/Vorjahresvergleich, Lieferantenkosten, SaaS-/Abokosten, Preisanstiege, Projekte/Tags, Liquidität, wiederkehrende Verpflichtungen, offene Nachweise und Fremdwährungskosten. Kein nicht nachvollziehbarer KI-Bericht als primäre Zahlenquelle.

Jeder Bericht zeigt: Datenstand, Zeitraum, netto/brutto, Zahlungs-/Leistungsbasis, Kontenfilter, offene Zuordnungen, Entwurfsanteil, Währungsregeln. Drill-down bis zum Beleg. Exporte behalten Filter und Berechnungsbasis. Ohne geprüfte Standardabbildung keine bestimmte DATEV-BWA-Form behaupten.

---

## 18. Übergaben, Formate und Steuerboard

### 18.1 Kompatibilitätsmatrix

Pro Format Version/Profil, Import/Export, Umfang, Testdateien, Einschränkungen und Status dokumentieren.

Geplant:
- Dokumente: PDF, JPEG, PNG, HEIC, TIFF; XRechnung und ZUGFeRD/Factur-X in getesteten Profilen; EML; MSG optionaler Adapter.
- Bank: MoneyMoney-Export, CSV-Profile, CAMT.053, MT940; OFX/QFX nach Bedarf; PDF nur getestete Profile.
- Ausgabe: Originale, JSON, CSV, XLSX, PDF-Berichte, versionierter DATEV-Buchungsstapel und vollständiges Archivpaket.

Keine pauschale „alle Formate“-Zusage. DATEV-Import und Online-API sind getrennte Integrationen. Lizenz-/Partnerbedingungen prüfen.

### 18.2 Exportpaket

Manifest mit Schema-/Softwareversion, stabilen IDs, Originalen, strukturierten Daten, Buchungen, Zuordnungen, offenen Fällen, Prüfsummen und Exportzeit. Restore-Fähigkeit getrennt von bloßer Lesbarkeit testen. Prüfungsdatenexport nicht ohne verifizierte Spezifikation als vollständig amtlich kompatibel bezeichnen.

### 18.3 Steuerboard — Entscheidung D-13

Späteres Plugin (**nach M15**, nicht im kritischen Pfad M00–M14). Zuerst dokumentierte offizielle Wege prüfen; danach E-Mail-Upload, dann begrenzte Browserautomation, zuletzt manuelles Paket. Ordnerzuordnung nicht ohne Prüfung voraussetzen. „Übertragen“, „empfangen“ und „fachlich verarbeitet“ getrennt halten.

---

## 19. Zielarchitektur

Modularer Monolith mit isolierten Adaptern; keine unnötige Microservice-Landschaft.

**Entscheidung D-10 (Stack):** **Tauri 2 + React + TypeScript** für Desktop/UI; **Rust** für Domäne und Anwendungsdienste; **SQLite** lokal mit Migrationen; content-addressierter Originalspeicher; SQLite-FTS; optionale semantische Indizes; kleiner nativer macOS-Adapter nur wo nötig; persistente Jobqueue; isolierter Browser-/Parser-/Plugin-Host.

Begründung kurz: kleinere Angriffsfläche und schlankeres lokales Binary gegenüber Electron; Rust-Domäne passt zu Finanzinvarianten und Plugin-Isolation. SwiftUI nur über späteren ADR — nicht parallel zwei Oberflächen.

Schichten:
```text
UI / CLI / REST / MCP
        ↓
Application Services + Permission Gate
        ↓
Domain: Documents / Finance / Matching / Accounting / Reporting
        ↓
Ports: Storage / Bank / Mail / OCR / AI / Delivery
        ↓
Adapters und isolierte Plugins
```

UI, API und Plugin-Code enthalten keine eigene konkurrierende Steuerlogik. Datenbankzugriff ausschließlich über klar definierte interne Dienste.

Detaillierter Vorschlag: `docs/belegwerk-architecture-proposal-v1.1.md`.

---

## 20. Offenes Plugin-Konzept – verpflichtend ab Projektbeginn

Offen bedeutet dokumentierte, versionierte Erweiterbarkeit, austauschbare Implementierungen und ein SDK. Es bedeutet **nicht**, dass das Gesamtprodukt unter Open Source steht (D-09).

### 20.1 Plugin-Typen

Bankimport, Rechnungsimport, Dokumentimport, Export, Portal, Mail, OCR, KI, Reporting und Übergabe. Steuerregelpakete sind besonders geschützte fachliche Erweiterungen und benötigen gesonderte Prüfung/Freigabe.

### 20.2 Manifest (Belegwerk)

```json
{
  "id": "belegwerk.connector.example",
  "version": "0.1.0",
  "sdkVersion": "1",
  "kind": "document-source",
  "displayName": "Example Connector",
  "capabilities": ["documents.list", "documents.download"],
  "permissions": {
    "networkDomains": ["billing.example.invalid"],
    "filesystem": [],
    "credentialScopes": ["example.billing.read"]
  },
  "entrypoint": "plugin-host",
  "schemas": {"configuration": "config.schema.json"}
}
```

### 20.3 Laufzeit und Rechte

- Keine beliebigen Drittanbieter-Bibliotheken mit Vollzugriff im Kernprozess.
- Plugins über isolierte Prozesse oder begrenzte Laufzeit; tatsächliche macOS-Isolation im Prototyp nachweisen.
- Typisierte Nachrichten/JSON-Schemas; keine direkten Journal-/DB-Schreibrechte.
- Credential Broker gibt nur notwendige Funktionen/Zugriffe frei.
- Rechte bei Installation und jeder Erweiterung sichtbar bestätigen.
- Versionierte Protokolle, Kompatibilitätsmatrix und Capability Negotiation.
- Deaktivierung ohne Verlust historischer Daten.
- Zeit-/Speicher-/Netzwerklimits, Crashisolation, Cancellation.
- Installation/Updates mit Herkunftsprüfung, Hash/Signaturstrategie und Rollback.
- Signatur bestätigt Herkunft, nicht fachliche Vertrauenswürdigkeit.
- Lokale Entwicklung erlaubt; unsignierte Plugins deutlich als Entwicklungsmodus markieren.

### 20.4 SDK-Abnahme

Ein Beispielplugin muss ohne Änderung des Kerncodes gebaut, installiert, deaktiviert und aktualisiert werden können. Entzug eines Rechts muss wirksam getestet sein. Ein abstürzendes Plugin darf Archiv und Journal nicht beschädigen.

Vertragsdetail: `docs/belegwerk-plugin-contract-v1.1.md`.

---

## 21. API, CLI, MCP und Mac-Integration

REST: versioniert, OpenAPI, standardmäßig lokal gebunden, Authentifizierung auch lokal, Scopes, Limits, Idempotenzschlüssel und sichere Origin-/Hostprüfung. Kein ungeprüftes localhost-Vertrauen. Netzwerkfreigabe nur separat.

Ressourcen:
```text
/v1/documents
/v1/transactions
/v1/matches
/v1/missing-document-cases
/v1/booking-drafts
/v1/reports
/v1/exports
/v1/jobs
```

MCP nutzt dieselben Anwendungsdienste/Rechte. Beispielwerkzeuge: `search_documents`, `list_missing_documents`, `get_transaction`, `suggest_match`, `create_booking_draft`, `prepare_export`, `request_delivery_approval`. Keine Bankzahlung und kein privilegierter SQL-/Shell-Endpunkt.

CLI: maschinenlesbare JSON-Ausgabe, dokumentierte Exitcodes, keine Secrets in Argumenten. Mac: Shortcuts/App Intents, Share Extension, überwachte Ordner und URL-Schema nach Bedarf; sensible Daten nicht in URLs. AppleScript nur klar begrenzte Funktionen.

---

## 22. Datenmodell

Kernentitäten:
```text
BusinessProfile, TaxProfileVersion, ReportingSchedule, Account, ImportBatch,
SourceTransaction, EconomicTransaction, Document, DocumentVersion,
Extraction, ExtractionField, Invoice, InvoiceLine, Counterparty,
Contract, Subscription, DocumentPlaceholder, ContextNode, ContextRelation,
Tag, Match, PaymentAllocation, BookingSplit, UsageAllocation,
BookingDraft, JournalEntry, JournalLine, TaxRuleVersion, Asset,
MissingDocumentCase, Reminder, SearchAttempt, Connector, CredentialReference,
AutomationJob, ProviderPolicy, ActionApproval, TaxReturnDraft,
TaxReturnFieldMapping, SubmissionRecord, ExportBatch, ExportItem,
DeliveryReceipt, AuditEvent, RetentionPolicy.
```

Jede Entität mit stabiler ID und definierten Lebenszyklusregeln. Zeitzonen: Speicherung von Zeitpunkten in UTC; fachliche Datumsfelder (Belegdatum, Leistungszeitraum) als kalendarische Daten in `Europe/Berlin` interpretieren, sofern nicht anders am Objekt vermerkt. Lösch-/Änderungsregeln vor Migration definieren. Domänenstatus über erlaubte Übergänge statt beliebig editierbare Strings.

---

## 23. Sicherheit, Datenschutz und Betriebsfähigkeit

- Bedrohungsmodell: verlorener Mac, kompromittiertes Plugin, bösartiger Beleg, Browser-Prompt-Injection, lokaler Fremdprozess, Fehlbedienung, Backupverlust.
- Dokumente/Mails/Webseiten sind untrusted input; keine Anweisung darf Rechte erweitern.
- Parser isolieren, Skripte nicht ausführen, Größen-/Zeitlimits.
- **Verschlüsselung (D-20):** SQLite-Datenbank at-rest verschlüsselt (Zieltechnologie: SQLCipher oder gleichwertig, in M01 ADR final); Schlüsselmaterial im macOS-Schlüsselbund; Originale im content-addressierten Store mit Dateischutz und Backup-Integrität. Wiederherstellungsschlüssel-Strategie in `BACKUP_RESTORE.md` dokumentieren — Nutzer bestätigt Backupziele vor Produktivbetrieb.
- Keine Telemetrie standardmäßig; Crashberichte nur bereinigt und freigegeben.
- Abhängigkeiten sperren, Lizenzen prüfen, SBOM und Schwachstellenscans vor Releases.
- Signierung/Notarisierung und sichere Updatekette vor produktiver Verteilung.
- Auth-Sitzungen und Backups in Datenschutzkonzept aufnehmen.
- Kein Cloudtransfer ohne Sichtbarkeit und erlaubte Policy.
- Datenminimierung und Aufbewahrung pro Datenklasse; keine pauschale Löschlogik im Journal.

GoBD-orientiertes Verfahren: Originalschutz, Historie, Festschreibung, Zugriffskontrolle, Auswertbarkeit, Wiederherstellung und Verfahrensdokumentation zusammen betrachten. Manipulationserkennbare Historie plus gesondert geschützte Sicherungen und organisatorische Kontrollen planen. Fachliche Prüfung vor Konformitätsaussagen. Keine Behauptung „GoBD-zertifiziert“.

---

## 24. Jobs, Backup und Wiederherstellung

Jobs persistent, idempotent, begrenzt wiederholbar, abbrechbar und nach Absturz fortsetzbar. Zustände: geplant, läuft, wartet auf Auth, pausiert, erfolgreich, fehlgeschlagen, Ergebnis unklar.

Schlafmodus, gesperrter Mac, fehlendes Netzwerk, nicht gestartetes MoneyMoney und abgelaufene Logins sichtbar behandeln. Keine garantierten Abrufzeiten bei ausgeschaltetem Mac versprechen.

Backup umfasst DB, Originale, nötige Konfiguration, Plugin-/Regelversionen und Wiederherstellungsinformationen. Suchindizes dürfen rekonstruierbar sein. Sichere Strategie für Schlüssel und Secrets separat dokumentieren. Restore auf frischer Umgebung testen. Vor Schema-Migration Sicherung und Rückfallstrategie.

**Messziele (D-08):** RPO ≤ 24 h (lokales Backup mindestens täglich oder bei kritischen Batch-Abschlüssen); RTO ≤ 4 h dokumentierte Wiederherstellung auf derselben Mac-Leistungsklasse. Keine SLA-Marketingaussagen.

---

## 25. Teststrategie und harte Abnahmekriterien

Testebenen: Unit, Property-/Invariantentests, Adapterverträge, Integration, UI, visuelle Regression, Berechtigungen, Sicherheit, Restore und fachliche Referenztests.

Pflichtfälle:
- Wiederholter Import erzeugt keine Dubletten.
- Mehrquellenimport erkennt gleiche wirtschaftliche Bewegung ohne unkontrolliertes Mergen.
- Gleiche Beträge beim selben Händler werden nicht unbegründet gematcht.
- Teilzahlung/Sammelzahlung und Split erhalten korrekte Reste.
- Kreditkartenausgleich erzeugt keine doppelte Ausgabe.
- Gemischte Steuergruppen werden korrekt gerechnet.
- Ausland wird nicht pauschal als Reverse Charge behandelt.
- Proxy wird nicht als Original oder Vorsteuernachweis gewertet.
- Originalveränderung wird erkannt.
- Regelupdate ändert historische Festschreibungen nicht.
- GrandTotal-Zahlungsstatus und Bankeingang werden nicht doppelt gezählt.
- Plugin ohne Recht kann nicht auf Datei, Domain oder Journal zugreifen.
- Mailversand ohne Freigabe scheitert über UI, CLI, API und MCP.
- Bankzahlungsfunktion ist nicht vorhanden.
- Prompt Injection in Beleg/Website kann keine Aktion autorisieren.
- Absturz im Import oder Journalzugriff hinterlässt konsistenten Zustand.
- Uploadtimeout erzeugt „unklar“ statt blindem Doppelsenden.
- Restore stellt Originale, Journal, Beziehungen und Regelversionen wieder her.
- Export wird im tatsächlichen Zielsystem getestet.

Keine Produktivfreigabe des steuerlich maßgeblichen Kerns ohne: bestätigtes Steuerprofil, freigegebene Fallmatrix, fachlich geprüfte Referenzbuchungen, Parallelabstimmung mindestens einer vollständigen Periode, reale Exportprüfung, Restore-Test und geprüfte Verfahrensdokumentation.

---

## 26. Modulplan mit Freigabegates

### M00 – Discovery und Spezifikation
Abhängigkeit: keine.  
Lieferung: Requirement-IDs, Traceability, Decision Register, Bedrohungsmodell-Skizze, Quellenregister, ADRs-Plan, Plugin-Vertragsentwurf, M00-Arbeitsplan.  
Abnahme: Nutzer bestätigt Begriffe, Grenzen, Rechte und Stack. **Kein Produktcode vorher.**  
Status v1.1: Spezifikationspaket liegt vor; **Warte auf Nutzerfreigabe vor M00-Engineering im Repo.**

### M01 – Repository, Architektur und Designsystem
Lieferung: CI, Lint/Tests, Domänengrenzen, Tokens, Komponentenstorys, Datenbankmigrationsgerüst, synthetische Fixtures.  
Abnahme: reproduzierbarer Build; UI-Grundzustände; Architekturtests; keine Secrets.

### M02 – Plugin-Host und Anwendungsrechte
Lieferung: Manifest, Protokoll, Capability Gate, Credential-Abstraktion, Beispielplugin, API-Grundvertrag.  
Abnahme: Plugin ohne Kernänderung installierbar; Rechteentzug und Crashisolation getestet.

### M03 – Archiv und Importpipeline
Lieferung: Originalspeicher, Importbatches, Hashes, Versionen, Dubletten, Viewer, sichere Dateiverarbeitung.  
Abnahme: Originale bleiben erhalten; Wiederholimporte und fehlerhafte Dateien getestet.

### M04 – MoneyMoney und Konten
Lieferung: Read-only-Adapter, Kontenauswahl, Rohdaten, Source/EconomicTransaction, Importhistorie.  
Abnahme: echte Schnittstelle mit freigegebenen Testdaten geprüft; keine schreibende Bankfunktion.

### M05 – Matching, Splits und Arbeitsoberfläche
Lieferung: Viele-zu-viele-Zuordnungen, Gründe, Restbeträge, Drag-and-drop, Batch-Vorschau, Statusmodell.  
Abnahme: Teil-/Sammel-/Kartenfälle; keine Doppelzählung; vollständige Auditspur.

### M06 – Proxy, Kontextobjekte und Reminder
Lieferung: Platzhalter, Tags, Abo-/Projektbeziehungen, fehlende Belege, Wiedervorlagen.  
Abnahme: Nutzerdefinitionen bestätigt; Originalnachreichung erhält Historie; keine automatische Vorsteuer aus Proxy.

### M07 – Extraktion, OCR und Suche
Lieferung: strukturierte Rechnungsparser, Text/OCR, Feldprovenienz, Volltextsuche, optional semantische Suche.  
Abnahme: Referenzkorpus und gemessene Fehlerquoten; unsichere Werte sichtbar; Indizes rekonstruierbar.

### M08 – KI-Adapter
Lieferung: Ollama, optional OpenRouter, Budgets, Schemas, Caches und Datenpolicy. Codex nur nach gesonderter Dev-Integrationsprüfung.  
Abnahme: offline nutzbarer Kern; keine stille Cloudübertragung; Fehler-/Budgettests.

### M09 – GrandTotal-Adapter
Lieferung: getestete Exportprofile (GrandTotal 10 dateibasiert), Rechnungs-/Positionsimport, Stornos, Zahlungsabgleich.  
Abnahme: reale Version geprüft; keine direkte proprietäre DB-Manipulation; keine Doppeleinnahme.

### M10 – Mail und erste Portalconnectoren
Lieferung: konkrete freigegebene Mailintegration, mindestens ein priorisierter Portalconnector, Auth-/MFA-/Fehlerstatus.  
Abnahme: nur erlaubte Aktionen; kein Versand; Layoutänderung stoppt; Downloads nachvollziehbar.

### M11 – Buchhaltungskern und fachliche Regelpakete
Lieferung: Konten-/Steuerprofilversionen, Journal, EÜR-Abbildung, Fallmatrix, Anlagen, Korrekturen, Festschreibung.  
Abnahme: fachkundige Prüfung und Referenztests; nicht unterstützte Fälle blockiert. Keine alleinige Freigabe durch KI.

### M12 – Steuerentwürfe und Managementberichte
Lieferung: USt-/EÜR-Vorbereitung, Jahresabstimmung, Formularmapping, Periodenvergleiche, Drill-down.  
Abnahme: Zahlen bis zur Quelle abstimmbar; Steuer-/Managementbasis klar; Historienwechsel getestet.

### M13 – Exporte und Automationsschnittstellen
Lieferung: vollständiges Archivpaket, ausgewählte DATEV-Profile, CLI, REST, MCP, SDK-Dokumentation.  
Abnahme: reale Zielimporte, Rechteparität und vollständiger Exit-Export. Verträge wurden bereits in M02 festgelegt.

### M14 – Produktivhärtung
Lieferung: Restore, Update-/Migrationsstrategie, Signierung, Betriebs-/Verfahrensdokumentation, Parallelbetrieb.  
Abnahme: harte Produktivkriterien aus Abschnitt 25 erfüllt.

### M15 – Spätere Erweiterungsmodule
Bank X Professional, zusätzliche Bank-/Portaladapter (inkl. `mobile-de`), Steuerboard, direkte ERiC-Übermittlung, EU-Verkaufsfälle, optional visuelle Node-Ansicht. Je Erweiterung eigene Spezifikation, Rechte, Quellen und Abnahme. Kein Vorrang vor Kernzuverlässigkeit.

Die Reihenfolge darf nur mit dokumentierter Begründung geändert werden. Ein Modul ist nicht „fertig“, solange Abnahmekriterien fehlen oder nur simulierte Integrationen als produktiv ausgegeben werden.

---

## 27. Repository- und Dokumentationsstruktur

Zielstruktur für `Gitit360/Belegwerk` (final in M01 ADR):
```text
apps/desktop/
crates/domain/
crates/application/
crates/infrastructure/
crates/plugin-host/
packages/design-system/
packages/contracts/
packages/plugin-sdk/
plugins/examples/
plugins/connectors/
tests/fixtures/synthetic/
tests/integration/
docs/product/
docs/architecture/adr/
docs/security/
docs/accounting/
docs/plugins/
docs/api/
docs/operations/
docs/acceptance/
```

Pflichtdokumente (im Repo ab M01):
- `PROJECT_STATUS.md`, `REQUIREMENTS.md`, `DECISIONS.md` + ADRs
- `OPEN_QUESTIONS.md` (nur noch bewusst zurückgestellte Punkte)
- `ACCOUNTING_CASE_MATRIX.md`, `SECURITY_MODEL.md`, `DATA_POLICY.md`
- `PLUGIN_SPEC.md`, OpenAPI, MCP-Toolvertrag
- `TEST_PLAN.md`, Abnahmeprotokolle
- `PROCEDURAL_DOCUMENTATION.md`, `BACKUP_RESTORE.md`, `MIGRATIONS.md`, `RELEASE_CHECKLIST.md`

Jeder relevante Codepfad verknüpft sich über Tests mit Requirement-IDs. Keine „fertig“-Markierung nur nach UI-Screenshot.

---

## 28. Definition of Done pro Arbeitspaket

- Umfang und Abhängigkeiten dokumentiert.
- Implementierung innerhalb definierter Modulgrenzen.
- Tests ergänzt und tatsächlich ausgeführt; Ergebnisse festgehalten.
- Lint/Build erfolgreich oder konkrete Blockierung offengelegt.
- Berechtigungen und Fehlerzustände geprüft.
- Datenmigration/Rückfall bei Bedarf dokumentiert.
- UI mit Tokens, Tastatur und Leer-/Fehlerzuständen.
- Keine Secrets, echten Belege oder unnötigen personenbezogenen Logs.
- Plugin-/API-Vertrag kompatibel oder Version bewusst erhöht.
- Dokumentation und Status aktualisiert.
- Bekannte Einschränkungen und Abnahmeanfrage geliefert.

Cursor-Abschlussformat:
```text
Arbeitspaket:
Umgesetzt:
Geänderte Dateien:
Datenmodell/Verträge:
Ausgeführte Tests und Ergebnisse:
Nicht ausgeführte Prüfungen:
Risiken/offene Fragen:
Anforderungserfüllung:
Nächster vorgeschlagener Schritt:
Benötigte Freigabe:
```

---

## 29. Decision Register (Kurzverweis)

Alle ehemaligen offenen Punkte aus Quelle §29 sind im Decision Register `docs/belegwerk-decision-register-v1.1.md` mit Optionen, Wahl und Folgen geschlossen oder bewusst zurückgestellt. Gap-Analyse: `docs/belegwerk-gap-analysis-v1.1.md`. Traceability: `docs/belegwerk-traceability-matrix-v1.1.md`.

**Vor M00-Engineering weiterhin nutzerseitig freizugeben:** Stack/Begriffe/Grenzen/Rechte (M00-Abnahme), sowie produktive Steuerannahmen (Rhythmus, Dauerfrist, Steuerjahr, Konten) beim Setup — Engineering-Defaults stehen, ersetzen aber keine Nutzerbestätigung für den eigenen Betrieb.

---

## 30. Referenzquellen und Prüfaufträge

Diese Links sind Rechercheeinstiege, keine unveränderlichen Spezifikationen. Aktualität, Version, Lizenz und Nutzungsvoraussetzungen vor Implementierung dokumentieren. Steuerquellen für den jeweiligen Gültigkeitszeitraum fachlich prüfen — **Engineering setzt Defaults und Blockaden, keine erfundenen Rechtsgewissheiten**.

- MoneyMoney AppleScript: https://moneymoney.app/api/applescript/
- GrandTotal Buchhaltungsexport: https://www.mediaatelier.com/en/Posts/Send-To-Bookkeeping/
- GrandTotal Hilfe: https://www.mediaatelier.com/en/Help/
- BMF GoBD-Handbuch: https://lsth.bundesfinanzministerium.de/ao/2025/Anhaenge/BMF-Schreiben-und-gleichlautende-Laendererlasse/Anhang-33/inhalt.html
- BMF GoBD-Änderung 14.07.2025: https://www.bundesfinanzministerium.de/Content/DE/Downloads/BMF_Schreiben/Weitere-Steuerthemen/Abgabenordnung/2025-07-14-GoBD-2-aenderung.html
- ELSTER Entwickler: https://www.elster.de/elsterweb/infoseite/entwickler?locale=de_DE
- ELSTER EÜR: https://www.elster.de/eportal/formulare-leistungen/alleformulare/euer
- Ollama strukturierte Ausgaben: https://docs.ollama.com/capabilities/structured-outputs
- Ollama Vision: https://docs.ollama.com/capabilities/vision
- OpenRouter Routing: https://openrouter.ai/docs/guides/routing/provider-selection
- OpenRouter ZDR: https://openrouter.ai/docs/guides/features/zdr
- Steuerboard Portalhilfe: https://help.steuerboard.com/de/collections/19707818-portal

Design-/Workflow-Inspiration, nicht als Compliance-Nachweis: Receipts Space, DEVONthink, Accountable, sevdesk und GetMyInvoices. Keine fremden Assets, Texte oder proprietären Implementierungen kopieren.

---

## 31. Unverhandelbare Schlussregel

Lieber einen ungeklärten Fall sichtbar offen lassen als eine falsche Sicherheit erzeugen. Jede Erweiterung muss nachvollziehbar, abschaltbar, berechtigungsbegrenzt und ohne Verlust historischer Daten möglich sein. Das offene Plugin-Konzept und die Arbeit in abgenommenen Modulschritten sind Kernanforderungen des Produkts, keine späteren Extras.

---

## Anhang A – Spezifikationspaket v1.1

| Dokument | Zweck |
|---|---|
| `docs/belegwerk-product-description-v1.1.md` | Dieses verbindliche Spezifikationsdokument |
| `docs/belegwerk-gap-analysis-v1.1.md` | Lücken aus v1.0 und Entscheidungen |
| `docs/belegwerk-traceability-matrix-v1.1.md` | Requirement-IDs → Module/Abschnitte |
| `docs/belegwerk-decision-register-v1.1.md` | Entscheidungsregister |
| `docs/belegwerk-architecture-proposal-v1.1.md` | Architekturvorschlag |
| `docs/belegwerk-plugin-contract-v1.1.md` | Plugin-/SDK-Vertragsskizze |
| `docs/belegwerk-m00-workplan-v1.1.md` | M00-Arbeitsplan inkl. Freigabe-Gate |
| `docs/README.md` | Index |

---

*Ende Belegwerk Product Description v1.1 — 12. November 2026 (Europe/Berlin).*
