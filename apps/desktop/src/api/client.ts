import { browserApi, type TaxProfile } from "./browserStore";

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    const { invoke: tauriInvoke } = await import("@tauri-apps/api/core");
    return await tauriInvoke<T>(cmd, args);
  } catch {
    throw new Error("tauri-unavailable");
  }
}

async function withFallback<T>(tauri: () => Promise<T>, fallback: () => Promise<T>): Promise<T> {
  try {
    return await tauri();
  } catch {
    return fallback();
  }
}

export const api = {
  async initDataRoot(): Promise<string | null> {
    try {
      return await invoke<string>("init_data_root");
    } catch {
      return null;
    }
  },

  matchingSnapshot: () =>
    withFallback(
      () => invoke("matching_snapshot"),
      () => browserApi.matchingSnapshot(),
    ),

  matchingConfirm: (transactionId: string, documentId: string) =>
    withFallback(
      () =>
        invoke("matching_confirm", {
          args: { transactionId, documentId },
        }),
      () => browserApi.matchingConfirm(transactionId, documentId),
    ),

  matchingReject: (transactionId: string, documentId: string) =>
    withFallback(
      () =>
        invoke("matching_reject", {
          args: { transactionId, documentId },
        }),
      () => browserApi.matchingReject(transactionId, documentId),
    ),

  matchingBatchPreview: (items: Array<{ transactionId: string; documentId: string }>) =>
    withFallback(
      () => invoke("matching_batch_preview", { items }),
      () => browserApi.matchingBatchPreview(items),
    ),

  importFiles: (
    sourceLabel: string,
    files: Array<{ filename: string; mimeType: string; textContent: string }>,
  ) =>
    withFallback(
      () =>
        invoke("import_synthetic_files", {
          sourceLabel,
          files: files.map((f) => ({
            filename: f.filename,
            mimeType: f.mimeType,
            textContent: f.textContent,
          })),
        }),
      () => browserApi.importFiles(sourceLabel, files),
    ),

  importListBatches: () =>
    withFallback(
      () => invoke("import_list_batches"),
      () => browserApi.importListBatches(),
    ),

  importListDocuments: (batchId?: string) =>
    withFallback(
      () => invoke("import_list_documents", { batchId: batchId ?? null }),
      () => browserApi.importListDocuments(batchId),
    ),

  remindersList: () =>
    withFallback(
      () => invoke("reminders_list"),
      () => browserApi.remindersList(),
    ),

  remindersEscalate: (caseId: string) =>
    withFallback(
      () => invoke("reminders_escalate", { caseId }),
      () => browserApi.remindersEscalate(caseId),
    ),

  remindersComplete: (caseId: string) =>
    withFallback(
      () => invoke("reminders_complete", { caseId }),
      () => browserApi.remindersComplete(caseId),
    ),

  journalList: () =>
    withFallback(
      () => invoke("journal_list_drafts"),
      () => browserApi.journalList(),
    ),

  journalGrantApproval: (draftId: string) =>
    withFallback(
      () => invoke("journal_grant_booking_approval", { draftId }),
      () => browserApi.journalGrantApproval(draftId),
    ),

  journalApprove: (draftId: string) =>
    withFallback(
      () => invoke("journal_approve_draft", { draftId }),
      () => browserApi.journalApprove(draftId),
    ),

  settingsGet: (): Promise<TaxProfile> =>
    withFallback(
      async () => {
        const raw = await invoke<Record<string, unknown>>("settings_get_tax_profile");
        return {
          vat_method: (raw.vat_method as TaxProfile["vat_method"]) ?? "ist",
          reporting_cadence:
            (raw.reporting_cadence as TaxProfile["reporting_cadence"]) ?? "monthly",
          dauerfrist_extension: Boolean(raw.dauerfrist_extension),
          tax_year: Number(raw.tax_year ?? 2026),
        };
      },
      () => browserApi.settingsGet(),
    ),

  settingsSave: (profile: TaxProfile) =>
    withFallback(
      async () => {
        const raw = await invoke<Record<string, unknown>>("settings_save_tax_profile", {
          profile,
        });
        return {
          vat_method: (raw.vat_method as TaxProfile["vat_method"]) ?? profile.vat_method,
          reporting_cadence:
            (raw.reporting_cadence as TaxProfile["reporting_cadence"]) ??
            profile.reporting_cadence,
          dauerfrist_extension: Boolean(raw.dauerfrist_extension),
          tax_year: Number(raw.tax_year ?? profile.tax_year),
        };
      },
      () => browserApi.settingsSave(profile),
    ),
};

export type { TaxProfile };
