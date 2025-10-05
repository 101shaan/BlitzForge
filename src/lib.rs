// blitzforge library - high-performance password cracker demo

pub mod core;
pub mod cli;
pub mod tools;

// re-exports for convenience
pub use core::{
    Engine, 
    CrackingResult,
    Algorithm,
    Hasher,
    Generator,
    DictionaryGenerator,
    MaskGenerator,
    BruteForceGenerator,
    Target,
    TargetMatch,
};

pub use cli::{
    TerminalUI,
    BenchmarkLogger,
};