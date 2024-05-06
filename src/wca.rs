pub const WCA_BASE_URL: &str = "https://www.worldcubeassociation.org";

pub struct Competition {
    pub name: String,
    pub marker_date: String,
    pub latitude_degrees: String,
    pub longitude_degrees: String,
    pub lat_long: (f64, f64),
    pub city_name: String,
    pub url: String,
}

impl Competition {
    pub fn new(name: String, marker_date: String, latitude_degrees: String, longitude_degrees: String, city_name: String, url: String) -> Self {
        let lat_long = (latitude_degrees.parse::<f64>().unwrap(), longitude_degrees.parse::<f64>().unwrap());
        let url = format!("{}{}", WCA_BASE_URL, url);
        Self {
            name,
            marker_date,
            latitude_degrees,
            longitude_degrees,
            lat_long,
            city_name,
            url,
        }
    }
    pub fn print(&self) {
        println!("Name: {}", self.name);
        println!("Marker Date: {}", self.marker_date);
        println!("Latitude Degrees: {}", self.latitude_degrees);
        println!("Longitude Degrees: {}", self.longitude_degrees);
        println!("Lat Long: {:?}", self.lat_long);
        println!("City Name: {}", self.city_name);
        println!("URL: {}", self.url);
    }
}
