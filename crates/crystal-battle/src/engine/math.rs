//! `engine/math/math.asm`: `Multiply` and `Divide`, operating on the same
//! (overlapping) HRAM bytes as the original. Battle code reads leftovers of
//! these buffers (for example `hMultiplicand + 1` after a `Divide`, which is
//! `hQuotient + 2`), so the byte-level behaviour is reproduced exactly.

use super::battle::Engine;
use super::sym::*;

impl Engine {
    fn h(&self, addr: u16) -> u8 {
        self.mem.rb(addr)
    }

    fn set_h(&mut self, addr: u16, v: u8) {
        self.mem.wb(addr, v);
    }

    /// `Multiply`: `hMultiplicand` (3 bytes, big endian) × `hMultiplier` →
    /// `hProduct` (4 bytes). A port of `_Multiply`.
    pub fn multiply(&mut self) {
        let mut b = 8u8;
        self.set_h(H_MULTIPLICAND - 1, 0);
        for i in 1..=4 {
            self.set_h(H_MATH_BUFFER + i, 0);
        }
        loop {
            // .loop
            let m = self.h(H_MULTIPLIER);
            let carry = m & 1 != 0;
            self.set_h(H_MULTIPLIER, m >> 1);
            if carry {
                let mut c = false;
                for (buf, mc) in [(4u16, 2u16), (3, 1), (2, 0)] {
                    let sum = u16::from(self.h(H_MATH_BUFFER + buf))
                        + u16::from(self.h(H_MULTIPLICAND + mc))
                        + u16::from(c);
                    self.set_h(H_MATH_BUFFER + buf, sum as u8);
                    c = sum > 0xff;
                }
                let sum = u16::from(self.h(H_MATH_BUFFER + 1)) + u16::from(self.h(H_MULTIPLICAND - 1)) + u16::from(c);
                self.set_h(H_MATH_BUFFER + 1, sum as u8);
            }
            // .next
            b -= 1;
            if b == 0 {
                break;
            }
            // hMultiplicand <<= 1 (4 bytes including hMultiplicand - 1)
            let mut c = false;
            for off in [2i16, 1, 0, -1] {
                let addr = (H_MULTIPLICAND as i16 + off) as u16;
                let v = self.h(addr);
                self.set_h(addr, v << 1 | u8::from(c));
                c = v & 0x80 != 0;
            }
        }
        // .done
        for (prod, buf) in [(3u16, 4u16), (2, 3), (1, 2), (0, 1)] {
            let v = self.h(H_MATH_BUFFER + buf);
            self.set_h(H_PRODUCT + prod, v);
        }
    }

    /// `Divide`: `hDividend` (`b` bytes, big endian) ÷ `hDivisor` →
    /// `hQuotient` (4 bytes), `hRemainder`. A port of `_Divide`.
    pub fn divide(&mut self, mut b: u8) {
        // The original loops forever dividing by zero (the game freezes, e.g.
        // pokecrystal's "Switching out against a mon with max HP below 4" bug).
        assert!(self.h(H_DIVISOR) != 0, "Divide by zero: the original game freezes here");
        for i in 0..=4 {
            self.set_h(H_MATH_BUFFER + i, 0);
        }
        let mut e = 9u8;
        loop {
            // .loop
            let c0 = self.h(H_MATH_BUFFER);
            let d1 = self.h(H_DIVIDEND + 1);
            let (d, borrow1) = d1.overflowing_sub(c0);
            let div = self.h(H_DIVISOR);
            let d0 = self.h(H_DIVIDEND);
            let (r, borrow2a) = d0.overflowing_sub(div);
            let (r, borrow2b) = r.overflowing_sub(u8::from(borrow1));
            if !(borrow2a || borrow2b) {
                self.set_h(H_DIVIDEND, r);
                self.set_h(H_DIVIDEND + 1, d);
                let q = self.h(H_MATH_BUFFER + 4).wrapping_add(1);
                self.set_h(H_MATH_BUFFER + 4, q);
                continue;
            }
            // .next
            if b == 1 {
                break;
            }
            let mut c = false;
            for buf in [4u16, 3, 2, 1] {
                let v = self.h(H_MATH_BUFFER + buf);
                self.set_h(H_MATH_BUFFER + buf, v << 1 | u8::from(c));
                c = v & 0x80 != 0;
            }
            e -= 1;
            if e == 0 {
                e = 8;
                let v = self.h(H_MATH_BUFFER);
                self.set_h(H_DIVISOR, v);
                self.set_h(H_MATH_BUFFER, 0);
                for i in 0..3 {
                    let v = self.h(H_DIVIDEND + i + 1);
                    self.set_h(H_DIVIDEND + i, v);
                }
            }
            // .next2
            if e == 1 {
                b -= 1;
            }
            // .okay
            let div = self.h(H_DIVISOR);
            let carry = div & 1 != 0;
            self.set_h(H_DIVISOR, div >> 1);
            let m0 = self.h(H_MATH_BUFFER);
            self.set_h(H_MATH_BUFFER, m0 >> 1 | if carry { 0x80 } else { 0 });
        }
        // .done
        let rem = self.h(H_DIVIDEND + 1);
        self.set_h(H_REMAINDER, rem);
        for (q, buf) in [(3u16, 4u16), (2, 3), (1, 2), (0, 1)] {
            let v = self.h(H_MATH_BUFFER + buf);
            self.set_h(H_QUOTIENT + q, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::battle::Flags;
    use crate::engine::mem::Mem;

    fn engine() -> Engine {
        Engine::new(Mem::default(), 1, Flags::default())
    }

    #[test]
    fn multiply_matches_arithmetic() {
        let mut e = engine();
        for (a, m) in [(0u32, 0u8), (1, 1), (300, 15), (0x123456, 0xff), (65535, 200)] {
            e.mem.wb(H_MULTIPLICAND, (a >> 16) as u8);
            e.mem.wb(H_MULTIPLICAND + 1, (a >> 8) as u8);
            e.mem.wb(H_MULTIPLICAND + 2, a as u8);
            e.mem.wb(H_MULTIPLIER, m);
            e.multiply();
            let p = (0..4).fold(0u32, |acc, i| acc << 8 | u32::from(e.mem.rb(H_PRODUCT + i)));
            assert_eq!(p, a * u32::from(m), "{a} * {m}");
        }
    }

    #[test]
    fn divide_matches_arithmetic() {
        let mut e = engine();
        for (n, d) in [(100u32, 10u8), (0x12345678, 7), (255, 255), (1, 3), (0xffff, 0x13)] {
            for i in 0..4 {
                e.mem.wb(H_DIVIDEND + i, (n >> (24 - 8 * i)) as u8);
            }
            e.mem.wb(H_DIVISOR, d);
            e.divide(4);
            let q = (0..4).fold(0u32, |acc, i| acc << 8 | u32::from(e.mem.rb(H_QUOTIENT + i)));
            assert_eq!(q, n / u32::from(d), "{n} / {d}");
            assert_eq!(u32::from(e.mem.rb(H_REMAINDER)), n % u32::from(d), "{n} % {d}");
        }
    }
}
