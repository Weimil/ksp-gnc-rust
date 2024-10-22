#![allow(warnings)]

mod ksp;
mod general;

use crate::general::state::State;
use crate::general::terminal::Terminal;
use crate::ksp::telemetry::{Telemetry, TelemetryStream};

use krpc_client::services::space_center::SpaceCenter;
use krpc_client::Client;
use std::fmt::Debug;
use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const TPS: u64 = 20;
const SEC: Duration = Duration::from_secs(1);
const MSPT: Duration = Duration::from_millis(1000 / TPS);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut total_ticks: u64 = 0;
    let mut terminal = Terminal::new(20, 64, 3, 1);
    let mut state = State {};

    let client = Client::new("GNC game loop", "127.0.0.1", 51000, 51001)
        .expect("\n\n\n  → Turn on kRPC server in KPS ←  \n\n\n\n");
    let space_center = SpaceCenter::new(client.clone());
    let vessel = space_center.get_active_vessel()?;
    let telemetry_stream = TelemetryStream::new(&vessel, &space_center)?;

    loop {
        let current_time = SystemTime::now();
        let telemetry = telemetry_stream.data()?;

        state = control(&state, &terminal, &telemetry);

        terminal.telemetry(&telemetry);
        terminal.render();
        // terminal.debug(&state);

        let duration = SystemTime::now().duration_since(current_time).unwrap();

        if MSPT > duration {
            sleep(MSPT - duration);
        } else {
            println! ("Tick took more than 50ms to process")
        }

        total_ticks += 1;
    }
}

fn control(state: &State, terminal: &Terminal, telemetry: &Telemetry) -> State {
    return State {};
}

fn dd(telemetry: impl Debug) {
    print!("{}", format!("{:?}", telemetry)
        .replace("{ ", "{\n\t")
        .replace(" }", "\n}")
        .replace(", ", ",\n\t")
    );
    exit(1);
}
fn dump(telemetry: impl Debug) {
    print!("{}", format!("{:?}", telemetry)
        .replace("{ ", "{\n\t")
        .replace(" }", "\n}")
        .replace(", ", ",\n\t")
    );
}
