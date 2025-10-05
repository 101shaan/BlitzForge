# BlitzForge Setup Guide

Complete setup instructions for your scholars showcase demonstration.

---

## Prerequisites

### 1. Install Rust

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts, then:
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 2. Clone/Create Project

```bash
# Create project directory
mkdir blitzforge
cd blitzforge

# Initialize git (optional)
git init
```

---

## Project Structure

Create the following directory structure:

```
blitzforge/
├── Cargo.toml
├── README.md
├── presentation.md
├── technical_doc.md
├── SETUP_GUIDE.md
├── demo_run.sh
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── engine.rs
│   │   ├── hasher.rs
│   │   ├── generator.rs
│   │   └── target.rs
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── commands.rs
│   │   ├── ui.rs
│   │   └── logger.rs
│   └── tools/
│       └── mod.rs
└── wordlists/
    └── common-1000.txt
```

---

## Build Instructions

### Standard Build

```bash
# Development build (slower, with debug info)
cargo build

# Release build (optimized)
cargo build --release
```

### Maximum Performance Build

```bash
# Use native CPU instructions for best performance
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Binary will be at: ./target/release/blitzforge
```

---

## Quick Start Demo

### 1. Run Self-Test

```bash
./target/release/blitzforge selftest
```

Expected output:
```
🧪 Running BlitzForge self-tests...

Test 1: Hash Algorithms
   MD5: OK
   SHA1: OK
   SHA256: OK
   ✅ All hash algorithms working

Test 2: Candidate Generators
   Mask generator: OK
   Brute force generator: OK
   ✅ All generators working

Test 3: Simple Crack
   ✅ Engine successfully cracked test password

✅ All self-tests passed!
```

### 2. Create Demo Data

```bash
# Create demo passwords file
cat > demo_passwords.txt << EOF
password123
secret
admin
letmein
qwerty
EOF

# Generate targets with BlitzHash
./target/release/blitzforge generate-targets \
  --out targets.json \
  --passwords demo_passwords.txt \
  --algorithms blitzhash,md5,sha256
```

### 3. Create Wordlist

```bash
# Create a small wordlist for demo
cat > wordlist.txt << EOF
password
password123
123456
admin
secret
letmein
welcome
qwerty
monkey
dragon
EOF
```

### 4. Run Demo

```bash
./target/release/blitzforge run \
  --targets targets.json \
  --strategy dictionary \
  --wordlist wordlist.txt \
  --workers 8 \
  --log results.csv
```

### 5. View Results

```bash
./target/release/blitzforge report --csv results.csv
```

---

## Automated Demo Script

Make the demo script executable:

```bash
chmod +x demo_run.sh
./demo_run.sh
```

This will:
1. Build BlitzForge in release mode
2. Create demo passwords and wordlist
3. Generate target hashes
4. Run cracking demo
5. Save results to CSV

---

## Scholars Showcase Setup

### Day Before Presentation

1. **Build and test everything**
   ```bash
   cargo clean
   RUSTFLAGS="-C target-cpu=native" cargo build --release
   ./target/release/blitzforge selftest
   ./demo_run.sh
   ```

2. **Print presentation materials**
   - Print `presentation.md` on A4 paper
   - Print `technical_doc.md` for technical judges
   - Bring copies of `README.md` as handouts

3. **Prepare laptop**
   - Full charge
   - Close unnecessary applications
   - Terminal with large font size (16-18pt)
   - Dark theme for better visibility
   - Pre-run demo once to warm up caches

### Live Demo Script (30 seconds)

**Opening:**
> "This is BlitzForge - a password cracker demo showing why strong passwords matter. I created this in Rust to demonstrate computational security."

**Run the demo:**
```bash
./demo_run.sh
```

**While it runs, explain:**
> "Watch the speed counter - it's testing millions of passwords per second. Common passwords like 'password123' are found instantly. This is why you should use long, random passwords and why services use slow hashing algorithms."

