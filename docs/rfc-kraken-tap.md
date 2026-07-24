# RFC: kraken-tap — Market Data Capture & Replay

**Status**: Draft for discussion
**Date**: 2026-07-23

## Summary

A standalone tool that captures Kraken WebSocket market data (trades + book depth) and
replays it into Krakatoa's matching engine to simulate real-time trading flow. Separate
repo, clean interface boundary with Krakatoa via ZeroMQ.

## Motivation

Krakatoa's test suite uses hard-coded orders (50000, 2 BTC). Validating against real
market data proves the engine handles production-grade load: volatile spreads, multi-level
sweeps, hundreds of trades per minute. Additionally, replaying real Kraken data into a
custom engine demonstrates domain understanding relevant to exchange infrastructure roles.

## Architecture

```
┌─────────────────────────────────────────────────┐
│                 kraken-tap                       │
│                                                  │
│  ┌──────────────┐    ┌──────────────────┐       │
│  │ Kraken WS    │───▶│ Sink (SQLite /   │       │
│  │ (trade+book) │    │  flat JSON files) │       │
│  └──────────────┘    └───────┬──────────┘       │
│                              │                   │
│  ┌───────────────────────────▼──────────────┐   │
│  │ Replay Engine                             │   │
│  │ - Timestamped or burst-mode               │   │
│  │ - Converts trades → place_limit_buy/sell  │   │
│  │ - Sends over ZeroMQ to Krakatoa engine    │   │
│  └───────────────────────────────────────────┘   │
│                                                  │
└──────────────────┬──────────────────────────────┘
                   │ ZeroMQ
┌──────────────────▼──────────────────────────────┐
│              krakatoa (existing)                  │
│  place_limit_buy/sell → crossing → OrderResult   │
└─────────────────────────────────────────────────┘
```

## Data Sources

Kraken WebSocket public API (no authentication needed):

| Channel | Endpoint | Feed |
|---|---|---|
| `trade` | `wss://ws.kraken.com/v2` | Public trades: price, qty, side, timestamp, symbol |
| `book` | `wss://ws.kraken.com/v2` | Order book snapshots + incremental updates (bid/ask changes, insert/delete/modify) |

Focus on BTC/USD (pair `XBT/USD`) initially — highest volume, richest data.

## Requirements

### Capture (REQ-CAP)

- **REQ-CAP-001**: Connect to Kraken WebSocket trade channel, capture every trade with price, qty, side, timestamp
- **REQ-CAP-002**: Periodically snapshot the order book (every 1 second, or on N trades)
- **REQ-CAP-003**: Store captured data to disk — format TBD (JSON lines, MessagePack, or SQLite)
- **REQ-CAP-004**: Support configurable capture duration (1 minute to 24+ hours)
- **REQ-CAP-005**: Handle disconnects with exponential backoff reconnection

### Replay (REQ-REP)

- **REQ-REP-001**: Read stored data and replay trades in original chronological order
- **REQ-REP-002**: Seed Krakatoa's OrderBook from a saved book snapshot before replay
- **REQ-REP-003**: Send each trade as a limit order via ZeroMQ in Krakatoa's wire format
- **REQ-REP-004**: Support burst-mode (as fast as possible, for stress testing) and real-time mode (original timestamps)
- **REQ-REP-005**: Compare Krakatoa's OrderResult against the original Kraken trade output (optional, for validation)

## Open Questions for Discussion

1. **Language**: Clojure (fits Krakatoa ecosystem, good for stream processing) or Rust (performance, self-contained binary)?
2. **Storage format**: JSON lines (human-readable, easy to inspect) vs MessagePack (compact, fast) vs SQLite (queryable, good for analysis)?
3. **Replay speed**: Burst-mode as default (stress tests engine throughput), with optional real-time mode?
4. **Book reconstruction**: Should kraken-tap maintain its own in-memory order book from incremental updates, or just snapshot periodically?
5. **Scope**: Capture tool only, or should validation (comparing Krakatoa output vs Kraken output) be in scope?

## Relationship to Krakatoa

Kraken-tap is a **separate project** — independent repo, independent build, independent deployment. The interface is ZeroMQ messages conforming to Krakatoa's existing wire format (currently `"ORDER-123,BUY,BTC,1.0,50000"` — needs a proper binary format).

Krakatoa receives orders exactly as it does today from the gateway. It doesn't know or care whether the source is a Kraken replay or a live trader.

## Job Application Angle

This project demonstrates:

- Understanding of exchange WebSocket APIs and market data feeds
- System integration across process boundaries (ZeroMQ)
- Performance considerations (burst-mode throughput testing)
- Ability to design and document technical architecture (this RFC)

Pairing it with Krakatoa's engine shows the full picture: you understand how data enters an exchange, how it's matched, and how it's reported back.
