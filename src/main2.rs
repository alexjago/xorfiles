jhbjhblhbelvhbelvheb
use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};


use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    if opt.verbose > 0 {
        eprintln!("{:#?}", opt);
    }
    
    let mut streams = Vec::new();
    
    // treat the files like the streams of bytes they are
    for fname in opt.files {
        streams.push(BufReader::new(File::open(fname)?).bytes());
    }
    
    eprintln!("{:#?}", &streams);
    
    // create our output
    
    let sto = io::stdout();
    
    let mut out = BufWriter::new(match opt.output {
        Some(p) => Box::new(File::create(p)?) as Box<dyn Write>,
        None => {Box::new(sto) as Box<dyn Write>}
    });
    
            
    let mut done = false;
    
    // the main game: iterate over each stream
    while !done {
        let mut byte_me = [0_u8; 1];
        for s in &mut streams {
            let n = s.next();
            if n.is_none() {
                done = true;
                break;
            } else {
                byte_me[0] ^= n.unwrap().unwrap();
            }
        }
        if !done {
            out.write(&byte_me)?;
        }
    }
    
    out.flush()?;
    
    Ok(())
}
