// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

const earth_second_to_year: u32 = 31557600;
const earth_to_mercury_year: f64 = 0.2408467;
const earth_to_venus_year: f64 = 0.61519726;
const earth_to_mars_year: f64 = 1.8808158;
const earth_to_jupiter_year: f64 = 11.862615;
const earth_to_saturn_year: f64 = 29.447498;
const earth_to_uranus_year: f64 = 84.016846;
const earth_to_neptune_year: f64 = 164.79132;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury {}
pub struct Venus {}
pub struct Earth {}
pub struct Mars {}
pub struct Jupiter {}
pub struct Saturn {}
pub struct Uranus {}
pub struct Neptune {}

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (earth_second_to_year as f64 * earth_to_mercury_year)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (earth_second_to_year as f64 * earth_to_venus_year)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (earth_second_to_year as f64)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (earth_second_to_year as f64 * earth_to_mars_year)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (earth_second_to_year as f64 * earth_to_jupiter_year)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (earth_second_to_year as f64 * earth_to_saturn_year)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (earth_second_to_year as f64 * earth_to_uranus_year)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (earth_second_to_year as f64 * earth_to_neptune_year)
    }
}
