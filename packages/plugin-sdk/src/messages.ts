export type MessageType = "request" | "response" | "event" | "error";

export type PluginMessage<T = unknown> = {
  v: 1;
  id: string;
  type: MessageType;
  method: string;
  correlationId?: string;
  payload: T;
  error?: unknown;
};
