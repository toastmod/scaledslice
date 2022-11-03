use std::ops::*;
use realfft::num_complex::{Complex, ComplexFloat};
use realfft::num_traits::Zero;
use std::slice::Iter;

pub struct ScaledComplex32<'a> {
    origin: &'a[Complex<f32>],
    scale: f32
}

impl<'a> ScaledComplex32<'a> {

    pub fn new(slice: &'a[Complex<f32>], scale: f32) -> Self {
        Self {
            origin: slice,
            scale,
        }
    }

    pub fn index(&mut self, i: usize) -> Complex<f32> {
        let z = Complex::zero();
        let orig = (i as f32 * self.scale);
        let floor = orig.floor();
        let ceil= orig.ceil();
        // let interp = orig - floor;

        // println!("{} <floor<->ceil> {}", floor, ceil);
            
        let (a,b) = (self.origin.get(floor as usize).unwrap_or(&z), self.origin.get(ceil as usize).unwrap_or(&z));

        let interp = b/a;

        let o = if interp.is_normal() {
            if interp.re.is_sign_negative() {
                let (a_r, a_theta) = a.to_polar();
                let r = (a_r*(interp.abs().powf(self.scale as f32))) * -1f32;

                Complex::<f32>::from_polar(r, a_theta * self.scale as f32)

            }else {
                a*(interp.powf(self.scale as f32))
            }
        }else{
            z.clone()
        };
        // println!("{} <{}> {}", a, o, b);
        o

    }
}
