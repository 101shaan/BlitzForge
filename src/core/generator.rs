use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

pub trait Generator: Send {
    fn next_batch(&mut self, size: usize) -> Option<Vec<Vec<u8>>>;
    fn estimated_size(&self) -> Option<u64>;
    fn reset(&mut self);
}

// dictionary generator - reads from wordlist file
pub struct DictionaryGenerator {
    reader: BufReader<File>,
    path: std::path::PathBuf,
    total_lines: Option<u64>,
}

impl DictionaryGenerator {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        let file = File::open(&path_buf)?;
        let reader = BufReader::new(file);
        
        Ok(Self {
            reader,
            path: path_buf,
            total_lines: None,
        })
    }
}

impl Generator for DictionaryGenerator {
    fn next_batch(&mut self, size: usize) -> Option<Vec<Vec<u8>>> {
        let mut batch = Vec::with_capacity(size);
        
        for _ in 0..size {
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => break,  // eof
                Ok(_) => {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        batch.push(trimmed.as_bytes().to_vec());
                    }
                }
                Err(_) => break,
            }
        }
        
        if batch.is_empty() {
            None
        } else {
            Some(batch)
        }
    }
    
    fn estimated_size(&self) -> Option<u64> {
        self.total_lines
    }
    
    fn reset(&mut self) {
        if let Ok(file) = File::open(&self.path) {
            self.reader = BufReader::new(file);
        }
    }
}

// mask generator - pattern-based generation
#[derive(Debug, Clone)]
pub struct CharSet {
    pub chars: Vec<u8>,
}

impl CharSet {
    pub fn lowercase() -> Self {
        Self {
            chars: (b'a'..=b'z').collect(),
        }
    }
    
    pub fn uppercase() -> Self {
        Self {
            chars: (b'A'..=b'Z').collect(),
        }
    }
    
    pub fn digits() -> Self {
        Self {
            chars: (b'0'..=b'9').collect(),
        }
    }
    
    pub fn special() -> Self {
        Self {
            chars: b"!@#$%^&*()_+-=[]{}|;:,.<>?".to_vec(),
        }
    }
    
    pub fn from_string(s: &str) -> Self {
        Self {
            chars: s.as_bytes().to_vec(),
        }
    }
}

pub struct MaskGenerator {
    pattern: Vec<CharSet>,
    current: Vec<usize>,
    exhausted: bool,
}

impl MaskGenerator {
    /// parse mask pattern like "?l?l?l?d?d"
    pub fn new(mask: &str) -> Result<Self> {
        let mut pattern = Vec::new();
        let chars: Vec<char> = mask.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            if chars[i] == '?' && i + 1 < chars.len() {
                let charset = match chars[i + 1] {
                    'l' => CharSet::lowercase(),
                    'u' => CharSet::uppercase(),
                    'd' => CharSet::digits(),
                    's' => CharSet::special(),
                    _ => return Err(anyhow::anyhow!("unknown charset: ?{}", chars[i + 1])),
                };
                pattern.push(charset);
                i += 2;
            } else {
                // literal character
                pattern.push(CharSet {
                    chars: vec![chars[i] as u8],
                });
                i += 1;
            }
        }
        
        let current = vec![0; pattern.len()];
        
        Ok(Self {
            pattern,
            current,
            exhausted: false,
        })
    }
    
    fn increment(&mut self) {
        for i in (0..self.current.len()).rev() {
            self.current[i] += 1;
            if self.current[i] < self.pattern[i].chars.len() {
                return;
            }
            self.current[i] = 0;
        }
        self.exhausted = true;
    }
}

impl Generator for MaskGenerator {
    fn next_batch(&mut self, size: usize) -> Option<Vec<Vec<u8>>> {
        if self.exhausted {
            return None;
        }
        
        let mut batch = Vec::with_capacity(size);
        
        while batch.len() < size && !self.exhausted {
            let candidate: Vec<u8> = self.pattern.iter()
                .zip(&self.current)
                .map(|(charset, &idx)| charset.chars[idx])
                .collect();
            
            batch.push(candidate);
            self.increment();
        }
        
        if batch.is_empty() {
            None
        } else {
            Some(batch)
        }
    }
    
    fn estimated_size(&self) -> Option<u64> {
        Some(
            self.pattern.iter()
                .map(|cs| cs.chars.len() as u64)
                .product()
        )
    }
    
    fn reset(&mut self) {
        self.current = vec![0; self.pattern.len()];
        self.exhausted = false;
    }
}

// brute force generator - exhaustive search
pub struct BruteForceGenerator {
    charset: Vec<u8>,
    min_length: usize,
    max_length: usize,
    current_length: usize,
    current: Vec<usize>,
    exhausted: bool,
}

impl BruteForceGenerator {
    pub fn new(charset: &str, min_length: usize, max_length: usize) -> Self {
        Self {
            charset: charset.as_bytes().to_vec(),
            min_length,
            max_length,
            current_length: min_length,
            current: vec![0; min_length],
            exhausted: false,
        }
    }
    
    fn increment_current(&mut self) -> bool {
        for i in (0..self.current.len()).rev() {
            self.current[i] += 1;
            if self.current[i] < self.charset.len() {
                return true;
            }
            self.current[i] = 0;
        }
        false
    }
}

impl Generator for BruteForceGenerator {
    fn next_batch(&mut self, size: usize) -> Option<Vec<Vec<u8>>> {
        if self.exhausted {
            return None;
        }
        
        let mut batch = Vec::with_capacity(size);
        
        while batch.len() < size {
            if self.current_length > self.max_length {
                self.exhausted = true;
                break;
            }
            
            let candidate: Vec<u8> = self.current.iter()
                .map(|&idx| self.charset[idx])
                .collect();
            
            batch.push(candidate);
            
            if !self.increment_current() {
                self.current_length += 1;
                if self.current_length <= self.max_length {
                    self.current = vec![0; self.current_length];
                }
            }
        }
        
        if batch.is_empty() {
            None
        } else {
            Some(batch)
        }
    }
    
    fn estimated_size(&self) -> Option<u64> {
        let base = self.charset.len() as u64;
        let mut total = 0u64;
        
        for len in self.min_length..=self.max_length {
            total = total.saturating_add(base.saturating_pow(len as u32));
        }
        
        Some(total)
    }
    
    fn reset(&mut self) {
        self.current_length = self.min_length;
        self.current = vec![0; self.min_length];
        self.exhausted = false;
    }
}