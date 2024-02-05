use clap::Parser;
use std::error::Error;
use std::io;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::fs::File;
use std::str;

const BUFFER_SIZE: usize = 8192;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug, Default)]
#[clap(author="Cristopher Holm", version, about="A JSON correcter")]
#[command(author, version, about, long_about = None)]
struct Config {
    /// Input file to read from (optional, default is STDIN)
    #[clap(short, long)]
    input: Option<String>,
    /// Output file to write to (optional, default is STDOUT)
    #[clap(short, long)]
    output: Option<String>,
}

fn main() -> MyResult<()> {
    let config = Config::parse();

    // create a buffered reader from either STDIN or the supplied input file
    let mut reader: Box<dyn BufRead>;
    if config.input.is_some() {
        let file = File::open(config.input.unwrap())?;
        reader = Box::new(BufReader::with_capacity(BUFFER_SIZE, file));
    } else {
        let stdin = io::stdin();
        reader = Box::new(BufReader::with_capacity(BUFFER_SIZE, stdin.lock())) as Box<dyn BufRead>
    }

    // create a buffered writer from either STDOUT or the supplied output file
    let mut writer: Box<dyn Write>;
    if config.output.is_some() {
        let file = File::create(config.output.unwrap())?;
        writer = Box::new(BufWriter::with_capacity(BUFFER_SIZE, file));
    } else {
        let stdout = io::stdout();
        writer = Box::new(stdout.lock()) as Box<dyn Write>
    }

    let _ = run(&mut reader, &mut writer).unwrap();

    Ok(())
}

fn run(reader: &mut Box<dyn BufRead>, writer: &mut Box<dyn Write>) -> MyResult<i32> {
    let mut corrections: i32 = 0;
    let mut parsing_string: bool = false;
    let mut last_char: char = '\0';

    loop {
        let buffer = reader.fill_buf()?;
        let buffer_len = buffer.len();
        if buffer_len == 0 {
            // must be at EOF
            break;
        }

        let fragment = str::from_utf8(&buffer).unwrap();
        let corrected = fragment.chars()
            .map(|ch| {
                if ch == '"' && last_char != '\\' {
                    // we've either started or finished parsing a string
                    parsing_string = !parsing_string
                } else if ch == ';' {
                    if !parsing_string && last_char == '"' {
                        // the last thing parsed was a string. a semi-colon following a double-quoted string
                        // is never valid json and we can make the assumption that this is the end of a
                        // key and that this semi-colon should be replaced with a colon.
                        corrections += 1;
                        last_char = ':';
                        return ':';
                    }
                }
                if !ch.is_whitespace() {
                    last_char = ch;
                }
                return ch;
            }).collect::<String>();
        writer.write_all(corrected.as_bytes())?;

        // don't reread the same data
        reader.consume(buffer_len);
    }

    Ok(corrections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line_no_problems() {
        let file = File::open("tests/input/single-line-no-problems.json").unwrap();
        let mut reader: Box<dyn BufRead> = Box::new(BufReader::with_capacity(30, file));
        let stdout = io::stdout();
        let mut writer = Box::new(stdout.lock()) as Box<dyn Write>;
        let corrections = run(&mut reader, &mut writer).unwrap();
        assert_eq!(corrections, 0);
    }

    #[test]
    fn single_line_with_2_problems() {
        let file = File::open("tests/input/single-line-with-2-problems.json").unwrap();
        let mut reader: Box<dyn BufRead> = Box::new(BufReader::with_capacity(30, file));
        let stdout = io::stdout();
        let mut writer = Box::new(stdout.lock()) as Box<dyn Write>;
        let corrections = run(&mut reader, &mut writer).unwrap();
        assert_eq!(corrections, 2);
    }

    #[test]
    fn multiple_lines_no_problems() {
        let file = File::open("tests/input/multiple-lines-no-problems.json").unwrap();
        let mut reader: Box<dyn BufRead> = Box::new(BufReader::with_capacity(30, file));
        let stdout = io::stdout();
        let mut writer = Box::new(stdout.lock()) as Box<dyn Write>;
        let corrections = run(&mut reader, &mut writer).unwrap();
        assert_eq!(corrections, 0);
    }

    #[test]
    fn multiple_lines_with_3_problems() {
        let file = File::open("tests/input/multiple-lines-with-3-problems.json").unwrap();
        let mut reader: Box<dyn BufRead> = Box::new(BufReader::with_capacity(30, file));
        let stdout = io::stdout();
        let mut writer = Box::new(stdout.lock()) as Box<dyn Write>;
        let corrections = run(&mut reader, &mut writer).unwrap();
        assert_eq!(corrections, 3);
    }
}
