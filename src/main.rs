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
use num::clamp;
use consts::*;
use num::traits::Pow;


pub fn complex_to_real2(c:Complex<f64>) -> (i32, i32) {
    return ((0.3 * (WIDTH as f64) * (c.re) + (WIDTH / 2) as f64) as i32, (0.3 * (WIDTH as f64) * (c.im) + (HEIGHT / 2) as f64) as i32);
}

fn generate_complex2(rng: &mut ThreadRng) -> Complex<f64> {
    return Complex::new(6.0 * rng.gen::<f64>() - 3.0, 6.0 * rng.gen::<f64>() - 3.0);
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
        }

        if z == prev {
            return None;
        }

        if steps == steplimit {
            prev = z.clone();
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
                let c = generate_complex2(&mut rng);

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
            let (x, y) = complex_to_real2(points[l as usize]);

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

pub fn render_image_mult(densityarr1: &Box<[i32]>, densityarr2: &Box<[i32]>, densityarr3: &Box<[i32]>){
    let (maxval1, maxval2, maxval3) = (densityarr1.iter().max().unwrap().clone(), densityarr2.iter().max().unwrap().clone(), densityarr3.iter().max().unwrap().clone());
    let mut ppm_file = ppm::PPM::new(WIDTH, HEIGHT);
    for i in 0..SIZE {
        let ramp1 = clamp((((densityarr1[i as usize]) as f64) / (maxval1 as f64)).pow(1.0 / 3.0), 0.0, 1.0);
        let ramp2 = clamp((((densityarr2[i as usize]) as f64) / (maxval2 as f64)).pow(1.0 / 3.0), 0.0, 1.0);
        let ramp3 = clamp((((densityarr3[i as usize]) as f64) / (maxval3 as f64)).pow(1.0 / 3.0), 0.0, 1.0);
        ppm_file.set_pixel_direct((i * 3) as usize, ppm::Color::new((ramp1 * 255.0) as u8, (ramp2 * 255.0) as u8, (ramp3 * 255.0) as u8));
    }
    ppm_file.write_file("yeet.ppm").unwrap();
}

fn main() {
    println!("{}", "started!!!");
    println!("using {} threads with {} samples per thread", TCOUNT, SAMPLESPT);


    //render_image_mult(&i1, &i2, &i3);
    let now1 = Instant::now();
    let i1 = generate_density_array((1000, 200));
    //let i2 = generate_density_array((500, 100));
    //let i3 = generate_density_array((50, 10));
    println!("{:?}", now1.elapsed());
    render_image(&i1, "yeet.ppm");
    //render_image_mult(&i1, &i2, &i3);
    //render_image_mult(&i1, &i2, &i3);

    //let mut f = File::create("log.txt").unwrap();
    //f.write("took this long lmao: ".as_bytes());
    //f.write(format!("{:?}", now2).as_bytes());
}
