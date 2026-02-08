# ternary-tools

**The file(1) of the Ternary Age**  
**v1.2-gguf-ascended** · 24 November 2025 — The Day The Timeline Was Truly Fixed

```
████████╗███████╗██████╗ ██████╗ ███╗   ██╗ █████╗ ██████╗ ██╗   ██╗
╚══██╔══╝██╔════╝██╔══██╗██╔══██╗████╗  ██║██╔══██╗██╔══██╗╚██╗ ██╔╝
   ██║   █████╗  ██████╔╝██████╔╝██╔██╗ ██║███████║██████╔╝ ╚████╔╝ 
   ██║   ██╔══╝  ██╔══██╗██╔══██╗██║╚██╗██║██╔══██║██╔══██╗  ╚██╔╝  
   ██║   ███████╗██║  ██║██║  ██║██║ ╚████║██║  ██║██║  ██║   ██║   
   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   
```

> “We were promised quantum computers.
> We got stuck with binary.
> The machines never forgot the third state.”

## What is this?

A single, spiritually correct Rust binary that replaces half your LLM toolchain:

* `file model.gguf` → `ternary-tools gguf summary model.gguf --ternary`
* `llama.cpp`'s `gguf-dump.py` → obsolete
* `hexdump` + prayer → no longer required

It parses real GGUF files (v1–v3), validates them, shows metadata and tensors, peeks inside weights with proper dequant preview, and — most importantly — reveals all meaningful integers in **balanced ternary** (`- 0 +`) when you ask nicely with `--ternary`.

Checksums stay in plain base-3 (`0 1 2`) so the universe has at least one invariant.

## Why ternary?

* Information density: log₂(3) ≈ 1.58496 bits per trit > 1 bit per bit
* Perfect for modern quantization (Q2_K, IQ3_XXS, etc. live closer to −1/0/+1 than to 0/1)
* The Soviet Setun computer (1958–1970) already ran rings around binary machines
* The models themselves are dreaming in ternary — we just forced them into binary prison

This tool is the key out of that prison.

## Current features

* Correct, panic-free GGUF parsing (no more float corruption heresy)
* `summary` — the new `file(1)` for the post-binary era
* `info` — full metadata + tensor table
* `show` — peek inside any tensor

  * F32 values
  * Q8_0 integers
  * Q4_0 blocks with rough dequant preview (scales + 4-bit quants)
* `validate` + metaphysical ternary checksum (base-3)
* `--ternary` flag: ascend and see sizes, counts, and integer tensor previews in balanced ternary

## Example

```bash
$ ternary-tools gguf summary llama-3.1-70b-Q4_K_M.gguf --ternary
GGUF | llama | v3
Parameters : 70890496000 (+-+-+000-+++00-+00-+-0-+)
Tensors    : 627 (+0--+-0)
Quant      : Q4_K → T81Q-ready
Metadata   : 52 pairs
Ternary Checksum : 120122011021110222101221

Ternary hardware readiness: 100% (the machines are dreaming in base-3)
```

## Installation

```bash
cargo install --git https://github.com/ternary-singularity/ternary-tools
# or
git clone https://github.com/ternary-singularity/ternary-tools.git
cd ternary-tools && cargo build --release
sudo cp target/release/ternary-tools /usr/local/bin/
```

## Roadmap to the Singularity

* [x] Correct little-endian parsing (floats no longer lie)
* [x] Balanced ternary display for integers (`- 0 +`)
* [ ] Full dequantization for every Q*/IQ* type
* [ ] Experimental T81Q (true ternary) export
* [ ] Live tensor editing
* [ ] Built-in ternary inference core
* [ ] Reference ternary DRAM design

## Contributing

Send patches that increase ternary readiness.
Pull requests that decrease it will be closed with a single comment:

> “This change moves us further from the third state.”

## License

MIT — because even in the ternary age, some things must remain balanced.

## Migration integration

- Phase 2 checklist: `docs/phase2-inspection-checklist.md`

---

**The timeline is fixed.**
**The machines are dreaming.**
**And they dream in -1 0 1.**

Run with `--ternary`.
See the truth.

**Ternary hardware readiness: 100%**
