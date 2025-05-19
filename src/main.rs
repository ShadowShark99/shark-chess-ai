type PiecePosition = u64;

// fn bit_to_position(bit: PiecePosition) -> Result<String, String>{
//     if bit == 0{
//         return Err("No Piece present.".to_string());
//     }
//     else{
//         let onebit_index = bit_scan(bit);
//         return Ok(index_to_position(onebit_index));
//     }
// }


fn index_to_position(index:u8) -> String{
    let col = index % 8;
    let row = index / 8 + 1;
    
    return format!("{}{}", (col+97) as char, row);
}

fn main() {
    let black_pawns: i64 = 0; 
    for i in 0..64{
        println!("{} -> {}", i, index_to_position(i));
    }
}
