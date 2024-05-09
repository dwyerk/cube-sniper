use std::error::Error;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use serde_json::Value;

use crate::geometry::haversine_distance;

pub const WCA_BASE_URL: &str = "https://www.worldcubeassociation.org";

pub struct Competition {
    pub name: String,
    pub marker_date: String,
    pub latitude_degrees: f64,
    pub longitude_degrees: f64,
    pub city_name: String,
    pub url: String,
}

impl Competition {
    pub fn new(name: String, marker_date: String, latitude_degrees: f64, longitude_degrees: f64, city_name: String, url: String) -> Self {
        let url = format!("{}{}", WCA_BASE_URL, url);
        Self {
            name,
            marker_date,
            latitude_degrees,
            longitude_degrees,
            city_name,
            url,
        }
    }
    pub fn print(&self) {
        println!("Name: {}", self.name);
        println!("Marker Date: {}", self.marker_date);
        println!("Latitude Degrees: {}", self.latitude_degrees);
        println!("Longitude Degrees: {}", self.longitude_degrees);
        println!("City Name: {}", self.city_name);
        println!("URL: {}", self.url);
    }
}


/// Get competitions from the HTML of the competitions page
/// 
/// # Examples
/// 
/// ```
/// use cube_sniper::wca::get_competitions;
/// let competitions_html = std::fs::read_to_string("tests/fixtures/competitions.html").unwrap();
/// let competitions = get_competitions(&competitions_html).unwrap();
/// assert_eq!(competitions.len(), 131);
/// assert_eq!(competitions[0].name, "Very Mini-Sota 2024");
/// ```
pub fn get_competitions(competitions_html: &str) -> Result<Vec<Competition>, Box<dyn Error>> {
    let document = Document::from(competitions_html);
    // find the competitions-list div and then find the script tag inside it
    let mut competitions_script = document.find(Attr("id", "competitions-list").child(Name("script")));
    // get the text of the script tag
    let competitions_script_text = competitions_script.next().unwrap().text();

    // parse the text of the script tag to get the competitions
    let competitions = competitions_script_text.split("var competitions = ").collect::<Vec<&str>>()[1].split(";\n").collect::<Vec<&str>>()[0];

    // Create a vector to store the competitions
    let mut competition_vec = Vec::new();

    let json_obj: Value = serde_json::from_str(competitions)?;

    for competition in json_obj.as_array().unwrap() {
        let name = competition["name"].as_str().unwrap();
        let marker_date = competition["marker_date"].as_str().unwrap();
        let latitude_degrees = competition["latitude_degrees"].as_f64().unwrap();
        let longitude_degrees = competition["longitude_degrees"].as_f64().unwrap();
        let city_name = competition["cityName"].as_str().unwrap();
        let url = competition["url"].as_str().unwrap();
        let competition_data = Competition::new(name.to_string(), marker_date.to_string(), latitude_degrees, longitude_degrees, city_name.to_string(), url.to_string());
        competition_vec.push(competition_data);
    }

    Ok(competition_vec)
}

/// Retrieve competitions from the WCA website
/// 
/// # Examples
/// 
/// ```no_run
/// use cube_sniper::wca::retrieve_competitions;
/// let region = "North America";
/// let competitions_html = retrieve_competitions(region).unwrap();
/// println!("{}", competitions_html);
/// ```
pub fn retrieve_competitions(region: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let res = client.get(format!("{}/competitions?region={}&search=&state=present&year=all+years&from_date=&to_date=&delegate=&display=map", WCA_BASE_URL, region))
        .send()?
        .text()?;
    Ok(res)
}

/// Find competitions within a certain distance of a location
/// 
/// # Examples
/// 
/// ```
/// use cube_sniper::wca::Competition;
/// use cube_sniper::geometry::haversine_distance;
/// use cube_sniper::wca::find_competitions_within_distance;
/// let mut competitions = vec![
///   Competition::new("Competition 1".to_string(), "2021-01-01".to_string(), 37.7749, -122.4194, "San Francisco".to_string(), "/competition/competition1".to_string()),
///   Competition::new("Competition 2".to_string(), "2021-01-02".to_string(), 34.0522, -118.2437, "Los Angeles".to_string(), "/competition/competition2".to_string()),
/// ];
/// let search_lat_long = (37.7749, -122.4194);
/// let distance = 150.0;
/// let competitions_within_distance = find_competitions_within_distance(&mut competitions, search_lat_long, distance);
/// assert_eq!(competitions_within_distance.len(), 1);
/// assert_eq!(competitions_within_distance[0].0.name, "Competition 1");
/// ```
pub fn find_competitions_within_distance(competitions: &mut [Competition], search_lat_long: (f64, f64), distance_miles: f64) -> Vec<(&Competition, f64)> {
    let mut competitions_within_distance: Vec<(&Competition, f64)> = Vec::new();
    for competition in competitions {
        let distance = haversine_distance(search_lat_long, (competition.latitude_degrees, competition.longitude_degrees));
        if distance <= distance_miles {
            competitions_within_distance.push((competition, distance));
        }
    }
    competitions_within_distance
}


/// Print a list of competitions
/// 
/// # Examples
/// 
/// ```
/// use cube_sniper::wca::{Competition, print_competitions};
/// let competitions = vec![
///   Competition::new("Competition 1".to_string(), "2021-01-01".to_string(), 37.7749, -122.4194, "San Francisco".to_string(), "/competition/competition1".to_string()),
///   Competition::new("Competition 2".to_string(), "2021-01-02".to_string(), 34.0522, -118.2437, "Los Angeles".to_string(), "/competition/competition2".to_string()),
/// ];
/// print_competitions(&competitions);
/// ```
pub fn print_competitions(competitions: &[Competition]) {
    for competition in competitions {
        competition.print();
        println!();
    }
}
