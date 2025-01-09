use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;

use itertools::izip;
use rayon::prelude::*;

use complex::Complex;
use grid::Grid;

use crate::image::save_grid;
use crate::mandel::{iter_mandel_many_inplace, iter_mandel_many_inplace_batched};

mod mandel;
mod grid;
mod complex;
mod image;


struct Mandelbrot {
    z: Grid<Complex>,
    c: Grid<Complex>,
    iters: Grid<usize>,
    total_iters: usize,
    finished: Grid<bool>,
}

impl Mandelbrot {
    fn new(size: (usize, usize), centre: (f64, f64), scale: (f64, f64)) -> Self {
        Self {
            z: Grid::full(size, Complex::ZERO),
            c: Grid::new(size, centre, scale),
            iters: Grid::full(size, 0),
            total_iters: 0,
            finished: Grid::full(size, false),
        }
    }

    fn iter_inplace(&mut self, n: usize) {
        izip!(
            self.z.data.iter_mut(),
            self.c.data.iter(),
            self.iters.data.iter_mut(),
            self.finished.data.iter_mut()
        )
            // .par_bridge()
            .for_each(|(z_row, c_row, i_row, f_row)| {
                (0..z_row.len()).for_each(|j| {
                    if !f_row[j] {
                        let new_iters = iter_mandel_many_inplace(&mut z_row[j], &c_row[j], n);
                        i_row[j] += new_iters;
                        if new_iters < n {
                            f_row[j] = true;
                        }
                    }
                });
            });

        self.total_iters += n;
    }

    fn exploded(&self) -> Grid<bool> {
        Grid {
            data: self.iters.data.iter().map(|row|
                row.iter().map(|&i| i < self.total_iters)
                    .collect()
            ).collect(),
        }
    }
}

impl Debug for Mandelbrot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Positions:")?;
        writeln!(f, "{}", self.c)?;
        writeln!(f, "Current state:")?;
        writeln!(f, "{}", self.z)?;
        writeln!(f, "Iterations:")?;
        writeln!(f, "{}", self.iters)
    }
}

impl Display for Mandelbrot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let exploded = self.exploded();
        let string_grid = Grid {
            data: (0..self.iters.data.len()).map(|i|
                (0..self.iters.data[0].len()).map(|j|
                    if exploded.data[i][j] {
                        format!("{}", self.iters.data[i][j])
                    } else {
                        "".to_string()
                    }
                ).collect()
            ).collect()
        };

        writeln!(f, "{}", string_grid)
    }
}

fn main() {
    // let mut mandel = Mandelbrot::new((1001, 1001), (0.,0.), (0.2, 0.2));
    let mut mandel = Mandelbrot::new((1080 * 2, 1920 * 2), (-0.11611300706863403, -0.8834812343120579), (1e-5, 1e-5));
    // println!("{}", mandel);

    for i in 0..10 {
        mandel.iter_inplace(100);
        println!("{}", i);
        // println!("{:?}", mandel);

        save_grid(&mandel.iters, mandel.total_iters, PathBuf::from(format!("out{i}.png")));
    }
}
