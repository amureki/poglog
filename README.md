# poglog

A fast, colorful PostgreSQL JSON log highlighter and SQL formatter for your terminal, written in Rust.

## Features

- Parses and highlights logs in PostgreSQL's `jsonlog` format
- Formats (pretty-prints) SQL queries for readability and highlights them with syntax coloring
- Colorizes timestamps, log levels, durations, and other key fields
- Indents and colorizes important keywords in non-SQL log messages

## PostgreSQL jsonlog support

poglog is designed for PostgreSQL's `jsonlog` format, providing robust and structured log parsing.

To enable `jsonlog` in your PostgreSQL configuration, add or modify the following lines in your `postgresql.conf`:

```conf
log_destination = 'jsonlog'
logging_collector = on
```

Restart PostgreSQL after making these changes. Your logs will now be written in JSON format, which poglog will automatically detect and process.

## Installation

Install poglog from [crates.io](https://crates.io/crates/poglog) using Cargo:

```sh
cargo install poglog
```

This will download, build, and install the `poglog` binary to your Cargo bin directory (usually `~/.cargo/bin`).

## Usage

Pipe your Postgres JSON logs into poglog:

```sh
tail -f /path/to/log.json | poglog
```

Or process a file:

```sh
cat /path/to/log.json | poglog
```

### Example

**Input (jsonlog):**
```json
{"timestamp":"2025-06-11 18:15:14.380 CEST","pid":47538,"user":"pogstar","dbname":"caps","error_severity":"LOG","message":"duration: 1.234 ms  statement: SELECT * FROM logs WHERE level = 'ERROR' AND service = 'auth'"}
{"timestamp":"2025-06-11 18:20:11.064 CEST","pid":47526,"error_severity":"LOG","message":"checkpoint starting: time"}
```

**Output (colorized, formatted):**
```
2025-06-11 18:15:14.380 CEST LOG pid=47538 user=pogstar db=caps duration: 1.234 ms
    SELECT *
    FROM logs
    WHERE level = 'ERROR'
      AND service = 'auth'
2025-06-11 18:20:11.064 CEST LOG pid=47526
    checkpoint starting: time
```
