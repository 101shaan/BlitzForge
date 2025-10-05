use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::{Clear, ClearType},
};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::io::{stdout, Write};
use std::collections::HashMap;

use crate::core::{Statistics, Target, CrackingResult};

pub struct TerminalUI {
    multi_progress: Option<MultiProgress>,
    target_bars: HashMap<String, ProgressBar>,
    stats_bar: Option<ProgressBar>,
}

impl TerminalUI {
    pub fn new() -> Self {
        Self {
            multi_progress: None,
            target_bars: HashMap::new(),
            stats_bar: None,
        }
    }
    
    pub fn print_warning(&self) {
        let mut stdout = stdout();
        
        println!();
        execute!(
            stdout,
            SetBackgroundColor(Color::Red),
            SetForegroundColor(Color::White),
            Print("╔═══════════════════════════════════════════════════════════════════╗"),
            ResetColor,
        ).ok();
        println!();
        
        execute!(
            stdout,
            SetBackgroundColor(Color::Red),
            SetForegroundColor(Color::White),
            Print("║           BLITZFORGE - DEMO MODE ACTIVE                           ║"),
            ResetColor,
        ).ok();
        println!();
        
        execute!(
            stdout,
            SetBackgroundColor(Color::Red),
            SetForegroundColor(Color::White),
            Print("║  ⚠️  USE ONLY ON YOUR OWN DEMO TARGETS - LEGAL USE ONLY  ⚠️      ║"),
            ResetColor,
        ).ok();
        println!();
        
        execute!(
            stdout,
            SetBackgroundColor(Color::Red),
            SetForegroundColor(Color::White),
            Print("╚═══════════════════════════════════════════════════════════════════╝"),
            ResetColor,
        ).ok();
        println!();
    }
    
    pub fn start_display(&mut self, targets: &[Target]) {
        let multi = MultiProgress::new();
        
        // create stats bar
        let stats_bar = multi.add(ProgressBar::new(100));
        stats_bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg}")
                .unwrap()
        );
        self.stats_bar = Some(stats_bar);
        
        // create progress bar for each target
        for target in targets {
            let bar = multi.add(ProgressBar::new(100));
            let style = ProgressStyle::default_bar()
                .template(&format!(" [{{bar:30.cyan/blue}}] {{percent:>3}}% {{msg}}"))
                .unwrap()
                .progress_chars("█▓▒░ ");
            
            bar.set_style(style);
            bar.set_message(format!("[{}@{}] ({})", target.username, target.id, target.algorithm));
            
            self.target_bars.insert(target.id.clone(), bar);
        }
        
        self.multi_progress = Some(multi);
    }
    
    pub fn update(&mut self, stats: &Statistics) {
        // update stats bar
        if let Some(ref stats_bar) = self.stats_bar {
            let msg = format!(
                "\n⚡ PERFORMANCE: {} | Found: {}/{} | Time: {:02}:{:02}:{:02}",
                format_hashes_per_sec(stats.hashes_per_second),
                stats.targets_found,
                stats.targets_total,
                (stats.start_time.elapsed().as_secs() / 3600),
                (stats.start_time.elapsed().as_secs() % 3600) / 60,
                stats.start_time.elapsed().as_secs() % 60,
            );
            stats_bar.set_message(msg);
        }
        
        // update target progress (estimated)
        // note: real implementation would track per-target progress
        for (_, bar) in &self.target_bars {
            let progress = (stats.guesses_tried as f64 / 1_000_000.0).min(99.0);
            bar.set_position(progress as u64);
        }
    }
    
    pub fn stop_display(&mut self) {
        if let Some(ref multi) = self.multi_progress {
            multi.clear().ok();
        }
    }
    
    pub fn print_results(&self, result: &CrackingResult) {
        println!("\n");
        println!("╔═══════════════════════════════════════════════════════════════════╗");
        println!("║                        CRACKING RESULTS                            ║");
        println!("╚═══════════════════════════════════════════════════════════════════╝");
        
        println!("\n📊 Statistics:");
        println!("   Total time:       {:.2}s", result.total_time);
        println!("   Guesses tried:    {}", format_number(result.statistics.guesses_tried));
        println!("   Hashes computed:  {}", format_number(result.statistics.hashes_computed));
        println!("   Throughput:       {}", format_hashes_per_sec(result.statistics.hashes_per_second));
        
        println!("\n🎯 Matches Found: {}/{}", result.matches.len(), result.statistics.targets_total);
        
        if result.matches.is_empty() {
            println!("   ❌ No passwords cracked");
        } else {
            for m in &result.matches {
                println!("\n   ✅ {}@{}", m.username, m.target_id);
                println!("      Password:     {}", m.password_string());
                println!("      Algorithm:    {}", m.algorithm);
                println!("      Found in:     {:.2}s", m.time_seconds);
                println!("      After:        {} guesses", format_number(m.guesses_tried));
            }
        }
        
        println!();
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