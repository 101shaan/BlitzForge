use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod core;
mod cli;
mod tools;

use cli::commands;

#[derive(Parser)]
#[command(name = "blitzforge")]
#[command(about = "High-performance password cracker demo - EDUCATIONAL USE ONLY", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// generate demo target hashes from known passwords
    GenerateTargets {
        /// output json file path
        #[arg(short, long)]
        out: PathBuf,
        
        /// input passwords file (one per line)
        #[arg(short, long)]
        passwords: PathBuf,
        
        /// comma-separated algorithms (md5,sha1,sha256,md4)
        #[arg(short, long, default_value = "md5,sha256")]
        algorithms: String,
    },
    
    /// run cracking job with live terminal ui
    Run {
        /// targets json file
        #[arg(short, long)]
        targets: PathBuf,
        
        /// attack strategy
        #[arg(short, long, value_enum)]
        strategy: cli::commands::Strategy,
        
        /// wordlist path (for dictionary/hybrid modes)
        #[arg(short, long)]
        wordlist: Option<PathBuf>,
        
        /// mask pattern (for mask/hybrid modes) e.g. ?l?l?l?d?d
        #[arg(short, long)]
        mask: Option<String>,
        
        /// charset (for brute force mode) e.g. "abc123"
        #[arg(short, long)]
        charset: Option<String>,
        
        /// minimum length (for brute force)
        #[arg(long, default_value = "1")]
        min_len: usize,
        
        /// maximum length (for brute force)
        #[arg(long, default_value = "8")]
        max_len: usize,
        
        /// number of worker threads (default: cpu count)
        #[arg(long)]
        workers: Option<usize>,
        
        /// batch size for candidate processing
        #[arg(long, default_value = "4096")]
        batch_size: usize,
        
        /// number of repeat runs for benchmarking
        #[arg(short, long, default_value = "1")]
        repeat: usize,
        
        /// csv log file for benchmark results
        #[arg(short, long)]
        log: Option<PathBuf>,
    },
    
    /// generate summary report from benchmark csv
    Report {
        /// csv file to analyze
        #[arg(short, long)]
        csv: PathBuf,
    },
    
    /// run internal tests and micro-benchmarks
    Selftest,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::GenerateTargets { out, passwords, algorithms } => {
            commands::generate_targets(out, passwords, algorithms)?;
        }
        
        Commands::Run {
            targets,
            strategy,
            wordlist,
            mask,
            charset,
            min_len,
            max_len,
            workers,
            batch_size,
            repeat,
            log,
        } => {
            commands::run_cracking(
                targets,
                strategy,
                wordlist,
                mask,
                charset,
                min_len,
                max_len,
                workers,
                batch_size,
                repeat,
                log,
            )?;
        }
        
        Commands::Report { csv } => {
            commands::generate_report(csv)?;
        }
        
        Commands::Selftest => {
            commands::run_selftest()?;
        }
    }
    
    Ok(())
}