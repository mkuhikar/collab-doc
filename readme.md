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