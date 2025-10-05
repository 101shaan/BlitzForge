use anyhow::Result;
use csv::Writer;
use std::fs::{File, OpenOptions};
use std::path::Path;
use chrono::Utc;

use crate::core::{CrackingResult, Target};

pub struct BenchmarkLogger {
    writer: Writer<File>,
}

impl BenchmarkLogger {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file_exists = path.as_ref().exists();
        
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        
        let mut writer = Writer::from_writer(file);
        
        // write header if new file
        if !file_exists {
            writer.write_record(&[
                "timestamp",
                "target_id",
                "algorithm",
                "strategy",
                "workers",
                "keyspace_size",
                "guesses_tried",
                "time_s",
                "hashes_per_s",
                "found",
                "password_length",
                "found_in_s",
            ])?;
            writer.flush()?;
        }
        
        Ok(Self { writer })
    }
    
    pub fn log_result(&mut self, result: &CrackingResult, targets: &[Target]) -> Result<()> {
        let timestamp = Utc::now().to_rfc3339();
        
        // log each target (found or not)
        for target in targets {
            let found_match = result.matches.iter()
                .find(|m| m.target_id == target.id);
            
            let (found, password_len, found_in_s) = if let Some(m) = found_match {
                (true, m.password.len(), m.time_seconds)
            } else {
                (false, 0, 0.0)
            };
            
            self.writer.write_record(&[
                &timestamp,
                &target.id,
                &target.algorithm.to_string(),
                "dictionary", // todo: pass actual strategy
                &"8".to_string(), // todo: pass actual worker count
                &"unknown".to_string(), // todo: pass actual keyspace
                &result.statistics.guesses_tried.to_string(),
                &result.total_time.to_string(),
                &result.statistics.hashes_per_second.to_string(),
                &found.to_string(),
                &if found { password_len.to_string() } else { String::new() },
                &if found { found_in_s.to_string() } else { String::new() },
            ])?;
        }
        
        self.writer.flush()?;
        Ok(())
    }
}