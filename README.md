# Rust Retrieval Engine

A full-text search engine built with Rust, Axum, and Tantivy.

## Tech Stack
- **Rust** — Systems programming language
- **Axum** — Web framework
- **Tantivy** — Full-text search engine
- **Tracing** — Structured logging
- **Docker** — Containerization

---

## Prerequisites

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
```

### Install Docker
Download Docker Desktop from https://www.docker.com/products/docker-desktop

---

## Project Structure
rust_retrieval_engine/
├── src/
│   ├── main.rs        # Entry point
│   ├── logger.rs      # Logger setup
│   ├── schema.rs      # Tantivy schema
│   ├── index.rs       # Index builder
│   ├── models.rs      # Request/Response models
│   ├── search.rs      # Search handler
│   └── server.rs      # HTTP server + middleware
├── Cargo.toml
├── Dockerfile
└── README.md

---

## Run Locally

### Step 1 — Clone the repo
```bash
git clone https://github.com/rajeevsahani/rust_retrieval_engine.git
cd rust_retrieval_engine
```

### Step 2 — Build and run
```bash
cargo build
cargo run
```

Server starts at `http://127.0.0.1:3000`

---

## Run with Docker

### Step 1 — Build Docker image
```bash
docker build -t rust_retrieval_engine .
```

### Step 2 — Run container
```bash
docker run -p 3001:3000 rust_retrieval_engine
```

Server starts at `http://127.0.0.1:3001`

---

## API Endpoints

### POST /search
Search for documents.

**Request:**
```bash
curl -X POST http://127.0.0.1:3001/search \
  -H "Content-Type: application/json" \
  -d '{"q": "rust"}'
```

**Response:**
```json
{
  "results": [
    "Rust is fast and memory safe.",
    "Axum is a modern Rust web framework.",
    "Tantivy is a full-text search engine in Rust."
  ]
}
```

### Error Responses

**404 — Route not found:**
```bash
curl http://127.0.0.1:3001/wrong-route
```
```json
{
  "code": 404,
  "status": "Not Found",
  "message": "Route not found"
}
```

**422 — Missing field:**
```bash
curl -X POST http://127.0.0.1:3001/search \
  -H "Content-Type: application/json" \
  -d '{}'
```

**400 — Bad query:**
```bash
curl -X POST http://127.0.0.1:3001/search \
  -H "Content-Type: application/json" \
  -d '{"q": "AND OR AND"}'
```

**500 — Test route:**
```bash
curl -X POST http://127.0.0.1:3001/test/500
```

---

## Push to GitHub

### Step 1 — Create SSH key
```bash
ssh-keygen -t ed25519 -C "your_email@gmail.com"
cat ~/.ssh/id_ed25519.pub
```

### Step 2 — Add SSH key to GitHub
1. Go to https://github.com/settings/ssh/new
2. Paste the key
3. Click **Add SSH key**

### Step 3 — Create GitHub repo
Go to https://github.com/new and create a new repo

### Step 4 — Push code
```bash
git init
git add .
git commit -m "initial commit"
git remote add origin git@github.com:YOUR_USERNAME/rust_retrieval_engine.git
git branch -M main
git push -u origin main
```

---

## Create GitHub Personal Access Token (HTTPS alternative)

1. Go to https://github.com/settings/tokens/new
2. Name: `rust_retrieval_engine`
3. Expiration: 90 days
4. Check `repo` scope
5. Click **Generate token**
6. Copy the token immediately

```bash
git remote set-url origin https://YOUR_USERNAME:YOUR_TOKEN@github.com/YOUR_USERNAME/rust_retrieval_engine.git
git push -u origin main
```

---

## Logs

Logs follow Python-style formatting:
2026-05-03 01:22:03  INFO src/main.rs:14: Starting rust_retrieval_engine...
2026-05-03 01:22:03  INFO src/schema.rs:5: Building schema...
2026-05-03 01:22:03  INFO src/schema.rs:9: Schema built successfully
2026-05-03 01:22:03  INFO src/index.rs:6: Creating in-RAM index...
2026-05-03 01:22:03  INFO src/index.rs:28: Indexed 5 documents successfully
2026-05-03 01:22:03  INFO src/server.rs:81: Building router...
2026-05-03 01:22:03  INFO src/server.rs:94: Server listening on http://127.0.0.1:3000
2026-05-03 01:22:22  INFO src/server.rs:61: --> POST /search
2026-05-03 01:22:22  INFO src/search.rs:22: Received search query: 'rust'
2026-05-03 01:22:22  INFO src/search.rs:60: Search returned 3 results
2026-05-03 01:22:22  INFO src/server.rs:67: <-- POST /search 200
2026-05-03 01:22:33  WARN src/server.rs:42: 404 - Route not found
2026-05-03 01:22:46  INFO src/server.rs:61: --> POST /search
2026-05-03 01:22:46  WARN src/server.rs:69: <-- POST /search 422