# Offsetta

> Programmable mobile money ↔ blockchain financial rails, built on the Stellar network.

Offsetta is an open-source settlement infrastructure that bridges mobile money networks, banks, and stablecoins into a single programmable layer — starting with Africa.

---

## What it does

| Layer | Description |
|---|---|
| **Fiat Onramp** | Accept mobile money (M-Pesa, MTN MoMo, Airtel) and convert to on-chain stablecoins |
| **Offramp** | Redeem stablecoins back to mobile money or bank accounts |
| **Routing Engine** | Intelligent path-finding across corridors and liquidity sources |
| **Liquidity Layer** | Pooled on-chain liquidity for instant settlement |
| **Developer API** | REST + webhook API for fintechs and wallets to plug in |

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     apps/web (Next.js)                  │
│              Dashboard · Docs · Status Page             │
└────────────────────────┬────────────────────────────────┘
                         │ REST / WebSocket
┌────────────────────────▼────────────────────────────────┐
│                    apps/api (Node.js)                   │
│         Onramp · Offramp · Routing · Webhooks           │
└────────────────────────┬────────────────────────────────┘
                         │ Stellar SDK / Soroban RPC
┌────────────────────────▼────────────────────────────────┐
│              contracts/ (Soroban / Rust)                │
│  CustodyLedger · SettlementEscrow · LiquidityPool       │
│  FeeEngine · ComplianceRegistry                         │
└─────────────────────────────────────────────────────────┘
```

---

## Monorepo Structure

```
offsetta/
├── apps/
│   ├── web/          # Next.js dashboard & developer portal
│   └── api/          # Node.js REST API & webhook engine
├── contracts/
│   ├── custody-ledger/
│   ├── settlement-escrow/
│   ├── liquidity-pool/
│   ├── fee-engine/
│   └── compliance-registry/
├── packages/
│   └── sdk/          # Shared TypeScript SDK (API client + Stellar helpers)
├── docs/
│   ├── architecture.md
│   ├── api-reference.md
│   └── contracts.md
└── README.md
```

---

## MVP Scope

- [ ] USDC deposit via mobile money (one corridor)
- [ ] USDC withdrawal to mobile money
- [ ] REST API with API key auth
- [ ] Settlement Escrow contract (Soroban)
- [ ] Liquidity Pool contract (Soroban)

---

## Tech Stack

| Area | Technology |
|---|---|
| Blockchain | Stellar · Soroban (Rust) |
| Stablecoin | USDC (Circle) |
| Backend | Node.js · Express · TypeScript |
| Frontend | Next.js · Tailwind CSS |
| Database | PostgreSQL · Redis |
| Infra | Docker · GitHub Actions |

---

## Getting Started

### Prerequisites

- Node.js ≥ 20
- Rust + `soroban-cli`
- Docker (for local Postgres + Redis)

### Install

```bash
git clone https://github.com/your-org/offsetta.git
cd offsetta
npm install
```

### Run locally

```bash
# Start API
cd apps/api && npm run dev

# Start Web
cd apps/web && npm run dev
```

### Build contracts

```bash
cd contracts/settlement-escrow
cargo build --target wasm32-unknown-unknown --release
```

---

## Contributing

Offsetta is fully open source and community-driven. We welcome contributors of all backgrounds — whether you write Rust, TypeScript, or just have strong opinions about payment UX.

### How to contribute

1. **Find an issue** — Browse [open issues](../../issues) or look for `good first issue` labels.
2. **Fork & branch** — Fork the repo and create a branch: `git checkout -b feat/your-feature`
3. **Build & test** — Make your changes, add tests where relevant.
4. **Open a PR** — Submit a pull request with a clear description of what you changed and why.
5. **Review** — A maintainer will review within 3–5 business days.

### Contribution areas

| Area | Skills needed |
|---|---|
| Smart contracts | Rust, Soroban SDK |
| API | TypeScript, Node.js, REST design |
| Frontend | React, Next.js, Tailwind |
| Mobile money integrations | API integration, fintech knowledge |
| Docs & DX | Technical writing |
| Security | Auditing, threat modelling |

### Guidelines

- Follow the existing code style (ESLint + Prettier for TS, `rustfmt` for Rust).
- Keep PRs focused — one feature or fix per PR.
- Write meaningful commit messages (`feat:`, `fix:`, `docs:`, `chore:`).
- All smart contract changes require a written rationale in the PR description.
- Be respectful. See [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).

### Reporting bugs

Open an issue with the `bug` label. Include steps to reproduce, expected vs actual behaviour, and your environment.

### Security vulnerabilities

Do **not** open a public issue. Email **security@offsetta.xyz** directly.

---

## Roadmap

| Phase | Focus |
|---|---|
| **Phase 1 (MVP)** | USDC rail · one corridor · core API |
| **Phase 2** | Multi-corridor · liquidity pools · SDK |
| **Phase 3** | Compliance registry · KYC hooks · partner integrations |
| **Phase 4** | Decentralised routing · governance |

---

## License

[Apache 2.0](./LICENSE)

---

<p align="center">Built for Africa. Open to the world.</p>
