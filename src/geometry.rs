const EARTH_RADIUS_MI: f64 = 3959.0; // Earth radius in miles

/// Calculate the great circle distance between two points on the Earth
/// (specified in decimal degrees)
/// 
/// # Examples
/// 
/// ```
/// use cube_sniper::geometry::haversine_distance;
/// let lat_long1 = (37.7749, -122.4194);
/// let lat_long2 = (34.0522, -118.2437);
/// let distance = haversine_distance(lat_long1, lat_long2);
/// assert_eq!(distance, 347.44284485743043);
/// ```
pub fn haversine_distance(lat_long1: (f64, f64), lat_long2: (f64, f64)) -> f64 {
    let (lat1, lon1) = lat_long1;
    let (lat2, lon2) = lat_long2;
    let radius = EARTH_RADIUS_MI;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin() * (dlat / 2.0).sin() + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin() * (dlon / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    radius * c
}
