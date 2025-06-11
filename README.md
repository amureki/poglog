# poglog

A fast, colorful PostgreSQL log highlighter and SQL formatter for your terminal, written in Rust.

## Features

- Highlights SQL statements in PostgreSQL logs with syntax coloring
- Formats (pretty-prints) SQL queries for readability
- Colorizes timestamps, log levels, and durations
- Works with standard Postgres log output (not JSON)

## Installation

1. Clone this repository:
   ```sh
   git clone <your-repo-url>
   cd poglog
   ```
2. Build with Cargo:
   ```sh
   cargo build --release
   ```
3. The binary will be at `target/release/poglog`

## Usage

Pipe your Postgres logs into poglog:

```sh
tail -f /path/to/postgresql.log | poglog
```
