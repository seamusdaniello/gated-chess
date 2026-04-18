use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::game::Position;
use crate::pieces::Color;

#[derive(Clone, Debug)]
pub enum SessionConfig {
    Local,
    Host { bind_addr: String },
    Join { server_addr: String },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SessionRole {
    Local,
    Host,
    Client,
}

#[derive(Clone, Debug)]
pub enum NetworkEvent {
    WaitingForOpponent(String),
    Connected(String),
    RemoteMove(Position, Position),
    InvalidMove(String),
    Disconnected(String),
    Error(String),
}

#[derive(Clone, Debug)]
pub enum NetworkCommand {
    SubmitMove(Position, Position),
    BroadcastMove(Position, Position),
    RejectMove(String),
    Shutdown,
}

pub struct OnlineSession {
    role: SessionRole,
    local_color: Color,
    tx: Sender<NetworkCommand>,
    rx: Receiver<NetworkEvent>,
}

impl OnlineSession {
    pub fn host(bind_addr: String) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        thread::spawn(move || run_host(bind_addr, cmd_rx, event_tx));

        Self {
            role: SessionRole::Host,
            local_color: Color::White,
            tx: cmd_tx,
            rx: event_rx,
        }
    }

    pub fn join(server_addr: String) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        thread::spawn(move || run_client(server_addr, cmd_rx, event_tx));

        Self {
            role: SessionRole::Client,
            local_color: Color::Black,
            tx: cmd_tx,
            rx: event_rx,
        }
    }

    pub fn role(&self) -> SessionRole {
        self.role
    }

    pub fn local_color(&self) -> Color {
        self.local_color
    }

    pub fn try_recv(&self) -> Option<NetworkEvent> {
        self.rx.try_recv().ok()
    }

    pub fn send(&self, command: NetworkCommand) {
        let _ = self.tx.send(command);
    }
}

impl Drop for OnlineSession {
    fn drop(&mut self) {
        let _ = self.tx.send(NetworkCommand::Shutdown);
    }
}

fn run_host(bind_addr: String, cmd_rx: Receiver<NetworkCommand>, event_tx: Sender<NetworkEvent>) {
    let listener = match TcpListener::bind(&bind_addr) {
        Ok(listener) => listener,
        Err(err) => {
            let _ = event_tx.send(NetworkEvent::Error(format!(
                "Failed to bind {}: {}",
                bind_addr, err
            )));
            return;
        }
    };

    let _ = event_tx.send(NetworkEvent::WaitingForOpponent(bind_addr.clone()));

    let (stream, peer_addr) = match listener.accept() {
        Ok(connection) => connection,
        Err(err) => {
            let _ = event_tx.send(NetworkEvent::Error(format!("Accept failed: {}", err)));
            return;
        }
    };

    let _ = stream.set_nodelay(true);
    let _ = event_tx.send(NetworkEvent::Connected(peer_addr.to_string()));

    let reader_stream = match stream.try_clone() {
        Ok(clone) => clone,
        Err(err) => {
            let _ = event_tx.send(NetworkEvent::Error(format!(
                "Failed to clone socket: {}",
                err
            )));
            return;
        }
    };

    let reader_tx = event_tx.clone();
    thread::spawn(move || host_reader_loop(reader_stream, reader_tx));

    writer_loop(stream, cmd_rx, event_tx, true);
}

fn run_client(
    server_addr: String,
    cmd_rx: Receiver<NetworkCommand>,
    event_tx: Sender<NetworkEvent>,
) {
    let stream = match TcpStream::connect(&server_addr) {
        Ok(stream) => stream,
        Err(err) => {
            let _ = event_tx.send(NetworkEvent::Error(format!(
                "Failed to connect to {}: {}",
                server_addr, err
            )));
            return;
        }
    };

    let _ = stream.set_nodelay(true);
    let _ = event_tx.send(NetworkEvent::Connected(server_addr));

    let reader_stream = match stream.try_clone() {
        Ok(clone) => clone,
        Err(err) => {
            let _ = event_tx.send(NetworkEvent::Error(format!(
                "Failed to clone socket: {}",
                err
            )));
            return;
        }
    };

    let reader_tx = event_tx.clone();
    thread::spawn(move || client_reader_loop(reader_stream, reader_tx));

    writer_loop(stream, cmd_rx, event_tx, false);
}

fn host_reader_loop(stream: TcpStream, event_tx: Sender<NetworkEvent>) {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(0) => {
                let _ = event_tx.send(NetworkEvent::Disconnected(
                    "Client disconnected".to_string(),
                ));
                break;
            }
            Ok(_) => {
                if let Some((from, to)) = parse_move_line(&line, "TRY") {
                    let _ = event_tx.send(NetworkEvent::RemoteMove(from, to));
                } else {
                    let _ = event_tx.send(NetworkEvent::Error(format!(
                        "Malformed client message: {}",
                        line.trim()
                    )));
                }
            }
            Err(err) => {
                let _ = event_tx.send(NetworkEvent::Disconnected(format!(
                    "Client connection error: {}",
                    err
                )));
                break;
            }
        }
    }
}

fn client_reader_loop(stream: TcpStream, event_tx: Sender<NetworkEvent>) {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(0) => {
                let _ = event_tx.send(NetworkEvent::Disconnected("Host disconnected".to_string()));
                break;
            }
            Ok(_) => {
                let trimmed = line.trim();

                if let Some((from, to)) = parse_move_line(trimmed, "APPLY") {
                    let _ = event_tx.send(NetworkEvent::RemoteMove(from, to));
                } else if let Some(reason) = trimmed.strip_prefix("INVALID ") {
                    let _ = event_tx.send(NetworkEvent::InvalidMove(reason.to_string()));
                } else {
                    let _ = event_tx.send(NetworkEvent::Error(format!(
                        "Malformed host message: {}",
                        trimmed
                    )));
                }
            }
            Err(err) => {
                let _ = event_tx.send(NetworkEvent::Disconnected(format!(
                    "Host connection error: {}",
                    err
                )));
                break;
            }
        }
    }
}

fn writer_loop(
    mut stream: TcpStream,
    cmd_rx: Receiver<NetworkCommand>,
    event_tx: Sender<NetworkEvent>,
    is_host: bool,
) {
    while let Ok(command) = cmd_rx.recv() {
        let line = match command {
            NetworkCommand::SubmitMove(from, to) => format_move_line("TRY", from, to),
            NetworkCommand::BroadcastMove(from, to) => format_move_line("APPLY", from, to),
            NetworkCommand::RejectMove(reason) => format!("INVALID {}\n", reason),
            NetworkCommand::Shutdown => break,
        };

        if let Err(err) = stream.write_all(line.as_bytes()) {
            let side = if is_host { "client" } else { "host" };
            let _ = event_tx.send(NetworkEvent::Disconnected(format!(
                "Failed to write to {}: {}",
                side, err
            )));
            break;
        }
    }
}

fn format_move_line(prefix: &str, from: Position, to: Position) -> String {
    format!(
        "{} {} {} {} {}\n",
        prefix, from.row, from.col, to.row, to.col
    )
}

fn parse_move_line(line: &str, prefix: &str) -> Option<(Position, Position)> {
    let mut parts = line.split_whitespace();

    if parts.next()? != prefix {
        return None;
    }

    let from_row = parts.next()?.parse().ok()?;
    let from_col = parts.next()?.parse().ok()?;
    let to_row = parts.next()?.parse().ok()?;
    let to_col = parts.next()?.parse().ok()?;

    Some((
        Position {
            row: from_row,
            col: from_col,
        },
        Position {
            row: to_row,
            col: to_col,
        },
    ))
}
