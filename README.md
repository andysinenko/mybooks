# MyBooks - Family Book Catalog

A simple home book tracking service built with Rust. Pet project for learning Rust, async programming, and containerization.

## Tech Stack

- **Rust** + **Axum** — — backend service 
- **Rust** + **Axum** — — gateway service (another project) 
- **PostgreSQL** — database
- **Docker Compose** — orchestration

## Architecture
Client → Gateway (Rust) → Backend (Rust) → PostgreSQL


## Getting Started

### Prerequisites
- Docker & Docker Compose

### Run

```bash

git clone git@github.com:andysinenko/mybooks.git
cd mybooks
cp .env.example .env
cargo install sqlx-cli
sqlx migrate run
docker compose up --build
```

Gateway will be available on `http://localhost:3000`.
MyBooks will be available on `http://localhost:8080`.

## Configuration

Copy `.env.example` to `.env` and change the values as needed.




