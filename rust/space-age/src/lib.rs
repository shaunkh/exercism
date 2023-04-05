// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / 31557600 as f64
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! impl_Planet {
    (for $($t:ty),+) => {
        $(impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                d.seconds / 31557600 as f64
            }
        })*
    }
}

impl_Planet!(for Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
