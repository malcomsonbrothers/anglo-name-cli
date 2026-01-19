# anglo-name-cli

A fast, feature-rich command-line tool for generating random Anglo names with optional credentials (email addresses, passwords, and birthdays). Perfect for generating test data, seeding databases, or creating mock user records.

## Features

- Generate random Anglo first names (male, female, or any)
- Optional middle names
- Email address generation with customisable domains
- Secure 32-character password generation
- Birthday generation (by date range or age)
- Multiple output formats: plain text, JSON, CSV
- Guaranteed unique names by default
- Embedded name data (741 male, 720 female first names, 482 surnames)
- Fast generation using `fastrand` for name selection

## Installation

### From Source

Requires Rust 1.80 or later.

```bash
git clone https://github.com/yourusername/anglo-name-cli.git
cd anglo-name-cli
cargo install --path .
```

### Build Only

```bash
# Debug build
cargo build

# Release build (optimised)
cargo build --release
```

The binary will be at `target/release/anglo-name-cli`.

## Usage

### Basic Name Generation

```bash
# Generate a single random name
anglo-name-cli

# Generate 10 names
anglo-name-cli -n 10

# Generate 5 male names
anglo-name-cli -n 5 -s male

# Generate 5 female names with middle names
anglo-name-cli -n 5 -s female -m

# Allow duplicate names (unique by default)
anglo-name-cli -n 100 --allow-duplicates
```

### Email Generation

```bash
# Generate name with email (random domain: gmail.com or outlook.com)
anglo-name-cli -e

# Generate with a specific domain
anglo-name-cli -e -d company.com

# Generate 10 names with emails
anglo-name-cli -n 10 -e -d mycompany.com
```

Email format: `first.middle.last@domain` (all lowercase, special characters removed)

### Password Generation

```bash
# Generate name with a secure 32-character password
anglo-name-cli -p

# Generate name with email and password
anglo-name-cli -e -p
```

Passwords include uppercase, lowercase, digits, and special characters (`!@#$%^&*`).

### Birthday Generation

Three mutually exclusive modes:

```bash
# Specific date range (DD-MM-YYYY format)
anglo-name-cli --birthday-from 01-01-1990 --birthday-to 31-12-1999

# Exact age (generates birthday for someone this age today)
anglo-name-cli --age 25

# Age range
anglo-name-cli --age-from 18 --age-to 65
```

### Output Formats

```bash
# Plain text (default) - one name per line
anglo-name-cli -n 5

# JSON format
anglo-name-cli -n 5 -f json

# CSV format
anglo-name-cli -n 5 -f csv

# Write to file
anglo-name-cli -n 100 -f json -o names.json
```

### Complete Example

Generate 50 unique female names with middle names, emails, passwords, and birthdays for people aged 25-40, output as CSV:

```bash
anglo-name-cli -n 50 -s female -m -e -d example.com -p --age-from 25 --age-to 40 -f csv -o users.csv
```

## CLI Reference

### Name Options

| Option | Short | Description |
|--------|-------|-------------|
| `--count <N>` | `-n` | Number of names to generate (default: 1) |
| `--sex <SEX>` | `-s` | Sex for first names: `male`, `female`, or `any` (default: any) |
| `--middle` | `-m` | Include middle name |
| `--allow-duplicates` | | Allow duplicate names (unique by default) |

### Credential Options

| Option | Short | Description |
|--------|-------|-------------|
| `--email` | `-e` | Generate email addresses |
| `--domain <DOMAIN>` | `-d` | Email domain (requires `--email`) |
| `--password` | `-p` | Generate 32-character passwords |

### Birthday Options

| Option | Description |
|--------|-------------|
| `--birthday-from <DATE>` | Start date in DD-MM-YYYY format (requires `--birthday-to`) |
| `--birthday-to <DATE>` | End date in DD-MM-YYYY format (requires `--birthday-from`) |
| `--age <AGE>` | Generate birthday for exact age |
| `--age-from <AGE>` | Minimum age (requires `--age-to`) |
| `--age-to <AGE>` | Maximum age (requires `--age-from`) |

### Output Options

| Option | Short | Description |
|--------|-------|-------------|
| `--format <FORMAT>` | `-f` | Output format: `plain`, `json`, or `csv` (default: plain) |
| `--output <FILE>` | `-o` | Output file path (default: stdout) |

### Information

| Option | Description |
|--------|-------------|
| `--help` | Show help message |
| `--version` | Show version |

## Output Format Examples

### Plain Text

```
John William Smith | john.william.smith@example.com | Kx9#mNpL2@vQrT8&... | 15-06-1992
```

### JSON

```json
[
  {
    "first_name": "John",
    "middle_name": "William",
    "last_name": "Smith",
    "full_name": "John William Smith",
    "email": "john.william.smith@example.com",
    "password": "Kx9#mNpL2@vQrT8&...",
    "birthday": "15-06-1992"
  }
]
```

### CSV

```csv
first_name,middle_name,last_name,full_name,email,password,birthday
John,William,Smith,John William Smith,john.william.smith@example.com,Kx9#mNpL2@vQrT8&...,15-06-1992
```

## Data Sources

Name lists are embedded in the binary at compile time:

- **Male first names**: 741 names (`files/male-first.txt`)
- **Female first names**: 720 names (`files/female-first.txt`)
- **Surnames**: 482 names (`files/last-names.txt`)

Maximum unique combinations without middle names: ~357,000 per sex.

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

### Linting and Formatting

```bash
cargo fmt
cargo clippy --all-features --all-targets
```

## Project Structure

```
anglo-name-cli/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library root
│   ├── cli.rs           # CLI argument definitions
│   ├── name/            # Name generation module
│   ├── creds/           # Email & password generation
│   ├── birthday/        # Birthday generation
│   └── output/          # Output formatters (plain, JSON, CSV)
├── files/               # Embedded name data
└── tests/               # Integration tests
```

## Licence

MIT
