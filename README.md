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
| Komargodski, Weinstein, [Proofs of Useful Work from Arbitrary Matrix Multiplication](https://arxiv.org/abs/2504.09971) | The compute-side counterpart. |
| Monfared, Ganji, Holcomb, Tajik, [Timing and Memory Telemetry on GPUs for AI Governance](https://arxiv.org/abs/2602.09369) (February 2026) | Software-only, timing-based probes of a GPU, including a memory probe. |
| Cankaya, [A system overview for near-term, low-trust AI compute verification](https://intelligence.org/wp-content/uploads/2026/06/A-system-overview-for-near-term-low-trust-AI-compute-verification.pdf) (MIRI, 2026) | Section 5.1.2, as Kubů summarises it: fill server memory with incompressible noise and challenge random samples, telling local from remote memory by response time. |
| Kubů, [Proof of useful work for verifying AI treaties](https://ondrejkubu.github.io/writing/pouw-verification.html) (October 2026) | A literature review. Compares the memory probes above and lists memory-bound work as an open problem. |

Missing something? [Open an issue](https://github.com/ai-verification-observatory/poc-memory-exhaustion/issues).

## Decisions

Every choice of stack, hardware and scope lands here, with its reason.

| Date | Decision | Why |
|---|---|---|
| | | |

## Repository

Maintained by [@percept-denigrate](https://github.com/percept-denigrate). The repository lives in the [AI Verification Observatory](https://github.com/ai-verification-observatory), a neutral GitHub organisation for AI verification work shared across teams.

No licence is chosen yet. The organisation's default is Apache-2.0 for code and CC BY 4.0 for text. Coding agents read [AGENTS.md](AGENTS.md) first.

## Contributing

The most useful contributions right now are an attack on the mechanism, a reference we missed, or hardware we can run it on. [Open an issue](https://github.com/ai-verification-observatory/poc-memory-exhaustion/issues) with any of them.