**Point out live metrics:**
- Hashes per second counter
- Progress bars for each target
- Found passwords appearing

**Closing:**
> "All tests here are on demo accounts I created - never use these tools on real systems. Questions?"

### Troubleshooting

**If build fails:**
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

**If demo is too slow:**
```bash
# Reduce workers
./target/release/blitzforge run \
  --workers 4 \
  --batch-size 2048 \
  [other args]
```

**If demo is too fast:**
```bash
# Use larger wordlist or harder algorithm
./target/release/blitzforge run \
  --strategy brute \
  --charset "abcdefghijklmnopqrstuvwxyz" \
  --min-len 6 \
  --max-len 8 \
  [other args]
```

---

## Performance Tips

### Maximize Speed

1. **Build with optimizations:**
   ```bash
   RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release
   ```

2. **Use all CPU cores:**
   ```bash
   # Linux/macOS: use all cores
   ./target/release/blitzforge run --workers $(nproc)
   
   # Or specify manually
   ./target/release/blitzforge run --workers 16
   ```

3. **Tune batch size:**
   - MD5/SHA1: Use 8192
   - SHA256: Use 4096
   - Smaller batches if memory constrained

4. **Choose fast algorithms:**
   - MD5 is ~3x faster than SHA256
   - Use MD5 for maximum speed demo

### Create Impressive Demos

**Quick Win Demo:**
```bash
# Cracks password123 in <1 second
./target/release/blitzforge run \
  --targets targets.json \
  --strategy dictionary \
  --wordlist common-1000.txt \
  --workers 8
```

**Long Duration Demo:**
```bash
# Shows progress over 30+ seconds
./target/release/blitzforge run \
  --targets targets.json \
  --strategy brute \
  --charset "abcdefghijklmnopqrstuvwxyz0123456789" \
  --min-len 6 \
  --max-len 7 \
  --workers 8
```

---

## Testing Before Showcase

### Run Full Test Suite

```bash
# Unit tests
cargo test

# Self-test
./target/release/blitzforge selftest

# Full demo run
./demo_run.sh

# Verify results
./target/release/blitzforge report --csv demo_results.csv
```

### Expected Performance (Reference)

On a modern 8-core CPU (e.g., Ryzen 5 5600X):
- **MD5**: 800 MH/s - 2 GH/s
- **SHA1**: 400 MH/s - 1 GH/s
- **SHA256**: 150 MH/s - 500 MH/s

Your results may vary based on:
- CPU model and generation
- Core count and frequency
- Background processes
- Thermal throttling

### Benchmark Your System

```bash
# Run 5 times and get median
./target/release/blitzforge run \
  --targets targets.json \
  --strategy dictionary \
  --wordlist wordlist.txt \
  --workers 8 \
  --repeat 5 \
  --log benchmark.csv

# View statistics
./target/release/blitzforge report --csv benchmark.csv
```

---

## Common Issues & Solutions

### Issue: Slow compilation

**Solution:** Use fewer optimizations during development
```bash
cargo build  # Instead of cargo build --release
```

### Issue: "Cannot find wordlist"

**Solution:** Use absolute path
```bash
./target/release/blitzforge run \
  --wordlist $(pwd)/wordlist.txt \
  [other args]
```

### Issue: No matches found

**Solution:** Ensure wordlist contains the demo passwords
```bash
# Check targets
cat targets.json | grep hash_hex

# Check wordlist
cat wordlist.txt

# Make sure passwords match
```

### Issue: Terminal UI looks broken

**Solution:** Update terminal or disable fancy display
```bash
# Use simpler output (if needed - requires code modification)
# Or use a modern terminal: kitty, alacritty, or Windows Terminal
```

---

## Customization Options

### Create Custom Targets

```bash
# Create your own passwords file
echo "MySecretPass2024" > my_passwords.txt
echo "another!password" >> my_passwords.txt

# Generate targets
./target/release/blitzforge generate-targets \
  --out my_targets.json \
  --passwords my_passwords.txt \
  --algorithms md5,sha1,sha256
```

