use crate::complex::Complex;

/// Single iteration of the mandelbrot calc
#[inline(always)]
fn iter_mandel_inplace(z: &mut Complex, c: &Complex) {
    z.square_inplace();
    z.add_inplace(&c);
}

/// Iterates the mandelbrot calc many times, up to a limit. If we explode we return early.
pub fn iter_mandel_many_inplace(z: &mut Complex, c: &Complex, limit: usize) -> usize {
    for i in 0..limit {
        if z.abs2() >= 4. {
            return i;
        }

        iter_mandel_inplace(z, c);
    }

    return limit;
}

/// Assumption: most of the time, the number won't have exploded. So we're better off doing a batch of computes,
///     checking at the end, and going back to do a proper check if it fails.
pub fn iter_mandel_many_inplace_batched(z: &mut Complex, c: &Complex, limit: usize) -> usize {
    // Sprint to the end
    let mut z_fast= z.clone();
    (0..limit).for_each(|_| iter_mandel_inplace(&mut z_fast, c));

    // If we're still in bounds, return as normal, else re-do harder comp
    let size = z_fast.abs2();
    return if size.is_finite() && size < 4. {
        limit
    } else {
        iter_mandel_many_inplace(z, c, limit)
    }
}

#[cfg(test)]
mod tests {
    use crate::complex::Complex;
    use crate::mandel::{iter_mandel_many_inplace, iter_mandel_many_inplace_batched};

    #[test]
    fn test1_inplace() {
        let mut z = Complex { r: 0., i: 0. };
        let c = Complex { r: 10., i: 10. };
        let n = iter_mandel_many_inplace(&mut z, &c, 100);
        assert_eq!(n, 1)
    }

    #[test]
    fn test2_inplace() {
        let mut z = Complex { r: 0., i: 0. };
        let c = Complex { r: 0., i: 0. };
        let n = iter_mandel_many_inplace(&mut z, &c, 100);
        assert_eq!(n, 100)
    }

    #[test]
    fn test1_batched() {
        let mut z = Complex { r: 0., i: 0. };
        let c = Complex { r: 10., i: 10. };
        let n = iter_mandel_many_inplace_batched(&mut z, &c, 100);
        assert_eq!(n, 1)
    }

    #[test]
    fn test2_batched() {
        let mut z = Complex { r: 0., i: 0. };
        let c = Complex { r: 0., i: 0. };
        let n = iter_mandel_many_inplace_batched(&mut z, &c, 100);
        assert_eq!(n, 100)
    }
    #[test]
    fn test3_batched() {
        let mut z = Complex { r: 0., i: 0. };
        let c = Complex { r: 0.5, i: 0.5 };
        let n = iter_mandel_many_inplace(&mut z, &c, 100);
        assert_eq!(n, 5)
    }
}