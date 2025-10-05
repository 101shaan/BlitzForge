#!/bin/bash
# blitzforge demo script for scholars showcase

set -e

echo "🔥 BlitzForge Demo Script"
echo "=========================="
echo

# build in release mode
echo "📦 Building BlitzForge (release mode)..."
RUSTFLAGS="-C target-cpu=native" cargo build --release
echo

# create demo passwords file
echo "📝 Creating demo passwords..."
cat > demo_passwords.txt << EOF
password123
secret
admin
letmein
correct-horse-battery-staple
EOF

echo "Created 5 demo passwords"
echo

# generate demo targets
echo "🎯 Generating demo targets..."
./target/release/blitzforge generate-targets \
  --out targets.json \
  --passwords demo_passwords.txt \
  --algorithms blitzhash,md5,sha256

echo

# create simple wordlist
echo "📚 Creating wordlist..."
cat > wordlist.txt << EOF
password
admin
letmein
secret
password123
123456
qwerty
welcome
monkey
dragon
EOF

echo "Created wordlist with 10 entries"
echo

# run dictionary attack
echo "⚡ Running dictionary attack..."
echo
./target/release/blitzforge run \
  --targets targets.json \
  --strategy dictionary \
  --wordlist wordlist.txt \
  --workers 8 \
  --batch-size 1000 \
  --log demo_results.csv

echo
echo "✅ Demo complete!"
echo
echo "📊 Check demo_results.csv for benchmark data"
echo "📋 Run './target/release/blitzforge report --csv demo_results.csv' for summary"