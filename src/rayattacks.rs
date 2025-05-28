type Bitboard = u64;


pub struct Rays{
  n_rays: Vec<Bitboard>,
  e_rays: Vec<Bitboard>,
  s_rays: Vec<Bitboard>,
  w_rays: Vec<Bitboard>,
}

macro_rules! make_rays{
  ($ray_fn:ident) => {{
    
    let mut rays = vec![];
    for row in 1..=8{
      for col in 1..=8{
        rays.push($ray_fn(row,col));
      }
    }
    rays
  }};
}

impl Rays{
  fn initialize() -> Self {
    let n_rays = make_rays!(n_ray);
    let e_rays = make_rays!(e_ray);
    let s_rays = make_rays!(s_ray);
    let w_rays = make_rays!(w_ray);
    

    Self{
      n_rays,
      e_rays,
      s_rays,
      w_rays,
    }
  }
}


//set bits for directional rays: n, e, s, w
macro_rules! define_ray{
  ($name:ident, $offset_fn:expr) => {
    fn $name(row: i64, col: i64) -> Bitboard{
      let mut bitboard = 0;
    
      for offset in 1..=8{
        bitboard = set_bit(bitboard, $offset_fn(row, col, offset));
      }
      bitboard
    }
  }

}

//each direction function is defined for n, e, s, w
define_ray!(n_ray, |row, col, offset| (row + offset, col));
define_ray!(e_ray, |row, col, offset| (row, col + offset));
define_ray!(s_ray, |row, col, offset| (row - offset, col));
define_ray!(w_ray, |row, col, offset| (row, col - offset));

fn set_bit(bitboard: Bitboard, row_col: (i64, i64)) -> Bitboard{
  let row = row_col.0;
  let col = row_col.1;
  //return board if out of bounds
  if(row < 1 || row > 8 || col < 1 || col > 8)
  {
    return bitboard;
  }
  //or with respective board coordinate, to set that square on the board
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
    println!("Here's the bitboard:\n--------\n{}\n---------", print_bitboard(e_ray(4,4), None));
    println!("Here's the bitboard:\n--------\n{}\n---------", print_bitboard(s_ray(4,4), None));
    println!("Here's the bitboard:\n--------\n{}\n---------", print_bitboard(w_ray(4,4), None));
  }
}