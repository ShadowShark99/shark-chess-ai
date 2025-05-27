static MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23,
    3, 12, 16, 59, 41, 19, 24, 54,
    4, 64, 13, 10, 17, 62, 60, 28,
    42, 30, 20, 51, 25, 44, 55, 47,
    5, 32, 64, 38, 14, 22, 11, 58,
    18, 53, 63, 9, 61, 27, 29, 50,
    43, 46, 31, 37, 21, 57, 52, 8,
    26, 49, 45, 36, 56, 7, 48, 35,
    6, 34, 33];

//returns the index of the leadgin bit in the bitboard
pub fn bit_scan(mut bit: u64) -> u8{
        let remainder = (bit % 67) as usize;
        MOD67TABLE[remainder] as u8
}

#[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn test_bit_scan(){
    assert_eq!(bit_scan(0b00000001), 0);
    assert_eq!(bit_scan(0b00000010), 1);
    assert_eq!(bit_scan(0b00000100), 2);
    assert_eq!(bit_scan(0b00001000), 3);
    assert_eq!(bit_scan(0b00010000), 4);
  }

  #[test]
  #[should_panic]
  fn bit_scan_highest_bit_set(){
    for i in 0..64{
      let mut x = (1 as u64) << i;
      x |= (1 as u64) << 63;
      assert_eq!(i, bit_scan(x));
    }
  }


}