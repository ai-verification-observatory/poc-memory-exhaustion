# poc-memory-exhaustion

A proof of concept for memory exhaustion: evidence that an AI accelerator's memory is fully occupied by a declared workload, so none is left for an undeclared one.

**Status:** scaffold. No code, no results, no stack chosen. The open questions below are answered by the maintainer, in this file, before code is written.

## The idea

SASH's concept note defines memory exhaustion as verifying that all of a device's memory is used by the authorised workload, leaving none for unauthorised workloads. A verifier who knows how much memory a device has, and can check that the declared state fills it, bounds what else can run there.

Two difficulties, both named in the concept note:

- **Compressibility.** The prover may hold the declared state in compressed form and use the freed memory for something else.
- **Locality.** The state must be shown to sit close to the compute cores, not in host memory or on another device. Timed challenges are the usual tool.

## Open questions

Each answer becomes a section of this README.

1. **Claim.** One sentence a verifier could accept or reject. Template: "On device D, during window T, at least X bytes of memory M held state S."
2. **Setting.** Which memory (HBM, device DRAM, host DRAM, caches), which devices, and what fills it: verifier-chosen filler only, or filler beside a declared workload.
3. **Adversary.** What the prover controls and what it wants. Candidates: compress the state, offload it to slower memory, recompute it on demand, or answer challenges from another device.
4. **Mechanism.** How the state is generated, how the verifier challenges it, how answers are checked, and which timing assumptions it relies on.
5. **Success criterion.** The measurement that shows the PoC works, and the result that would show the approach fails.
6. **Out of scope.** What this PoC deliberately leaves out.

## How results are reported

Every result states:

- the claim it supports and its strength: cryptographic, hardware-attested, statistical (sampled) or timing-based;
- the hardware, driver, firmware and library versions, the commit, and the exact command to reproduce it;
- what a dishonest prover can still do (the residual).

A memory check establishes a fact about a device during a window. Say "bound", "attested", "sampled" or "timed"; never "provably safe".

## Neighbouring work

Memory is one of three resources a verifier can try to exhaust, with compute (FLOPs) and memory bandwidth. The longer-term aim is to layer the three, so keep the challenge interface simple enough for a compute or bandwidth challenge to run beside it.

## Related work

- SASH, [Exploring Memory Exhaustion as a Verification Approach](https://www.aisafety.sg/blog/exploring-memory-exhaustion-as-a-verification-approach) (October 2026). The concept, the compressibility issue, timed challenges, and a list of open questions.
- Dziembowski, Faust, Kolmogorov, Pietrzak, [Proofs of Space](https://eprint.iacr.org/2013/796) (CRYPTO 2015). Proving dedicated storage instead of computation, via graphs with high pebbling complexity. Written for disk space, not accelerator memory.
- Komargodski, Weinstein, [Proofs of Useful Work from Arbitrary Matrix Multiplication](https://arxiv.org/abs/2504.09971). The compute-side counterpart.
- Monfared, Ganji, Holcomb, Tajik, [Timing and Memory Telemetry on GPUs for AI Governance](https://arxiv.org/abs/2602.09369) (February 2026). Software-only, timing-based probes of a GPU, including a memory probe.
- Cankaya, [A system overview for near-term, low-trust AI compute verification](https://intelligence.org/wp-content/uploads/2026/06/A-system-overview-for-near-term-low-trust-AI-compute-verification.pdf) (MIRI, 2026). Section 5.1.2, as Kubů summarises it: fill server memory with incompressible noise and challenge random samples, telling local from remote memory by response time.
- Kubů, [Proof of useful work for verifying AI treaties](https://ondrejkubu.github.io/writing/pouw-verification.html) (October 2026). A literature review; compares the memory probes above and lists memory-bound work as an open problem.

## Decisions

| Date | Decision | Why |
|---|---|---|
| | | |

Record the stack, the hardware and every scope change here.

## Repository

- Maintainer: [@percept-denigrate](https://github.com/percept-denigrate).
- Home: the [AI Verification Observatory](https://github.com/ai-verification-observatory) GitHub organisation, a neutral place for shared AI verification work across teams.
- Licence: not chosen yet. The organisation's default is Apache-2.0 for code and CC BY 4.0 for text.
- Coding agents: read [AGENTS.md](AGENTS.md).

## Contributing

Open an issue: an attack on the mechanism, a reference we missed, or hardware you can run it on.
