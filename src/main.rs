mod citiesregions;
mod coordinates;
mod countries;
mod find_coordinate;
mod regions;
mod request;
mod timefromcoordinates;
mod timefromplace;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print!("Usage: {} [country] [region] [city]", args[0]);
        return;
    }

    let country: &String = &args[1];
    let region: &String = &args[2];
    let city: &String = &args[3];

    let coordinates: coordinates::Coordinates =
        coordinates::Coordinates::new(country, region, city);

    let coordinate: &coordinates::Coordinates = coordinates.get_coordinates();

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

    let time_from_coordinates: timefromcoordinates::TimeFromCoordinates =
        timefromcoordinates::TimeFromCoordinates::new(
            &coordinate.latitude.to_owned().to_string(),
            &coordinate.longitude.to_owned().to_string(),
            &date,
            "1",
            "180",
        );

    let city = time_from_coordinates.get_times();

    for (timezone, times) in city {
        println!(
            "{}: \n\tImsak: {} \n\tGunes: {}, \n\tOgle: {}, \n\tIkindi: {}, \n\tAksam: {}, \n\tYatsi: {}",
            timezone, times[0], times[1], times[2], times[3], times[4], times[5]
        );
    }
}
