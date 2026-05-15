import { describe, it, expect } from "vitest";
import {
  isProviderId,
  isProviderStatus,
  PROVIDER_DISPLAY_NAMES,
  type ProviderId,
  type ProviderStatus,
  type UsageData,
  type BurnRate,
  type Balance,
  type Transaction,
  type Budget,
} from "./provider";

describe("ProviderId", () => {
  it("validates known provider ids", () => {
    const validIds: ProviderId[] = [
      "anthropic",
      "cursor",
      "open_ai",
      "copilot",
      "windsurf",
      "kilo",
    ];
    for (const id of validIds) {
      expect(isProviderId(id)).toBe(true);
    }
  });

  it("rejects invalid provider ids", () => {
    expect(isProviderId("invalid")).toBe(false);
    expect(isProviderId("")).toBe(false);
    expect(isProviderId(42)).toBe(false);
    expect(isProviderId(null)).toBe(false);
    expect(isProviderId(undefined)).toBe(false);
  });
});

describe("ProviderStatus", () => {
  it("validates known statuses", () => {
    const validStatuses: ProviderStatus[] = [
      "active",
      "idle",
      "error",
      "disconnected",
    ];
    for (const status of validStatuses) {
      expect(isProviderStatus(status)).toBe(true);
    }
  });

  it("rejects invalid statuses", () => {
    expect(isProviderStatus("online")).toBe(false);
    expect(isProviderStatus(123)).toBe(false);
  });
});

describe("PROVIDER_DISPLAY_NAMES", () => {
  it("has a name for every provider", () => {
    const ids: ProviderId[] = [
      "anthropic",
      "cursor",
      "open_ai",
      "copilot",
      "windsurf",
      "kilo",
    ];
    for (const id of ids) {
      expect(PROVIDER_DISPLAY_NAMES[id]).toBeTruthy();
    }
  });

  it("matches expected display names", () => {
    expect(PROVIDER_DISPLAY_NAMES.anthropic).toBe("Anthropic");
    expect(PROVIDER_DISPLAY_NAMES.open_ai).toBe("OpenAI");
    expect(PROVIDER_DISPLAY_NAMES.copilot).toBe("GitHub Copilot");
  });
});

describe("Type shapes", () => {
  it("UsageData shape is correct", () => {
    const usage: UsageData = {
      provider: "anthropic",
      input_tokens: 1500,
      output_tokens: 500,
      cost_usd: 0.042,
      model: "claude-opus-4-6",
      timestamp: "2026-05-15T12:00:00Z",
    };
    expect(usage.provider).toBe("anthropic");
    expect(usage.input_tokens + usage.output_tokens).toBe(2000);
  });

  it("BurnRate shape is correct", () => {
    const rate: BurnRate = {
      dollars_per_hour: 3.41,
      window_seconds: 3600,
      sample_count: 15,
    };
    expect(rate.dollars_per_hour).toBe(3.41);
  });

  it("Balance shape with remaining", () => {
    const balance: Balance = {
      provider: "open_ai",
      remaining_usd: 42.5,
      credits_used: 57.5,
      billing_period_start: "2026-05-01T00:00:00Z",
      billing_period_end: "2026-05-31T23:59:59Z",
    };
    expect(balance.remaining_usd).toBe(42.5);
  });

  it("Balance shape with null remaining", () => {
    const balance: Balance = {
      provider: "cursor",
      remaining_usd: null,
      credits_used: 100.0,
      billing_period_start: "2026-05-01T00:00:00Z",
      billing_period_end: "2026-05-31T23:59:59Z",
    };
    expect(balance.remaining_usd).toBeNull();
  });

  it("Transaction shape is correct", () => {
    const txn: Transaction = {
      id: 1,
      provider: "cursor",
      model: "gpt-4o",
      input_tokens: 3000,
      output_tokens: 1200,
      cost_usd: 0.085,
      timestamp: "2026-05-15T14:30:00Z",
    };
    expect(txn.input_tokens + txn.output_tokens).toBe(4200);
  });

  it("Budget shape with calendar month", () => {
    const budget: Budget = {
      monthly_limit_usd: 200.0,
      daily_limit_usd: 20.0,
      threshold_pcts: [50, 75, 90, 100],
      billing_period: "calendar_month",
    };
    expect(budget.threshold_pcts).toHaveLength(4);
  });

  it("Budget shape with custom billing period", () => {
    const budget: Budget = {
      monthly_limit_usd: 300.0,
      daily_limit_usd: null,
      threshold_pcts: [80, 100],
      billing_period: { custom: 15 },
    };
    expect(budget.daily_limit_usd).toBeNull();
  });
});
