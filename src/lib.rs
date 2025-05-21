use bitflags::bitflags;

type PiecePosition = u64;

fn bit_to_position(bit: PiecePosition) -> Result<String, String>{
    if bit == 0{
        return Err("No Piece present.".to_string());
    }
    else{
        let onebit_index = bit_scan(bit);
        return Ok(index_to_position(onebit_index));
    }
}



fn index_to_position(index:u8) -> String{
    let col = index % 8;
    let row = index / 8 + 1;
    
    return format!("{}{}", (col+97) as char, row);
}

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
fn bit_scan(mut bit: u64) -> u8{
        let remainder = (bit % 67) as usize;
        MOD67TABLE[remainder] as u8
}

#[derive(Debug, PartialEq)]
enum Color{
    White,
    Black,
}

#[derive(Debug, PartialEq)]
enum PieceType{
    Pawn, 
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, PartialEq)]
struct Piece{
    piece_type: PieceType,
    color: Color,
    position: PiecePosition,
}

#[derive(Debug, PartialEq)]
enum Square{
    Empty,
    //index of piece to find in pieces vector
    Occupied(usize),
}


fn vector_reverse<T>(vec: &mut Vec<T>){
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
    active_color: Color,
    castling_rights: CastlingRights,
    en_passant: Option<PiecePosition>,
    halfmove_clock: usize,
    fullmove_number: usize,
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
          let i: usize = 0;
          //use .chars
          let pos = info[3].chars().next().unwrap() as usize - 97;
          let row = info[3].chars().nth(1).unwrap().to_digit(10).unwrap() as usize - 1;
          Some(1 << (pos + row * 8))
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