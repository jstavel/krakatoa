# Vision: The Full Loop

**Created**: 2026-07-23

## Context

While implementing the Rust matching engine (Milestone 2) and discussing next steps for
Krakatoa, the conversation surfaced a compelling vision: the project isn't just a matching
engine — it's a **trade tape analysis platform** in the spirit of Richard Wyckoff.

## The Wyckoff Connection

Richard Wyckoff's *Studies in Tape Reading* (1910) describes reading the stream of
individual trade prints off the ticker tape to infer supply/demand dynamics — accumulation
and distribution by the "composite operator." His schematic drawings of price action at
volume remain influential over a century later.

Krakatoa already generates the raw material Wyckoff was reading:

- **Trades**: chronological prints with price, quantity, and taker side (buy-initiated vs
  sell-initiated)
- **Order book depth**: bids and asks at every price level
- **Volume per level**: aggregated quantities consumed during crossing

This is fascinating because we're not just building a matching engine — we're building a
**tape generation machine** that mirrors what ran through ticker machines on the floor of
the New York Stock Exchange in 1910.

## The Full Loop

```
┌─────────────────────────────────────────────────────┐
│ 1. ENGINE (Rust)                                    │
│    Generates Trade records + order book snapshots   │
│    Zero-allocation crossing loop                    │
└──────────────────────┬──────────────────────────────┘
                       │ ZeroMQ
┌──────────────────────▼──────────────────────────────┐
│ 2. GATEWAY (Clojure — Polylith)                     │
│    Validates orders via Malli schemas               │
│    Routes trades to Kafka for event sourcing        │
│    Enriches trades with analytics context           │
└──────────────────────┬──────────────────────────────┘
                       │ WebSocket / SSE
┌──────────────────────▼──────────────────────────────┐
│ 3. ANALYSIS (Clojure)                               │
│    Volume per price level reconstruction            │
│    Absorption / distribution pattern detection       │
│    Tape speed / activity metrics                    │
│    Wyckoff schematic recognition                    │
└──────────────────────┬──────────────────────────────┘
                       │ re-frame events
┌──────────────────────▼──────────────────────────────┐
│ 4. UI (ClojureScript — re-frame SPA)                │
│    Live order book (bids descending, asks ascending) │
│    Trade tape stream (chronological prints)          │
│    Volume profile visualization                     │
│    Order submission panel                           │
└─────────────────────────────────────────────────────┘
```

## What Exists Today

| Layer | Status |
|---|---|
| Rust engine — limit buy/sell + crossing | ✅ Done (26 tests) |
| Rust engine — ZMQ integration | ❌ `main.rs` echoes ACK, never calls OrderBook |
| Clojure gateway — ZMQ transport | ✅ Done |
| Clojure gateway — Kafka client | ✅ Done |
| Clojure gateway — order validation | ❌ `order-book-vld` skeleton only |
| Clojure gateway — chaos orchestration | ❌ `chaos-orchestra` skeleton only |
| Analysis pipeline | ❌ Not started |
| re-frame UI | ❌ Not started |

## Suggested Next Steps

1. **`order-book-vld`** (Clojure) — Malli validation of orders before ZMQ send. Closes
   the validation gap in the gateway.
2. **ZMQ ↔ OrderBook integration** (Rust) — Engine processes real payloads from the
   gateway. Makes the full stack operational.
3. **re-frame SPA** (ClojureScript) — Live order book + trade tape UI. The visual layer
   that turns engine output into something you can watch.

## Why This Matters (Portfolio Narrative)

Most matching engine projects stop at "it matches orders." Krakatoa's differentiator is
the full loop:

- **Zero-allocation Rust engine** — systems-level performance work
- **Polylith Clojure gateway** — functional, REPL-driven architecture
- **Wyckoff-inspired analysis** — domain depth, not just plumbing
- **Re-frame data flow** — modern ClojureScript, real-time rendering

The project demonstrates polyglot boundary design, event sourcing, lock-free routing,
and — uniquely — historical trading theory implemented in code.
