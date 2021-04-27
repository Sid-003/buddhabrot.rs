mod ppm;
mod consts;

use num::complex::Complex;
use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;
use std::{
    time::{Instant},
    thread,
    sync::mpsc,
};
use consts::*;

pub fn complex_to_real(c:Complex<f64>) -> (i32, i32) {
    return ((0.3 * (WIDTH as f64) * (c.re) + (WIDTH / 2) as f64) as i32, (0.3 * (WIDTH as f64) * (c.im) + (HEIGHT / 2) as f64) as i32);
}

fn generate_complex(rng: &mut ThreadRng) -> Complex<f64> {
    return Complex::new(6.0 * rng.gen::<f64>() - 3.0, 6.0 * rng.gen::<f64>() - 3.0);
}

//source: https://davidaramant.github.io/post/the-buddhabrot-part-3/
fn inside_cardioid(c: Complex<f64>) -> bool {
    let res = c.re - 0.25;
    let ims = c.im * c.im;
    let q = res * res + ims;

    return (q * (q + res)) < (0.25 * ims);
}

pub fn inbuddhabrot(c:Complex<f64>, iterations:u32, miniters:u32) -> Option<(Vec<Complex<f64>>, u32)> {
    let mut z = Complex::new(0.0, 0.0);
    let mut prev = Complex::new(0.0, 0.0);
    let mut points = Vec::with_capacity(iterations as usize);

    let mut steps = 0;
    let mut steplimit = 2;

    for i in 0..iterations {
        z = z * z + c;
        points.push(z);
        if z.norm_sqr() > 4.0 {
            if i >= miniters {
                return Some((points, i));
            }
            break;
        }

        if z == prev {
            break;
        }

        if steps == steplimit {
            prev = z;
            steps = 0;
            steplimit *= 2;
        }

        steps += 1;
    }

    return None
}

pub fn generate_density_array(iters: (u32, u32)) -> Box<[i32]> {
    let mut img = vec![0; SIZE as usize].into_boxed_slice();
    let (tx, rx) = mpsc::channel();
    let mut children = Vec::new();

    for _i in 0..TCOUNT {
        let thread_tx = tx.clone();

        let child = thread::spawn(move || {

            let mut rng = thread_rng();
            for _i in 0..SAMPLESPT {

                let c = generate_complex(&mut rng);
                if inside_cardioid(c) {
                    continue;
                }

                match inbuddhabrot(c, iters.0, iters.1) {
                    Some(trajectory) => {
                        thread_tx.send(trajectory).unwrap();
                    },
                    None => { }
                }
            }
        });
        children.push(child);
    }
    drop(tx);

    while let Ok(paths) = rx.recv() {
        let (points, limit) = paths;

        for l in 0..limit {

            let (x, y) = complex_to_real(points[l as usize]);

            if x < 0 || y < 0 || x >= (WIDTH as i32) || y >= (HEIGHT as i32)
            {
                continue;
            }

            let offset = y * (WIDTH as i32) + x;

            img[offset as usize] += 1
        }
    }

    for child in children {
        child.join().expect("oh no the child is stupid")
    }
    img
}

pub fn render_image(densityarr: &Box<[i32]>, name: &str){
    let maxval = *densityarr.iter().max().unwrap();
    let mut ppm_file = ppm::PPM::new(WIDTH, HEIGHT);
    for i in 0..SIZE {
        let ramp = ((densityarr[i as usize] as f64) * 1.0 / (maxval as f64)).powf(1.0/3.0);
        ppm_file.set_pixel_direct((i * 3) as usize, ppm::Color::new((ramp * 255.0) as u8, (ramp * 255.0) as u8, (ramp * 255.0) as u8));
    }
    ppm_file.write_file(name).unwrap();
}

fn main() {
    println!("using {} threads with {} samples per thread", TCOUNT, SAMPLESPT);

    let now1 = Instant::now();
    let i1 = generate_density_array((1000, 200));
    render_image(&i1, "yeet.ppm");

    println!("{:?}", now1.elapsed());
}
