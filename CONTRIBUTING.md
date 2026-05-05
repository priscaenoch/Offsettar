# Contributing to Offsetta

Thank you for your interest in contributing. Offsetta is open source and community-driven — every contribution matters, whether it's a bug fix, a new corridor integration, or improved documentation.

---

## Before you start

- Read the [README](./README.md) to understand the project.
- Read the [Architecture doc](./docs/architecture.md).
- Check [open issues](../../issues) — especially ones labelled `good first issue` or `help wanted`.

---

## Development setup

### Requirements

- Node.js ≥ 20
- Rust (stable) + `wasm32-unknown-unknown` target
- `soroban-cli`
- Docker (for local Postgres + Redis)

### Install

```bash
git clone https://github.com/your-org/offsetta.git
cd offsetta
npm install
```

### Run

```bash
# API (port 4000)
cd apps/api && cp .env.example .env && npm run dev

# Web (port 3000)
cd apps/web && npm run dev
```

### Build contracts

```bash
cd contracts
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

---

## Workflow

1. Fork the repository.
2. Create a branch: `git checkout -b feat/your-feature` or `fix/your-fix`.
3. Make your changes.
4. Run lint: `npm run lint` from the root.
5. Commit using [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` new feature
   - `fix:` bug fix
   - `docs:` documentation only
   - `chore:` tooling / config
   - `refactor:` code change with no feature/fix
6. Push and open a Pull Request against `main`.

---

## Pull Request guidelines

- Keep PRs focused — one concern per PR.
- Describe *what* you changed and *why* in the PR description.
- Link the related issue (e.g. `Closes #42`).
- Smart contract changes must include a written rationale.
- All CI checks must pass before merge.

---

## Code style

| Language | Tool |
|---|---|
| TypeScript | ESLint + Prettier |
| Rust | `rustfmt` (run `cargo fmt`) |
| Commits | Conventional Commits |

---

## Reporting bugs

Open an issue with the `bug` label. Include:
- Steps to reproduce
- Expected vs actual behaviour
- Your OS, Node version, Rust version

## Security vulnerabilities

Do **not** open a public issue. Email **security@offsetta.xyz** directly.

---

## Code of Conduct

By contributing, you agree to abide by our [Code of Conduct](./CODE_OF_CONDUCT.md). Be respectful and constructive.
