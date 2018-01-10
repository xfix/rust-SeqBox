use super::Error;
use super::sbx_specs;
use super::sbx_specs::Version;
use std::sync::{Arc, Mutex};

use super::time;

type SharedStats = Arc<Mutex<Stats>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Stats {
    pub sbx_version         : Version,
    pub meta_blocks_written : u64,
    pub data_blocks_written : u64,
    pub data_bytes_encoded  : u64,
    pub start_time          : u64,
    pub data_shards         : usize,
    pub parity_shards       : usize
}

impl Stats {
    pub fn new(version : Version) -> Self {
        Stats {
            sbx_version         : version,
            meta_blocks_written : 0,
            data_blocks_written : 0,
            data_bytes_encoded  : 0,
            start_time          : time::precise_time_ns(),
            data_shards         : 0,
            parity_shards       : 0
        }
    }

    pub fn time_elapsed(&self) -> u64 {
        time::precise_time_ns() - self.start_time
    }
}

fn reader(version : Version)
          -> Result<(), Error> {
    Ok(())
}

fn packer(version : Version)
          -> Result<(), Error> {
    use self::Version::*;
    match version {
        V1  | V2  | V3 => {
        },
        V11 | V12 | V13 => {
            
        }
    }
    Ok(())
}

fn hasher() {}

fn writer() -> Result<(), Error> {
    Ok(())
}

pub fn encode_file(in_filename  : String,
                   out_filename : String,
                   version      : Version)
                   -> Result<Stats, Error> {
    
    Ok(Stats::new(version))
}
