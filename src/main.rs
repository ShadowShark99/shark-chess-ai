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

static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

fn index_to_position(index:usize) -> String{
    let col = (index % 8);
    let row = index / 8 + 1;
    //
}

fn main() {
    let black_pawns: i64 = 0b; 
    println!("Hello, world!");
}
