use rand::Rng;

pub fn compare_floats_eps(left: f64, right: f64, epsilon: f64) -> bool {
    return (left - right).abs() < epsilon;
}

pub fn compare_floats(left: f64, right: f64) -> bool {
    return compare_floats_eps(left, right, 0.000_001);
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random_double() -> f64 {
    // returns a random real in [0,1)
    rand::thread_rng().gen::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // returns a random real in [min,max)
    min + (max - min) * random_double()
}
