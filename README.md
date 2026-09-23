# accesslog

A fast, headless CLI to analyze NGINX/Apache access logs. Fills the gap left by GoAccess (which is heavy on UI) and manual `awk` scripts by providing a dedicated command line tool to aggregate IP addresses, paths, or status codes.

## Status

**built, untested**: the access log regex parser and aggregator is built. It has not been heavily tested on varied access log formats.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
# Top 10 IPs
cat access.log | accesslog --top ip

# Top 5 paths
accesslog /var/log/nginx/access.log --top path --limit 5
```
