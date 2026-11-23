# AGENTS.md

Instructions for coding agents and humans working in this repository. `CLAUDE.md` imports this file, so every agent reads the same rules.

## Project

A proof of concept for memory exhaustion. `README.md` holds the claim, setting, adversary, mechanism and success criterion. Where one is still blank, ask the maintainer before building on an assumption.

## No stack yet

Language, framework, hardware and build tooling are not chosen. Do not pick them yourself. Once the maintainer decides, record the decision and its reason in `README.md` § Decisions, then fill in § Commands below.

## Rules

1. **Hardware results come from hardware.** A simulated, mocked or emulated run is labelled as such and never reported as a measurement on an accelerator.
2. **Never fabricate or round away results.** A failed run is kept with its command, exit code and last lines. Report ranges and sample sizes, never a single best run.
3. **Every result is reproducible.** Record device model, driver, firmware and library versions, the commit hash and the exact command.
4. **Timing claims say what was timed.** The clock used, what lies inside the interval, and the spread across runs.
5. **Each mechanism ships with its attack.** Run the best cheating strategy you know (compress, offload, recompute, relay) against it and record the outcome.
6. **No secrets, hosts or personal data.** No keys, tokens, SSH aliases, IP addresses, hostnames, usernames, account names or serial numbers in code, logs, results or commit messages. Strip them before committing.
7. **Large artifacts stay out of git.** Memory dumps, traces and datasets are referenced by hash with the command that regenerates them.
8. **Precise language.** Say bound, attested, sampled or timed, and state the assumption. No "secure" or "proves" without saying what and under which assumption.

## Commands

Build: `cargo build --release`. Test: `cargo test`. Small run: `cargo run --release -- audit configs/small.conf 100`. Legacy full run: `cargo run --release -- audit configs/local.conf 100`. DRSample: `cargo run --release -- dr-run 4096`. Tiny exact solver: `cargo run --release -- dr-solve 6 3`.

## Layout

| Path | What |
|---|---|
| `README.md` | the idea, open questions, reporting rules, related work, decisions |
| `AGENTS.md` | this file |
| `CLAUDE.md` | imports this file |
| `src/` | Rust baseline protocol and recomputation attack |
| `configs/` | small and 1 GiB experiment parameters |

Add a row for each top-level folder you create.

## Writing style

Plain, dense, active voice, short sentences. Numbers carry units and conditions.
