use core::panic;

use super::*;


#[test]
fn search_for_real_solution() {        
    let board = create_board();
    let blue_vert = create_blue(false);
    let green_west = create_green(Orientation::West);
    let red = create_red();

    for i in 0..board.fields.len() {
        for j in 0..board.fields[i].len() {
            
            if passing_piece(&board.fields, &red, i, j) {
                let mut copy_field = board.fields;
                set_piece(&mut copy_field, &red, i, j);
                println!("After setting red at ({},{})", i, j);
                visualize_board(&copy_field);
                for i in 0..copy_field.len() {
                    for j in 0..copy_field[i].len() {    
                        
                        if passing_piece(&mut copy_field, &green_west, i, j) {
                            let mut copy_field1 = copy_field;
                            set_piece(&mut copy_field1, &green_west, i, j);
                            println!("After setting green at ({},{})", i, j);
                            visualize_board(&copy_field1);                            
                            for i in 0..copy_field1.len() {
                                for j in 0..copy_field1[i].len() {    
                                    
                                    if passing_piece(&mut copy_field1, &blue_vert, i, j) {
                                        let mut copy_field2 = copy_field1;
                                        set_piece(&mut copy_field2, &blue_vert, i, j);
                                        println!("After setting blue at ({},{})", i, j);
                                        visualize_board(&copy_field2);      
                                        let result = check_board_complete(&copy_field2);
                                        match result {
                                            Some((row, col)) => {
                                                panic!("Reihe {}, Spalte {}", row, col);
                                            }
                                            None => {
                                                println!("Found solution!");
                                                return; 
                                            }
                                        }  
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("Test did not succeed"); 
}

#[test]
fn real_solution() {        
    let mut board = create_board();
    let blue_vert = create_blue(false);
    let green_west = create_green(Orientation::West);
    let red = create_red();

    /* Board before */
    // 1 1 1 1 0
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
    
    if passing_piece(&board.fields, &blue_vert, 0, 0) {
        set_piece(&mut board.fields, &blue_vert, 0, 0);
    }
    assert_eq!(board.fields[0][0], false);
    assert_eq!(board.fields[0][1], false);
    assert_eq!(board.fields[0][2], false);

    if passing_piece(&board.fields, &green_west, 1, 0) {
        set_piece(&mut board.fields, &green_west, 1, 0);
    }
    
    assert_eq!(board.fields[1][0], false);
    assert_eq!(board.fields[1][1], false);
    assert_eq!(board.fields[1][2], false);
    assert_eq!(board.fields[2][2], false);

    if passing_piece(&board.fields, &red, 2, 0) {
        set_piece(&mut board.fields, &red, 2, 0);
    }

    let result = check_board_complete(&board.fields);

    match result {
        Some((row, col)) => {
            panic!("Reihe {}, Spalte {}", row, col);
        }
        None => {
            // Board is complete
        }
    }

    /* Board after */
    // 0 0 0 0 0
    // 0 0 0 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
}


#[test]
fn test_set_blue_hori_and_violet_hori() {        
    let mut board = create_board();
    let blue_hori = create_blue(true);
    let violet_hori = create_violet(true);

    /* Board before */
    // 1 1 1 1 0
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
    
    assert_eq!(board.fields[1][0], true);
    assert_eq!(board.fields[2][0], true);
    assert_eq!(board.fields[3][0], true);
    assert_eq!(board.fields[0][1], true);
    assert_eq!(board.fields[1][1], true);
    if passing_piece(&board.fields, &blue_hori, 1, 0) {
        set_piece(&mut board.fields, &blue_hori, 1, 0);
    }
    if passing_piece(&board.fields, &violet_hori, 0, 1) {
        set_piece(&mut board.fields, &violet_hori, 0, 1);
    }
    assert_eq!(board.fields[1][0], false);
    assert_eq!(board.fields[2][0], false);
    assert_eq!(board.fields[3][0], false);
    assert_eq!(board.fields[0][1], false);
    assert_eq!(board.fields[1][1], false);

    /* Board after */
    // 1 0 0 0 0
    // 0 0 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
}


#[test]
fn test_set_blue_hori() {        
    let mut board = create_board();
    let blue_hori = create_blue(true);

    /* Board before */
    // 1 1 1 1 0
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
    
    assert_eq!(board.fields[1][0], true);
    assert_eq!(board.fields[2][0], true);
    assert_eq!(board.fields[3][0], true);
    if passing_piece(&board.fields, &blue_hori, 1, 0) {
        set_piece(&mut board.fields, &blue_hori, 1, 0);
    }
    assert_eq!(board.fields[1][0], false);
    assert_eq!(board.fields[2][0], false);
    assert_eq!(board.fields[3][0], false);

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
    let blue_vert = create_blue(false);

    /* Board before */
    // 1 1 1 1 1
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
    
    assert_eq!(board.fields[1][0], true);
    assert_eq!(board.fields[1][1], true);
    assert_eq!(board.fields[1][2], true);
    if passing_piece(&board.fields, &blue_vert, 1, 0) {
        set_piece(&mut board.fields, &blue_vert, 1, 0);
    }
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
    if passing_piece(&board.fields, &violet_vert, 1, 0) {
        set_piece(&mut board.fields, &violet_vert, 1, 0);
    }

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
fn test_set_violet_vert_no_effect() {        
    let mut board = create_board();
    let violet_vert = create_violet(false);

    /* Board before */
    // 1 1 1 1 0
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0   
    
    assert_eq!(board.fields[4][0], false);
    assert_eq!(board.fields[4][1], false);
    if passing_piece(&board.fields, &violet_vert, 4, 0) {
        set_piece(&mut board.fields, &violet_vert, 4, 0);
    }
    
    assert_eq!(board.fields[4][0], false);
    assert_eq!(board.fields[4][1], false);

    /* Board after */
    // 1 1 1 1 0
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0  
}


#[test]
fn canfit_blue_hori() {        
    let board = create_board();
    let blue = create_blue(true);
    
    assert!(   passing_piece(&board.fields, &blue, 0, 1));
    assert!(   passing_piece(&board.fields, &blue, 1, 1));
    assert!(   passing_piece(&board.fields, &blue, 1, 0));
    assert!( ! passing_piece(&board.fields, &blue, 1, 2));
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

    // Board looks like this
    // 1 1 1 1 0
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0 

    /* TRUE */
    assert!(passing_piece(&board.fields, &green_north, 0, 0));
    assert!(passing_piece(&board.fields, &green_north, 0, 1));
    assert!(passing_piece(&board.fields, &green_north, 1, 0));
    
    /* FALSE */
    assert!( ! passing_piece(&board.fields, &green_north, 0, 2));
    assert!( ! passing_piece(&board.fields, &green_north, 2, 2));
}       

#[test]
fn canfit_green_west() {
    let board = create_board();
    let green_west = create_green(Orientation::West);

    // Board looks like this
    // 1 1 1 1 0
    // 1 1 1 1 0 
    // 1 1 1 0 0 
    // 0 0 0 0 0 
    // 0 0 0 0 0 

    // Green Piece looks like this
    // 1 0 0 0  
    // 1 0 0 0  
    // 1 1 0 0   

    /* TRUE */
    assert!(passing_piece(&board.fields, &green_west, 0, 0));
    assert!(passing_piece(&board.fields, &green_west, 1, 0));
    
    /* FALSE */
    assert!( ! passing_piece(&board.fields, &green_west, 2, 0));
    assert!( ! passing_piece(&board.fields, &green_west, 0, 2));
}       