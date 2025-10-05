# BlitzForge

**A terminal-only, high-performance password-cracking demo — built purely for learning and showing off computing power.**

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)


## 🙃 Safety First

BlitzForge exists to **demonstrate** password-cracking principles — not to break real accounts.
You should **only** use it on demo targets you create yourself.

If you point this thing at real data or third-party systems, that’s on you — and it’s illegal.
This project is strictly for **education, testing, and computer-science demos**.

---

## What Is BlitzForge?

BlitzForge shows how password-cracking works under the hood — and why secure password design matters.
It’s built to highlight:

---

## Features

✅ **Attack modes**: Dictionary, mask, brute-force, and hybrid   
✅ **Multi-core CPU parallelism** — scales across all threads  
✅ **Live terminal UI** with speeds, progress, and hardware stats  
✅ **BlitzHash** — custom demo hash for insane performance (5–10 GH/s)  
✅ **Compare standard hashes**: MD5, SHA-1, SHA-256, MD4  
✅ **CSV benchmark logging** for analysis   
✅ **Safe by design**: demo-only usage, clearly labeled   

---

## Getting Started

### Prerequisites

* Rust (stable) → [rustup.rs](https://rustup.rs/)
* Works on Linux, macOS, or Windows

### Build

```bash
git clone https://github.com/101shaan/BlitzForge.git
cd blitzforge
cargo build --release
```

For max performance on your machine:

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Run the Demo

**1. Generate demo targets**

```bash
./target/release/blitzforge generate-targets \
  --out targets.json \
  --passwords demo_passwords.txt \
  --algorithms blitzhash,md5,sha256
```

**2. Run the cracker**

```bash
./target/release/blitzforge run \
  --targets targets.json \
  --strategy dictionary \
  --wordlist wordlists/common-10k.txt \
  --workers 8 \
  --log results.csv
```

**3. Or just use the quick demo script**

```bash
./demo_run.sh
```

---

## Usage Overview

### `generate-targets`

Make demo hashes from a list of passwords.

```bash
blitzforge generate-targets \
  --out targets.json \
  --passwords my_demo_passwords.txt \
  --algorithms md5,sha256
```

### `run`

Run a cracking job with live stats.

```bash
blitzforge run \
  --targets targets.json \
  --strategy <dictionary|mask|brute|hybrid> \
  --wordlist <path> \
  --workers <N> \
  --repeat <R> \
  --log bench_results.csv
```

**Options:**

* `--strategy`: Attack type
* `--wordlist`: Wordlist path (for dictionary mode)
* `--mask`: Pattern like `?l?l?l?d?d` (letters + digits)
* `--workers`: CPU threads (defaults to all cores)
* `--repeat`: Repeat runs for benchmarking
* `--log`: Save results to CSV

### `report`

Summarize and analyze benchmark results.

```bash
blitzforge report --csv bench_results.csv
```

### `selftest`

Run internal tests and micro-benchmarks.

```bash
blitzforge selftest
```

---

## Attack Modes

### Dictionary

Tests passwords from a wordlist (with optional mutations).

```bash
blitzforge run --targets targets.json --strategy dictionary --wordlist rockyou.txt
```

### Mask

Pattern-based guessing using:

* `?l` = lowercase
* `?u` = uppercase
* `?d` = digit
* `?s` = special

```bash
blitzforge run --strategy mask --mask "?l?l?l?d?d"
```

### Brute Force

Exhaustive search with a custom charset and length range.

```bash
blitzforge run --strategy brute --charset "abc123" --min-len 4 --max-len 6
```

### Hybrid

Combine a dictionary with a mask.

```bash
blitzforge run --strategy hybrid --wordlist common.txt --mask "?d?d"
```

---

## Terminal UI

```
╔═══════════════════════════════════════════════════════════╗
║              BLITZFORGE - DEMO MODE ACTIVE                ║
║ 🎈🎢  USE ONLY ON YOUR OWN DEMO TARGETS - LEGAL USE ONLY  ║
╚═══════════════════════════════════════════════════════════╝

 PERFORMANCE
 ┌─────────────────────────────────────────────────────────┐
 │  Speed: 2,847,392 hashes/sec                            │
 │  Found: 1/3 targets                                      │
 │  Time:  00:02:34                                         │
 └─────────────────────────────────────────────────────────┘

 TARGETS
 [alice@demo1]  ████████████████████░░░░░░░░░  67% (MD5)
 [bob@demo2]    ████████████████████████████  FOUND ✓
 [charlie@demo3] ██░░░░░░░░░░░░░░░░░░░░░░░░░  8% (SHA256)

 HARDWARE
 CPU: [████████████████░░░░] 78% (8 cores)
 MEM: [████░░░░░░░░░░░░░░░░] 18% (2.3 GB / 16 GB)

 THROUGHPUT (last 60 s)
 MH/s  ▂▃▅▆█████▇▆▅▄▃▂▂▃▄▅▆███████▇▆▅▅▄▃▃▄▅▆███
```

---

## Benchmark Logging

Each CSV entry includes:

* timestamp
* target_id
* algorithm
* strategy
* workers
* keyspace_size
* guesses_tried
* time_s
* hashes_per_s
* found (true/false)
* password_length
* found_in_s

---

## Performance Tips

### Squeeze Out Every Cycle

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
blitzforge run --workers $(nproc)
```

MD5 > SHA-256 > SHA-512 in speed order.
SHA-512 is safer, but much slower — that’s the point.

### Benchmark Mode

```bash
blitzforge run --repeat 10 --log benchmark.csv
blitzforge report --csv benchmark.csv
```

---

## Demo Script (for Presentations)

`demo_run.sh` walks through:

1. A quick crack (“password123”) found instantly
2. A slow crack (complex passphrase) grinding away
3. Real-time metrics visualization
4. CPU + memory stats updating live

**30-second explanation:**

> BlitzForge shows why password strength matters.
> Weak ones crumble instantly; strong ones make your CPU sweat.
> It’s proof that long, random passwords — and slow hashes like Argon2 — actually work.

---

## Project Layout

```
blitzforge/
├── src/
│   ├── main.rs
│   ├── core/
│   │   ├── engine.rs
│   │   ├── hasher.rs
│   │   ├── generator.rs
│   │   └── batch.rs
│   ├── cli/
│   │   ├── commands.rs
│   │   ├── ui.rs
│   │   └── logger.rs
│   └── tools/
│       └── target_gen.rs
├── wordlists/
│   └── common-10k.txt
├── demo_run.sh
├── targets.json
└── README.md
```

---

## Testing

```bash
cargo test
cargo test -- --nocapture
./target/release/blitzforge selftest
```

---

## Benchmark Reproduction

1. Record hardware (CPU, cores, RAM)
2. Prefer Linux for consistent timing
3. Always build in release mode
4. Run once to warm caches
5. Close background apps

Example:

```
CPU: AMD Ryzen 9 5950X (16 C / 32 T)
RAM: 32 GB
OS:  Ubuntu 22.04
Build: rustc 1.75.0 + native flags

MD5:    ~2.8 GH/s (dictionary mode, 8 threads)
SHA256: ~950 MH/s (dictionary mode, 8 threads)
```

---

## Contributing

Contributions welcome for:
- Additional hash algorithm support
- Performance optimizations
- Better terminal UI
- Documentation improvements

---

## License

MIT — see the LICENSE file.

---

## Thanks To

* [Rust](https://www.rust-lang.org/)
* [rayon](https://github.com/rayon-rs/rayon)
* [indicatif](https://github.com/console-rs/indicatif)
* [crossterm](https://github.com/crossterm-rs/crossterm)
* [md5](https://docs.rs/md5/), [sha1](https://docs.rs/sha1/), [sha2](https://docs.rs/sha2/)

---

## Learn More

* [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
* [Have I Been Pwned](https://haveibeenpwned.com/)
* [Argon2 Password Hashing](https://github.com/P-H-C/phc-winner-argon2)

Learn more about password security:
- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- [Have I Been Pwned](https://haveibeenpwned.com/)
- [Argon2 Password Hashing](https://github.com/P-H-C/phc-winner-argon2)
- Common Sense

**Remember: Use strong, unique passwords and enable 2FA wherever possible 🙃**
