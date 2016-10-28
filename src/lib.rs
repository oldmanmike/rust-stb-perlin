extern crate libc;

use libc::{c_float, c_int};

extern {
    pub fn stb_perlin_noise3(x: c_float, y: c_float, z: c_float, x_wrap: c_int, y_wrap: c_int, z_wrap: c_int) -> f32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test = unsafe { stb_perlin_noise3(552.0, 32.32, 6.0, 0, 0, 0) };
        println!("Hopefully {:?}", test);
    }
}
