# LDO Arbitrage Bot Documentation

Date: 17/11/2023
Authors: Joshua Cohen-Dumani and Kelyan Hangard
# Overview

The LDO Arbitrage Bot, developed in Rust, is designed to identify and potentially exploit arbitrage opportunities between Binance and OKX exchanges, specifically for the LDO/USDT trading pair. This application uses asynchronous programming to fetch real-time trading data, facilitating efficient and rapid analysis of potential arbitrage scenarios.
### Structure

The application's codebase is structured into various modules, each with a specific role:

- main.rs: This is the entry point of the application. It initiates the various components and manages the flow of the arbitrage detection process.

- arbitrage.rs: Contains the core logic for arbitrage detection. It is divided into two primary functionalities: historic_arbitrage and live_arbitrage.

- utils.rs: Offers utility functions for common operations throughout the application. A notable function is the timestamp parser, essential for time-based data analysis.

- api_clients: This directory includes the binance.rs and okx.rs modules. These are responsible for interacting with the respective APIs of Binance and OKX to retrieve trading data.

# Functionalities

## Historic Arbitrage Analysis (historic_arbitrage):
- Retrieves historical trade data from both Binance and OKX.
- Analyzes past data to identify potential arbitrage opportunities, considering the trade fees.
- Outputs these opportunities into a CSV file, historic_arbitrage.csv, for further examination.

## Live Arbitrage Monitoring (live_arbitrage):
- Continuously fetches the latest trade data from the two exchanges.
- Identifies real-time arbitrage opportunities and calculates potential net profits.
- Displays detailed information about these opportunities, including execution time for API calls and logic, in the console.


## Prerequisites
- Rust programming language
- Cargo package manager

## Installation
1. Clone the repository in your desired folder: **git clone git@github.com:j0shcd/rust_arbitrage_bot.git**
2. Navigate to the project directory: **cd rust_arbitrage_bot**
3. Build the project using Cargo: **cargo build**

# Usage

To run the LDO Arbitrage Bot, execute the following command in your terminal:

**cargo run**

Upon execution, the bot first performs historical arbitrage analysis, saving its findings in historic_arbitrage.csv. Following this, it commences live arbitrage monitoring, presenting any detected opportunities in the terminal.
### Future Enhancements

The application, while operational, can be further improved:

- Complex Fee Structures: Adapt to incorporate diverse fee tiers based on trading volume or account status.

- Advanced Arbitrage Algorithm: Enhance the algorithm to account for factors like order book depth and slippage.

- Expansion to More Exchanges and Pairs: Extend capabilities to include more exchanges and various trading pairs. 

- Automated Trading: Implement features to automatically execute trades when arbitrage is detected, in line with predefined user parameters.

- Historical Data Analysis: Analyze a broader range of historical data for better understanding of arbitrage opportunity patterns.

- User Interface: Create a user-friendly interface for easier monitoring and trading.

## Disclaimer

This tool should be used with caution. Cryptocurrency trading involves significant risk, and the authors are not liable for any financial losses incurred while using this software.
