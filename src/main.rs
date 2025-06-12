use colored::*;
use serde_json::Value;
use std::io::{self, BufRead};
use regex::Regex;
use sqlformat::{format, FormatOptions, QueryParams};
use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                match serde_json::from_str::<Value>(&line) {
                    Ok(json) => {
                        print_log_entry(&json);
                    }
                    Err(_) => {
                        eprintln!("{} {}", "[INVALID JSON]".red().bold(), line);
                    }
                }
            }
            Err(e) => {
                eprintln!("{} {}", "[READ ERROR]".red().bold(), e);
            }
        }
    }
}

fn print_log_entry(json: &Value) {
    let ts = json.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
    let level = json.get("error_severity").and_then(|v| v.as_str()).unwrap_or("");
    let msg = json.get("message").and_then(|v| v.as_str()).unwrap_or("");
    let pid = json.get("pid").map(|v| v.to_string()).unwrap_or_else(|| "".to_string());
    let user = json.get("user").and_then(|v| v.as_str()).unwrap_or("");
    let db = json.get("dbname").and_then(|v| v.as_str()).unwrap_or("");

    let level_colored = match level {
        "ERROR" => level.red().bold(),
        "WARNING" => level.yellow().bold(),
        "LOG" | "INFO" => level.green(),
        "FATAL" | "PANIC" => level.on_red().white().bold(),
        _ => level.normal(),
    };

    // Build header with only non-empty fields (no backend_type)
    let mut header = format!("{} {} pid={}", ts.dimmed(), level_colored, pid.cyan());
    if !user.is_empty() {
        header.push_str(&format!(" user={}", user.blue()));
    }
    if !db.is_empty() {
        header.push_str(&format!(" db={}", db.magenta()));
    }

    // Regex for SQL statement with duration
    let re_sql = Regex::new(r"duration: ([0-9.]+ ms)\s+statement: (.*)").unwrap();
    // Regex for SQL statement without duration
    let re_statement = Regex::new(r"statement: (.*)").unwrap();

    let (duration, statement) = if let Some(caps) = re_sql.captures(msg) {
        (caps.get(1).map(|m| m.as_str()).unwrap_or(""), caps.get(2).map(|m| m.as_str()).unwrap_or(""))
    } else if let Some(caps) = re_statement.captures(msg) {
        ("", caps.get(1).map(|m| m.as_str()).unwrap_or(""))
    } else {
        ("", "")
    };

    // Colorize duration
    let duration_colored = if !duration.is_empty() {
        format!("duration: {}", duration).yellow().bold().to_string()
    } else {
        String::new()
    };

    // Print header and duration
    if !duration_colored.is_empty() {
        println!("{} {}", header, duration_colored);
    } else {
        println!("{}", header);
    }

    // If SQL statement, pretty-print and highlight
    if !statement.is_empty() {
        let formatted_sql = format(
            statement,
            &QueryParams::None,
            FormatOptions::default(),
        );
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let syntax = syntax_set.find_syntax_by_extension("sql").unwrap();
        let mut h = HighlightLines::new(syntax, &theme_set.themes["base16-ocean.dark"]);
        let highlighted_sql = LinesWithEndings::from(&formatted_sql)
            .map(|line| {
                let ranges: Vec<(Style, &str)> = h.highlight_line(line, &syntax_set).unwrap();
                as_24_bit_terminal_escaped(&ranges, false)
            })
            .collect::<Vec<_>>()
            .join("");
        println!("    {}", highlighted_sql.replace("\n", "\n    "));
    } else {
        // Otherwise, print message as indented, colorized plain text
        // Highlight some keywords
        let keywords = [
            ("checkpoint", "checkpoint".bold().cyan().to_string()),
            ("complete", "complete".bold().green().to_string()),
            ("starting", "starting".bold().yellow().to_string()),
            ("autovacuum", "autovacuum".bold().magenta().to_string()),
        ];
        let mut colored_msg = msg.to_string();
        for (kw, colored_kw) in &keywords {
            colored_msg = colored_msg.replace(kw, colored_kw);
        }
        for line in colored_msg.lines() {
            println!("    {}", line);
        }
    }
}
