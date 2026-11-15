# Univer OSS — Head-to-Head Benchmark (Gate 0)

> Observations only. Decisions go to `ADRs/`.
> Run: 2026-11-14 · Univer OSS 1.4.0 · Node 26.13.0 · host: i5-8257U, 8 GB, macOS 15.7.7
> Harness: `research/benchmarks/univer-oss/` · Engine: TypeScript (OSS), not the Pro Rust engine

## 1. What was measured

Same workload shape as `research/benchmarks/results.md`:

- Column A: values `1..n`.
- Column B: `=A{i}*2` (copied) or `=A{i}*$E$1` (broad), n formulas.
- C1: `=SUM(B1:Bn)`.
- Measure: set values, set formulas, first correct calculation, then edit A1
  (copied) or E1 (broad) and measure edit → correct C1.

Univer OSS was driven through its headless Facade API
(`createWorkbook` → `getRange().setValues/setFormulas` → poll `getValue`).
Calculation is asynchronous; the poll interval was 25 ms and is included in the
times.

## 2. Results — n = 100,000

| Case | Set values | Set formulas | First calc | Edit + recalc | Correct |
| ---- | ---------- | ------------ | ---------- | ------------- | ------- |
| copied | 993.4 ms | 1,415.1 ms | **11,099.8 ms** | **4,929.6 ms** | yes |
| broad | 962.5 ms | 1,290.5 ms | **12,660.9 ms** | **did not complete in ~10 min** (killed) | initial yes |

For comparison, Formualizer 0.14.0 on the same host (`results.md`):

| Case | Build (values+formulas) | First calc | Edit + recalc |
| ---- | ----------------------- | ---------- | ------------- |
| copied | 307–323 ms | 239–263 ms | 1.6–2.3 ms (narrow) |
| broad | 272–278 ms (values+formulas+param) | 258–270 ms | **23–27 ms** |

Approximate ratios on this workload:

- First calculation: **~44× slower** in Univer OSS.
- Narrow incremental recalc: **~2,000–3,000× slower** (4.93 s vs ~2 ms).
- Broad parameter edit: **>24,000× slower** — Univer did not finish 100k in 10
  minutes; Formualizer finished in ~25 ms.
- Setup: ~2.4 s vs ~0.31 s (~8×).

## 3. Interpretation — and its limits

- The OSS Univer formula engine is a **TypeScript engine**, and Univer's own FAQ
  says it is slower than their commercial one. These numbers are consistent with
  that: on headless batch recalculation the gap is one to four orders of
  magnitude.
- The broad edit is the important failure: changing one parameter read by all
  100k formulas did not complete in 10 minutes. We did not profile the cause, so
  do not attribute a mechanism; record the behaviour.
- Correctness passed in every completed case (SUM matched the closed form).
- **Do not generalise this to Univer's Pro Rust engine.** `@univerjs-pro/engine-formula-rust`
  is a separate, licensed package and was not installed or tested. Any claim
  about Univer's commercial-tier performance is out of scope here.
- This is a **workload comparison, not an engine microbenchmark**. The APIs
  differ (Univer's transactional command model vs direct engine calls), and
  browser vs headless Node may differ.
- One run per case; no statistical treatment. 1M was not attempted because the
  100k broad edit already failed to converge and 1M would add memory/time risk
  without new information.

## 4. What it supports

The Rust-first thesis has real evidence on headless/batch workloads: an
order-of-magnitude performance wedge over the OSS baseline, and a broad-edit case
that the other engine could not complete. It does **not** prove the product wins:
Univer's UX, collaboration and ecosystem remain ahead, and the Pro engine is
unmeasured.

## 5. Reproduce

```sh
cd research/benchmarks/univer-oss
npm ci
node bench.mjs 100000 copied
node bench.mjs 100000 broad   # expect the initial calc, then a long/stalled edit
```

The harness raises the worksheet `rowCount` from Univer's default 1000; the
default workbook otherwise rejects 100k ranges. `node_modules` is not committed.

## 6. Next

- Run the 84-vector fidelity corpus against Univer OSS (fidelity comparison).
- SDK/API comparison: install time, headless API surface, licence gates.
- Price the Pro Rust engine; if affordable, re-run this benchmark on it before
  making any public performance claim.
