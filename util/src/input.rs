use crate::*;
use anyhow::{bail, Result};
use memmap::Mmap;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

pub struct Input(Mmap);

impl Input {
    pub fn lines(&self) -> Lines {
        Lines(self.as_ref())
    }
}

impl AsRef<[u8]> for Input {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<str> for Input {
    fn as_ref(&self) -> &str {
        to_str(self.as_ref())
    }
}

pub struct Lines<'a>(&'a [u8]);

impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        let start = self.0;
        let mut end = self.0.len();
        let mut next = self.0.len();
        for i in 0..self.0.len() {
            let c = self.0[i];
            if c == b'\r' || c == b'\n' {
                end = i;
                next = i + 1;
                if c == b'\r' && self.0.len() > next && self.0[next] == b'\n' {
                    next += 1;
                }
                break;
            }
        }
        self.0 = &self.0[next..];
        Some(to_str(&start[..end]))
    }
}

fn get_input_path(day: &str) -> Result<PathBuf> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        Ok(Path::new(day).join("data/input.txt"))
    } else if args.len() == 2 && args[1] == "-e" {
        Ok(Path::new(day).join("data/example.txt"))
    } else if args.len() == 3 && args[1] == "-i" {
        Ok(PathBuf::from(&args[2]))
    } else {
        bail!("Bad command line arguments. Expected nothing, `-e`, or `-i <file>`")
    }
}

pub fn open_input(day: &str) -> Result<Input> {
    let path = get_input_path(day)?;
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };

    #[cfg(feature = "validation")]
    if !std::str::from_utf8(&mmap)?.is_ascii() {
        bail!("Input contains non-ascii data");
    }

    Ok(Input(mmap))
}
