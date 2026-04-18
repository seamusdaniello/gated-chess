use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

struct WaitingEntry {
    stream: TcpStream,
    initial_seconds: u32,
    increment_seconds: u32,
    notify_tx: mpsc::Sender<()>,
}

type Queue = Arc<Mutex<Option<WaitingEntry>>>;

fn main() {
    let port = std::env::args().nth(1).unwrap_or_else(|| "4000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).expect("Failed to bind");
    println!("Gated Chess relay listening on {}", addr);

    let queue: Queue = Arc::new(Mutex::new(None));

    for stream in listener.incoming().flatten() {
        let queue = Arc::clone(&queue);
        thread::spawn(move || handle(stream, queue));
    }
}

fn handle(stream: TcpStream, queue: Queue) {
    let _ = stream.set_nodelay(true);

    let read_clone = match stream.try_clone() {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut reader = BufReader::new(read_clone);
    let mut line = String::new();
    if reader.read_line(&mut line).unwrap_or(0) == 0 {
        return;
    }

    let (initial_seconds, increment_seconds) = match parse_find(line.trim()) {
        Some(tc) => tc,
        None => {
            eprintln!("Bad FIND from client: {:?}", line.trim());
            return;
        }
    };

    let mut my_stream = stream;
    if my_stream.write_all(b"WAITING\n").is_err() {
        return;
    }

    let (notify_tx, notify_rx) = mpsc::channel::<()>();

    let paired = {
        let mut q = queue.lock().unwrap();
        if let Some(waiting) = q.take() {
            let _ = waiting.notify_tx.send(());
            Some((
                waiting.stream,
                my_stream,
                waiting.initial_seconds,
                waiting.increment_seconds,
            ))
        } else {
            let queue_stream = match my_stream.try_clone() {
                Ok(c) => c,
                Err(_) => return,
            };
            *q = Some(WaitingEntry {
                stream: queue_stream,
                initial_seconds,
                increment_seconds,
                notify_tx,
            });
            None
        }
    };

    let (mut white, mut black, tc_init, tc_inc) = match paired {
        Some(p) => p,
        None => {
            let _ = notify_rx.recv();
            return;
        }
    };

    let white_msg = format!("START WHITE\nCONFIG {} {}\n", tc_init, tc_inc);
    let black_msg = format!("START BLACK\nCONFIG {} {}\n", tc_init, tc_inc);

    if white.write_all(white_msg.as_bytes()).is_err()
        || black.write_all(black_msg.as_bytes()).is_err()
    {
        return;
    }

    println!("Match started ({} + {} s)", tc_init, tc_inc);

    let white_read = match white.try_clone() {
        Ok(c) => c,
        Err(_) => return,
    };
    let black_write = match black.try_clone() {
        Ok(c) => c,
        Err(_) => return,
    };

    thread::spawn(move || relay_loop(white_read, black_write));
    relay_loop(black, white);

    println!("Match ended");
}

fn relay_loop(reader_stream: TcpStream, mut writer: TcpStream) {
    let mut reader = BufReader::new(reader_stream);
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if writer.write_all(line.as_bytes()).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}

fn parse_find(line: &str) -> Option<(u32, u32)> {
    let mut parts = line.split_whitespace();
    if parts.next()? != "FIND" {
        return None;
    }
    let initial = parts.next()?.parse().ok()?;
    let increment = parts.next()?.parse().ok()?;
    Some((initial, increment))
}
