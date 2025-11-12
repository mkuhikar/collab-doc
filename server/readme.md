# Collab-Doc

A collaborative document editing project built with Rust and PostgreSQL.

## Prerequisites

Before running the project, ensure the following are installed:

1. **PostgreSQL 17**  
   - Download and install from [PostgreSQL Downloads](https://www.postgresql.org/download/).

2. **Rust**  
   - Install via [rustup](https://rustup.rs/).

3. **SQLx CLI**  
   - Install with:
     ```bash
     cargo install sqlx-cli --no-default-features --features postgres
     ```

## Running the Project

1. Build the project:
   ```bash
   cargo build
2. Run the project:
    ```bash
    cargo run

## Database Setup

1. Verify that PostgreSQL is running locally.
2. Run migrations to set up your database schema:
   ```bash
   sqlx migrate run
3. Your database connection string should look like this:
   ```bash
   postgres://postgres:qazwsx@localhost:5432/collab-db
   ```
4. VS Code Configuration (for SQLx support)

      To remove red squiggly lines in SQLx query macros inside VS Code:

      Press Cmd + Shift + P → search for Open User Settings (JSON).

      Add the following snippet inside the JSON:

      "rust-analyzer.cargo.extraEnv": {
         "DATABASE_URL": "postgres://postgres:qazwsx@localhost:5432/collab-db"
      }


      Save and reload VS Code (Cmd + Shift + P → Developer: Reload Window).