### Use Different Strategies

**Mask attack:**
```bash
./target/release/blitzforge run \
  --targets targets.json \
  --strategy mask \
  --mask "?l?l?l?d?d?d" \
  --workers 8
```

**Brute force:**
```bash
./target/release/blitzforge run \
  --targets targets.json \
  --strategy brute \
  --charset "abc123" \
  --min-len 4 \
  --max-len 6 \
  --workers 8
```

---

## Presentation Day Checklist

### Equipment
- [ ] Laptop fully charged
- [ ] Power adapter (backup)
- [ ] HDMI/display adapter (if presenting on screen)
- [ ] Printed presentation.md (A4)
- [ ] Printed technical_doc.md
- [ ] README handouts (optional)

### Software
- [ ] BlitzForge built in release mode
- [ ] Self-test passes
- [ ] Demo script works end-to-end
- [ ] Terminal font size increased
- [ ] Dark theme enabled
- [ ] All files in project directory

### Preparation
- [ ] Run demo at least 3 times
- [ ] Time the demo (should be 30-45 seconds)
- [ ] Prepare answers to common questions
- [ ] Review safety/legal disclaimers
- [ ] Practice explaining password security

### Common Questions to Prepare For

1. **"How fast is it?"**
   - Mention your specific numbers (MH/s or GH/s)
   - Compare to GPU speeds (10-100x faster)

2. **"Is this legal?"**
   - Only on your own demo data
   - Emphasize educational purpose
   - Mention laws (Computer Misuse Act)

3. **"Can you crack my password?"**
   - Politely decline
   - Suggest online password strength checkers instead

4. **"Why Rust?"**
   - Memory safety without garbage collection
   - C-like performance with modern features
   - Great for systems programming

5. **"How does it work?"**
   - Hash the candidate password
   - Compare to target hash
   - Millions per second across CPU cores

---

## Post-Showcase

### Save Your Demo Results

```bash
# Archive all results
mkdir showcase_results
cp demo_results.csv showcase_results/
cp targets.json showcase_results/
cp demo_passwords.txt showcase_results/
cp wordlist.txt showcase_results/

# Add timestamp
tar czf showcase_results_$(date +%Y%m%d).tar.gz showcase_results/
```

### Share Your Project

```bash
# Push to GitHub
git add .
git commit -m "BlitzForge - Scholars Showcase 2025"
git push origin main
```

### Document Feedback

Create `FEEDBACK.md` with:
- Questions people asked
- Suggestions for improvement
- Performance observations
- What worked well / what didn't

---

## Next Steps

### Potential Enhancements

1. **GPU Acceleration**
   - Integrate with hashcat
   - OpenCL kernel implementation

2. **More Algorithms**
   - Bcrypt, Argon2 (slow hashes)
   - PBKDF2, scrypt

3. **Better UI**
   - Real-time graphs
   - Hardware monitoring
   - Better progress estimation

4. **Hybrid Strategies**
   - Dictionary + mask combinations
   - Markov chain generation
   - Statistical password models

5. **Distributed Mode**
   - Network protocol for cluster cracking
   - Work distribution across machines

---

## Support & Resources

### Learning Resources
- Rust Book: https://doc.rust-lang.org/book/
- Cryptography basics: https://cryptopals.com/
- Password security: https://cheatsheetseries.owasp.org/

### Community
- Rust Discord: https://discord.gg/rust-lang
- r/rust subreddit
- Rust Users Forum: https://users.rust-lang.org/

### Security Resources
- OWASP Password Guidelines
- Have I Been Pwned
- National Cyber Security Centre (UK)

---

## Good Luck!

You've built something impressive. Remember:
- Stay calm during the demo
- Focus on the educational message
- Have fun showing off your work!

**You've got this! 🔥**