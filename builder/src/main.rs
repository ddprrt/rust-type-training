// Typestate Programming and Builders

pub struct Worker {
    pub workload: String, // Python Code
    pub memsize: u128,
    pub keep_alive: bool,
}

#[derive(Debug)]
struct CreationError;

impl Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error creating worker")
    }
}

impl std::error::Error for CreationError {}

struct NoWorkload;

struct WorkerBuilder<W> {
    workload: W,
    memsize: u128,
    keep_alive: bool,
}

impl WorkerBuilder<NoWorkload> {
    fn new() -> Self {
        Self {
            workload: NoWorkload,
            memsize: 128 * 1024,
            keep_alive: false,
        }
    }

    fn workload(self, workload: impl Into<String>) -> WorkerBuilder<String> {
        WorkerBuilder {
            workload: workload.into(),
            memsize: self.memsize,
            keep_alive: self.keep_alive,
        }
    }
}

impl WorkerBuilder<String> {
    fn build(self) -> Worker {
        Worker {
            workload: self.workload,
            memsize: self.memsize,
            keep_alive: self.keep_alive,
        }
    }

    fn workload(mut self, workload: impl Into<String>) -> Self {
        self.workload = workload.into();
        self
    }
}

impl<W> WorkerBuilder<W> {
    fn memsize(mut self, memsize: u128) -> Self {
        self.memsize = memsize;
        self
    }

    fn keep_alive(mut self, keep_alive: bool) -> Self {
        self.keep_alive = keep_alive;
        self
    }
}

use std::{fmt::Display, process::Command};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Command::new("ls").current_dir("..").arg("-la").spawn()?;
    let _output = cmd.wait_with_output()?;
    //println!("{:?}", output);

    let builder = WorkerBuilder::new();
    let builder = builder.memsize(1000).keep_alive(true).workload("");
    let _worker = builder.workload("").memsize(1000).build();

    Ok(())
}
