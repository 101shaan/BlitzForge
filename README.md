# BlitzForge 🔥

**A terminal-only, high-performance password cracker demo for educational purposes.**

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## ⚠️ CRITICAL SAFETY WARNING

**THIS TOOL IS FOR DEMONSTRATION AND EDUCATIONAL PURPOSES ONLY.**

- Use ONLY on demo targets you created yourself
- Do NOT use on real accounts, third-party systems, or unauthorized data
- Unauthorized password cracking is ILLEGAL and UNETHICAL
- This tool includes ONLY demo accounts created by the operator

**By using this tool, you acknowledge you will only use it on your own demo data.**

---

## What is BlitzForge?

BlitzForge is a high-performance password cracking demonstration tool that shows:
- How different password strategies resist attacks
- The importance of password complexity
- Real-time performance metrics and hardware utilization
- Why slow hashing algorithms (like Argon2) are critical for security

Perfect for security education, computer science demonstrations, and understanding computational limits.

---

## Features

✅ **Multiple attack modes**: Dictionary, mask, brute-force, hybrid  
✅ **Multi-threaded CPU optimization**: Scales across all cores  
✅ **Live terminal UI**: Real-time hashes/sec, progress bars, hardware metrics  
✅ **Supported algorithms**: MD5, SHA1, SHA256, MD4 (NTLM)  
✅ **Benchmark logging**: CSV export for analysis  
✅ **Safe by design**: Demo-only with prominent warnings  

---

## Quick Start

### Prerequisites

