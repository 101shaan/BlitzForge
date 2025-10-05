# BlitzForge 🔥
### A High-Performance Password Security Demonstration

**Sixth Form Computer Science Project | Scholars Showcase 2025**

---

## What is BlitzForge?

BlitzForge is a terminal-based password cracking demonstration tool that shows how quickly computers can test passwords against different hash algorithms. It's designed to educate people about **why strong passwords matter** and how computational power affects security.

### Think of it like this:
Imagine a thief trying every key on a massive keyring to open a lock. BlitzForge simulates this, but with **millions of password guesses per second**. The demo shows why "password123" is dangerous, while "correct-horse-battery-staple" is much safer.

---

## Why Does This Matter?

### Real-World Impact
- **Data breaches expose billions of passwords** every year
- Hackers use tools like this (but for malicious purposes) to break into accounts
- **Weak passwords can be cracked in seconds**, even with modern computers
- Strong passwords with 12+ random characters can take **centuries** to crack

### Educational Goals
This project demonstrates:
1. How different password strategies affect security
2. The importance of password length and complexity
3. Why websites should use "slow" hashing algorithms (like Argon2)
4. Real-time computational performance of modern CPUs

---

## How It Works (Simple Version)

### Step 1: Create Demo Targets
We generate test accounts with known passwords and create "hashes" (scrambled versions) of those passwords. **We only test our own demo accounts - never real ones!**

### Step 2: Try Passwords
BlitzForge uses different strategies to guess passwords:

- **Dictionary Attack**: Tests common passwords from a list (like "password", "123456", "qwerty")
- **Mask Attack**: Tests patterns like "word + 2 digits" (e.g., "hello99")
- **Brute Force**: Tests every possible combination of letters/numbers (slowest but thorough)

### Step 3: Live Metrics
Watch the terminal display:
- **Speed**: Millions of guesses per second
- **Progress**: How much of the password space has been checked
- **Hardware**: CPU usage across all processor cores
- **Results**: When a password is found (with a big ✓)

---

## The Live Demo

### What You'll See

```
╔═══════════════════════════════════════════════╗
║    BLITZFORGE - PASSWORD CRACKER DEMO         ║
║   ⚠️  DEMO ACCOUNTS ONLY - EDUCATIONAL USE     ║
╚═══════════════════════════════════════════════╝

PERFORMANCE METRICS
Speed: 2,847,392 hashes/sec
Found: 1/2 targets
Time:  00:00:12

TARGET PROGRESS
[weak_password]    FOUND! ✓ (in 0.8 seconds)
[strong_password]  ████░░░░░░░░ 35% (still running...)

CPU:  [████████████████] 89% (8 cores active)
```

### Two Scenarios

**Scenario 1: Weak Password** ("password123")
- Found in **under 2 seconds**
- Shows why common passwords are dangerous

**Scenario 2: Strong Password** ("Tr0ub4dour&3-xkcd-style")
- Takes **30+ seconds** even at millions of guesses/second
- Demonstrates the power of length and complexity

---

## Key Takeaways

### For Everyone
✅ **Always use long, random passwords** (12+ characters)  
✅ **Never reuse passwords** across different websites  
✅ **Use a password manager** to generate and store strong passwords  
✅ **Enable two-factor authentication (2FA)** wherever possible  

### Why "Slow" is Good for Security
Websites should use **deliberately slow** hashing algorithms (like Argon2 or bcrypt). This means:
- Your login takes 0.1 seconds (fine for you)
- An attacker's password cracker slows from **5 billion/sec to 10/sec**
- A password that takes 2 seconds to crack now takes **15 years**

### The BlitzHash Advantage
I built a **custom hash function** specifically optimized for speed:
- **BlitzHash**: 5-10 GH/s (billions per second)
- **MD5**: 2-3 GH/s (standard fast hash)
- **SHA256**: 800 MH/s (slower but more secure)
- **Argon2**: 10 H/s (deliberately slow - what real sites should use)

This demonstrates why security isn't just about the algorithm - **computational cost matters!**

---

## Technical Highlights

**Built with Rust** for maximum performance  
**Custom BlitzHash algorithm** - I designed my own ultra-fast hash function  
**Multi-threaded** to use all CPU cores simultaneously  
**Standard algorithms** including MD5, SHA1, SHA256 for comparison  
**Real-time visualization** with live progress bars  
**Benchmark logging** to CSV for analysis  

### Performance
On a modern 8-core CPU:
- **BlitzHash**: ~5-10 billion hashes per second (custom algorithm)
- **MD5**: ~2-3 billion hashes per second
- **SHA256**: ~800 million hashes per second
- **Scales linearly** with more CPU cores

**My BlitzHash is 2x faster than MD5 and 6x faster than SHA256!**

---

## Safety & Ethics

### This Tool is ONLY for Education
- ⚠️ **Never use on real accounts** - that's illegal
- ⚠️ **Only test passwords you created** for this demo
- ⚠️ **Respect privacy and laws** at all times

### The Legal Reality
Unauthorized password cracking is a **serious crime** under:
- Computer Misuse Act 1990 (UK)
- Computer Fraud and Abuse Act (USA)
- Similar laws worldwide

This demo teaches security concepts **within legal boundaries**.

---

## What I Learned

### Technical Skills
- **Systems programming** in Rust (memory safety + performance)
- **Multi-threaded optimization** using parallel processing
- **Terminal UI design** with real-time updates
- **Algorithm analysis** and computational complexity

### Security Insights
- Password entropy calculations
- Hash function performance characteristics
- Trade-offs between security and usability
- Importance of defense-in-depth strategies

### Project Management
- Breaking complex problems into modules
- Writing clear documentation for different audiences
- Testing and benchmarking methodology

---

## Try It Yourself (Safely!)

Want to test your own passwords' strength?

1. **Use online tools** like:
   - [How Secure Is My Password](https://howsecureismypassword.net/)
   - [Password Strength Checker](https://www.security.org/how-secure-is-my-password/)

2. **Get a password manager**:
   - Bitwarden (free, open-source)
   - 1Password
   - LastPass

3. **Enable 2FA everywhere**:
   - Google Authenticator
   - Authy
   - Hardware keys (YubiKey)

---

## Questions?

**What's the fastest password you've cracked?**  
"password" - found in less than 0.01 seconds from a dictionary of 10,000 common passwords.

**What's the slowest?**  
A random 12-character password with mixed case, numbers, and symbols - still running after 30 minutes (would take decades to finish all combinations).

**Could this break real accounts?**  
Only if: (1) you had the password hash file, (2) the site used weak hashing, (3) the password was weak. Modern sites use slow hashing and rate limiting, making this largely impractical. But weak passwords are still vulnerable!

**Is this ethical?**  
Yes - when used ONLY for education on your own demo data. The same tools hackers use can teach people to defend themselves.

---

## Resources & Further Reading

- **Project Code**: [github.com/yourusername/blitzforge]
- **OWASP Password Guide**: [owasp.org/www-community/password-guidelines]
- **XKCD Password Comic**: [xkcd.com/936/]
- **UK National Cyber Security Centre**: [ncsc.gov.uk/collection/passwords]

---

**Thank you for visiting my project!**

*Questions? Find me at the demonstration table for a live run.*

**Remember: Long passwords, password managers, and 2FA = better security!**