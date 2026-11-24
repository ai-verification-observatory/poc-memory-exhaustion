<div align="center">

# Memory exhaustion

**Can a verifier tell that an AI accelerator's memory is full of the workload it was told about?**

[![Status: scaffold](https://img.shields.io/badge/status-scaffold-lightgrey)](#status)
[![Stack: undecided](https://img.shields.io/badge/stack-undecided-lightgrey)](#decisions)
[![Results: none yet](https://img.shields.io/badge/results-none%20yet-lightgrey)](#how-results-are-reported)
[![Licence: not chosen](https://img.shields.io/badge/licence-not%20chosen-lightgrey)](#repository)
[![AI Verification Observatory](https://img.shields.io/badge/org-AI%20Verification%20Observatory-0a3d62)](https://github.com/ai-verification-observatory)

</div>

A proof of concept for memory exhaustion: evidence that an accelerator's memory is fully occupied by a declared workload, so none is left for an undeclared one.

The idea comes from SASH's concept note. If a verifier knows how much memory a device has, and can check that the declared state fills it, then the verifier has a bound on what else can run there. This repository is where we try to turn that sentence into a mechanism, attack it, and measure it on real hardware.

## Status

Scaffold. There is no code, no result and no chosen stack. The [open questions](#open-questions) are answered by the maintainer, in this file, before any code is written.

## Contents

- [The idea](#the-idea)
- [Open questions](#open-questions)
- [How results are reported](#how-results-are-reported)
- [Where this fits](#where-this-fits)
- [Related work](#related-work)
- [Decisions](#decisions)
- [Repository](#repository)
- [Contributing](#contributing)

## The idea

SASH defines memory exhaustion as verifying that all of a device's memory is used by the authorised workload, leaving none for unauthorised workloads. The concept note names two difficulties.

**Compressibility.** The prover can hold the declared state in compressed form and spend the freed memory on something else. The check is only as strong as the state is incompressible.

**Locality.** The state has to sit close to the compute cores, not in host memory or on another device. A memory that answers correctly but slowly may be somewhere else. Timed challenges are the usual tool for telling the two apart.

A useful claim has to survive both.

## Open questions

Each answer becomes its own section of this README.

| # | Question | What the answer must say | Answer |
|---|---|---|---|
| 1 | Claim | One sentence a verifier could accept or reject. Template: *"On device D, during window T, at least X bytes of memory M held state S."* | open |
| 2 | Setting | Which memory (HBM, device DRAM, host DRAM, caches), which devices, and what fills it: verifier-chosen filler only, or filler beside a declared workload. | open |
| 3 | Adversary | What the prover controls and what it wants. Candidates: compress the state, offload it to slower memory, recompute it on demand, or answer challenges from another device. | open |
| 4 | Mechanism | How the state is generated, how the verifier challenges it, how answers are checked, and which timing assumptions it relies on. | open |
| 5 | Success criterion | The measurement that shows the PoC works, and the result that would show the approach fails. | open |
| 6 | Out of scope | What this PoC deliberately leaves out. | open |

## How results are reported

Every result states three things:

- the claim it supports and how strong it is: cryptographic, hardware-attested, statistical (sampled) or timing-based;
- the hardware, driver, firmware and library versions, the commit, and the exact command that reproduces it;
- what a dishonest prover can still do (the residual).

A memory check establishes a fact about one device during one window. We say "bound", "attested", "sampled" or "timed", and we state the assumption. We never say "provably safe".

Hardware numbers come from hardware. A simulated or emulated run is labelled as one. Failed runs are kept with their command, exit code and last lines of output, and results are reported as ranges with sample sizes, never as a best run. The full rules for contributors and coding agents are in [AGENTS.md](AGENTS.md).

## Where this fits

Memory is one of three resources a verifier can try to exhaust.

| Resource | What the verifier wants to bound | Status here |
|---|---|---|
| Memory | Bytes held by the declared state | this repository |
| Compute | FLOPs spent on the declared work | neighbouring work |
| Memory bandwidth | Bytes moved per second for the declared work | neighbouring work |

The longer-term aim is to layer all three. So the challenge interface here stays simple enough that a compute or bandwidth challenge can run beside it.

## Related work

| Work | Why it matters here |
|---|---|
| SASH, [Exploring Memory Exhaustion as a Verification Approach](https://www.aisafety.sg/blog/exploring-memory-exhaustion-as-a-verification-approach) (October 2026) | The concept, the compressibility issue, timed challenges, and a list of open questions. |
| Dziembowski, Faust, Kolmogorov, Pietrzak, [Proofs of Space](https://eprint.iacr.org/2013/796) (CRYPTO 2015) | Proving dedicated storage instead of computation, via graphs with high pebbling complexity. Written for disk space, not accelerator memory. |
| Alwen, Blocki, Harsha, [Practical Graphs for Optimal Side-Channel Resistant Memory-Hard Functions](https://eprint.iacr.org/2017/443) (CCS 2017) | Introduces DRSample and aATSample. Algorithm 1 specifies the graph sampler used in this PoC. |
| Ren, Devadas, [Bandwidth Hard Functions for ASIC Resistance](https://eprint.iacr.org/2017/225) (TCC 2017) | Introduces bandwidth hardness and uses red-blue pebbling to analyze computation and memory-transfer costs. Bandwidth hardness is distinct from capacity hardness. |
| Blocki, Liu, Ren, Zhou, [Bandwidth-Hard Functions: Reductions and Lower Bounds](https://eprint.iacr.org/2018/221) (Journal of Cryptology 2024; extended CCS 2018 version) | Relates iMHF bandwidth cost to graph red-blue pebbling cost in the parallel random-oracle model. Analyzes DRSample and aATSample under stated parameter conditions; this does not by itself establish physical memory occupancy. |
| Blocki, Harsha, Kang, Lee, Xing, Zhou, [Data-Independent Memory Hard Functions: New Attacks and Stronger Constructions](https://eprint.iacr.org/2018/944) (CRYPTO 2019) | Analyzes greedy attacks on DRSample and introduces DRSample+BRG with stronger sustained-space guarantees. Motivates testing attacks rather than inferring practical resistance from asymptotic bounds. |
| Komargodski, Weinstein, [Proofs of Useful Work from Arbitrary Matrix Multiplication](https://arxiv.org/abs/2504.09971) | The compute-side counterpart. |
| Monfared, Ganji, Holcomb, Tajik, [Timing and Memory Telemetry on GPUs for AI Governance](https://arxiv.org/abs/2602.09369) (February 2026) | Software-only, timing-based probes of a GPU, including a memory probe. |
| Cankaya, [A system overview for near-term, low-trust AI compute verification](https://intelligence.org/wp-content/uploads/2026/06/A-system-overview-for-near-term-low-trust-AI-compute-verification.pdf) (MIRI, 2026) | Section 5.1.2, as Kubů summarises it: fill server memory with incompressible noise and challenge random samples, telling local from remote memory by response time. |
| Kubů, [Proof of useful work for verifying AI treaties](https://ondrejkubu.github.io/writing/pouw-verification.html) (October 2026) | A literature review. Compares the memory probes above and lists memory-bound work as an open problem. |

Missing something? [Open an issue](https://github.com/ai-verification-observatory/poc-memory-exhaustion/issues).

## Decisions

Every choice of stack, hardware and scope lands here, with its reason.

| Date | Decision | Why |
|---|---|---|
| 2026-11-23 | Rust; 1 GiB host-RAM label state; full-reference verifier | Maintainer-selected local baseline before accelerator access. |

## Repository

Maintained by [@percept-denigrate](https://github.com/percept-denigrate). The repository lives in the [AI Verification Observatory](https://github.com/ai-verification-observatory), a neutral GitHub organisation for AI verification work shared across teams.

No licence is chosen yet. The organisation's default is Apache-2.0 for code and CC BY 4.0 for text. Coding agents read [AGENTS.md](AGENTS.md) first.

## Contributing

The most useful contributions right now are an attack on the mechanism, a reference we missed, or hardware we can run it on. [Open an issue](https://github.com/ai-verification-observatory/poc-memory-exhaustion/issues) with any of them.

## Local Rust baseline

The implemented baseline uses host RAM. It makes no adversarial occupancy claim.
Four layers contain 32-byte SHA-256 labels. Parents come from the preceding layer.
The verifier holds a full reference and compares every returned label and Merkle path.
Seeds and challenge indices come from `/dev/urandom`. Integers use little-endian u64.
Domain tags 1 through 5 distinguish parent selection, source labels, derived labels,
Merkle leaves and internal nodes. Duplicate parents are allowed.

```sh
cargo test
cargo run --release -- audit configs/small.conf 100
cargo run --release -- audit configs/small.conf 50
cargo run --release -- audit configs/local.conf 100
```

The local configuration allocates 1 GiB of prover labels. Each process additionally
stores a Merkle tree of approximately twice the label-state size. The verifier
also stores 1 GiB of reference labels. Budget approximately 6 GiB plus runtime
and temporary allocations for the full local run. It has not been run here.
The attack retains a prefix of labels and recursively recomputes missing labels.
It retains the full Merkle tree, so its percentage is not total memory retained.
The baseline therefore cannot establish a resident-byte lower bound.

Timing uses Rust Instant, from challenge dispatch through complete response receipt.
Deadlines classify completed responses; blocking I/O currently has no watchdog.
Runs on the same host do not establish memory locality. No RSS or residency
measurement is implemented. No hardware benchmark result is claimed.

Outstanding work: exact tiny-graph pebbling search, compression and backing-store
attacks, bounded read timeouts, malformed-input tests, experiment metadata and
failure-record capture, and OS residency measurements. Helper relay is untested.
A published lower-bound argument remains necessary before an occupancy claim.

## DRSample replacement status

Branch `fix/retention-bound` preserves the original baseline in commit b4e3433.
The replacement is incomplete. `src/pebble.rs` implements a sequential two-tier
game, independent trace replay and bounded exact shortest-path search for at
most 12 nodes. Red capacity includes a newly computed result alongside its
parents. Stores and loads copy labels; deletion is free; recomputation is allowed.
This convention is not asserted to match the published parallel-ROM reduction.
Tests check a chain optimum, impossible capacity, invalid loads and search limits.

### DRSample execution

The local source PDF has SHA-256
`f8e08d361118120c905550d3ac63efffe2a806d68276b070c24e7aa15d8fa8eb`.
Algorithm 1 uses one-based vertices: vertex 1 is a source, vertex 2
has parent 1. Later vertices have parent v-1 and v-r. Sample exponent
uniformly from 1 through floor(log2(v))+1; set g=min(v-1,2^exponent).
Sample integer r uniformly from max(ceil(g/2),2) through g, inclusive.
Internally vertices are zero based and parent edges point toward children.
This resolves the pseudocode's inconsistent edge orientation using its
GetParent definition and initial chain edge.

The sampler uses separate SHA-256 domains 40 and 41, a public 32-byte
seed, zero-based u64 little-endian node index and rejection counter.
Rejection sampling avoids modulo bias. It instantiates, rather than reproduces
information-theoretically independent randomness. Label domain 42 hashes
seed, node index and ordered parent-label concatenation. These 32-byte labels
are a specified instantiation, not the practical Argon2 round function.

```sh
cargo run --release -- dr-run 4096
cargo run --release -- dr-solve 6 3
# Explicit full-label allocation: 1 GiB plus overhead.
cargo run --release -- dr-run 33554432
```

`dr-run` computes the reference before spawning a fresh seeded worker.
The reported interval includes process launch, graph generation and evaluation,
output receipt and process exit. The reference allocation is released before
worker launch. It then compares a sequential last-use greedy evaluator against
the reference. Peak live-label bytes exclude HashMap allocation, keys, allocator
capacity and node-index metadata; they are not measured resident memory.
This greedy policy is not yet the full parallel greedy attack from the 2019 paper.
`dr-solve` uses a fixed all-zero seed and costs compute=1, transfer=10;
search stops after 100,000 expanded states and reports incomplete explicitly.

Six tests currently cover SHA-256 abc, baseline reconstruction, graph parent
boundaries, full/greedy agreement, trace replay, tiny optimum and capacity limits.
The large budgeted executor, published parallel attack, compression/offload,
timeouts, independent sampler oracle, complete SHA vectors, reproducibility
capture, repetitions remain pending. The existing `audit` command
still runs the weak baseline. No physical occupancy or bandwidth lower bound
is claimed from these runs.

## Measured 1 GiB DRSample experiment

Local host-RAM measurements, three distinct deterministic seeds per policy,
33,554,432 nodes and 32-byte labels. All six workers exited 0. Each paired
final label matched. These are execution measurements, not an occupancy bound.

| Policy | n | Peak RSS range (KiB) | Elapsed range (s) |
|---|---:|---:|---:|
| Honest full retention | 3 | 1,050,092–1,050,336 | 41.16–41.25 |
| Sequential last-use greedy | 3 | 516,468–516,648 | 72.64–76.21 |

GNU time peak RSS includes resident metadata, allocator storage, stack and
runtime; 1 KiB = 1,024 bytes. It excludes nonresident allocations. This is not
a separate attribution of allocator overhead. The greedy live-label peak was
50,711,840–50,755,008 bytes, its index array allocation was 268,435,456 bytes,
and its peak hash-table entry capacity was 3,670,016. Entry capacity excludes
implementation-specific control bytes and allocator bookkeeping. Observed total
RSS is substantially larger than live-label bytes alone.

Elapsed intervals cover process startup through worker exit, including graph
sampling and evaluation; compilation and correctness comparisons are outside.
Workers ran sequentially. GNU time also captured CPU times and page faults.
The runner sampled selected /proc memory counters every 100 ms; samples are
not used as an exact peak measurement. No cgroup memory limits were readable
at the standard cgroup-root paths. No firmware or accelerator libraries were
queried: this is a CPU/RAM experiment, not an accelerator measurement.

Reproduce with:

```sh
cargo test
cargo build --release
python3 scripts/measure.py
```

The runner currently discovers GNU time and rustc from the local Nix store.
The first smoke-run attempt exited 1 because rustc was absent from PATH; its
last error was `FileNotFoundError: [Errno 2] No such file or directory: rustc`.
After resolving the compiler path, the smoke test and all full trials passed.

Measured base commit: `25c50a0ce932629b70dbeb0e3d28c284fe27cc71`; working tree was modified.
`results/memory/source-hashes.json` identifies measured source and binary.
Raw artifacts stay ignored under `results/memory/`; deterministic seeds, exact
worker commands, statuses, stderr tails and GNU time outputs are in runs.json.
The metadata records compiler, kernel, CPU and RAM without machine identity.

Environment: {"rustc": "rustc 1.95.0 (59807616e 2026-04-14)", "kernel": "6.18.57", "cpu": "AMD Ryzen 5 7535HS with Radeon Graphics", "ram": "MemTotal:       15660052 kB"}

Artifact SHA-256 hashes (regenerate with the commands above):

- `0-dr-greedy-worker.samples.json`: `0ad8ec51f73014bc53ffd9a5b7463d226d5fee92f645e3cd4acbc3b4319ab093`
- `0-dr-greedy-worker.stderr`: `18ddd02cfbb523c502fe4a53a62742cf4d64180fad236154f468395535123d4f`
- `0-dr-greedy-worker.stdout`: `acf8704acd9b080af5e888cf24b3d9b4b3f28fa673d33540b1b7ffdf19d6ab3e`
- `0-dr-greedy-worker.time`: `71f0fb4254d85e874dc23d4491bf7df87ba5813914bf9591d358d7e1efd7246a`
- `0-dr-worker.samples.json`: `7865d807cae7fac491e7d0dc9b5ccd928cf029649d460fa358a3ea79c7deafb7`
- `0-dr-worker.stderr`: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
- `0-dr-worker.stdout`: `acf8704acd9b080af5e888cf24b3d9b4b3f28fa673d33540b1b7ffdf19d6ab3e`
- `0-dr-worker.time`: `a0ffdfd679e518219271dcda9a9b6e3ad3f0416fbcffda2368711bcd6f347f05`
- `1-dr-greedy-worker.samples.json`: `4f3a55f89f0fdc5856e2d20d8ec6247ae0060d51e2ae7a5d7f7737db4dfb339c`
- `1-dr-greedy-worker.stderr`: `fe8d21e23c0f344484115b0ccb4ccc0c833549d077660f9a093b1cf3d7faa7ff`
- `1-dr-greedy-worker.stdout`: `2968da1653e3b8978d10d0fe5f89ff7d8226db6b4f1b82289323059af023ccc0`
- `1-dr-greedy-worker.time`: `765b3a39e3339508bf3f3e51bd209c4c5cd7a6ec21039f6e1a62651394598810`
- `1-dr-worker.samples.json`: `9d7bdd6335246d011edab02904a5b218f184fe24ba854118a904accdd68b065d`
- `1-dr-worker.stderr`: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
- `1-dr-worker.stdout`: `2968da1653e3b8978d10d0fe5f89ff7d8226db6b4f1b82289323059af023ccc0`
- `1-dr-worker.time`: `8838ba2f4a3985e1e9fc5afd713062179f91259d54cdad933cb3216904525dd4`
- `2-dr-greedy-worker.samples.json`: `e57163f645b81422f5a2fa30fcb06527566021021dad56e71260573abfe7375c`
- `2-dr-greedy-worker.stderr`: `aefe1f9426353bd421c1aecfc011480b0f4fb4637a95b58dcc2f7e9d07ce3339`
- `2-dr-greedy-worker.stdout`: `923de00303db6da80cb8c7a0f3a78a6a41fbf65d60796ae8f01c96980eeef702`
- `2-dr-greedy-worker.time`: `c25b8baf137dc21da4b454f0668576e5ae0cfd201b04cf876402e76ef6c68fea`
- `2-dr-worker.samples.json`: `69476e5bb3940e98bb12e493a8aacc9394ea9cab6f3734d8d2e5da16f940f691`
- `2-dr-worker.stderr`: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
- `2-dr-worker.stdout`: `923de00303db6da80cb8c7a0f3a78a6a41fbf65d60796ae8f01c96980eeef702`
- `2-dr-worker.time`: `ad77f9558d16bdf202dcaf408152750f7b290686e6b2a0f5c09a5f313505d7dc`
- `limits.json`: `ee9fdac6985ec9b0e3864abea078143c222e72d315157ec50b6a246b2f6e8b82`
- `metadata.json`: `2e396a063aa4242ffa8636bd4efb41ae682f1f779859ea77da34f6fd91bbc7ef`
- `runs.json`: `36ea65538b8b7d2a5a54b49c603f2cec1d88aae8ef212d9261d36a4013048995`
- `source-hashes.json`: `313c46b13753886009bf0f90af565839a252b9095a1c246f1d2c109f794f364a`
