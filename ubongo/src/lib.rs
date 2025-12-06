pub struct Board {
    // fields[x][y]
    // pub fields : Vec<Vec<bool>>,
    pub fields : [[bool; 5]; 5],
    pub count : u8
}


pub fn fit(_board: Board, _piece: Piece) -> bool {
    todo!("implement fit function");
//     // copy fields to change it from now on
//     let mut fields = board.fields;
//     let x = 0;
//     let y = 0;
//     visualize(&fields);

//     if passing_piece(&fields, &piece, x, y) {
//         fields = set_piece(fields, &piece, x, y);
//     }
    
//     visualize(&fields);

//     true
}

pub fn visualize_board(fields: &[[bool;5];5]) {
    println!();
    for y in 0..5 {
        for x in 0..5 {
            print!("{} ", fields[x][y] as u8);
        }
        println!();
    }        
} 


pub fn visualize_piece(fields: &[[bool;4];4]) {
    println!();
    for x in 0..4 {
        for y in 0..4 {
            print!("{} ", fields[x][y] as u8);
        }
        println!();
    }        
} 

pub fn set_piece(mut _fields: Vec<Vec<bool>>, _piece: &Piece, _x: usize, _y: usize) -> Vec<Vec<bool>> {
    todo!("implement set_piece function");
//     for i in 0..piece.fields.len() {
//         for j in 0..piece.fields[i].len() {
//             if piece.fields[i][j] {
//                 if  fields[i+y][j+x] {
//                     // overwrite the remaining board field elements with the piece
//                     fields[i+y][j+x] = !piece.fields[i][j];    
//                 } 
//             } 
//         }
//     }        
//     fields
}

pub fn passing_piece(fields : &[[bool; 5]; 5], piece: &Piece, x_off: usize, y_off: usize) -> bool {

    // Return false if piece if larger than the board
    if x_off >= fields.len() || y_off >= fields[0].len() {
        return false;
    }

    let mut result = true;

    for i in 0..piece.fields.len() {
        for j in 0..piece.fields[i].len() {
            if piece.fields[i][j] {
                let x = i + x_off;
                let y = j + y_off;
                // piece leaves fields to the right or to the bottom
                if x >= fields.len() || y >= fields[0].len() {
                    return false;
                }
                    
                // no free field for piece
                if  ! fields[x][y] {
                    println!("Piece does not fit field at ({x})({y})");
                    visualize_board(fields);
                    visualize_piece(&piece.fields);

                    result = false;
                } 
            } 
        }
    }  
    result
}

#[cfg(test)]
mod tests {
     use super::*;
    
    #[test]
    fn test_canfit_blue() {        
        let mut board = Board {
            fields : [[false; 5]; 5],
            count : 0
        };

        let mut blue_piece = Piece {
            fields : [[false; 4]; 4],
            count : 0
        };

        init_board(&mut board);
        init_blue_piece(&mut blue_piece);  

        /* Board */
        // 1 1 1 1 1
        // 1 1 1 1 0 
        // 1 1 1 0 0 
        // 0 0 0 0 0 
        // 0 0 0 0 0    

        // TRUE
        assert!(passing_piece(&board.fields, &blue_piece, 0, 2));
        assert!(passing_piece(&board.fields, &blue_piece, 1, 1));
        assert!(passing_piece(&board.fields, &blue_piece, 2, 0));

        // FALSE
        assert!( ! passing_piece(&board.fields, &blue_piece, 1, 2));
        assert!( ! passing_piece(&board.fields, &blue_piece, 2, 1));
    }

        #[test]
    fn test_canfit_red() {        
        let mut board = Board {
            fields : [[false; 5]; 5],
            count : 0
        };

        let mut red_piece = Piece {
            fields : [[false; 4]; 4],
            count : 0
        };

        init_board(&mut board);
        init_red_piece(&mut red_piece);  

        /* Board */
        // 1 1 1 1 1
        // 1 1 1 1 0 
        // 1 1 1 0 0 
        // 0 0 0 0 0 
        // 0 0 0 0 0    

        // TRUE
        assert!(passing_piece(&board.fields, &red_piece, 2, 0));

        // FALSE
        assert!( ! passing_piece(&board.fields, &red_piece, 2, 1));
        assert!( ! passing_piece(&board.fields, &red_piece, 3, 0));
    }

    #[test]
    fn test_canfit_green() {

        let mut board = Board {
            fields : [[false; 5]; 5],
            count : 0
        };

        let mut green1_piece = Piece {
            fields : [[false; 4]; 4],
            count : 0
        };

        let mut green2_piece = Piece {
            fields : [[false; 4]; 4],
            count : 0
        };

        init_board(&mut board); 
        init_green1_piece(&mut green1_piece);  
        init_green2_piece(&mut green2_piece);  

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

    fn init_board(board: &mut Board) {
        // 1 1 1 1 1
        // 1 1 1 1 0 
        // 1 1 1 0 0 
        // 0 0 0 0 0 
        // 0 0 0 0 0 
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
    }
    fn init_red_piece(piece: &mut Piece) {
        // 1 1 0 0  
        // 1 1 0 0  
        // 0 0 0 0  
        piece.fields[0][0] = true;
        piece.fields[0][1] = true;
        piece.fields[1][0] = true;
        piece.fields[1][1] = true;
        piece.count = 4;
    }
    
    fn init_blue_piece(piece: &mut Piece) {
        // 1 1 1 0  
        // 0 0 0 0  
        // 0 0 0 0  
        piece.fields[0][0] = true;
        piece.fields[1][0] = true;
        piece.fields[2][0] = true;
        piece.count = 3;
    }

    fn init_green1_piece(piece: &mut Piece) {
        // 1 1 0 0  
        // 0 0 0 0  
        // 0 0 0 0  
        piece.fields[0][0] = true;
        piece.fields[1][0] = true;
        piece.count = 2;
    }
    
    // rotation of green1
    fn init_green2_piece(piece: &mut Piece) {
        // 0 1 0 0  
        // 0 1 0 0  
        // 0 0 0 0  
        piece.fields[1][0] = true;
        piece.fields[1][1] = true;
        piece.count = 2;
    }

}

pub struct Piece {
    pub fields : [[bool;4];4],
    pub count : u8
}

impl Piece {
    pub fn visualize(&self) {
        println!();
        for i in 0..self.fields.len() {
            for j in 0..self.fields[i].len() {
                print!("{} ", self.fields[i][j] as u8);
            }
        println!();
        }        
    } 
}
