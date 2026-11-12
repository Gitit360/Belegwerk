#!/usr/bin/env node
/**
 * Belegwerk CLI stub (M13) — health/status only.
 * No secrets in argv. Exit codes: 0 ok, 2 usage, 3 blocked action.
 */

type Health = {
  name: string;
  version: string;
  bankPaymentsEnabled: boolean;
  locale: string;
};

const health: Health = {
  name: "Belegwerk",
  version: "0.1.0",
  bankPaymentsEnabled: false,
  locale: "de-DE",
};

function printHelp(): void {
  console.log(`belegwerk <command>

Befehle:
  health   JSON-Gesundheitsstatus
  status   Kurzstatus (deutsch)
  help     Diese Hilfe
`);
}

function main(argv: string[]): number {
  const cmd = argv[2] ?? "help";
  switch (cmd) {
    case "health":
      console.log(JSON.stringify(health, null, 2));
      return 0;
    case "status": {
      console.log(
        [
          `App: ${health.name} ${health.version}`,
          `Locale: ${health.locale}`,
          `Bankzahlungen: ${health.bankPaymentsEnabled ? "an" : "aus (REQ-S-004)"}`,
          "Hinweis: CLI-Stub — volle Automations-API folgt in M13.",
        ].join("\n"),
      );
      return 0;
    }
    case "pay":
    case "bank-pay":
      console.error(
        JSON.stringify({
          code: "bank_payment_blocked",
          message: "Bankzahlungen sind nicht implementiert und bleiben gesperrt.",
          blocked: true,
        }),
      );
      return 3;
    case "help":
    case "--help":
    case "-h":
      printHelp();
      return 0;
    default:
      console.error(`Unbekannter Befehl: ${cmd}`);
      printHelp();
      return 2;
  }
}

process.exit(main(process.argv));
