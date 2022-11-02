use std::ops::*;
use realfft::num_complex::{Complex, ComplexFloat};
use realfft::num_traits::Zero;
use std::slice::Iter;

pub struct ScaledComplex<'a> {
    origin: &'a[Complex<f64>],
    scale: f64
}

impl<'a> ScaledComplex<'a> {

    pub fn new(slice: &'a[Complex<f64>], scale: f64) -> Self {
        Self {
            origin: slice,
            scale,
        }
    }

    pub fn index(&mut self, i: usize) -> Complex<f64> {
        let z = Complex::zero();
        let orig = (i as f64 * self.scale);
        let floor = orig.floor();
        let ceil= orig.ceil();
        // let interp = orig - floor;

        // println!("{} <floor<->ceil> {}", floor, ceil);
            
        let (a,b) = (self.origin.get(floor as usize).unwrap_or(&z), self.origin.get(ceil as usize).unwrap_or(&z));

        let interp = b/a;

        let o = if interp.is_normal() {
            if interp.re.is_sign_negative() {
                let (a_r, a_theta) = a.to_polar();
                let r = (a_r*(interp.abs().powf(self.scale as f64))) * -1f64;

                Complex::<f64>::from_polar(r, a_theta * self.scale as f64)

            }else {
                a*(interp.powf(self.scale as f64))
            }
        }else{
            z.clone()
        };
        // println!("{} <{}> {}", a, o, b);
        o

    }
}
