use ubongo::*;

fn main() {
    // implement fit function for two pieces
    // hand over array of pieces to fit function

    let mut board = Board {
        fields : [[false; 5]; 5],
        count : 0
    };

    let mut red_piece = ubongo::Piece {
        fields : [[false; 4]; 4],
        count : 0
    };

    let mut blue_piece = Piece {
        fields : [[false; 4]; 4],
        count : 0
    };

    init_board(&mut board);
    init_red_piece(&mut red_piece);    
    init_blue_piece(&mut blue_piece);    

    todo!("Repair fit method");
    // println!("Piece fits into board: {}", fit(board, red_piece));
}

fn init_red_piece(piece: &mut Piece) {
    piece.fields[0][0] = true;
    piece.fields[0][1] = true;
    piece.fields[1][0] = true;
    piece.fields[1][1] = true;
    piece.count = 4;
}

fn init_blue_piece(piece: &mut Piece) {
    piece.fields[0][0] = true;
    piece.fields[0][1] = true;
    piece.fields[0][2] = true;
    piece.count = 3;
}

fn init_board(board: &mut Board) {
    board.fields[0][0] = true;
    board.fields[0][1] = true;
    board.fields[0][2] = true;
    board.fields[0][3] = true;
    
    board.fields[1][0] = true;
    board.fields[1][1] = true;
    board.fields[1][2] = true;
    board.fields[1][3] = true;

    board.fields[2][0] = true;
    board.fields[2][1] = true;
    board.fields[2][2] = true;

    board.count = 11;
}

