extern crate libc;

use libc::{c_float, c_int};

extern {
    pub fn stb_perlin_noise3(x: c_float, y: c_float, z: c_float, x_wrap: c_int, y_wrap: c_int, z_wrap: c_int) -> f32;
}

pub fn perlin_noise3(x: f32, y: f32, z: f32, x_wrap: i32, y_wrap: i32, z_wrap: i32) -> f32 {
    unsafe { stb_perlin_noise3(x, y, z, x_wrap, y_wrap, z_wrap) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test1 = unsafe { stb_perlin_noise3(552.0, 32.32, 6.0, 0, 5, 0) };
        let test2 = perlin_noise3(552.0, 32.32, 6.0, 0, 5, 0);
        println!("stb_perlin_noise3 call: {:?}", test1);
        println!("perlin_noise3 call: {:?}", test2);
    }
}
