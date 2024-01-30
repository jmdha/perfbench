mod engine_wrapper;
mod test_suite;

use clap::Parser;
use std::{error::Error, path::PathBuf, time::Instant};

use crate::{engine_wrapper::EngineWrapper, test_suite::TestSuite};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// An optional custom test_suite
    #[arg(short, long)]
    test_suite: Option<PathBuf>,
    /// Default number of iterations to run for each test case
    #[arg(short, long, default_value_t = 10)]
    iterations: usize,
    /// Whether to estimate NPS (Not implemented)
    #[arg(short, long)]
    nps: bool,
    /// Whether to validate perft output (Not implemented)
    #[arg(short, long)]
    validate: bool,
    #[arg(required = true)]
    engines: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Loading test suite...");
    let test_suite = match args.test_suite {
        Some(path) => TestSuite::parse_csv(path)?,
        None => TestSuite::default(),
    };
    println!("Loading engines...");
    let engines: Vec<EngineWrapper> = args
        .engines
        .into_iter()
        .map(|p| {
            println!("\tLoading {:?}...", p);
            EngineWrapper::new(p)
        })
        .collect();
    println!("Running perft...");

    let engine_names: Vec<&str> = engines.iter().map(|e| e.name()).collect();
    let max_engine_name_len: usize = engine_names.iter().map(|n| n.len()).max().unwrap() + 1;

    let mut header = "".to_owned();
    for name in engine_names.iter() {
        header.push_str(&format!("{:<1$}", name, max_engine_name_len));
    }
    println!("{}fen", header);
    for test_case in test_suite.0.iter() {
        let mut times: Vec<Vec<u128>> = vec![vec![]; engines.len()];
        for (i, engine) in engines.iter().enumerate() {
            for _ in 0..test_case.iterations.unwrap_or(args.iterations) {
                let t = Instant::now();

                let _ = engine.run(test_case);

                let elapsed: u128 = t.elapsed().as_millis();
                times[i].push(elapsed);
            }
        }
        let avgs: Vec<u128> = times
            .iter()
            .map(|v| v.iter().sum::<u128>() / v.len() as u128)
            .collect();
        let mut line = "".to_owned();
        for avg in avgs.iter() {
            line.push_str(&format!("{:<1$}", avg, max_engine_name_len));
        }
        println!("{}{}", line, test_case.fen);
    }

    Ok(())
}
