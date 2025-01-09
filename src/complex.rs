use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Complex {
    pub(crate) r: f64,
    pub(crate) i: f64,
}

impl Complex {
    #[inline(always)]
    pub fn add_inplace(&mut self, other: &Self) {
        self.r += other.r;
        self.i += other.i;
    }

    #[inline(always)]
    pub fn square_inplace(&mut self) {
        (self.r, self.i) = (
            self.r.powi(2) - self.i.powi(2),
            2. * self.r * self.i
        );
    }

    #[inline(always)]
    pub fn abs2(&self) -> f64 {
        self.r.powi(2) + self.i.powi(2)
    }

    pub(crate) const ZERO: Complex = Self { r: 0., i: 0. };
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>5.1e} {:>+5.1e}i", self.r, self.i)
    }
}

#[cfg(test)]
mod tests {
    use crate::complex::Complex;

    #[test]
    fn test_zero() {
        let mut x = Complex::ZERO;
        let y = Complex::ZERO;
        x.r = 1.;
        println!("{} {}", x, y);
    }
}
