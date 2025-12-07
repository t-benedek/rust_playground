use super::*;

// Board looks like this
// 1 1 1 1 1
// 1 1 1 1 0 
// 1 1 1 0 0 
// 0 0 0 0 0 
// 0 0 0 0 0 
fn create_board() -> Board {
    let mut board = Board {
        fields: [[false; 5]; 5],
        count: 0,
    };
    board.fields[0][0] = true;
    board.fields[1][0] = true;
    board.fields[2][0] = true;
    board.fields[3][0] = true;
    board.fields[4][0] = true;
    
    board.fields[0][1] = true;
    board.fields[1][1] = true;
    board.fields[2][1] = true;
    board.fields[3][1] = true;

    board.fields[0][2] = true;
    board.fields[1][2] = true;
    board.fields[2][2] = true;
    board.count = 11;
    
    return board;
}

// Red Piece looks like this
// 1 1 0 0  
// 1 1 0 0  
// 0 0 0 0  
fn create_red_piece() -> Piece {
    let mut piece = Piece {
        fields : [[false; 4]; 4],
        count : 0
    };
    piece.fields[0][0] = true;
    piece.fields[0][1] = true;
    piece.fields[1][0] = true;
    piece.fields[1][1] = true;
    piece.count = 4;

    return piece;
}

// Blue Piece looks like this
// 1 1 1 0  
// 0 0 0 0  
// 0 0 0 0  
fn create_blue_piece() -> Piece {
    let mut piece = Piece {
        fields : [[false; 4]; 4],
        count : 0
    };

    piece.fields[0][0] = true;
    piece.fields[1][0] = true;
    piece.fields[2][0] = true;
    piece.count = 3;

    return piece;
}

// Green_1 Piece looks like this
// 1 1 0 0  
// 0 0 0 0  
// 0 0 0 0  
fn create_green1_piece() -> Piece {
    let mut piece = Piece {
        fields : [[false; 4]; 4],
        count : 0
    };
    
    piece.fields[0][0] = true;
    piece.fields[1][0] = true;
    piece.count = 2;
    return piece;
}
    
// green2 is a rotation of green1
// 0 1 0 0  
// 0 1 0 0  
// 0 0 0 0  
fn create_green2_piece() -> Piece {
    let mut piece = Piece {
        fields : [[false; 4]; 4],
        count : 0
    };

    piece.fields[1][0] = true;
    piece.fields[1][1] = true;
    piece.count = 2;
    return piece;
}

#[test]
fn test_set_blue() {        
    let mut board = create_board();
    let blue_piece = create_blue_piece();

    /* Board before */
    // 1 1 1 1 1
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
    
    assert_eq!(board.fields[2][0], true);
    assert_eq!(board.fields[3][0], true);
    assert_eq!(board.fields[4][0], true);
    set_piece(&mut board.fields, &blue_piece, 2, 0);
    assert_eq!(board.fields[2][0], false);
    assert_eq!(board.fields[3][0], false);
    assert_eq!(board.fields[4][0], false);

    /* Board after */
    // 1 1 0 0 0
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
}


#[test]
fn test_canfit_blue() {        
    let board = create_board();
    let blue_piece = create_blue_piece();
    
    assert!(   passing_piece(&board.fields, &blue_piece, 0, 1));
    assert!(   passing_piece(&board.fields, &blue_piece, 1, 1));
    assert!(   passing_piece(&board.fields, &blue_piece, 2, 0));
    assert!( ! passing_piece(&board.fields, &blue_piece, 1, 2));
    assert!( ! passing_piece(&board.fields, &blue_piece, 2, 1));
}

#[test]
fn test_canfit_red() {        
    let board = create_board();
    let piece = create_red_piece(); 
    
    assert!(   passing_piece(&board.fields, &piece, 2, 0));
    assert!( ! passing_piece(&board.fields, &piece, 2, 1));
    assert!( ! passing_piece(&board.fields, &piece, 3, 0));
}

#[test]
fn test_canfit_green() {
    let board = create_board();
    let green1_piece = create_green1_piece();
    let green2_piece = create_green2_piece();

    /* TRUE */
    assert!(passing_piece(&board.fields, &green1_piece, 3, 0));
    assert!(passing_piece(&board.fields, &green1_piece, 2, 1));
    assert!(passing_piece(&board.fields, &green1_piece, 1, 2));
    assert!(passing_piece(&board.fields, &green1_piece, 1, 2));
    assert!(passing_piece(&board.fields, &green2_piece, 0, 1));
    assert!(passing_piece(&board.fields, &green2_piece, 1, 1));
    
    /* FALSE */
    assert!( ! passing_piece(&board.fields, &green1_piece, 3, 1));
    assert!( ! passing_piece(&board.fields, &green1_piece, 2, 2));
    assert!( ! passing_piece(&board.fields, &green2_piece, 1, 3));
    assert!( ! passing_piece(&board.fields, &green2_piece, 0, 4));
    assert!( ! passing_piece(&board.fields, &green2_piece, 0, 2));
    assert!( ! passing_piece(&board.fields, &green2_piece, 2, 1));
    assert!( ! passing_piece(&board.fields, &green2_piece, 3, 0));
}       