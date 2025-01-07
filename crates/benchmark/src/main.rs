use anstyle::Style;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::OpenOptions;
use std::hint::black_box;
use std::io::{self, Write as _};
use std::time::{Duration, Instant};
use terminal_colorsaurus::{color_palette, Error, QueryOptions, Result};

#[derive(Parser, Debug)]
struct Args {
    term: String,
    machine: String,
    #[arg(short = 'I', long, default_value_t = 10_000)]
    iterations: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    #[cfg(debug_assertions)]
    eprintln!(
        "{label_style}warning{label_style:#}{style}: you should run this example in release mode{style:#}",
        label_style = Style::new().bold().fg_color(Some(anstyle::AnsiColor::Yellow.into())),
        style = Style::new().bold()
    );

    eprintln!(
        "{style}Running benchmark with {iterations} iterations{style:#}",
        style = Style::new().bold(),
        iterations = args.iterations
    );

    let bar = ProgressBar::new(args.iterations as u64)
        .with_style(ProgressStyle::default_bar().progress_chars("██░"));

    let measurements = (0..args.iterations)
        .map(|_| bench())
        .inspect(|_| bar.inc(1))
        .collect::<Result<Vec<_>>>()?;
    bar.finish();

    let supported = match color_palette(QueryOptions::default()) {
        Ok(_) => true,
        Err(Error::UnsupportedTerminal(_)) => false,
        Err(e) => return Err(e),
    };

    save_results(&measurements, args.term, supported, args.machine)?;

    Ok(())
}

fn bench() -> Result<Duration> {
    let start = Instant::now();
    match black_box(color_palette(QueryOptions::default())) {
        Ok(_) | Err(Error::UnsupportedTerminal(_)) => Ok(start.elapsed()),
        Err(err) => Err(err),
    }
}

fn save_results(
    results: &[Duration],
    term: String,
    supported: bool,
    machine: String,
) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("benchmark/raw.tsv")?;
    for result in results {
        writeln!(
            file,
            "{}\t{}\t{}\t{}",
            term,
            result.as_nanos(),
            supported as u8,
            machine
        )?;
    }
    Ok(())
}
