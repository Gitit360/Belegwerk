//! Named actions. Bank payment actions intentionally absent (REQ-S-004).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownAction {
    DocumentsRead,
    DocumentsImport,
    BankTransactionsRead,
    MailSearch,
    MailDraftCreate,
    MailSend,
    BookingPost,
    TaxSubmit,
    ExportBuild,
    DeliverySubmit,
    PluginInstall,
    /// Explicitly unsupported — gate always denies.
    BankPaymentCreate,
}

impl KnownAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DocumentsRead => "documents.read",
            Self::DocumentsImport => "documents.import",
            Self::BankTransactionsRead => "bank.transactions.read",
            Self::MailSearch => "mail.search",
            Self::MailDraftCreate => "mail.draft.create",
            Self::MailSend => "mail.send",
            Self::BookingPost => "booking.post",
            Self::TaxSubmit => "tax.submit",
            Self::ExportBuild => "export.build",
            Self::DeliverySubmit => "delivery.submit",
            Self::PluginInstall => "plugin.install",
            Self::BankPaymentCreate => "bank.payment.create",
        }
    }

    pub fn requires_approval(self) -> bool {
        matches!(
            self,
            Self::MailSend
                | Self::BookingPost
                | Self::TaxSubmit
                | Self::DeliverySubmit
                | Self::PluginInstall
        )
    }

    pub fn is_always_denied(self) -> bool {
        matches!(self, Self::BankPaymentCreate)
    }
}
