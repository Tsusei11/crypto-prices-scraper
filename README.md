# Crypto Market Data Scraper 

A high-performance data scraper and API for cryptocurrency markets, written entirely in Rust.

This application connects to multiple cryptocurrency exchanges (Binance, ByBit, KuCoin) via asynchronous WebSockets to stream real-time market data. It processes this data into 1-minute OHLC (Open, High, Low, Close) bars, saves them to a PostgreSQL database, and exposes the data through a lightweight HTTP API built with Axum.

## Features

* **Multi-Exchange Support:** Concurrently scrapes data from **Binance**, **ByBit**, and **KuCoin**.
* **Real-Time Data:** Uses asynchronous WebSockets for low-latency data streaming.
* **Data Aggregation:** Parses raw trade data into 1-minute OHLC bars.
* **Persistent Storage:** Saves all OHLC data to a PostgreSQL database using **Diesel**.
* **Web API:** Provides a simple **Axum**-based HTTP API to query the collected data.
* **Asynchronous:** Built on the Tokio runtime for efficient, non-blocking I/O.

---

## Technologies Used

* **Language:** Rust
* **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
* **Database ORM:** [Diesel](https://diesel.rs/)
* **Database:** PostgreSQL
* **Concurrency:** [Tokio](https://tokio.rs/)
* **WebSockets:** [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite) (or similar)

---

## Project Structure

The project is organized into a multi-crate workspace to separate concerns:

* `scrapper_engine`: The core logic for managing WebSocket connections, handling subscriptions, and retrieving raw data.
* `exchanges`: Contains exchange-specific modules for defining request formats and parsing data structures for each supported exchange.
* `db`: A dedicated crate for all database interactions, using Diesel. It handles saving the 1-minute bars from the scraper and fetching data for the API.
* `api`: The Axum application that defines and serves the HTTP GET endpoints.

---

## Configuration

All configuration is managed through a `.env` file in the project root. Create this file and add the following:

```ini
# A comma-separated list of markets for Binance (e.g., btcusdt,ethusdt)
MARKETS_BINANCE=btcusdt,ethusdt

# A comma-separated list of markets for KuCoin (e.g., btc-usdt,eth-usdt)
MARKETS_KUCOIN=btc-usdt,eth-usdt

# A comma-separated list of markets for ByBit (e.g., btcusdt,ethusdt)
MARKETS_BYBIT=btcusdt,ethusdt

# Standard PostgreSQL connection string
DATABASE_URL=postgres://user:password@localhost/crypto_db

# Interval in seconds for sending periodic pings to keep WebSocket connections alive
PING_INTERVAL=30
```

---

## Database Schema

The application uses a single table in a PostgreSQL database to store the OHLC data.

**Table: `ohlc_1min`** (example name)

| Column | Type | Description |
| :--- | :--- | :--- |
| `id` | `Serial` | Primary Key |
| `exchange` | `Varchar` | Name of the exchange (e.g., "Binance") |
| `market` | `Varchar` | Market pair (e.g., "btcusdt") |
| `timestamp` | `Timestamp` | Start time of the 1-minute bar (UTC) |
| `o` | `Numeric` | Open price |
| `l` | `Numeric` | Low price |
| `h` | `Numeric` | High price |
| `c` | `Numeric` | Close price |

---

## API Endpoints

The application exposes the following `GET` endpoints:

### 1. List Available Exchanges

Returns a JSON array of all configured exchange names.

* **Endpoint:** `/exchanges`
* **Response:**
    ```json
    ["Binance", "KuCoin", "ByBit"]
    ```

### 2. List Available Markets

Returns a JSON array of all configured market pairs.

* **Endpoint:** `/markets`
* **Response:**
    ```json
    ["btcusdt", "ethusdt", "btc-usdt", "eth-usdt"]
    ```

### 3. Get Last 1-Minute Bar

Returns the most recent 1-minute OHLC bar for a specific exchange and market.

* **Endpoint:** `/last_min`
* **Query Parameters:**
    * `exchange` (string, case-insensitive): The name of the exchange (e.g., `binance`).
    * `market` (string, case-insensitive): The market pair (e.g., `btcusdt`). Note: The API uses a standardized format (`btcusdt`) even if the exchange's native format is different (`btc-usdt`).
* **Example:** `GET /last_min?exchange=binance&market=btcusdt`
* **Response:**
    ```json
    {
      "open": 101213.3,
      "close": 101245.4,
      "min": 101205.2,
      "max": 101278.8
    }
    ```

---

## Project Learnings

This project served as a deep dive into practical, high-performance Rust development. Key challenges and learnings included:

* **Software Design:** Architecting a multi-crate workspace for a concurrent application.
* **Rust Abstractions:** Solved design problems using **trait objects** to create generic interfaces for different exchanges.
* **Concurrency:** Managed multiple asynchronous WebSocket connections and shared state safely and efficiently.
* **Optimization:** Refactored the data ingestion pipeline to eliminate blocking read operations, ensuring the application remains responsive.
* **Ecosystem:** Gained hands-on experience with **Axum** for web servers, **Diesel** for database operations, and the **Tokio** ecosystem for asynchronous I/O.
* **Git Workflow:** Maintained a clean git history using feature branches and merge requests for all major additions.
