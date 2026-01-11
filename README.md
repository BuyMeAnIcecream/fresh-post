# Fresh Post - LinkedIn Job Scraper

A Rust-based tool to scrape fresh job postings from LinkedIn, filter duplicates, and notify you of new opportunities.

## Features

- 🔍 Scrapes LinkedIn job postings using HTTP requests
- 📅 Filters to only show jobs posted today
- 🚫 Tracks seen jobs to avoid duplicates across multiple runs
- 💾 Persistent state management (file-based)
- 🧪 Comprehensive test coverage
- 🦀 Built with Rust for performance and safety

## Requirements

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Internet connection to access LinkedIn

## Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd notifier

# Build the project
cargo build --release

# Run the scraper
cargo run --release
```

## Usage

Currently, search parameters are hardcoded in `src/main.rs`. Edit the following lines to customize:

```rust
let keywords = "rust developer";
let location = "San Francisco Bay Area";
```

The tool will:
1. Fetch jobs from LinkedIn matching your criteria
2. Filter to only jobs posted today
3. Remove jobs you've already seen (tracked in `.notifier_state.json`)
4. Display new jobs in the console
5. Save state for the next run

## State Management

The tool maintains a state file (`.notifier_state.json`) that tracks which jobs you've already seen. This file is automatically created on first run and updated after each scan.

**Note:** The state file is gitignored and should not be committed.

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run only library tests
cargo test --lib

# Run with output
cargo test -- --nocapture
```

## Project Structure

```
notifier/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library root
│   ├── models.rs        # JobPosting data structure
│   ├── scraper.rs       # LinkedIn scraping logic
│   ├── state.rs         # Duplicate tracking
│   └── filters.rs       # Date filtering
├── tests/
│   ├── fixtures/        # Sample HTML for testing
│   └── integration_test.rs
└── Cargo.toml
```

## Future Enhancements

- [ ] Telegram bot integration for notifications
- [ ] Config file support for search parameters
- [ ] Multiple search queries
- [ ] Email notifications
- [ ] Database storage option
- [ ] Rate limiting and retry logic

## Important Notes

⚠️ **LinkedIn Terms of Service**: This tool scrapes LinkedIn, which may violate their Terms of Service. Use responsibly and at your own risk.

⚠️ **HTML Structure Changes**: LinkedIn's HTML structure may change, which could break the scraper. The CSS selectors in `scraper.rs` may need updates.

⚠️ **Rate Limiting**: The tool currently doesn't implement rate limiting. For production use, consider adding delays between requests.

## License

[Add your license here]

## Contributing

[Add contribution guidelines if needed]
