use super::*;

#[test]
fn test_set_blue_hori() {        
    let mut board = create_board();
    let blue_hori = create_blue_piece(true);

    /* Board before */
    // 1 1 1 1 1
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
    
    assert_eq!(board.fields[2][0], true);
    assert_eq!(board.fields[3][0], true);
    assert_eq!(board.fields[4][0], true);
    set_piece(&mut board.fields, &blue_hori, 2, 0);
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
fn test_set_blue_vert() {        
    let mut board = create_board();
    let blue_vert = create_blue_piece(false);

    /* Board before */
    // 1 1 1 1 1
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
    
    assert_eq!(board.fields[1][0], true);
    assert_eq!(board.fields[1][1], true);
    assert_eq!(board.fields[1][2], true);
    set_piece(&mut board.fields, &blue_vert, 1, 0);
    assert_eq!(board.fields[1][0], false);
    assert_eq!(board.fields[1][1], false);
    assert_eq!(board.fields[1][2], false);

    /* Board after */
    // 1 0 1 1 1
    // 1 0 1 1 0 
    // 1 0 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
}


#[test]
fn test_set_violet_vert() {        
    let mut board = create_board();
    let violet_vert = create_violet(false);

    /* Board before */
    // 1 1 1 1 1
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
    
    assert_eq!(board.fields[1][0], true);
    assert_eq!(board.fields[1][1], true);
    set_piece(&mut board.fields, &violet_vert, 1, 0);
    assert_eq!(board.fields[1][0], false);
    assert_eq!(board.fields[1][1], false);

    /* Board after */
    // 1 0 1 1 1
    // 1 0 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
}


#[test]
fn test_canfit_blue() {        
    let board = create_board();
    let blue = create_blue_piece(true);
    
    assert!(   passing_piece(&board.fields, &blue, 0, 1));
    assert!(   passing_piece(&board.fields, &blue, 1, 1));
    assert!(   passing_piece(&board.fields, &blue, 2, 0));
    assert!( ! passing_piece(&board.fields, &blue, 1, 2));
    assert!( ! passing_piece(&board.fields, &blue, 2, 1));
}

#[test]
fn test_canfit_red() {        
    let board = create_board();
    let piece = create_red(); 
    
    assert!(   passing_piece(&board.fields, &piece, 2, 0));
    assert!( ! passing_piece(&board.fields, &piece, 2, 1));
    assert!( ! passing_piece(&board.fields, &piece, 3, 0));
}

#[test]
fn test_canfit_green_north() {
    let board = create_board();
    let green_north = create_green(Orientation::North);

    /* TRUE */
    assert!(passing_piece(&board.fields, &green_north, 0, 0));
    assert!(passing_piece(&board.fields, &green_north, 0, 1));
    assert!(passing_piece(&board.fields, &green_north, 2, 0));
    
    /* FALSE */
    assert!( ! passing_piece(&board.fields, &green_north, 0, 2));
    assert!( ! passing_piece(&board.fields, &green_north, 2, 2));
}       