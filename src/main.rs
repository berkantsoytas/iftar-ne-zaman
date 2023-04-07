mod args_config;
mod citiesregions;
mod coordinates;
mod countries;
mod find_coordinate;
mod regions;
mod request;
mod table;
mod timefromcoordinates;
mod timefromplace;

//-------------------
use crate::args_config::Config;
use crate::coordinates::Coordinates;
use crate::table::PrintableTable;
use crate::timefromcoordinates::TimeFromCoordinates;
//-----------------------

use prettytable::{Cell, Row};
use termion::{color, style};

use std::{
    env,
    io::{stdout, Write},
};

fn main() {
    let mut table = PrintableTable::new();
    let args: Vec<String> = env::args().collect();
    let conf: Config = Config::new(args);

    let coordinates: Coordinates = Coordinates::new(&conf.country, &conf.city, &conf.region);

    let coordinate: &Coordinates = coordinates.get_coordinates();

    // get time to (YYYY-MM-DD) format std lib but if hour is 24:00:00 it will be 00:00:00
    let mut date = chrono::Local::now().format("%Y-%m-%d").to_string();

    // but if hour is grater than 22:00:00 it will be 00:00:00
    if chrono::Local::now()
        .format("%H")
        .to_string()
        .parse::<i32>()
        .unwrap()
        > 22
    {
        date = chrono::Local::now()
            .checked_add_signed(chrono::Duration::days(1))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();
    }

    let time_from_coordinates: TimeFromCoordinates = TimeFromCoordinates::new(
        &coordinate.latitude.to_owned().to_string(),
        &coordinate.longitude.to_owned().to_string(),
        &date,
        "1",
        "180",
    );

    let city = time_from_coordinates.get_times();

    table.add_row(vec![
        "Tarih", "Imsak", "Gunes", "Ogle", "Ikindi", "Aksam", "Yatsi",
    ]);

    println!("\nBugunun ibadet saatleri:");

    let mut aksam: String = String::new();

    for (timezone, times) in city {
        let row = Row::new(vec![
            Cell::new(&timezone),
            Cell::new(&times[0]),
            Cell::new(&times[1]),
            Cell::new(&times[2]),
            Cell::new(&times[3]),
            Cell::new(&times[4]),
            Cell::new(&times[5]),
        ]);

        aksam = times[4].to_owned().to_string();

        table.add_row_cell(row);
    }

    table.print();

    // check if aksam is grater than time now and if it is then print aksam time

    // parse akşam and time now to i32
    // but aksam variable format is HH:MM and time now format is HH:MM:SS
    // so we need to remove :SS from time now

    // TODO: print remaining time to aksam

    // parse aksam to DateTime
    let aksam = chrono::NaiveTime::parse_from_str(&aksam, "%H:%M").unwrap();

    // parse time now to DateTime
    let time_now = chrono::NaiveTime::parse_from_str(
        &chrono::Local::now().format("%H:%M").to_string(),
        "%H:%M",
    );

    let mut stdout = stdout();
    // check if aksam is grater than time now
    if aksam > time_now.unwrap() {
        // calculate remaining time to aksam
        let remaining_time = aksam - time_now.unwrap();

        // print remaining time to aksam
        write!(
            stdout,
            "\n\nIftara Kalan Vakit:\n\t{}{}{}{} SAAT {}{}{}{} DAKIKA{}{}\n\n",
            color::Fg(color::Red),
            style::Bold,
            remaining_time.num_hours(),
            style::Reset,
            color::Fg(color::Green),
            style::Bold,
            remaining_time.num_minutes() - (remaining_time.num_hours() * 60),
            style::Reset,
            color::Fg(color::Blue),
            style::Bold,
        )
        .unwrap();
    }
    stdout.flush().unwrap();
}
