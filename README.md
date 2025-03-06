# OrthoTerm

A command-line tool for fetching Orthodox Christian calendar data and generating iCal files. OrthoTerm provides easy access to daily saints, readings, and liturgical notes from the Orthodox calendar, with support for both Gregorian and Julian dates.

## Data Source

The calendar data is sourced from [Holy Trinity Orthodox Church](https://holytrinityorthodox.com/calendar/), which provides:
- Daily saints and commemorations
- Scripture readings from the Orthodox lectionary
- Troparia and kontakia
- Fasting guidelines
- Major feast days

Please note that this tool respects the source website by:
- Including appropriate delays between requests
- Implementing retry logic with exponential backoff
- Caching data locally to minimize server load

## Features

- Fetch Orthodox calendar data for any year
- Generate iCal files for calendar integration
- Support for both Gregorian and Julian calendar dates
- Local data storage using XDG base directories
- Automatic caching of calendar data

## Installation

### Using Nix

TODO: Add Nix installation instructions

### From Source

1. Ensure you have Rust installed via [rustup](https://rustup.rs/)
2. Clone and build the repository:

```bash
git clone https://github.com/yourusername/orthoterm.git
cd orthoterm
cargo install --path .
```

## Usage

### Basic Usage

# Fetch current year's calendar data
orthoterm

# Fetch specific year's data
orthoterm 2025

# Generate iCal file for a year
orthoterm -i 2025

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 

### Command-line Options

- `[YEAR]`: Optional. The year to fetch calendar data for (defaults to current year)
- `-i`: Generate an iCal file for the specified year

### Output Files

OrthoTerm stores its data in standard XDG directories:
- Calendar data: `~/.local/share/orthoterm/data/calendar_YEAR.json`
- iCal files: `~/.local/share/orthoterm/ical/calendar_YEAR.ics`

## Development

### Using Nix Development Shell

The project includes a `flake.nix` for a reproducible development environment:

```bash
# Enter development shell
nix develop

# Now you can build and test
cargo build
cargo test
```

### Manual Development Setup

1. Install Rust via [rustup](https://rustup.rs/)
2. Clone the repository:
```bash
git clone https://github.com/yourusername/orthoterm.git
cd orthoterm
```

3. Build and run:
```bash
cargo build
cargo run
```

### Running Tests

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 