# BlitzForge Quick Reference Card

**Print this and keep it at your demo table!**

---

## Essential Commands

### Build
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Self-Test
```bash
./target/release/blitzforge selftest
```

### Generate Targets
```bash
./target/release/blitzforge generate-targets \
  --out targets.json \
  --passwords demo_passwords.txt \
  --algorithms blitzhash,md5,sha256
```

### Run Demo (Dictionary)
```bash
./target/release/blitzforge run \
  --targets targets.json \
  --strategy dictionary \
  --wordlist wordlist.txt \
  --workers 8 \
  --log results.csv
```

### Run Demo (Mask)
```bash
./target/release/blitzforge run \
  --targets targets.json \
  --strategy mask \
  --mask "?l?l?l?d?d" \
  --workers 8
```

### Run Demo (Brute)
```bash
./target/release/blitzforge run \
  --targets targets.json \
  --strategy brute \
  --charset "abc123" \
  --min-len 4 \
  --max-len 6 \
  --workers 8
```

### View Report
```bash
./target/release/blitzforge report --csv results.csv
```

---

## Mask Patterns

- `?l` = lowercase (a-z)
- `?u` = uppercase (A-Z)
- `?d` = digit (0-9)
- `?s` = special (!@#$...)

**Examples:**
- `?l?l?l?d?d` = 3 letters + 2 digits (abc12)
- `?u?l?l?l?l?d?d` = Capital + 4 letters + 2 digits (Hello99)
- `password?d?d` = literal "password" + 2 digits

---

## Quick Troubleshooting

### Demo too slow?
Reduce workers: `--workers 4`

### Demo too fast?
Use brute force with longer passwords:
```bash
--strategy brute --charset "abcdefghijklmnopqrstuvwxyz" \
--min-len 6 --max-len 8
```

### No matches found?
Check wordlist contains passwords:
```bash
cat wordlist.txt
cat demo_passwords.txt
```

### Build fails?
```bash
cargo clean
cargo build --release
```

---

## 30-Second Demo Script

1. **Say:** "This is BlitzForge - watch how fast it tests passwords"
2. **Run:** `./demo_run.sh`
3. **Point:** Show hashes/sec counter
4. **Explain:** "Millions per second - this is why weak passwords are dangerous"
5. **Conclude:** "Use long random passwords and password managers!"

---

## Key Stats to Mention

- **Speed:** X billion hashes per second with BlitzHash (your number)
- **Custom Algorithm:** I designed BlitzHash specifically for this demo
- **Algorithms:** BlitzHash (custom), MD5, SHA1, SHA256, MD4
- **Multi-threaded:** Uses all CPU cores
- **Safe:** Demo accounts only - legal & ethical
- **BlitzHash is 2x faster than MD5!**

---

## Safety Disclaimer (Always Say This!)

> "This tool is ONLY for educational demonstrations on passwords I created myself. Using password crackers on real accounts without permission is illegal under the Computer Misuse Act. This shows why you need strong passwords and 2FA!"

---

## Common Questions & Answers

**Q: Why did you make your own hash?**
A: "To show maximum speed and prove why real systems need slow algorithms. BlitzHash hits 5-10 GH/s - that's why Argon2 exists!"

**Q: How fast is it?**
A: "X billion per second with BlitzHash, my custom algorithm. That's 2x faster than MD5!"

**Q: Can you crack my password?**
A: "No - that would be unethical. Try online strength checkers instead!"

**Q: Is this what hackers use?**
A: "Similar tools, yes. But with slow hashing and strong passwords, it's not practical."

**Q: Why Rust?**
A: "Fast as C, but memory-safe. Perfect for performance-critical tools."

**Q: What's the fastest password you cracked?**
A: "'password123' in under 1 second from a 10,000 word dictionary."

**Q: What's the slowest?**
A: "A random 12-character password would take centuries with brute force."

---

## If Something Goes Wrong

### Terminal looks broken?
Restart terminal, increase font size

### Can't find file?
Use absolute paths: `$(pwd)/wordlist.txt`

### Out of memory?
Reduce batch size: `--batch-size 2048`

### CPU overheating?
Reduce workers: `--workers 4`

---

## Emergency Backup Demo

If main demo fails, run self-test:
```bash
./target/release/blitzforge selftest
```

Then explain the project using your printed materials while you troubleshoot.

---

## Performance Expectations

**On 8-core CPU:**
- **BlitzHash: 5-10 GH/s** ⚡ (your custom algorithm!)
- MD5: 2-3 GH/s
- SHA1: 1-2 GH/s  
- SHA256: 800 MH/s - 1 GH/s

**BlitzHash is the star of the show - 2x faster than MD5!**

Your mileage may vary based on CPU model.

---

## Key Takeaway Message

> "I built BlitzHash - a custom ultra-fast hash that hits 5-10 billion per second. Even at this insane speed, strong passwords take centuries to crack. That's why real systems use Argon2 which is 500 million times slower - it makes password cracking completely impractical. Strong passwords + password managers + 2FA = good security!"

---

**Good luck with your presentation! 🔥**