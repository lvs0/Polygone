# Contributing to Polygone

## Setup

### Prerequisites
- Rust (stable) + Cargo
- Docker & Docker Compose (optional, for local cluster testing)

### Build

```bash
git clone https://github.com/lvs0/Polygone.git
cd Polygone
cargo build --workspace
```

## Tests

Run all tests (unit + integration):

```bash
cargo test --workspace
```

Run tests for a specific crate, e.g. crypto:

```bash
cargo test -p polygone-crypto
```

## Lint & Format

```bash
# Check formatting
cargo fmt --all -- --check

# Lint (clippy)
cargo clippy --workspace -- -D warnings
```

## Running Locally

To run a single node (CLI demo):

```bash
cargo run --bin polygone
```

## Running a Local Cluster (Docker)

We provide a docker-compose file to launch 3 nodes locally:

```bash
docker compose up --build -d
```

Logs:

```bash
docker compose logs -f node1
docker compose logs -f node2
docker compose logs -f node3
```

To stop:

```bash
docker compose down
```

## Submitting Changes

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/xxx`).
3. Commit your changes (`git commit -am 'Add feature xxx'`).
4. Push to the branch (`git push origin feature/xxx`).
5. Create a Pull Request.

## Network manual test (quick check)

You can manually run 3 local nodes to verify that P2P networking works:

```bash
# Terminal 1
cargo run --bin polygone -- --listen /ip4/127.0.0.1/tcp/10000 --pseudo node1

# Terminal 2
cargo run --bin polygone -- --listen /ip4/127.0.0.1/tcp/10001 --pseudo node2 --peer /ip4/127.0.0.1/tcp/10000/p2p/...PEER_ID_OF_NODE1...

# Terminal 3
cargo run --bin polygone -- --listen /ip4/127.0.0.1/tcp/10002 --pseudo node3 --peer /ip4/127.0.0.1/tcp/10000/p2p/...PEER_ID_OF_NODE1...
```

Replace `...PEER_ID_OF_NODE1...` with the actual peer ID printed by node1 at startup.

You should see logs confirming connections and, if supported by the binary, gossip messages propagating. If `polygone` does not expose these flags yet, you may run the test via the test harness:

```bash
cargo test -p polygone-network
```

Note: a full integration test with libp2p is available in `crates/network/tests/` (may require libp2p features enabled). For faster CI we retain lightweight unit tests for crypto and use this manual step for network verification.

## Deploy a node (free)

You can deploy a public Polygone node in ~5 minutes on one of these platforms.

### 1. Render.com (simple, free, sleeps after 15 min — keepalive compensates)

1. Push this repo to your GitHub.
2. On Render, click **New Web Service** → select this repo.
3. Use settings:
   - Runtime: Docker
   - Dockerfile path: `Dockerfile`
   - Plan: Free
   - Add env vars: `NODE_PSEUDO`, optionally `PUBLIC_URL` and `BOOTSTRAP_PEERS`.
4. Click **Create Web Service**.
5. Render will build and deploy. The `/health` endpoint and keepalive prevent permanent sleep.

### 2. Fly.io (free, no sleep, requires flyctl)

1. Install flyctl: `curl -L https://fly.io/install.sh | sh`
2. `fly auth login`
3. `fly launch` (select this repo; say yes to Dockerfile)
4. Edit generated `fly.toml` if desired.
5. Set secrets (example):
   ```bash
   fly secrets set NODE_PSEUDO=polygone-fly PUBLIC_URL=https://polygone-node.fly.dev
   ```
6. `fly deploy`

### 3. Railway (free tier with monthly credits)

1. Log in to railway.app and create a new project.
2. Choose **Deploy from repo** → select this repository.
3. Railway will detect `railway.json` (or you can use `Dockerfile`).
4. Add environment variables: `NODE_PSEUDO`, `PUBLIC_URL`, `BOOTSTRAP_PEERS`.
5. Deploy.

All platforms will expose port 8080 with `/health`. Use the `PUBLIC_URL` env to advertise the node to others and `BOOTSTRAP_PEERS` to join an existing mesh.

## Deploy a public node (Render - 5 min)

1. Create a free account on [Render](https://render.com).
2. Click **New Web Service** → select this GitHub repo.
3. Settings:
   - **Runtime**: Docker
   - **Dockerfile path**: `Dockerfile`
   - **Plan**: Free
   - **Auto Deploy**: ON
4. Environment variables (add these):
   - `NODE_PSEUDO` = `polygone-demo`
   - `PUBLIC_URL` = (leave blank, Render sets it automatically)
   - `BOOTSTRAP_PEERS` = (empty for first node, or list existing peers)
5. Click **Create Web Service**.

Render will build and deploy. After a few minutes you will get a public URL like:
```
https://polygone-demo.onrender.com
```
The dashboard Web UI will be available at `/` on that URL (port 9050 internally mapped if you also expose it), but the main HTTP API (health) will be at the root.

### Verify the node is alive

```bash
curl https://polygone-demo.onrender.com/health
```

Expected response:
```json
{"status":"ok"}
```

### Access the Web UI

If you map port 9050 in Render (or use a custom domain), open:
```
https://polygone-demo.onrender.com
```
(or http://127.0.0.1:9050 if run locally).

### Command to test P2P file retrieval (if Drive is enabled)

```bash
curl https://polygone-demo.onrender.com/p2p/get/demo.txt
```
