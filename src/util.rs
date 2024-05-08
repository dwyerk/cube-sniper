/// Parse a string of latitude and longitude into a tuple of f64
///
/// # Examples
///
/// ```
/// use cube_sniper::util::parse_lat_long;
/// let lat_long = "37.7749,-122.4194";
/// let (lat, long) = parse_lat_long(lat_long);
/// assert_eq!(lat, 37.7749);
/// assert_eq!(long, -122.4194);
/// ```
pub fn parse_lat_long(lat_long: &str) -> (f64, f64) {
    let lat_long = lat_long.split(",").collect::<Vec<&str>>();
    (lat_long[0].parse::<f64>().unwrap(), lat_long[1].parse::<f64>().unwrap())
}
