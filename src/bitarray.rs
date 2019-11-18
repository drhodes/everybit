// /**
//  * Copyright (c) 2019 MIT License by Derek Rhodes (ported to rust)
//  * Copyright (c) 2012 MIT License by 6.172 Staff
//  *
//  * Permission is hereby granted, free of charge, to any person obtaining a copy
//  * of this software and associated documentation files (the "Software"), to
//  * deal in the Software without restriction, including without limitation the
//  * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
//  * sell copies of the Software, and to permit persons to whom the Software is
//  * furnished to do so, subject to the following conditions:
//  *
//  * The above copyright notice and this permission notice shall be included in
//  * all copies or substantial portions of the Software.
//  *
//  * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
//  * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
//  * IN THE SOFTWARE.
//  **/
use rand::Rng;
   
/// Abstract data type representing an array of bits.
#[derive(Debug)]
pub struct BitArray {
    /// The number of bits represented by this bit array.
    /// Need not be divisible by 8.
    bit_sz: usize,

    /// The underlying memory buffer that stores the bits in
    /// packed form (8 per byte).
    data: Vec<u8>,
}

impl BitArray {
    /// Allocates space for a new bit array.
    /// bit_sz is the number of bits storable in the resultant bit array
    /// bitarray_t* bitarray_new(const size_t bit_sz);
    pub fn new(bit_sz: usize) -> BitArray {
        let num_els = bit_sz / 8;
        let data = vec![0; num_els + 1];
        BitArray { bit_sz, data }
    }
    
    pub fn from_u8(n: u8) -> BitArray {
        let data = vec![n];
        BitArray { bit_sz: 8, data }
    }
    
    pub fn from_str(bits: &str) -> BitArray {
        let mut arr = BitArray::new(bits.len());
        for (i, b) in bits.chars().rev().enumerate() {
            arr.set(i, match b {
                '0' => false,
                '1' => true,
                _ => panic!(format!("BitArray::from_str gets bad input {}", b)),
            });
        }
        return arr;
    }
    
    /// Returns the number of bits stored in a bit array.
    /// Note the invariant bitarray_get_bit_sz(bitarray_new(n)) = n.
    pub fn get_bit_sz(&self) -> usize {
        self.bit_sz
    }

    /// Does a random fill of all the bits in the bit array.
    pub fn randfill(&mut self) {
        // possible optimizations:
        // put the generator in the struct
        // use unsafe to cast our vector of u8 to u32 to vectorize the randomization
        let mut rng = rand::thread_rng();

        for byte in &mut self.data {
            *byte = rng.gen();
        }
    }

    /// Indexes into a bit array, retreiving the bit at the specified zero-based
    /// index.
    pub fn get(&self, bit_index: usize) -> bool {
        assert_eq!(true, bit_index < self.bit_sz);
        let byte_idx = bit_index / 8;
        let target_byte = self.data[byte_idx];
        target_byte & BitArray::bitmask(bit_index) != 0
    }
    
    fn bitmask(bit_index: usize) -> u8 {
        1 << (bit_index % 8)
    }

    /// Indexes into a bit array, setting the bit at the specified zero-based index.
    pub fn set(&mut self, bit_index: usize, val: bool) {
        assert_eq!(true, bit_index < self.bit_sz);
        let byte_idx = bit_index / 8;
        // self.data[byte_idx] &= BitArray::bitmask(bit_index);

        // We're storing bits in packed form, 8 per byte.  So to set
        // the nth bit, we want to set the (n mod 8)th bit of the
        // (floor(n/8)th) byte.
        //
        // In rust, integer division is floored explicitly, so we can
        // just do it to get the byte; we then bitwise-and the byte
        // with an appropriate mask to clear out the bit we're about
        // to set.  We bitwise-or the result with a byte that has
        // either a 1 or a 0 in the correct place.
        let mask = BitArray::bitmask(bit_index);
        if val {
            // set a one
            self.data[byte_idx] |= mask
        } else {
            // set a zero
            self.data[byte_idx] &= !mask            
        }
        
    }

    /// Rotates a subarray.
    ///
    /// bit_offset is the index of the start of the subarray
    /// bit_length is the length of the subarray, in bits
    /// bit_right_amount is the number of places to rotate the subarray right
    ///
    /// The subarray spans the half-open interval
    /// [bit_offset, bit_offset + bit_length)
    /// That is, the start is inclusive, but the end is exclusive.
    ///
    /// Note: bit_right_amount can be negative, in which case a left rotation is
    /// performed.
    ///
    /// Example:
    /// Let ba be a bit array containing the byte 0b10010110; then,
    /// bitarray_rotate(ba, 0, bitarray_get_bit_sz(ba), -1)
    /// left-rotates the entire bit array in place.  After the rotation, ba
    /// contains the byte 0b00101101.
    ///
    /// Example:
    /// Let ba be a bit array containing the byte 0b10010110; then,
    /// bitarray.rotate(2, 5, 2) rotates the third through seventh
    /// (inclusive) bits right two places.  After the rotation, ba contains the
    /// byte 0b10110100.
    
