use crate::ksp::telemetry::Telemetry;
use crate::State;
use std::collections::VecDeque;
use std::io::{stdout, Write};

const CHARS: [&str; 11] = ["─", "│", "╭", "┬", "╮", "├", "┼", "┤", "╰", "┴", "╯"];

#[derive(Debug)]
pub(crate) struct Terminal {
    width: usize,
    length: usize,

    announcements: VecDeque<String>,
    max_announcements: usize,

    telemetries: VecDeque<String>,
    max_telemetry: usize,

    debugs: VecDeque<String>,
    max_debug: usize,
}

impl Terminal {
    pub(crate) fn new(length: usize, width: usize, max_announcements: usize, max_debug: usize) -> Terminal {
        let mut max_telemetry = length - 2;

        if max_announcements > 0 {
            max_telemetry -= max_announcements + 2;
        }

        if max_debug > 0 {
            max_telemetry -= max_debug + 2;
        }

        Terminal {
            width,
            length,

            announcements: VecDeque::from(vec![" ".repeat(width - 4); max_announcements]),
            max_announcements,

            telemetries: VecDeque::from(vec![" ".repeat(width - 4); max_telemetry]),
            max_telemetry,

            debugs: VecDeque::from(vec![" ".repeat(width - 4); max_debug]),
            max_debug,
        }
    }

    fn top_line(&self) -> String { format!("{}{}{}", CHARS[2], CHARS[0].repeat(self.width - 2), CHARS[4]) }

    fn bot_line(&self) -> String { format!("{}{}{}", CHARS[8], CHARS[0].repeat(self.width - 2), CHARS[10]) }

    pub(crate) fn announcement(&mut self, str: &str) {
        self.announcements.pop_back();
        self.announcements.push_front(str_right_pad(str, self.width - 4, " "));
    }

    pub(crate) fn debug(&mut self, state: &State) {}

    pub(crate) fn telemetry(&mut self, telemetry: &Telemetry) {
        let txt_len = 18;
        let tel_len = self.width - 4 - txt_len - 1;

        self.telemetries.clear();
        self.telemetries.push_back(format!("{}:{}",
            str_right_pad("Periapsis", txt_len, " "),
            str_left_pad(format!("{:.2}", telemetry.periapsis).as_str(), tel_len, " ")
        ));
        self.telemetries.push_back(format!("{}:{}",
            str_right_pad("Time To Periapsis", txt_len, " "),
            str_left_pad(format!("{:.2}", telemetry.time_to_periapsis).as_str(), tel_len, " ")
        ));
        self.telemetries.push_back(format!("{}:{}",
            str_right_pad("Apoapsis", txt_len, " "),
            str_left_pad(format!("{:.2}", telemetry.apoapsis).as_str(), tel_len, " ")
        ));
        self.telemetries.push_back(format!("{}:{}",
            str_right_pad("Time To Apoapsis", txt_len, " "),
            str_left_pad(format!("{:.2}", telemetry.time_to_apoapsis).as_str(), tel_len, " ")
        ));
        self.telemetries.push_back(format!("{}:{}",
            str_right_pad("Inclination", txt_len, " "),
            str_left_pad(format!("{:.2}", telemetry.inclination).as_str(), tel_len, " ")
        ));
        self.telemetries.push_back(format!("{}:{}",
            str_right_pad("Dynamic Pressure", txt_len, " "),
            str_left_pad(format!("{:.2}", telemetry.dynamic_pressure).as_str(), tel_len, " ")
        ));
        self.telemetries.push_back(format!("{}:{}",
            str_right_pad("Surface Speed", txt_len, " "),
            str_left_pad(format!("{:.2}", telemetry.surface_speed).as_str(), tel_len, " ")
        ));
        self.telemetries.push_back(format!("{}:{}",
            str_right_pad("Orbital Speed", txt_len, " "),
            str_left_pad(format!("{:.2}", telemetry.orbital_speed).as_str(), tel_len, " ")
        ));
        self.telemetries.push_back(format!("{}:{}",
            str_right_pad("Sea Altitude", txt_len, " "),
            str_left_pad(format!("{:.2}", telemetry.mean_altitude).as_str(), tel_len, " ")
        ));
        self.telemetries.push_back(format!("{}:{}",
            str_right_pad("Surface Altitude", txt_len, " "),
            str_left_pad(format!("{:.2}", telemetry.surface_altitude).as_str(), tel_len, " ")
        ));
    }

    pub(crate) fn clear(&self) {
        print!("{}", "\n".repeat(self.length));
    }

    pub(crate) fn render(&self) {
        print!("\x1b[{}A", self.length);

        if self.max_announcements > 0 {
            let mut txt_announcements = String::new();

            for (i, str) in self.announcements.iter().enumerate() {
                let mut str = str.clone();
                if i != 0 { str = format!("\x1b[38;5;8m{}\x1b[0m", str) }
                txt_announcements.push_str(format!("{} {} {}\n", CHARS[1], str, CHARS[1]).as_str())
            }

            println!("{}", format!("{}\n{}{}", self.top_line(), txt_announcements, self.bot_line()));
        }

        if self.max_telemetry > 0 {
            let mut text_telemetry = String::new();

            for (i, str) in self.telemetries.iter().enumerate() {
                text_telemetry.push_str(format!("{} {} {}\n", CHARS[1], str.clone(), CHARS[1]).as_str())
            }

            println!("{}", format!("{}\n{}{}", self.top_line(), text_telemetry, self.bot_line()));
        }

        if self.max_debug > 0 {
            let mut text_debug = String::new();

            for (i, str) in self.debugs.iter().enumerate() {
                text_debug.push_str(format!("{} {} {}\n", CHARS[1], str.clone(), CHARS[1]).as_str())
            }

            println!("{}", format!("{}\n{}{}", self.top_line(), text_debug, self.bot_line()));
        }

        stdout().flush().unwrap();
    }
}

fn str_left_pad(str: &str, len: usize, pad: &str) -> String {
    let mut str = str.trim();

    if str.chars().count() > len { str = &str[..len] }

    return format!("{}{}", pad.repeat(len - str.chars().count()), str);
}

fn str_right_pad(str: &str, len: usize, pad: &str) -> String {
    let mut str = str.trim();

    if str.chars().count() > len { str = &str[..len] }

    return format!("{}{}", str, pad.repeat(len - str.chars().count()));
}