- Rust (stable) - install from [rustup.rs](https://rustup.rs/)
- Linux/macOS/Windows (cross-platform)

### Build

```bash
git clone https://github.com/yourusername/blitzforge.git
cd blitzforge
cargo build --release
```

For maximum performance on your CPU:
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Run the Demo

**Step 1: Generate demo targets**
```bash
./target/release/blitzforge generate-targets \
  --out targets.json \
  --passwords demo_passwords.txt
```

**Step 2: Run the cracker**
```bash
./target/release/blitzforge run \
  --targets targets.json \
  --strategy dictionary \
  --wordlist wordlists/common-10k.txt \
  --workers 8 \
  --log results.csv
```

**Step 3: Quick demo script**
```bash
./demo_run.sh
```

---

## Usage

### Commands

#### `generate-targets`
Create demo target hashes from known passwords.

```bash
blitzforge generate-targets \
  --out targets.json \
  --passwords my_demo_passwords.txt \
  --algorithms md5,sha256
```

#### `run`
Execute cracking job with live metrics.

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
- `--strategy`: Attack mode (dictionary/mask/brute/hybrid)
- `--wordlist`: Path to dictionary file (for dictionary mode)
- `--mask`: Pattern like `?l?l?l?d?d` (lowercase letters + digits)
- `--workers`: Number of CPU threads (default: CPU count)
- `--repeat`: Run N times for benchmarking (default: 1)
- `--log`: CSV file to append results

#### `report`
Summarize benchmark results.

```bash
blitzforge report --csv bench_results.csv
```

#### `selftest`
Run internal tests and micro-benchmarks.

```bash
blitzforge selftest
```

---

## Attack Strategies

### Dictionary Attack
Tests passwords from a wordlist with optional mutations.

```bash
blitzforge run --targets targets.json --strategy dictionary --wordlist rockyou.txt
```

### Mask Attack
Pattern-based generation using character sets:
- `?l` = lowercase (a-z)
- `?u` = uppercase (A-Z)
- `?d` = digit (0-9)
- `?s` = special (!@#$...)

```bash
blitzforge run --targets targets.json --strategy mask --mask "?l?l?l?d?d"
```

### Brute Force
Exhaustive search over character set with length range.

```bash
blitzforge run --targets targets.json --strategy brute --charset "abc123" --min-len 4 --max-len 6
```

### Hybrid
Dictionary + mask rules combined.

```bash
blitzforge run --targets targets.json --strategy hybrid --wordlist common.txt --mask "?d?d"
```

---

## Terminal UI

When running, BlitzForge displays:

```
╔═══════════════════════════════════════════════════════════╗
║              BLITZFORGE - DEMO MODE ACTIVE                ║
║   ⚠️  USE ONLY ON YOUR OWN DEMO TARGETS - LEGAL USE ONLY  ║
╚═══════════════════════════════════════════════════════════╝

 PERFORMANCE METRICS
 ┌─────────────────────────────────────────────────────────┐
 │  Speed: 2,847,392 hashes/sec                            │
 │  Found: 1/3 targets                                      │
 │  Time:  00:02:34                                         │
 └─────────────────────────────────────────────────────────┘

 TARGET PROGRESS
 [alice@demo1]  ████████████████████░░░░░░░░░  67% (MD5)
 [bob@demo2]    ████████████████████████████  FOUND! ✓
 [charlie@demo3] ██░░░░░░░░░░░░░░░░░░░░░░░░░  8% (SHA256)

 HARDWARE UTILIZATION
 CPU:  [████████████████░░░░] 78%  (8 cores)
 MEM:  [████░░░░░░░░░░░░░░░░] 18%  (2.3 GB / 16 GB)

 THROUGHPUT HISTORY (last 60s)
 MH/s  ▂▃▅▆█████▇▆▅▄▃▂▂▃▄▅▆███████▇▆▅▅▄▃▃▄▅▆███
```

---

## Benchmark Results

Results are logged to CSV with columns:
- `timestamp`: ISO 8601 timestamp
- `target_id`: Target identifier
- `algorithm`: Hash algorithm used
- `strategy`: Attack strategy
- `workers`: Number of CPU threads
- `keyspace_size`: Total candidates possible
- `guesses_tried`: Candidates tested
- `time_s`: Runtime in seconds
- `hashes_per_s`: Throughput
- `found`: Success (true/false)
- `password_length`: Length of found password
- `found_in_s`: Time to crack (if found)

---

## Performance Tips

### Maximum Speed
```bash
# use native CPU instructions
RUSTFLAGS="-C target-cpu=native" cargo build --release

# set workers to core count
blitzforge run --workers $(nproc)

# use efficient algorithms (MD5 > SHA256 > SHA512)
```

### Benchmark Mode
```bash
# run 10 times and export median stats
blitzforge run --repeat 10 --log benchmark.csv
blitzforge report --csv benchmark.csv
```

---

## Demo Script for Presentations

The included `demo_run.sh` demonstrates:

1. **Quick crack** - Weak password ("password123") found in ~2 seconds
2. **Slow crack** - Strong password showing progress over 30+ seconds
3. **Live metrics** - Real-time performance visualization
4. **Hardware utilization** - CPU/memory graphs

**30-second pitch:**
> "BlitzForge is a password cracker demo showing why password strength matters. Watch as it finds 'password123' instantly, but struggles with complex passphrases. This demonstrates why you should use long, random passwords and why services should use slow hashing algorithms like Argon2."

---

## Project Structure

```
blitzforge/
├── src/
│   ├── main.rs              # cli entry point
│   ├── core/
│   │   ├── engine.rs        # cracking engine
│   │   ├── hasher.rs        # hash implementations
│   │   ├── generator.rs     # candidate generators
│   │   └── batch.rs         # batch processing
│   ├── cli/
│   │   ├── commands.rs      # subcommand handlers
│   │   ├── ui.rs            # terminal interface
│   │   └── logger.rs        # csv logging
│   └── tools/
│       └── target_gen.rs    # demo target generator
├── wordlists/
│   └── common-10k.txt       # sample wordlist
├── demo_run.sh              # quick demo script
├── targets.json             # example targets file
└── README.md
```

---

## Testing

```bash
# run all tests
cargo test

# run with output
cargo test -- --nocapture

# run selftest command
./target/release/blitzforge selftest
```

---

## Reproducible Benchmarks

To reproduce performance numbers:

1. **Hardware**: Document your CPU model, core count, RAM
2. **OS**: Linux preferred for consistent performance
3. **Build**: Use release mode with native CPU flags
4. **Warm-up**: First run warms caches; subsequent runs are faster
5. **Background**: Close other applications for clean benchmarks

Example benchmark report:
```
System: AMD Ryzen 9 5950X (16C/32T), 32GB RAM, Ubuntu 22.04
Build:  rustc 1.75.0, RUSTFLAGS="-C target-cpu=native"
MD5:    2.8 GH/s (dictionary mode, 8 workers)
SHA256: 950 MH/s (dictionary mode, 8 workers)
```

---

## Contributing

This is an educational demonstration tool. Contributions welcome for:
- Additional hash algorithm support
- Performance optimizations
- Better terminal UI
- Documentation improvements

Please maintain the educational focus and safety-first approach.

---

## License

MIT License - see LICENSE file

---

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [rayon](https://github.com/rayon-rs/rayon) - Data parallelism
- [indicatif](https://github.com/console-rs/indicatif) - Progress bars
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [md5](https://docs.rs/md5/), [sha1](https://docs.rs/sha1/), [sha2](https://docs.rs/sha2/) - Hash implementations

---

## Educational Resources

Learn more about password security:
- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- [Have I Been Pwned](https://haveibeenpwned.com/)
- [Argon2 Password Hashing](https://github.com/P-H-C/phc-winner-argon2)

**Remember: Use strong, unique passwords and enable 2FA wherever possible!**