use anyhow::Result;
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use clap::ValueEnum;

use crate::core::*;
use crate::cli::{TerminalUI, BenchmarkLogger};
use crate::tools;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Strategy {
    Dictionary,
    Mask,
    Brute,
    Hybrid,
}

pub fn generate_targets(
    out: PathBuf,
    passwords: PathBuf,
    algorithms: String,
) -> Result<()> {
    println!("🔧 Generating demo targets...");
    
    let algos: Vec<Algorithm> = algorithms
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    
    if algos.is_empty() {
        anyhow::bail!("no valid algorithms specified");
    }
    
    // read passwords
    let file = File::open(&passwords)?;
    let reader = BufReader::new(file);
    let passwords: Vec<String> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.trim().is_empty())
        .collect();
    
    if passwords.is_empty() {
        anyhow::bail!("no passwords found in file");
    }
    
    // generate targets
    let targets = tools::generate_demo_targets(&passwords, &algos);
    
    // write to json
    let json = serde_json::to_string_pretty(&targets)?;
    fs::write(&out, json)?;
    
    println!("✅ Generated {} targets → {}", targets.len(), out.display());
    println!("   Algorithms: {}", algorithms);
    println!("   Passwords: {}", passwords.len());
    
    Ok(())
}

pub fn run_cracking(
    targets_path: PathBuf,
    strategy: Strategy,
    wordlist: Option<PathBuf>,
    mask: Option<String>,
    charset: Option<String>,
    min_len: usize,
    max_len: usize,
    workers: Option<usize>,
    batch_size: usize,
    repeat: usize,
    log: Option<PathBuf>,
) -> Result<()> {
    // load targets
    let json = fs::read_to_string(&targets_path)?;
    let targets: Vec<Target> = serde_json::from_str(&json)?;
    
    if targets.is_empty() {
        anyhow::bail!("no targets found in file");
    }
    
    // create generator based on strategy
    let generator: Box<dyn Generator> = match strategy {
        Strategy::Dictionary => {
            let wordlist_path = wordlist.ok_or_else(|| anyhow::anyhow!("--wordlist required for dictionary strategy"))?;
            Box::new(DictionaryGenerator::new(wordlist_path)?)
        }
        
        Strategy::Mask => {
            let mask_pattern = mask.ok_or_else(|| anyhow::anyhow!("--mask required for mask strategy"))?;
            Box::new(MaskGenerator::new(&mask_pattern)?)
        }
        
        Strategy::Brute => {
            let charset_str = charset.unwrap_or_else(|| "abcdefghijklmnopqrstuvwxyz0123456789".to_string());
            Box::new(BruteForceGenerator::new(&charset_str, min_len, max_len))
        }
        
        Strategy::Hybrid => {
            anyhow::bail!("hybrid strategy not yet implemented");
        }
    };
    
    let workers_count = workers.unwrap_or_else(num_cpus::get);
    
    // create terminal ui
    let mut ui = TerminalUI::new();
    
    // print warning banner
    ui.print_warning();
    
    // print configuration
    println!("\n📋 Configuration:");
    println!("   Targets:    {}", targets.len());
    println!("   Strategy:   {:?}", strategy);
    println!("   Workers:    {}", workers_count);
    println!("   Batch size: {}", batch_size);
    println!("   Repeats:    {}", repeat);
    
    if let Some(est) = generator.estimated_size() {
        println!("   Keyspace:   {}", format_number(est));
    }
    
    println!("\n⚡ Starting cracking engine...\n");
    
    // create benchmark logger if needed
    let mut logger = if let Some(log_path) = log {
        Some(BenchmarkLogger::new(log_path)?)
    } else {
        None
    };
    
    // run for each repeat
    for run in 1..=repeat {
        if repeat > 1 {
            println!("\n🔄 Run {}/{}", run, repeat);
        }
        
        // create engine
        let mut engine = Engine::new(
            targets.clone(),
            generator,
            workers_count,
            batch_size,
        );
        
        // run with ui callback
        ui.start_display(&targets);
        
        let result = engine.run(|stats| {
            ui.update(stats);
        })?;
        
        ui.stop_display();
        
        // print results
        ui.print_results(&result);
        
        // log to csv if enabled
        if let Some(ref mut log) = logger {
            log.log_result(&result, &targets)?;
        }
        
        // prepare for next run if needed
        if run < repeat {
            generator = match strategy {
                Strategy::Dictionary => {
                    let wordlist_path = wordlist.clone().unwrap();
                    Box::new(DictionaryGenerator::new(wordlist_path)?)
                }
                Strategy::Mask => {
                    let mask_pattern = mask.clone().unwrap();
                    Box::new(MaskGenerator::new(&mask_pattern)?)
                }
                Strategy::Brute => {
                    let charset_str = charset.clone().unwrap_or_else(|| "abcdefghijklmnopqrstuvwxyz0123456789".to_string());
                    Box::new(BruteForceGenerator::new(&charset_str, min_len, max_len))
                }
                Strategy::Hybrid => unreachable!(),
            };
        }
    }
    
    println!("\n✅ All runs completed!");
    
    Ok(())
}

