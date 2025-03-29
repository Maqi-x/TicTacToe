use std::io::Write;
use rand::Rng;
use rand as rand_lib;

pub fn rand<T: rand_lib::distributions::uniform::SampleUniform + PartialOrd>(min: T, max: T) -> T {
    let mut rng = rand_lib::thread_rng();
    rng.gen_range(min..=max)
}

#[macro_export] macro_rules! randElm {
    ($array:expr) => {{
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..$array.len());
        &$array[index]
    }};
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}