    pub fn rotate(&mut self,
                  bit_offset: usize,
                  bit_length: usize,
                  bit_right_amount: isize) {
        assert_eq!(true, bit_offset + bit_offset <= self.bit_sz);
        
        if bit_length == 0 {
            return;
        }
        
        // Convert a rotate left or right to a left rotate only, and eliminate
        // multiple full rotations.
        self.rotate_left(bit_offset,
                         bit_length,
                         BitArray::modulo(-bit_right_amount, bit_length));
    }

    fn rotate_left(&mut self,
                       bit_offset: usize,
                       bit_length: usize,
                       bit_left_amount: usize) {
        for _ in 0 .. bit_left_amount {
            self.rotate_left_one(bit_offset, bit_length);
        }
    }
    
    fn rotate_left_one(&mut self, bit_offset: usize, bit_length: usize) {
        // Grab the first bit in the range, shift everything left by
        // one, and then stick the first bit at the end.
        let first_bit = self.get(bit_offset);
        let mut i = bit_offset;
        
        while i + 1 < bit_offset + bit_length {
            self.set(i, self.get(i+1));
            i += 1;
        }
        self.set(i, first_bit) ;
    }
    
    fn modulo(n: isize, m: usize) -> usize {
        let signed_m = m as isize;
        assert_eq!(true, signed_m > 0);
        let result = ((n % signed_m) + signed_m) % signed_m;
        assert_eq!(true, result >= 0);
        return result as usize;
    }

    pub fn show(&self) -> String {
        let mut s = String::from("");
        let n = self.get_bit_sz();
        for i in 0 .. n {
            if self.get(i) {
                s.insert(0, '1');
            } else {
                s.insert(0, '0');
            }
        }
        s
    }
}

impl PartialEq for BitArray {
    fn eq(&self, other: &Self) -> bool {
        if self.get_bit_sz() != other.get_bit_sz() {
            return false;
        }
        for i in 0 .. self.get_bit_sz() {
            if self.get(i) != other.get(i) {
                return false;
            }
        }
        return true;
    }
}
impl Eq for BitArray {}


#[cfg(test)]
mod tests {
    use super::*;
    const N: usize = 1000;
    
    #[test]
    fn test_set1() {
        let mut arr = BitArray::new(N);
        for i in 0..N {
            arr.set(i, true);
            assert_eq!(arr.get(i), true);
        }
    }

    #[test]
    fn test_set2() {
        let mut arr = BitArray::new(N);
        for i in 0..N {
            arr.set(i, false);
            assert_eq!(arr.get(i), false);
        }
    }

    #[test]
    fn test_set3() { 
        let mut rng = rand::thread_rng();        
        let mut arr = BitArray::new(N);

        for i in 0..N {
            let x: bool = rng.gen();
            arr.set(i, x);
            assert_eq!(arr.get(i), x);
        }
    }

    #[test]
    fn test_u8_constructor() {
        let ba = BitArray::from_u8(0b10010110);
        assert_eq!(ba.get_bit_sz(), 8);
        
        assert_eq!(ba.get(0), false);
        assert_eq!(ba.get(1), true);
        assert_eq!(ba.get(2), true);
        assert_eq!(ba.get(3), false);
        assert_eq!(ba.get(4), true);
        assert_eq!(ba.get(5), false);
        assert_eq!(ba.get(6), false);
        assert_eq!(ba.get(7), true);
    }

    #[test]
    fn test_str_constructor() {
        let ba1 = BitArray::from_str("10010110");
        let ba2 = BitArray::from_u8(0b10010110);

        for i in 0 .. ba1.bit_sz {        
            assert_eq!(ba1.get(i), ba2.get(i))
        }
    }

    #[test]
    fn test_str_constructor_2() {
        let ba = BitArray::from_str("111111111111111");
        assert_eq!(15, ba.get_bit_sz());
        
        for i in 0 .. ba.get_bit_sz() {        
            assert_eq!(ba.get(i), true);
        }
    }

    
    #[test]
    fn test_rotate_left_one_1() {
        let mut ba = BitArray::from_u8(0b10010110);
        ba.rotate_left_one(0, 8);        
        let expected =  BitArray::from_u8(0b01001011);
        assert_eq!(ba.data, expected.data);
    }

    #[test]
    fn test_rotate_left_0() {
        let (start, expected) = (0b10010110,
                                 0b10010110);
        let mut ba = BitArray::from_u8(start);
        let exp_ba = BitArray::from_u8(expected);
        ba.rotate_left(0, 8, 0);        
        assert_eq!(ba.data, exp_ba.data);
    }

    #[test]
    fn test_rotate_left_one_from_str() {
        let mut ba1 = BitArray::from_str("111111101111111");
        let exp = BitArray::from_str(    "111111110111111");

        println!("{:?}", ba1.show());
        println!("{:?}", exp.show());
        
        ba1.rotate_left_one(0, exp.get_bit_sz());
        assert_eq!(ba1, exp);
    }
    
