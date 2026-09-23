use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, ValueEnum, Debug)]
enum Top {
    Ip,
    Path,
    Status,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The access log file to read (or - for stdin)
    #[arg(default_value = "-")]
    file: String,

    /// Field to aggregate by
    #[arg(short, long, value_enum, default_value_t = Top::Ip)]
    top: Top,

    /// Number of top results to show
    #[arg(short, long, default_value_t = 10)]
    limit: usize,
}

fn read_input(file: &str) -> Result<Box<dyn BufRead>> {
    if file == "-" {
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        let f = File::open(file).with_context(|| format!("Failed to open {}", file))?;
        Ok(Box::new(BufReader::new(f)))
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Combined Log Format Regex:
    // 127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326
    let re = Regex::new(
        r#"^(?P<ip>\S+) \S+ \S+ \[[^\]]+\] "(?P<method>\S+) (?P<path>\S+) [^"]+" (?P<status>\d{3}) \d+"#
    ).unwrap();

    let reader = read_input(&args.file)?;

    let mut counts: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        if let Some(caps) = re.captures(&line) {
            let key = match args.top {
                Top::Ip => caps.name("ip").unwrap().as_str().to_string(),
                Top::Path => caps.name("path").unwrap().as_str().to_string(),
                Top::Status => caps.name("status").unwrap().as_str().to_string(),
            };
            *counts.entry(key).or_insert(0) += 1;
        }
    }

    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by_key(|b| std::cmp::Reverse(b.1));

    for (k, v) in sorted.into_iter().take(args.limit) {
        println!("{:>8} {}", v, k);
    }

    Ok(())
}
