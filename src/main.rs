use clap::Parser;
use chrono;
use cube_sniper::util::parse_lat_long;
use cube_sniper::wca::{get_competitions_from_json, retrieve_competitions_json, find_competitions_within_distance};


#[derive(Parser)]
struct Cli {
    region: String,
    lat_long: String,
    #[arg(default_value_t = 150.0)]
    distance: f64,
}


fn main() {
    let args = Cli::parse();

    // let competitions_html = match retrieve_competitions_html(&args.region) {
    //     Ok(competitions_html) => competitions_html,
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         return;
    //     }
    // };

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let competitions_json = match retrieve_competitions_json(&args.region, &today.as_str()) {
        Ok(competitions_json) => competitions_json,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut competitions = Vec::new();
    for competition_page in competitions_json {
        let parsed_competitions = get_competitions_from_json(&competition_page.as_str()).unwrap();
        for competition in parsed_competitions {
            competitions.push(competition);
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
