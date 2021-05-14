//! # Luthien Plugin IO
//!
//! This module provides functions for reading and writing from Luthien's stdio.

use crate::Input;
use ipipe::{OnCleanup, Pipe};

pub use ipipe;

impl Input {
    /// Get an [`ipipe::Pipe`] which can be used to read input and print output to the parent
    /// Luthien process.
    pub fn io(&self) -> Option<ipipe::Result<Pipe>> {
        self.pipe_path
            .as_ref()
            .map(|path| Pipe::open(path, OnCleanup::NoDelete))
    }
}
