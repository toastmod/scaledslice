mod scaled;
mod scaledcomplex;
pub use crate::scaled::*;
pub use crate::scaledcomplex::*;

mod test {

    use std::sync::Arc;
    use realfft::num_complex::Complex;
    use realfft::{RealToComplex, ComplexToReal, RealFftPlanner};
    use std::collections::VecDeque;
    use std::ops::Deref;


    struct FFTProcessor {
        fft: Arc<dyn RealToComplex<f64>>,
        ifft: Arc<dyn ComplexToReal<f64>>,

        fft_input: Vec<f64>,
        fft_input_cur: usize,
        fft_output: Vec<Complex<f64>>,

        ifft_input: Vec<Complex<f64>>,
        ifft_input_zero: Box<[Complex<f64>]>,
        ifft_output: Vec<f64>,
        ifft_output_cur: usize,

        master_outbuf: VecDeque<f64>,

        size: usize

    }

    impl FFTProcessor {

        pub fn new(size: usize) -> Self {

            let fft = RealFftPlanner::new().plan_fft_forward(size);
            let ifft = RealFftPlanner::new().plan_fft_inverse(size);

            let fft_input = fft.make_input_vec();
            let fft_output = fft.make_output_vec();
            let ifft_input = ifft.make_input_vec();
            let ifft_input_zero = ifft.make_input_vec().into_boxed_slice();
            let ifft_output = ifft.make_output_vec();
            let master_outbuf = VecDeque::from(ifft.make_output_vec());

            Self {
                fft,
                ifft,
                fft_input,
                fft_input_cur: 0,
                fft_output,
                ifft_input,
                ifft_input_zero,
                ifft_output,
                ifft_output_cur: 0,
                master_outbuf,
                size,
            }
        }

        pub fn process(&mut self, ins: f64, bin_process: &mut dyn FnMut(&mut [Complex<f64>], &mut[Complex<f64>], usize) -> ()) -> f64 {

            self.fft_input[self.fft_input_cur] = ins;
            self.fft_input_cur += 1;

            if self.fft_input_cur == self.fft_input.len() {

                // reset ifft input
                self.ifft_input.copy_from_slice(self.ifft_input_zero.deref());

                // process
                self.fft.process(self.fft_input.as_mut_slice(), self.fft_output.as_mut_slice()).unwrap();
                bin_process(self.fft_output.as_mut_slice(), self.ifft_input.as_mut_slice(), self.size);
                self.ifft.process(self.ifft_input.as_mut_slice(), self.ifft_output.as_mut_slice()).unwrap();

                // append output
                for s in self.ifft_output.iter_mut() {
                   self.master_outbuf.push_back(s.clone());
                }

                self.fft_input_cur = 0;
            }

            match self.master_outbuf.pop_front() {
                None => 0f64,
                Some(s) => s
            }

        }
    }

    use crate::*;

    #[test]
    fn main() {

        const RES: usize = 200;
        const SCALE: f64 = 1.5;
        // let mut s1 = vec![];
        let mut fft = FFTProcessor::new(RES);
        let mut s = vec![];
        for i in 0..RES {
            let smp = f64::sin(5f64 * i as f64);
            debug_plotter::debug_plot!(smp where path = "C:\\Users\\anumr\\plots\\firstplot.jpg");
            // fft.process(smp as f64, &mut |x,y, d|{
            //     y.copy_from_slice(x);
            //     let mut ss = Scaled::new(y, SCALE);

            //     println!("\n!");
            //     for i in 0..RES {
            //         let dbg = ss.index(i).re;
            //         debug_plotter::debug_plot!(dbg where path = "C:\\Users\\anumr\\plots\\myplot.jpg", caption = format!("scale: {}", SCALE));
            //     }

            //     println!("\nDONE!");
            // });
            s.push(smp);
        }

        let mut scaled = Scaled::new(s.as_slice(), 1f64);
        for i in 0..RES {
            let smpsmp = scaled.index(i);
            println!("orig: {}",s[i]);
            debug_plotter::debug_plot!(smpsmp where path = "C:\\Users\\anumr\\plots\\secondplot.jpg");

        }

    }

}