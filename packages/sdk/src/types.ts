export interface DepositRequest {
  amount: number;
  currency: string;
  mobileNumber: string;
  provider: "mpesa" | "mtn" | "airtel";
  stellarAddress: string;
}

export interface WithdrawRequest {
  amount: number;
  stellarAddress: string;
  mobileNumber: string;
  provider: "mpesa" | "mtn" | "airtel";
}

export interface QuoteRequest {
  fromCurrency: string;
  toCurrency: string;
  amount: number;
  corridor: string;
}

export interface QuoteResponse {
  estimatedAmount: number;
  fee: number;
  rate: number;
  expiresAt: string;
}
