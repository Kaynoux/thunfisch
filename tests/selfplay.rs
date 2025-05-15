use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::sync::mpsc::channel;
use std::thread;
use thunfisch::communication::uci;
use thunfisch::prelude::*;

const STARTPOS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// Ein Reader, der seine Daten per Channel bekommt
struct ChanReader {
    rx: std::sync::mpsc::Receiver<String>,
    buf: Vec<u8>,
}
impl Read for ChanReader {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if self.buf.is_empty() {
            match self.rx.recv() {
                Ok(line) => self.buf = line.into_bytes(),
                Err(_) => return Ok(0),
            }
        }
        let n = std::cmp::min(out.len(), self.buf.len());
        out[..n].copy_from_slice(&self.buf[..n]);
        self.buf.drain(..n);
        Ok(n)
    }
}
impl BufRead for ChanReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.buf.is_empty() {
            match self.rx.recv() {
                Ok(line) => self.buf = line.into_bytes(),
                Err(_) => return Ok(&[]),
            }
        }
        Ok(&self.buf)
    }
    fn consume(&mut self, amt: usize) {
        self.buf.drain(..amt);
    }
}

// Ein Writer, der seine Daten per Channel abliefert
struct ChanWriter {
    tx: std::sync::mpsc::Sender<String>,
    buf: Vec<u8>,
}
impl Write for ChanWriter {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(data);
        Ok(data.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        if !self.buf.is_empty() {
            let s = String::from_utf8(self.buf.clone()).unwrap();
            self.tx.send(s).unwrap();
            self.buf.clear();
        }
        Ok(())
    }
}

#[test]
fn self_play_via_inproc_uci() {
    let (cmd_tx, cmd_rx) = channel();
    let (out_tx, out_rx) = channel();

    // Reader/Writer für handle_uci aufbauen
    let reader = ChanReader {
        rx: cmd_rx,
        buf: Vec::new(),
    };
    let writer = ChanWriter {
        tx: out_tx,
        buf: Vec::new(),
    };

    // UCI‐Schleife in Background‐Thread starten
    let handle = thread::spawn(move || {
        uci::handle_uci(reader, writer).unwrap();
    });

    // Unser lokales Board für FEN-Erzeugung und make_move
    let mut board = Board::from_fen(STARTPOS_FEN);

    // 10 Halbzüge spielen
    for _ in 0..10 {
        // 1) Position setzen
        let fen = board.generate_fen();
        cmd_tx.send(format!("position fen {}\n", fen)).unwrap();

        // 2) Suchbefehl
        cmd_tx.send("go wtime 100 btime 100\n".into()).unwrap();

        // 3) Antwort lesen: alle info ignorieren, bis bestmove kommt
        let mv_line = loop {
            let line = out_rx.recv().unwrap();
            print!("< {}", line); // mit --nocapture siehst Du’s live
            if !line.starts_with("info") {
                break line;
            }
        };
        assert!(mv_line.starts_with("bestmove "), "Unexpected: {}", mv_line);

        // 4) bestmove parsen und auf Board anwenden
        let mv_str = mv_line.split_whitespace().nth(1).unwrap();
        let decoded = DecodedMove::from_coords(mv_str.to_string(), &board);
        board.make_move(&decoded);
    }

    // Engine sauber beenden
    cmd_tx.send("quit\n".into()).unwrap();
    handle.join().unwrap();
}
