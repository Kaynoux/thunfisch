//! Utilities for reading and writing training positions in EPD-like format.
//!
//! The training data parser supports lines of the form:
//! `FEN side castling ep "result";` and converts them into a simplified
//! `TrainingData` struct that stores the FEN and the game result.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// One training position used for Texel tuning.
///
/// The stored FEN is normalized to include the standard halfmove and fullmove
/// counters (`0 1`) because the engine needs a complete FEN string for position
/// reconstruction.
#[derive(Clone)]
pub struct TrainingSample {
    pub fen: String,
    pub result: GameResult,
}

/// The outcome of the game used for supervised tuning labels.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameResult {
    WhiteWin,
    Draw,
    BlackWin,
}

impl From<GameResult> for f64 {
    fn from(value: GameResult) -> Self {
        match value {
            GameResult::WhiteWin => 1.0,
            GameResult::Draw => 0.5,
            GameResult::BlackWin => 0.0,
        }
    }
}

impl TrainingSample {
    /// Reads an EPD-style training data file and returns all valid positions.
    ///
    /// Blank lines and comment lines starting with `#` are ignored.
    /// Each valid line must contain a FEN prefix plus a quoted result.
    pub fn read_epd_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Self>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut positions = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            let line = line?;
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            match Self::parse_epd_line(trimmed) {
                Ok(Some(position)) => positions.push(position),
                Ok(None) => continue,
                Err(error) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{} at line {}", error, index + 1),
                    ));
                }
            }
        }

        Ok(positions)
    }

    /// Writes the given training positions back to an EPD-style file.
    ///
    /// The output format preserves the FEN and the original result label.
    pub fn write_epd_file<P: AsRef<Path>>(path: P, positions: &[Self]) -> io::Result<()> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = File::create(path)?;

        for position in positions {
            writeln!(file, "{} \"{}\";", position.fen, position.result)?;
        }

        Ok(())
    }

    /// Parses a single EPD line into a normalized `TrainingData` entry.
    ///
    /// The function supports an abbreviated FEN prefix and appends `0 1`
    /// for the halfmove/fullmove counters required by the engine.
    fn parse_epd_line(line: &str) -> Result<Option<Self>, String> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return Ok(None);
        }

        let quote_start = trimmed
            .find('"')
            .ok_or_else(|| "missing quoted game result".to_string())?;
        let quote_end = trimmed[quote_start + 1..]
            .find('"')
            .map(|offset| quote_start + 1 + offset)
            .ok_or_else(|| "missing closing quote for game result".to_string())?;

        let result_text = trimmed[quote_start + 1..quote_end].trim();
        let result = GameResult::parse(result_text)?;

        let fen_tokens: Vec<&str> = trimmed[..quote_start].split_whitespace().collect();
        if fen_tokens.len() < 4 {
            return Err("EPD line does not contain enough FEN fields".to_string());
        }

        let fen = format!(
            "{} {} {} {} 0 1",
            fen_tokens[0], fen_tokens[1], fen_tokens[2], fen_tokens[3]
        );

        Ok(Some(Self { fen, result }))
    }
}

impl GameResult {
    /// Converts a quoted label into a `GameResult` value.
    ///
    /// Supported labels are `1-0`, `1/2-1/2`, and `0-1`.
    fn parse(text: &str) -> Result<Self, String> {
        match text {
            "1-0" => Ok(GameResult::WhiteWin),
            "1/2-1/2" => Ok(GameResult::Draw),
            "0-1" => Ok(GameResult::BlackWin),
            _ => Err(format!("unsupported game result '{}'", text)),
        }
    }

    /// Formats the enum back to the EPD-style result string.
    fn as_str(&self) -> &'static str {
        match self {
            GameResult::WhiteWin => "1-0",
            GameResult::Draw => "1/2-1/2",
            GameResult::BlackWin => "0-1",
        }
    }
}

impl std::fmt::Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_epd_line() {
        let line = r#"r2qkr2/p1pp1ppp/1pn1pn2/2P5/3Pb3/2N1P3/PP3PPP/R1B1KB1R b KQq - c9 "0-1";"#;
        let result = TrainingSample::parse_epd_line(line).expect("should parse line").unwrap();

        assert_eq!(result.fen, "r2qkr2/p1pp1ppp/1pn1pn2/2P5/3Pb3/2N1P3/PP3PPP/R1B1KB1R b KQq - 0 1");
        matches!(result.result, GameResult::BlackWin);
    }
}
