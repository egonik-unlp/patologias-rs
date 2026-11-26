# patologias-rs

Pathology report manager (cytology and histopathology studies). Built with
[Leptos](https://leptos.dev) + Axum, PostgreSQL via Diesel, and Tailwind.

## Requirements

- [rustup](https://rustup.rs). The nightly toolchain is picked up automatically from `rust-toolchain.toml`.
- PostgreSQL (any recent version) and its client library, `libpq`.

### Linux (Debian/Ubuntu)

```bash
sudo apt install libpq-dev postgresql
```

On Fedora: `sudo dnf install libpq-devel postgresql-server`.

### macOS

```bash
brew install libpq postgresql@16
brew services start postgresql@16
export PQ_LIB_DIR="$(brew --prefix libpq)/lib"   # add to your ~/.zshrc
```

### Postgres with Docker (either OS)

If you'd rather not install Postgres natively (you still need `libpq` from above):

```bash
docker run -d --name patologias-db -p 5432:5432 \
  -e POSTGRES_PASSWORD=postgres postgres:16
```

### Rust tools (both OSes)

```bash
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install cargo-leptos --locked
cargo install diesel_cli --no-default-features --features postgres
```

## Setup

1. Create a `.env` in the project root:

   ```env
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/patologias
   PAGE_LENGTH=20
   ```

2. Create the database and run the migrations:

   ```bash
   diesel setup
   ```

3. Create the first admin user (the password is read from stdin):

   ```bash
   cargo run --features ssr --bin create-admin -- <username> "<full name>"
   ```

## Run

```bash
cargo leptos watch
```

Open <http://127.0.0.1:3000/login>. The page reloads automatically when you edit the code.

## Optional

- **Test data:** put `studies-test.csv` in `data/` (CSV files aren't committed), then run
  `cargo run --features ssr --bin load-reports`. It only runs on an empty `studies` table.
- **Release build:** `cargo leptos build --release` writes the server binary to
  `target/release/` and the static site to `target/site/`.
