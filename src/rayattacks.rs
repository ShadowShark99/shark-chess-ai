type Bitboard = u64;


pub struct Rays{
  n_rays: Vec<u64>,
}

impl Rays{
  fn initialize() -> Self {
    let mut n_rays = vec![];
    
    for row in 1..=8{
      for col in 1..=8{
        n_rays.push(n_ray(row,col));
      }
    }

    Self{
      n_rays,
    }
  }
}

fn n_ray(row: u64, col: u64) -> Bitboard{
  let mut bitboard = 0;

  for offset in 1..=8{
    if row + offset > 8 {
      break;
    }
    bitboard = set_bit(bitboard, row + offset, col);
  }
  bitboard
}

fn set_bit(bitboard: Bitboard, row: u64, col: u64) -> Bitboard{
  bitboard | (1 << ((col - 1) + (row - 1) * 8))
}

fn print_bitboard(bitboard: Bitboard, mark: Option<usize>) -> String{
  let mut row = "".to_owned();
  let mut board = "".to_owned();
  for i in 0..64{
    let value = (bitboard >> i) & 1;

    let s = if value == 0{
      ".".to_owned()
    }
    else
    {
      value.to_string()
    };

    match mark {
      Some (idx) => if i == idx{
        row.push_str("X");
      } else {
        row.push_str(&s);
      },
      None => row.push_str(&s),
    }

    //at the end of the row 0..7, after i % 8 === 7, insert completed row
    if((i+1) % 8) == 0 {
      row.push_str("\n");
      board.insert_str(0, &row);
      println!("Here's the bitboard:\n--------\n{}\n---------", board);
      row.clear();
    }
  }
  board
}

#[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn make_n_ray(){
    println!("Here's the bitboard:\n--------\n{}\n---------", print_bitboard(n_ray(4,4), None));
  }
}