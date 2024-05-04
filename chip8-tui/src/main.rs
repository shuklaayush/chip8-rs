mod args;
mod drivers;
mod error;
mod terminal;

use args::CmdArgs;
use chip8_core::{
    input::{InputEvent, InputKind},
    keypad::Key,
    Chip8,
};
use clap::Parser;
use csv::{Reader, Writer, WriterBuilder};
use drivers::input::CsvRecord;
use error::TuiError;
use rand::{random, rngs::StdRng, SeedableRng};
use std::fs::{self, OpenOptions};
use terminal::{restore_terminal, setup_terminal};

use crate::drivers::{
    audio::TerminalAudio, display::TerminalDisplay, input::TerminalKeyboardInput,
};

async fn app() -> Result<(), TuiError> {
    let args = CmdArgs::parse();

    let rom = fs::read(args.rom).map_err(|e| TuiError::RomReadError(e.to_string()))?;
    let terminal =
        setup_terminal(args.headless).map_err(|e| TuiError::TerminalSetupError(e.to_string()))?;

    let (inputs, input_writer) = if let Some(input_file) = &args.input_file {
        if args.overwrite {
            let writer = Writer::from_path(input_file).expect("Failed to open or create file");
            (vec![], Some(writer))
        } else {
            let mut reader = Reader::from_path(input_file).expect("Failed to open file");
            let parsed: Vec<(u64, InputEvent)> = reader
                .deserialize()
                .map(|result| {
                    let record: CsvRecord =
                        result.map_err(|e| TuiError::InputError(e.to_string()))?;
                    let key = Key::try_from(record.key)
                        .map_err(|e| TuiError::InputError(e.to_string()))?;
                    let kind = InputKind::try_from(record.kind)
                        .map_err(|e| TuiError::InputError(e.to_string()))?;
                    Ok((record.clk, InputEvent { key, kind }))
                })
                .collect::<Result<_, _>>()?;
            let f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(input_file)
                .expect("Failed to open file");
            let writer = WriterBuilder::new()
                .has_headers(parsed.is_empty())
                .from_writer(f);
            (parsed, Some(writer))
        }
    } else {
        (vec![], None)
    };

    let input_driver = TerminalKeyboardInput::new(input_writer);
    let display_driver = {
        if !args.headless {
            Some(TerminalDisplay::new(
                terminal,
                args.refresh_rate,
                args.bg_color,
                args.fg_color,
                args.border_color,
            ))
        } else {
            None
        }
    };
    let audio_driver = {
        if !args.headless {
            Some(TerminalAudio::default())
        } else {
            None
        }
    };

    let seeded_rng = StdRng::seed_from_u64(args.random_seed.unwrap_or(random()));
    let mut chip8 = Chip8::new(args.clk_freq, seeded_rng, inputs);
    let res = chip8
        .load_and_run(rom.as_slice(), input_driver, display_driver, audio_driver)
        .await
        .map_err(TuiError::Chip8Error);

    restore_terminal(args.headless).map_err(|e| TuiError::TerminalRestoreError(e.to_string()))?;
    res
}

#[tokio::main]
async fn main() {
    if let Err(e) = app().await {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
