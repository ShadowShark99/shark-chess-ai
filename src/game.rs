use bitflags::bitflags;
use crate::utils::*;

type PiecePosition = u64;

pub fn bit_to_position(bit: PiecePosition) -> Result<String, String>{
    if bit == 0{
        return Err("No Piece present.".to_string());
    }
    else{
        let onebit_index = bit_scan(bit);
        return Ok(index_to_position(onebit_index));
    }
}

pub fn position_to_bit(position: String) -> Result<PiecePosition, String>{
    if position.len() != 2{
        return Err("Invalid position".to_string());
    }

    let bytes = position.as_bytes();
    let byte0 = bytes[0];
    let byte1 = bytes[1];
    if byte0 < 97 || byte0 > 104{
        return Err("Invalid position".to_string());
    }

    let col = byte0 - 97 as u8;
    let row;
    match (byte1 as char).to_digit(10){
        Some(r) => {
            if r < 1 || r > 8{
              return Err("Invalid position".to_string());
            }
            else{
                row = (r - 1) as u8;
            }
        },
        None => {
            return Err("Invalid position".to_string());
        }
    }

    
    Ok(1 << (col + row * 8))
}



pub fn index_to_position(index:u8) -> String{
    let col = index % 8;
    let row = index / 8 + 1;
    
    return format!("{}{}", (col+97) as char, row);
}



#[derive(Debug, PartialEq)]
pub enum Color{
    White,
    Black,
}

#[derive(Debug, PartialEq)]
pub enum PieceType{
    Pawn, 
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, PartialEq)]
pub struct Piece{
    piece_type: PieceType,
    color: Color,
    position: PiecePosition,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Square{
    Empty,
    //index of piece to find in pieces vector
    Occupied(usize),
}


pub fn vector_reverse<T>(vec: &mut Vec<T>){
    let mut i = 0;
    let mut j = vec.len() - 1;
    while i < j{
        vec.swap(i, j);
        i += 1;
        j -= 1;
    }
}

#[derive(Debug, PartialEq)]
pub struct Board{
    pieces: Vec<Piece>,
    squares: Vec<Square>,
}


impl Board{
    pub fn initialize() -> Board{
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
    }

    pub fn to_string(&self) -> String{
        let mut board = "".to_owned();
        let mut temp = "".to_owned();

        for(i, square) in self.squares.iter().enumerate(){
            match square{
                Square::Empty => temp.push_str(&index_to_position(i.try_into().unwrap())),
                Square::Occupied(idx) => temp.push_str(&self.pieces[*idx].to_string()),
                
            }
            if(i+1) % 8 == 0 {
                temp.push_str("\n");
                board.insert_str(0, &temp);
                temp.clear();
            }
            //board.insert_str(0,&temp);
        }
        board
    }

    //generate a game from fen string
    pub fn from_fen(fen: &str) -> Board{
        let mut pieces = vec![];
        let mut squares = vec![];
        let mut position = 0;
        
        //parse the fen string and create a game object
        let mut rows = fen.split("/").collect::<Vec<_>>();
        vector_reverse(&mut rows);

        for row in rows{
            for c in row.chars(){
                if(c.is_digit(10)){
                    let num = c.to_digit(10).unwrap();
                    for _ in 0..num{

                        squares.push(Square::Empty);
                        position += 1;
                    }
                }
                else{
                    let piece_type = match c.to_ascii_lowercase(){
                        'p' => PieceType::Pawn,
                        'r' => PieceType::Rook,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        _ => panic!("Invalid piece type"),
                    };
                    let color = if c.is_uppercase(){Color::White}else{Color::Black};
                    pieces.push(Piece{piece_type, color, position: 1<<position});
                    
                    squares.push(Square::Occupied(pieces.len()-1));
                    position += 1;
                }
            }
        }
        Board{
            pieces,
            squares,
        }

    }
}

bitflags!{
  struct CastlingRights: u8{
      const NONE = 0;
      const WHITEKINGSIDE = 1 << 0;
      const WHITEQUEENSIDE = 1 << 1;
      const BLACKKINGSIDE = 1 << 2;
      const BLACKQUEENSIDE = 1 << 3;
      const ALL = Self::WHITEKINGSIDE.bits | Self::WHITEQUEENSIDE.bits | Self::BLACKKINGSIDE.bits | Self::BLACKQUEENSIDE.bits;

  }
}

#[derive(Debug, PartialEq)]
pub struct Game{
    board: Board,
    pub active_color: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<PiecePosition>,
    pub halfmove_clock: usize,
    pub fullmove_number: usize,
}

impl Game{
  pub fn initialize() -> Game{
      
      //eventual will be Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
      Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
  }

  pub fn to_string(&self) -> String{
      Board::to_string(&self.board)
  }

  //generate a game from fen string
  //"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
  pub fn from_fen(fen: &str) -> Game{
      //parse the fen string and create a game object
      let info = fen.split(" ").collect::<Vec<_>>();
      let board = Board::from_fen(info[0]);
      let active_color = if info[1] == "w" {Color::White} else {Color::Black};
      let castling_rights = match info[2]{
          "-" => CastlingRights::NONE,
          _ => {
              let mut rights = CastlingRights::NONE;
              for c in info[2].chars(){
                  match c{
                      'K' => rights |= CastlingRights::WHITEKINGSIDE,
                      'Q' => rights |= CastlingRights::WHITEQUEENSIDE,
                      'k' => rights |= CastlingRights::BLACKKINGSIDE,
                      'q' => rights |= CastlingRights::BLACKQUEENSIDE,
                      _ => panic!("Invalid castling rights"),
                  }
              }
              rights
          }
      };
      let en_passant = if info[3] == "-" {
          None
      } else {
          //error
          // let i: usize = 0;
          // //use .chars
          // let pos = info[3].chars().next().unwrap() as usize - 97;
          // let row = info[3].chars().nth(1).unwrap().to_digit(10).unwrap() as usize - 1;
          // Some(1 << (pos + row * 8))
          match position_to_bit(info[3].to_string()){
              Ok(bit) => Some(bit),
              Err(msg) => panic!("{}", msg),
          }

      };
      let halfmove_clock = info[4].parse::<usize>().unwrap();
      let fullmove_number = info[5].parse::<usize>().unwrap();
      Game{
          board,
          active_color,
          castling_rights,
          en_passant,
          halfmove_clock,
          fullmove_number,
      }

  }
}

impl Piece{
    fn to_string(&self) -> String{
        let mut result = match self.piece_type{
            PieceType::Pawn => "p ",
            PieceType::Rook => "r ",
            PieceType::Knight => "n ",
            PieceType::Bishop => "b ",
            PieceType::Queen => "q ",
            PieceType::King => "k ",
        }.to_string();

        if(self.color == Color::White){
            result.make_ascii_uppercase();
        }

        result
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn read_initial_position(){
        let game = Game::initialize();
        assert_eq!(game.active_color, Color::White);
        assert_eq!(game.castling_rights, CastlingRights::ALL);
        assert_eq!(game.en_passant, None);
        assert_eq!(game.halfmove_clock, 0);
        assert_eq!(game.fullmove_number, 1);
    }

    #[test]
    fn black_to_move(){
        let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
        assert_eq!(game.active_color, Color::Black);
    }

    #[test]
    fn castling_rights_none(){
        let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - - 0 1");
        assert_eq!(game.castling_rights, CastlingRights::NONE);
    }

    #[test]
    fn en_passant_none(){
        let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - - 0 1");
        assert_eq!(game.en_passant, None);
    }
}