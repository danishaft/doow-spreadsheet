# Test Corpus — Executable Correctness Specification

> The corpus defines calculation correctness. Prose does not override it.

## Purpose

Hold formula and edge-case vectors derived from Excel, LibreOffice and
HyperFormula. Every engine change must keep the corpus green; new behavior adds
vectors.

## Status

**Gate 0 seed: 84 vectors** in `vectors/gate0-fidelity.jsonl` (dates, lookups,
dynamic arrays, errors, floating point). Run against `formualizer` 0.14.0:
83 pass, 1 known divergence (Excel's 1900 leap-year quirk).

These expectations are **not oracle-verified yet** — no Excel/LibreOffice was
available at authoring time. `source: excel` means "documented Excel behaviour",
not "confirmed by a pinned build". Oracle confirmation is the next step.

## Format

JSONL, one vector per line:

```json
{
  "id": "date-001",
  "area": "date",
  "source": "excel",
  "notes": "why this vector exists",
  "cells": [{"addr": "A1", "kind": "number", "value": 1}],
  "formula": "=YEAR(DATE(2024,2,29))",
  "expected": {"kind": "number", "value": 2024},
  "tolerance": 0
}
```

- `source`: `excel` | `libreoffice` | `hyperformula` | `ieee` | `authored`.
  Every `authored` vector needs an explicit rationale in `notes`.
- `expected.kind`: `number` | `text` | `bool` | `error` | `array` | `spill`.
  `spill` means an Excel dynamic-array block anchored at the formula cell: the
  expected value is the full block, read from the anchor and neighbouring cells.
- `tolerance`: absolute numeric tolerance; `0` means bit-exact.
- `error` values use Excel spellings (`#N/A`, `#DIV/0!`, …).

## Runner

Gate 0 harness (throwaway, not product code):

```sh
cd research/corpus-probe/spike
cargo build --release --locked
./target/release/corpus_probe ../../../test-corpus/vectors/gate0-fidelity.jsonl
```

The lockfile pins `zip` 7.2.0; see `research/benchmarks/results.md` §6.

## Policy

- Discrepancies between sources are recorded, not silently resolved.
- A vector is only added with a source or an explicit authored rationale.
- A green vector means "engine agrees with the expected value", not "all of
  Excel is covered".
- Known accepted divergence: Excel's phantom 1900-02-29. If we choose Excel
  compatibility there, the vector stays red until implemented; if out of scope,
  record the exclusion here rather than deleting the vector.
