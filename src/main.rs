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
struct Game{
    pieces: Vec<Piece>,
    squares: Vec<Square>,
    active_color: Color,
    castling_rights: CastlingRights,
    en_passant: Option<PiecePosition>,
    halfmove_clock: usize,
    fullmove_number: usize,
}

impl Game{
    fn initialize() -> Game{
        Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
    }

    fn to_string(&self) -> String{
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
    fn from_fen(fen: &str) -> Game{
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
        Game{
            pieces,
            squares,
            active_color: Color::White,
            castling_rights: CastlingRights::ALL,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
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

fn main() {
    
    let game = Game::initialize();

    game.to_string();
    println!("{}", game.to_string());
}
