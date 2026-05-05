# Architecture

## System Overview

Offsetta is a monorepo with three primary layers:

1. **apps/web** — Next.js dashboard and developer portal
2. **apps/api** — Node.js REST API that orchestrates onramp/offramp flows
3. **contracts/** — Soroban smart contracts deployed on Stellar

## Data Flow: Onramp (Mobile Money → USDC)

```
User → Mobile Money Provider
     → Offsetta API (webhook confirmation)
     → CustodyLedger.credit()
     → SettlementEscrow.lock()
     → USDC transferred to user's Stellar address
     → SettlementEscrow.release()
```

## Data Flow: Offramp (USDC → Mobile Money)

```
User → Offsetta API (withdraw request)
     → USDC locked in SettlementEscrow
     → Mobile Money payout initiated
     → On confirmation: SettlementEscrow.release()
     → CustodyLedger.debit()
```

## Contracts

| Contract | Responsibility |
|---|---|
| CustodyLedger | Tracks fiat-backed balances on-chain |
| SettlementEscrow | Holds funds during in-flight settlements |
| LiquidityPool | Pooled USDC for instant settlement |
| FeeEngine | Calculates and distributes protocol fees |
| ComplianceRegistry | KYC/AML allowlist and blocklist |

## Routing Engine

The routing engine (in `apps/api`) selects the optimal path for a given corridor based on:
- Available liquidity
- Fee minimisation
- Settlement speed
