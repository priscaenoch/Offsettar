import type { DepositRequest, WithdrawRequest, QuoteRequest, QuoteResponse } from "./types";

export class OffSettaClient {
  private baseUrl: string;
  private apiKey: string;

  constructor(baseUrl: string, apiKey: string) {
    this.baseUrl = baseUrl;
    this.apiKey = apiKey;
  }

  private async post<T>(path: string, body: unknown): Promise<T> {
    const res = await fetch(`${this.baseUrl}${path}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "X-API-Key": this.apiKey,
      },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`Offsetta API error: ${res.status}`);
    return res.json() as Promise<T>;
  }

  deposit(req: DepositRequest) {
    return this.post("/v1/onramp/deposit", req);
  }

  withdraw(req: WithdrawRequest) {
    return this.post("/v1/offramp/withdraw", req);
  }

  quote(req: QuoteRequest): Promise<QuoteResponse> {
    return this.post<QuoteResponse>("/v1/routing/quote", req);
  }
}
