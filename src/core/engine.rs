use std::sync::{Arc, Mutex};
use std::time::Instant;
use rayon::prelude::*;
use hashbrown::HashMap;
use anyhow::Result;

use super::{Generator, Hasher, Target, TargetMatch};

#[derive(Debug, Clone)]
pub struct Statistics {
    pub guesses_tried: u64,
    pub hashes_computed: u64,
    pub targets_found: usize,
    pub targets_total: usize,
    pub start_time: Instant,
    pub hashes_per_second: f64,
}

impl Statistics {
    pub fn new(targets_total: usize) -> Self {
        Self {
            guesses_tried: 0,
            hashes_computed: 0,
            targets_found: 0,
            targets_total,
            start_time: Instant::now(),
            hashes_per_second: 0.0,
        }
    }
    
    pub fn update_throughput(&mut self) {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.hashes_per_second = self.hashes_computed as f64 / elapsed;
        }
    }
}

pub struct Engine {
    targets: Vec<Target>,
    generator: Box<dyn Generator>,
    workers: usize,
    batch_size: usize,
    stats: Arc<Mutex<Statistics>>,
}

#[derive(Debug, Clone)]
pub struct CrackingResult {
    pub matches: Vec<TargetMatch>,
    pub statistics: Statistics,
    pub total_time: f64,
}

impl Engine {
    pub fn new(
        targets: Vec<Target>,
        generator: Box<dyn Generator>,
        workers: usize,
        batch_size: usize,
    ) -> Self {
        let stats = Arc::new(Mutex::new(Statistics::new(targets.len())));
        
        Self {
            targets,
            generator,
            workers,
            batch_size,
            stats,
        }
    }
    
    pub fn run<F>(&mut self, mut callback: F) -> Result<CrackingResult>
    where
        F: FnMut(&Statistics),
    {
        let start = Instant::now();
        let mut matches = Vec::new();
        let mut found_ids = std::collections::HashSet::new();
        
        // build target index by algorithm for fast lookup
        let mut targets_by_algo: HashMap<String, Vec<&Target>> = HashMap::new();
        for target in &self.targets {
            targets_by_algo
                .entry(target.algorithm.to_string())
                .or_insert_with(Vec::new)
                .push(target);
        }
        
        // configure rayon thread pool
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.workers)
            .build()?;
        
        // main cracking loop
        loop {
            // check if all targets found
            if found_ids.len() >= self.targets.len() {
                break;
            }
            
            // get next batch of candidates
            let batch = match self.generator.next_batch(self.batch_size) {
                Some(b) => b,
                None => break,  // exhausted keyspace
            };
            
            let batch_size = batch.len() as u64;
            
            // process batch in parallel for each algorithm
            for (algo_str, algo_targets) in &targets_by_algo {
                if algo_targets.is_empty() {
                    continue;
                }
                
                // skip if all targets for this algorithm are found
                if algo_targets.iter().all(|t| found_ids.contains(&t.id)) {
                    continue;
                }
                
                // create hasher for this algorithm
                let hasher = super::hasher::create_hasher(algo_targets[0].algorithm);
                
                // process batch in parallel
                let batch_matches: Vec<TargetMatch> = pool.install(|| {
                    batch.par_iter()
                        .flat_map(|candidate| {
                            let mut local_matches = Vec::new();
                            
                            for target in algo_targets.iter() {
                                // skip if already found
                                if found_ids.contains(&target.id) {
                                    continue;
                                }
                                
                                // compute hash
                                let hash = if target.salt.is_empty() {
                                    hasher.hash(candidate)
                                } else {
                                    hasher.hash_with_salt(candidate, &target.salt_bytes())
                                };
                                
                                // check match
                                if target.matches(&hash) {
                                    let stats = self.stats.lock().unwrap();
                                    let time_elapsed = start.elapsed().as_secs_f64();
                                    
                                    local_matches.push(TargetMatch {
                                        target_id: target.id.clone(),
                                        username: target.username.clone(),
                                        password: candidate.clone(),
                                        algorithm: target.algorithm,
                                        guesses_tried: stats.guesses_tried,
                                        time_seconds: time_elapsed,
                                    });
                                }
                            }
                            
                            local_matches
                        })
                        .collect()
                });
                
                // record found matches
                for m in batch_matches {
                    if found_ids.insert(m.target_id.clone()) {
                        matches.push(m);
                    }
                }
            }
            
            // update statistics
            {
                let mut stats = self.stats.lock().unwrap();
                stats.guesses_tried += batch_size;
                stats.hashes_computed += batch_size * targets_by_algo.len() as u64;
                stats.targets_found = found_ids.len();
                stats.update_throughput();
            }
            
            // callback for ui updates
            {
                let stats = self.stats.lock().unwrap();
                callback(&stats);
            }
        }
        
        let total_time = start.elapsed().as_secs_f64();
        let final_stats = self.stats.lock().unwrap().clone();
        
        Ok(CrackingResult {
            matches,
            statistics: final_stats,
            total_time,
        })
    }
    
    pub fn get_stats(&self) -> Statistics {
        self.stats.lock().unwrap().clone()
    }
}