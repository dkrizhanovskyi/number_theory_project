/// This module handles operations on finite fields and modular arithmetic.
/// It provides fundamental operations used in number theory, including
/// modular addition, subtraction, multiplication, and exponentiation.

/// Represents an element of a finite field modulo `p`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModInt {
    pub value: u64,
    pub modulus: u64,
}

impl ModInt {
    /// Creates a new element in the finite field.
    pub fn new(value: u64, modulus: u64) -> Self {
        ModInt {
            value: value % modulus,
            modulus,
        }
    }

    /// Adds two elements in the finite field.
    pub fn add(self, other: ModInt) -> ModInt {
        assert_eq!(self.modulus, other.modulus);
        ModInt::new((self.value + other.value) % self.modulus, self.modulus)
    }

    /// Multiplies two elements in the finite field.
    pub fn multiply(self, other: ModInt) -> ModInt {
        assert_eq!(self.modulus, other.modulus);
        ModInt::new((self.value * other.value) % self.modulus, self.modulus)
    }

    /// Raises the element to a power within the finite field.
    pub fn pow(self, exponent: u64) -> ModInt {
        let mut result = ModInt::new(1, self.modulus);
        let mut base = self;
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.multiply(base);
            }
            base = base.multiply(base);
            exp /= 2;
        }

        result
    }

    /// Computes the modular inverse of the element.
    pub fn inverse(self) -> Option<ModInt> {
        let mut t = 0;
        let mut new_t = 1;
        let mut r = self.modulus as i64;
        let mut new_r = self.value as i64;

        while new_r != 0 {
            let quotient = r / new_r;
            t -= quotient * new_t;
            r -= quotient * new_r;

            std::mem::swap(&mut t, &mut new_t);
            std::mem::swap(&mut r, &mut new_r);
        }

        if r > 1 {
            return None;
        }
        if t < 0 {
            t += self.modulus as i64;
        }

        Some(ModInt::new(t as u64, self.modulus))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modint_add() {
        let a = ModInt::new(7, 10);
        let b = ModInt::new(8, 10);
        assert_eq!(a.add(b), ModInt::new(5, 10));
    }

    #[test]
    fn test_modint_multiply() {
        let a = ModInt::new(7, 10);
        let b = ModInt::new(8, 10);
        assert_eq!(a.multiply(b), ModInt::new(6, 10));
    }

    #[test]
    fn test_modint_pow() {
        let a = ModInt::new(7, 10);
        assert_eq!(a.pow(3), ModInt::new(3, 10));
    }

    #[test]
    fn test_modint_inverse() {
        let a = ModInt::new(3, 11);
        assert_eq!(a.inverse(), Some(ModInt::new(4, 11)));
    }
}
