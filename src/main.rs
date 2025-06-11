use std::io::{self, BufRead};
use regex::Regex;
use colored::*;
use sqlformat::{format, FormatOptions, QueryParams};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

// Extract colorization logic into a function
fn colorize_line(line: &str, timestamp_re: &Regex, level_re: &Regex, duration_re: &Regex) -> String {
    let line = timestamp_re.replace(line, |caps: &regex::Captures| caps[1].cyan().to_string()).to_string();
    let line = level_re.replace(&line, |caps: &regex::Captures| caps[0].yellow().to_string()).to_string();
    let line = duration_re.replace(&line, |caps: &regex::Captures| caps[1].green().to_string()).to_string();
    line
}

fn main() {
    let stdin = io::stdin();
    let timestamp_re = Regex::new(r"^(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d+ [A-Z]+ \[\d+\])").unwrap();
    let level_re = Regex::new(r"\b(LOG|ERROR|WARNING|FATAL|PANIC|INFO|DEBUG):").unwrap();
    let duration_re = Regex::new(r"(duration: [0-9.]+ ms)").unwrap();
    let statement_re = Regex::new(r"(statement: )(.*)").unwrap();

    // Setup syntect for SQL highlighting
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    // Handle missing theme gracefully
    let theme = ts.themes.get("base16-ocean.dark");
    if theme.is_none() {
        eprintln!("Warning: Theme 'base16-ocean.dark' not found. SQL highlighting will be skipped.");
    }
    let syntax = ps.find_syntax_by_extension("sql").unwrap();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                // Colorize timestamp, log level, and duration
                let colored_line = colorize_line(&line, &timestamp_re, &level_re, &duration_re);

                // Highlight and format SQL statements
                if let Some(caps) = statement_re.captures(&colored_line) {
                    let prefix = &caps[1];
                    let sql = &caps[2];
                    let sql_formatted = format(sql, &QueryParams::None, FormatOptions::default());
                    if let Some(theme) = theme {
                        let mut h = HighlightLines::new(syntax, theme);
                        let ranges = h.highlight(&sql_formatted, &ps);
                        let sql_highlighted = as_24_bit_terminal_escaped(&ranges[..], false);
                        // Indent SQL output for clarity
                        let indented_sql = sql_highlighted
                            .lines()
                            .map(|l| format!("    {}", l))
                            .collect::<Vec<_>>()
                            .join("\n");
                        println!("{}\n{}", prefix, indented_sql);
                    } else {
                        // If theme is missing, just print formatted SQL indented
                        let indented_sql = sql_formatted
                            .lines()
                            .map(|l| format!("    {}", l))
                            .collect::<Vec<_>>()
                            .join("\n");
                        println!("{}\n{}", prefix, indented_sql);
                    }
                } else {
                    println!("{}", colored_line);
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
}
