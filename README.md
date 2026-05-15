# Ipocrate

Ipocrate is a Rust backend serving a React application shell from a single local service.

## Requirements

- Rust stable
- Node.js 22 or newer
- npm

## Run Locally

From a clean checkout:

```sh
cd frontend
npm ci
npm run build
cd ..
cargo build
cargo run
```

Then open <http://localhost:8080/>.

The backend listens on port `8080` by default. You can override runtime configuration with either CLI flags or environment variables:

```sh
cargo run -- --port 3000 --frontend-dist frontend/dist
```

```sh
PORT=3000 FRONTEND_DIST=frontend/dist cargo run
```

CLI flags take precedence over environment variables, and environment variables take precedence over defaults.

## Health Check

```sh
curl http://localhost:8080/health
```

Expected response:

```json
{"status":"ok"}
```

## Project Layout

- `src/`: Rust backend
- `frontend/`: React, TypeScript, Vite, Tailwind CSS, and shadcn/ui frontend
- `frontend/dist`: generated React static assets served by the backend

The backend expects `frontend/dist/index.html` to exist before startup. Run `npm run build` in `frontend/` before starting the backend.

## Local Validation

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build
cd frontend
npm ci
npm run lint
npm run build
```
