use clap::Parser;
use cube_sniper::util::parse_lat_long;
use cube_sniper::wca::{get_competitions, retrieve_competitions, find_competitions_within_distance};


#[derive(Parser)]
struct Cli {
    region: String,
    lat_long: String,
    #[arg(default_value_t = 150.0)]
    distance: f64,
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
