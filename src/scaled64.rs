pub struct Scaled64<'a> {
    origin: &'a[f64],
    scale: f64
}

impl<'a> Scaled64<'a> {

    pub fn new(slice: &'a[f64], scale: f64) -> Self {
        Self {
            origin: slice,
            scale,
        }
    }

    pub fn index(&mut self, i: usize) -> f64 {
        let z = 0f64;
        let orig = i as f64 * self.scale;
        let floor = orig.floor();
        let ceil= orig.ceil();
        // let interp = orig - floor;

        // println!("{} <floor<->ceil> {}", floor, ceil);
            
        let (a,b) = (self.origin.get(floor as usize).unwrap_or(&z), self.origin.get(ceil as usize).unwrap_or(&z));

        let interp = b/a;

        let o = if interp.is_normal() {
            if interp.is_sign_negative() {
                (a*(interp.abs().powf(self.scale as f64))) * -1f64
            }else {
                a*(interp.powf(self.scale as f64))
            }
        }else{
            0f64
        };
        // println!("{} <{}> {}", a, o, b);
        o

    }
}
