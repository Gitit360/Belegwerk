/** Shared wire types between UI, local API, and plugins. */

export type MoneyAmount = {
  amountMinor: number;
  currency: string;
};

export type SourceTransactionDto = {
  id: string;
  providerId: string;
  sourceAccountId: string;
  sourceTransactionRef: string;
  bookingDate: string;
  valueDate?: string;
  amount: MoneyAmount;
  counterparty?: string;
  purpose?: string;
  status: string;
  importedAt: string;
  rawRef?: string;
};

export type EconomicTransactionDto = {
  id: string;
  amount: MoneyAmount;
  bookingDate: string;
  description?: string;
  sourceTransactionIds: string[];
  status: string;
};

export type StatusDimensionsDto = {
  documentCompleteness: string;
  matchState: string;
  extractionState: string;
  bookingState: string;
  exportState: string;
};

export type LocalApiError = {
  code: string;
  message: string;
  blocked?: boolean;
};
