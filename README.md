# OrthoTerm

A command-line tool for fetching and managing Orthodox Christian calendar data. This tool fetches liturgical data from holytrinityorthodox.com and can generate both JSON and iCal formats for easy integration with other applications.

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

- Fetches Orthodox calendar data including:
  - Daily saints and feasts
  - Scripture readings
  - Troparia and other liturgical texts
- Saves data in JSON format for easy parsing
- Optional iCal generation for calendar integration
- Incremental fetching with automatic resume
- Resilient network handling with automatic retries

## Installation 