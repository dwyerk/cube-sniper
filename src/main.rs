use std::error::Error;

use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use clap::Parser;
use cube_sniper::wca::{WCA_BASE_URL, Competition};

const EARTH_RADIUS_MI: f64 = 3959.0; // Earth radius in miles

#[derive(Parser)]
struct Cli {
    region: String,
    lat_long: String,
    #[arg(default_value_t = 150.0)]
    distance: f64,
}

fn parse_lat_long(lat_long: &str) -> (f64, f64) {
    let lat_long = lat_long.split(",").collect::<Vec<&str>>();
    (lat_long[0].parse::<f64>().unwrap(), lat_long[1].parse::<f64>().unwrap())
}

fn main() {
    let args = Cli::parse();

    let competitions_html = match retrieve_competitions(&args.region) {
        Ok(competitions_html) => competitions_html,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    
    let mut competitions = get_competitions(&competitions_html.as_str()).unwrap();

    // debug
    if false {
        for competition in competitions.as_slice() {
            competition.print();
            println!();
        }
    }

    // Find competitions within a certain distance of a location
    let search_lat_long = parse_lat_long(&args.lat_long);
    let mut competitions_within_distance = find_competitions_within_distance(competitions.as_mut_slice(), search_lat_long, args.distance);

    competitions_within_distance.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (competition, distance) in competitions_within_distance {
        competition.print();
        println!("Distance: {:.2} miles", distance);
        println!();
    }
}

// This function will access the world cubing association.org website and scrape the competitions-list div
// It will then parse the script tag inside the div to get the competitions and return the vector
fn get_competitions(competitions_html: &str) -> Result<Vec<Competition>, Box<dyn Error>> {
    let document = Document::from(competitions_html);
    // find the competitions-list div and then find the script tag inside it
    let mut competitions_script = document.find(Attr("id", "competitions-list").child(Name("script")));
    // get the text of the script tag
    let competitions_script_text = competitions_script.next().unwrap().text();

    // parse the text of the script tag to get the competitions
    let competitions = competitions_script_text.split("var competitions = ").collect::<Vec<&str>>()[1].split(";\n").collect::<Vec<&str>>()[0];

    // Create a vector to store the competitions
    let mut competition_vec = Vec::new();

    // Iterate over each competition and store the data in the vector
    for competition in competitions.split("},{") {
        let competition = competition.replace("{", "").replace("}", "").replace("\"", "");

        // Each competition item is a string with the following format
        //   "id": "KansasChampionship2024",
        //   "name": "Kansas Championship 2024",
        //   "latitude_degrees": 37.689175,
        //   "longitude_degrees": -97.346626,
        //   "cityName": "Wichita, Kansas",
        //   "marker_date": "May 3 - 5, 2024",
        //   "is_probably_over": false,
        //   "url": "/competitions/KansasChampionship2024"

        let mut name = String::new();
        let mut marker_date = String::new();
        let mut latitude_degrees = String::new();
        let mut longitude_degrees = String::new();
        let mut city_name = String::new();
        let mut url = String::new();

        // For each key/value pair, split the string and get the value
        let key_value_pairs = competition.split(",").collect::<Vec<&str>>();
        for key_value_pair in key_value_pairs {
            let key_value_pair = key_value_pair.split(":").collect::<Vec<&str>>();
//            println!("{:?}", key_value_pair);
            match key_value_pair[0] {
                "id" => (),
                "name" => name = key_value_pair[1].to_string(),
                "latitude_degrees" => latitude_degrees = key_value_pair[1].to_string(),
                "longitude_degrees" => longitude_degrees = key_value_pair[1].to_string(),
                "cityName" => city_name = key_value_pair[1].to_string(),
                "marker_date" => marker_date = key_value_pair[1].to_string(), // FIXME this is currently losing the second half of the date because it has a comma in it
                "is_probably_over" => (),
                "url" => url = key_value_pair[1].to_string(),
                _ => (),
            }
        }

        let competition_data = Competition::new(name, marker_date, latitude_degrees, longitude_degrees, city_name, url);
        competition_vec.push(competition_data);
    }
    Ok(competition_vec)
}

fn retrieve_competitions(region: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let res = client.get(format!("{}/competitions?region={}&search=&state=present&year=all+years&from_date=&to_date=&delegate=&display=map", WCA_BASE_URL, region))
        .send()?
        .text()?;
    Ok(res)
}

fn haversine_distance(lat_long1: (f64, f64), lat_long2: (f64, f64)) -> f64 {
    let (lat1, lon1) = lat_long1;
    let (lat2, lon2) = lat_long2;
    let radius = EARTH_RADIUS_MI;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin() * (dlat / 2.0).sin() + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin() * (dlon / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    radius * c
}


// This function will take a vector of competitions and a search location and return a vector of competitions within a certain distance of the search location
// Shouldn't I rewrite this to return Some/None instead of Vec?
fn find_competitions_within_distance(competitions: &mut [Competition], search_lat_long: (f64, f64), distance_miles: f64) -> Vec<(&Competition, f64)> {
    let mut competitions_within_distance: Vec<(&Competition, f64)> = Vec::new();
    for competition in competitions {
        let distance = haversine_distance(search_lat_long, competition.lat_long);
        if distance <= distance_miles {
            competitions_within_distance.push((competition, distance));
        }
    }
    competitions_within_distance
}