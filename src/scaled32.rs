pub struct Scaled32<'a> {
    origin: &'a[f32],
    scale: f32
}

impl<'a> Scaled32<'a> {

    pub fn new(slice: &'a[f32], scale: f32) -> Self {
        Self {
            origin: slice,
            scale,
        }
    }

    pub fn index(&mut self, i: usize) -> f32 {
        let z = 0f32;
        let orig = i as f32 * self.scale;
        let floor = orig.floor();
        let ceil= orig.ceil();
        // let interp = orig - floor;

        // println!("{} <floor<->ceil> {}", floor, ceil);
            
        let (a,b) = (self.origin.get(floor as usize).unwrap_or(&z), self.origin.get(ceil as usize).unwrap_or(&z));

        let interp = b/a;

        let o = if interp.is_normal() {
            if interp.is_sign_negative() {
                (a*(interp.abs().powf(self.scale as f32))) * -1f32
            }else {
                a*(interp.powf(self.scale as f32))
            }
        }else{
            0f32
        };
        // println!("{} <{}> {}", a, o, b);
        o

    }
}