pub fn generate_report(csv_path: PathBuf) -> Result<()> {
    println!("📊 Generating report from: {}", csv_path.display());
    
    let mut reader = csv::Reader::from_path(&csv_path)?;
    
    let mut runs = Vec::new();
    for result in reader.deserialize() {
        let record: BenchmarkRecord = result?;
        runs.push(record);
    }
    
    if runs.is_empty() {
        println!("⚠️  No benchmark data found");
        return Ok(());
    }
    
    println!("\n📈 Summary:");
    println!("   Total runs: {}", runs.len());
    
    // group by algorithm
    use std::collections::HashMap;
    let mut by_algo: HashMap<String, Vec<&BenchmarkRecord>> = HashMap::new();
    
    for run in &runs {
        by_algo.entry(run.algorithm.clone())
            .or_insert_with(Vec::new)
            .push(run);
    }
    
    println!("\n🔐 Performance by Algorithm:");
    for (algo, records) in by_algo {
        let throughputs: Vec<f64> = records.iter().map(|r| r.hashes_per_s).collect();
        let median = median(&throughputs);
        let max = throughputs.iter().copied().fold(0.0_f64, f64::max);
        
        println!("\n   {}:", algo.to_uppercase());
        println!("      Runs:             {}", records.len());
        println!("      Median H/s:       {}", format_hashes_per_sec(median));
        println!("      Peak H/s:         {}", format_hashes_per_sec(max));
        
        let found_count = records.iter().filter(|r| r.found).count();
        println!("      Success rate:     {}/{}", found_count, records.len());
    }
    
    Ok(())
}

pub fn run_selftest() -> Result<()> {
    println!("🧪 Running BlitzForge self-tests...\n");
    
    // test 1: hash algorithms
    println!("Test 1: Hash Algorithms");
    test_hash_algorithms()?;
    println!("   ✅ All hash algorithms working\n");
    
    // test 2: generators
    println!("Test 2: Candidate Generators");
    test_generators()?;
    println!("   ✅ All generators working\n");
    
    // test 3: simple crack
    println!("Test 3: Simple Crack");
    test_simple_crack()?;
    println!("   ✅ Engine successfully cracked test password\n");
    
    println!("✅ All self-tests passed!");
    
    Ok(())
}

// helper: test hash algorithms
fn test_hash_algorithms() -> Result<()> {
    use crate::core::hasher::*;
    use crate::core::blitzhash;
    
    let test_input = b"password";
    
    // test blitzhash
    let hash = blitzhash::blitz_hash(0, test_input);
    assert_eq!(hash.len(), 32);
    println!("   BlitzHash: OK (custom algorithm)");
    
    let md5_hasher = Md5Hasher;
    let hash = md5_hasher.hash(test_input);
    assert_eq!(hex::encode(hash), "5f4dcc3b5aa765d61d8327deb882cf99");
    println!("   MD5: OK");
    
    let sha1_hasher = Sha1Hasher;
    let hash = sha1_hasher.hash(test_input);
    assert_eq!(hex::encode(hash), "5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8");
    println!("   SHA1: OK");
    
    let sha256_hasher = Sha256Hasher;
    let hash = sha256_hasher.hash(test_input);
    assert_eq!(hex::encode(hash), "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8");
    println!("   SHA256: OK");
    
    Ok(())
}

// helper: test generators
fn test_generators() -> Result<()> {
    // test mask generator
    let mut mask_gen = MaskGenerator::new("?d?d")?;
    let batch = mask_gen.next_batch(5).unwrap();
    assert_eq!(batch.len(), 5);
    assert_eq!(batch[0], b"00");
    assert_eq!(batch[1], b"01");
    println!("   Mask generator: OK");
    
    // test brute force generator
    let mut brute_gen = BruteForceGenerator::new("ab", 2, 2);
    let batch = brute_gen.next_batch(10).unwrap();
    assert_eq!(batch.len(), 4); // aa, ab, ba, bb
    println!("   Brute force generator: OK");
    
    Ok(())
}

// helper: test simple crack
fn test_simple_crack() -> Result<()> {
    use crate::core::hasher::*;
    use crate::core::blitzhash;
    
    // create test target with blitzhash
    let password = b"password";
    let hash = blitzhash::blitz_hash(0, password);
    
    let target = Target {
        id: "test".to_string(),
        username: "testuser".to_string(),
        algorithm: Algorithm::BlitzHash,
        hash: hex::encode(hash),
        salt: String::new(),
    };
    
    // create simple generator with known password
    let mut gen = BruteForceGenerator::new("password", 8, 8);
    
    // create engine
    let mut engine = Engine::new(
        vec![target],
        Box::new(gen),
        2,
        100,
    );
    
    // run
    let result = engine.run(|_| {})?;
    
    assert_eq!(result.matches.len(), 1);
    assert_eq!(result.matches[0].password_string(), "password");
    
    Ok(())
}

// helper types
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BenchmarkRecord {
    timestamp: String,
    target_id: String,
    algorithm: String,
    strategy: String,
    workers: usize,
    keyspace_size: String,
    guesses_tried: u64,
    time_s: f64,
    hashes_per_s: f64,
    found: bool,
    password_length: String,
    found_in_s: String,
}

// helper functions
fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}

fn format_number(n: u64) -> String {
    if n >= 1_000_000_000_000 {
        format!("{:.2}T", n as f64 / 1_000_000_000_000.0)
    } else if n >= 1_000_000_000 {
        format!("{:.2}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.2}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.2}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn format_hashes_per_sec(h: f64) -> String {
    if h >= 1_000_000_000.0 {
        format!("{:.2} GH/s", h / 1_000_000_000.0)
    } else if h >= 1_000_000.0 {
        format!("{:.2} MH/s", h / 1_000_000.0)
    } else if h >= 1_000.0 {
        format!("{:.2} KH/s", h / 1_000.0)
    } else {
        format!("{:.0} H/s", h)
    }
}

// note: num_cpus is not in dependencies, add inline helper
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}