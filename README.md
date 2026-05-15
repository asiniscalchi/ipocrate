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

The repository includes a local `.env` that binds the backend to all interfaces on port `8080`:

```sh
HOST=0.0.0.0
PORT=8080
FRONTEND_DIST=frontend/dist
```

The backend defaults to `127.0.0.1:8080` when no `.env`, environment variable, or CLI flag is present. You can override runtime configuration with CLI flags, environment variables, or `.env` values:

```sh
cargo run -- --host 127.0.0.1 --port 3000 --frontend-dist frontend/dist
```

```sh
HOST=127.0.0.1 PORT=3000 FRONTEND_DIST=frontend/dist cargo run
```

CLI flags take precedence over environment variables, `.env` fills in missing environment variables, and defaults are used last.

When binding to all interfaces with `HOST=0.0.0.0`, startup logs include the concrete local URLs detected from network interfaces.

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
