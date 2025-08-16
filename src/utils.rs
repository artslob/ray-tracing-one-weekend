pub fn compare_floats_eps(left: f64, right: f64, epsilon: f64) -> bool {
    return (left - right).abs() < epsilon;
}

pub fn compare_floats(left: f64, right: f64) -> bool {
    return compare_floats_eps(left, right, 0.000_001);
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.
}