    #[test]    
    fn test_rotate_left_1() {
        let (start, expected) = (0b10010110,
                                 0b01001011);
        let mut ba = BitArray::from_u8(start);
        let exp_ba = BitArray::from_u8(expected);
        ba.rotate_left(0, 8, 1);        
        assert_eq!(ba.data, exp_ba.data);
    }

    #[test]
    fn test_rotate_left_2() {
        let (start, expected) = (0b10010110,
                                 0b10100101,);
        let mut ba = BitArray::from_u8(start);
        let exp_ba = BitArray::from_u8(expected);
        ba.rotate_left(0, 8, 2);        
        assert_eq!(ba.data, exp_ba.data);
    }

    #[test]
    fn test_rotate_left_142() {
        let (start, expected) = (0b10010110,
                                 //   ||||  
                                 0b10011100,);
        let mut ba = BitArray::from_u8(start);
        let exp_ba = BitArray::from_u8(expected);
        ba.rotate_left(1, 4, 2);        
        println!("{:?} =? {:?}", ba.show(), exp_ba.show());
        assert_eq!(ba.data, exp_ba.data);
    }

    #[test]
    fn test_rotate_left_144() {
        let (start, expected) = (0b10010110,
                                 //   ||||  
                                 0b10010110,);
        let mut ba = BitArray::from_u8(start);
        let exp_ba = BitArray::from_u8(expected);
        ba.rotate_left(1, 4, 4);        
        println!("{:?} =? {:?}", ba.show(), exp_ba.show());
        assert_eq!(ba.data, exp_ba.data);
    }
    
    #[test] 
    fn test_rotate_right() {
        // Let ba be a bit array containing the byte 0b10010110; then,
        // bitarray_rotate(ba, 2, 5, 2) rotates the third through seventh
        // (inclusive) bits right two places.  After the rotation, ba contains the
        // byte 0b10110100.

        // 0b10010110
        //    |||||
        //    10010
        //    11001
        // 0b11100110
        let (start, expected) = (0b10010110,
                                 // |||||
                                 // 01010 
                                 // 10100 
                                 0b11010010);
        let mut ba = BitArray::from_u8(start);
        let exp_ba = BitArray::from_u8(expected);
        ba.rotate(2,5,2);
        println!("expected {} ", exp_ba.show());
        println!("     got {} ", ba.show());
        assert_eq!(ba.data, exp_ba.data);
    }

    #[test]
    fn test_modulo() {
        // these cases were generated from the output of the C modulo
        // function
        assert_eq!(BitArray::modulo(-5, 1), 0);
        assert_eq!(BitArray::modulo(-5, 2), 1);
        assert_eq!(BitArray::modulo(-5, 3), 1);
        assert_eq!(BitArray::modulo(-5, 4), 3);
        assert_eq!(BitArray::modulo(-4, 1), 0);
        assert_eq!(BitArray::modulo(-4, 2), 0);
        assert_eq!(BitArray::modulo(-4, 3), 2);
        assert_eq!(BitArray::modulo(-4, 4), 0);
        assert_eq!(BitArray::modulo(-3, 1), 0);
        assert_eq!(BitArray::modulo(-3, 2), 1);
        assert_eq!(BitArray::modulo(-3, 3), 0);
        assert_eq!(BitArray::modulo(-3, 4), 1);
        assert_eq!(BitArray::modulo(-2, 1), 0);
        assert_eq!(BitArray::modulo(-2, 2), 0);
        assert_eq!(BitArray::modulo(-2, 3), 1);
        assert_eq!(BitArray::modulo(-2, 4), 2);
        assert_eq!(BitArray::modulo(-1, 1), 0);
        assert_eq!(BitArray::modulo(-1, 2), 1);
        assert_eq!(BitArray::modulo(-1, 3), 2);
        assert_eq!(BitArray::modulo(-1, 4), 3);
        assert_eq!(BitArray::modulo(0, 1), 0);
        assert_eq!(BitArray::modulo(0, 2), 0);
        assert_eq!(BitArray::modulo(0, 3), 0);
        assert_eq!(BitArray::modulo(0, 4), 0);
        assert_eq!(BitArray::modulo(1, 1), 0);
        assert_eq!(BitArray::modulo(1, 2), 1);
        assert_eq!(BitArray::modulo(1, 3), 1);
        assert_eq!(BitArray::modulo(1, 4), 1);
        assert_eq!(BitArray::modulo(2, 1), 0);
        assert_eq!(BitArray::modulo(2, 2), 0);
        assert_eq!(BitArray::modulo(2, 3), 2);
        assert_eq!(BitArray::modulo(2, 4), 2);
        assert_eq!(BitArray::modulo(3, 1), 0);
        assert_eq!(BitArray::modulo(3, 2), 1);
        assert_eq!(BitArray::modulo(3, 3), 0);
        assert_eq!(BitArray::modulo(3, 4), 3);
        assert_eq!(BitArray::modulo(4, 1), 0);
        assert_eq!(BitArray::modulo(4, 2), 0);
        assert_eq!(BitArray::modulo(4, 3), 1);
        assert_eq!(BitArray::modulo(4, 4), 0);
    }   
}

