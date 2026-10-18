use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::Parser;
use parol_runtime::Report;
use serde_json::Value as JsonValue;

use gon::*;
use tempfile::NamedTempFile;

#[derive(Parser)]
#[command(name = "gon", version, about = "CLI-utility for working with GON data", long_about = None)]
struct Args {
    /// What can I do for you?
    verb: Verb,
    /// How many characters to indent formatted output with?
    /// Only works with the `fmt` and `from` verbs.
    #[arg(long, short = 'w', default_value_t = 4)]
    indent_width: usize,
    /// What characters to indent formatted output with?
    /// Only works with the `fmt` and `into` verbs.
    #[arg(long, short = 'c', default_value_t = ' ')]
    indent_char: char,
    /// Put commas after last entries in lists and objects in formatted output?
    /// Only works with the `fmt` verb.
    #[arg(long, short, action)]
    trailing_commas: bool,
    /// The maximum width to which string literals get wrapped.
    /// This also squashes multiple spaces into a single one in every string. Use 0 to disable.
    /// Only works with the `fmt` and `from` verbs.
    #[arg(long, short, default_value_t = 0)]
    max_width: usize,
    /// Format in-place?
    /// WARNING: Writes the formatted output directly into the old file. ABSOLUTELY NO WARRANTY!
    /// Only works with `fmt` and `min`.
    #[arg(long, short, action)]
    in_place: bool,
    /// The input file. Leave empty for stdin.
    file: Option<PathBuf>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Verb {
    /// Minify the input
    Min,
    /// Format the input
    Fmt,
    /// Convert input to json
    Into,
    /// Convert json input to gon
    From,
    /// Verify the syntax of the given file
    Verify,
}

struct ErrorReporter;
impl Report for ErrorReporter {}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.verb {
        Verb::Min => {
            let value = get_gon_input(args.file.as_ref().cloned())?;
            print_or_write_to_file(&value.min_spell(), args.file)?;
        }
        Verb::Fmt => {
            let value = get_gon_input(args.file.as_ref().cloned())?;
            let spell_config = SpellConfig {
                indent_amount: args.indent_width,
                indent_char: args.indent_char,
                trailing_commas: args.trailing_commas,
                max_width: args.max_width,
            };
            print_or_write_to_file(&value.spell(spell_config)?, args.file)?;
        }
        Verb::Into => {
            let value = get_gon_input(args.file)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::Value::from(value))
                    .map_err(|e| Box::new(e))?
            );
        }
        Verb::From => {
            let json = get_json_input(args.file)?;
            let spell_config = SpellConfig {
                indent_amount: args.indent_width,
                indent_char: args.indent_char,
                trailing_commas: args.trailing_commas,
                max_width: args.max_width,
            };
            println!("{}", Value::from(json).spell(spell_config)?);
        }
        Verb::Verify => match get_gon_input(args.file) {
            Ok(value) => {
                println!("VALID");
                return Ok(());
            }
            Err(e) => {
                println!("INVALID");
                return Err(e);
            }
        },
    }
    Ok(())
}

fn print_or_write_to_file(out: &str, file: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    if let Some(file) = file {
        Ok(std::fs::write(file, out)?)
    } else {
        println!("{out}");
        Ok(())
    }
}

fn get_src(file: Option<&PathBuf>) -> Result<String, Box<dyn Error>> {
    let src = if let Some(file) = file {
        let file = File::open(file).map_err(|e| Box::new(e))?;
        std::io::read_to_string(file).map_err(|e| Box::new(e))?
    } else {
        let mut input = Vec::new();
        std::io::stdin()
            .read_to_end(&mut input)
            .map_err(|e| Box::new(e))?;
        String::from_utf8(input).map_err(|e| Box::new(e))?
    };
    Ok(src)
}

fn get_json_input(file: Option<PathBuf>) -> Result<JsonValue, Box<dyn Error>> {
    let src = get_src(file.as_ref())?;
    serde_json::from_str(&src).map_err(|e| e.into())
}

fn get_gon_input(file: Option<PathBuf>) -> Result<Value, Box<dyn Error>> {
    let src = get_src(file.as_ref())?;
    let res = if let Some(path) = &file {
        parse_file(&src, path).map_err(Into::into)
    } else {
        parse_file(&src, "").map_err(Into::into)
    };

    match res.as_ref() {
        Err(GonError::ParserErr(e)) => {
            let path = if let Some(path) = file {
                path
            } else {
                let tmp = NamedTempFile::new().expect("tempfile");
                std::fs::write(&tmp, &src).expect("tempfile io");
                tmp.path().into()
            };
            ErrorReporter::report_error(e, path);
        }
        _ => (),
    }

    res.map_err(Into::into)
}
