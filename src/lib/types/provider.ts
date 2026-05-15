/** Matches Rust ProviderId enum (serde: snake_case). */
export type ProviderId =
  | "anthropic"
  | "cursor"
  | "open_ai"
  | "copilot"
  | "windsurf"
  | "kilo";

/** Display name mapping for each provider. */
export const PROVIDER_DISPLAY_NAMES: Record<ProviderId, string> = {
  anthropic: "Anthropic",
  cursor: "Cursor",
  open_ai: "OpenAI",
  copilot: "GitHub Copilot",
  windsurf: "Windsurf",
  kilo: "Kilo Code",
};

/** Matches Rust ProviderStatus enum (serde: snake_case). */
export type ProviderStatus = "active" | "idle" | "error" | "disconnected";

/** Matches Rust UsageData struct. */
export interface UsageData {
  provider: ProviderId;
  input_tokens: number;
  output_tokens: number;
  cost_usd: number;
  model: string;
  timestamp: string; // ISO 8601
}

/** Matches Rust BurnRate struct. */
export interface BurnRate {
  dollars_per_hour: number;
  window_seconds: number;
  sample_count: number;
}

/** Matches Rust Balance struct. */
export interface Balance {
  provider: ProviderId;
  remaining_usd: number | null;
  credits_used: number;
  billing_period_start: string; // ISO 8601
  billing_period_end: string; // ISO 8601
}

/** Matches Rust Transaction struct. */
export interface Transaction {
  id: number;
  provider: ProviderId;
  model: string;
  input_tokens: number;
  output_tokens: number;
  cost_usd: number;
  timestamp: string; // ISO 8601
}

/** Matches Rust BillingPeriod enum. */
export type BillingPeriod =
  | "calendar_month"
  | "rolling_30"
  | { custom: number };

/** Matches Rust Budget struct. */
export interface Budget {
  monthly_limit_usd: number;
  daily_limit_usd: number | null;
  threshold_pcts: number[];
  billing_period: BillingPeriod;
}

// --- Type guards ---

const PROVIDER_IDS: ReadonlySet<string> = new Set([
  "anthropic",
  "cursor",
  "open_ai",
  "copilot",
  "windsurf",
  "kilo",
]);

export function isProviderId(value: unknown): value is ProviderId {
  return typeof value === "string" && PROVIDER_IDS.has(value);
}

const PROVIDER_STATUSES: ReadonlySet<string> = new Set([
  "active",
  "idle",
  "error",
  "disconnected",
]);

export function isProviderStatus(value: unknown): value is ProviderStatus {
  return typeof value === "string" && PROVIDER_STATUSES.has(value);
}
