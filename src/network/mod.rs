use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::game::Position;
use crate::pieces::Color;
use crate::time_control::TimeControl;

#[derive(Clone, Debug)]
pub enum SessionConfig {
    Local,
    Host { bind_addr: String },
    Join { server_addr: String },
    FindMatch { addr: String },
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
    TimeControlUpdated(TimeControl),
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
    state: Arc<Mutex<SessionState>>,
    tx: Sender<NetworkCommand>,
    rx: Receiver<NetworkEvent>,
}

#[derive(Copy, Clone)]
struct SessionState {
    role: SessionRole,
    local_color: Color,
}

impl OnlineSession {
    pub fn host(bind_addr: String, time_control: TimeControl) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        let state = Arc::new(Mutex::new(SessionState {
            role: SessionRole::Host,
            local_color: Color::White,
        }));

        thread::spawn({
            let state = Arc::clone(&state);
            move || run_host(bind_addr, time_control, cmd_rx, event_tx, state)
        });

        Self {
            state,
            tx: cmd_tx,
            rx: event_rx,
        }
    }

    pub fn join(server_addr: String) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        let state = Arc::new(Mutex::new(SessionState {
            role: SessionRole::Client,
            local_color: Color::Black,
        }));

        thread::spawn({
            let state = Arc::clone(&state);
            move || run_client(server_addr, cmd_rx, event_tx, state)
        });

        Self {
            state,
            tx: cmd_tx,
            rx: event_rx,
        }
    }

    pub fn find_match(addr: String, time_control: TimeControl) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        let state = Arc::new(Mutex::new(SessionState {
            role: SessionRole::Host,
            local_color: Color::White,
        }));

        thread::spawn({
            let state = Arc::clone(&state);
            move || run_find_match(addr, time_control, cmd_rx, event_tx, state)
        });

        Self {
            state,
            tx: cmd_tx,
            rx: event_rx,
        }
    }

    pub fn role(&self) -> SessionRole {
        self.state
            .lock()
            .map(|state| state.role)
            .unwrap_or(SessionRole::Local)
    }

    pub fn local_color(&self) -> Color {
        self.state
            .lock()
            .map(|state| state.local_color)
            .unwrap_or(Color::White)
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

fn run_host(
    bind_addr: String,
    time_control: TimeControl,
    cmd_rx: Receiver<NetworkCommand>,
    event_tx: Sender<NetworkEvent>,
    state: Arc<Mutex<SessionState>>,
) {
    set_state(&state, SessionRole::Host, Color::White);
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

    let (mut stream, peer_addr) = match listener.accept() {
        Ok(connection) => connection,
        Err(err) => {
            let _ = event_tx.send(NetworkEvent::Error(format!("Accept failed: {}", err)));
            return;
        }
    };

    let _ = stream.set_nodelay(true);

    if let Err(err) = stream.write_all(format_time_control_line(time_control).as_bytes()) {
        let _ = event_tx.send(NetworkEvent::Error(format!(
            "Failed to send time control: {}",
            err
        )));
        return;
    }

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
    state: Arc<Mutex<SessionState>>,
) {
    set_state(&state, SessionRole::Client, Color::Black);
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

fn run_find_match(
    addr: String,
    time_control: TimeControl,
    cmd_rx: Receiver<NetworkCommand>,
    event_tx: Sender<NetworkEvent>,
    state: Arc<Mutex<SessionState>>,
) {
    let _ = event_tx.send(NetworkEvent::WaitingForOpponent(format!(
        "Searching for match at {}",
        addr
    )));

    if let Some(socket_addr) = resolve_socket_addr(&addr) {
        if TcpStream::connect_timeout(&socket_addr, Duration::from_millis(700)).is_ok() {
            run_client(addr, cmd_rx, event_tx, state);
            return;
        }
    }

    run_host(addr, time_control, cmd_rx, event_tx, state);
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

                if let Some(time_control) = parse_time_control_line(trimmed) {
                    let _ = event_tx.send(NetworkEvent::TimeControlUpdated(time_control));
                } else if let Some((from, to)) = parse_move_line(trimmed, "APPLY") {
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

fn resolve_socket_addr(addr: &str) -> Option<std::net::SocketAddr> {
    addr.to_socket_addrs().ok()?.next()
}

fn set_state(state: &Arc<Mutex<SessionState>>, role: SessionRole, local_color: Color) {
    if let Ok(mut session_state) = state.lock() {
        session_state.role = role;
        session_state.local_color = local_color;
    }
}

fn format_time_control_line(time_control: TimeControl) -> String {
    format!(
        "CONFIG {} {}\n",
        time_control.initial_seconds, time_control.increment_seconds
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

fn parse_time_control_line(line: &str) -> Option<TimeControl> {
    let mut parts = line.split_whitespace();

    if parts.next()? != "CONFIG" {
        return None;
    }

    Some(TimeControl::new(
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
    ))
}
