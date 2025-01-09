use std::fmt::{Display, Formatter};

use crate::complex::Complex;

pub struct Grid<T> {
    pub(crate) data: Vec<Vec<T>>,
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let strings = self.data.iter().map(|row|
            row.iter()
                .map(|cell| format!("{}", cell))
                .collect::<Vec<_>>()
        ).collect::<Vec<_>>();

        let max_string_len = strings.iter().map(|row|
            row.iter().map(|s|
                s.len()
            ).max().unwrap()
        ).max().unwrap();

        let s = strings.iter().map(|row|
            row.iter().map(|s|
                format!("{}{}", (s.len()..max_string_len).map(|_| " ").collect::<String>(), s)
            ).collect::<Vec<_>>()
                .join(" | ")
        ).collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", s)
    }
}

impl Grid<Complex> {
    pub(crate) fn new(size: (usize, usize), centre: (f64, f64), scale: (f64, f64)) -> Self {
        let mid_y = (size.0 - 1) as f64 / 2.;
        let mid_x = (size.1 - 1) as f64 / 2.;

        Self {
            data: (0..size.0).map(|i|
                (0..size.1).map(|j|
                    Complex {
                        r: (i as f64 - mid_y) * scale.0 + centre.0,
                        i: (j as f64 - mid_x) * scale.1 + centre.1,
                    }
                ).collect()
            ).collect()
        }
    }
}

impl<T: Copy> Grid<T> {
    pub(crate) fn full(size: (usize, usize), value: T) -> Self {
        Self {
            data: (0..size.0).map(|_|
                (0..size.1).map(|_|
                    value
                ).collect()
            ).collect()
        }
    }
}
