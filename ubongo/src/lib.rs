
pub struct Board {
    pub fields : [[bool; 5]; 5],
    pub count : u8
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

pub enum Orientation {
    North,
    East,
    South,
    West    
} 

#[cfg(test)]
mod tests;

/**
* Set a given piece on the fields with the offset (x_off,y_off) 
* After this function the amount of free items in the field is reduced by number of items of the piece
* If piece does not fit on the field, this function returns without any effect
**/
pub fn set_piece(fields: &mut [[bool; 5]; 5], piece: &Piece, x_off: usize, y_off: usize) {
    if offset_outside_fields(fields, x_off, y_off) {
        return;
    }
    
    // iterate over all items of the piece and set field item to "0" if piece item is "1"
    for i in 0..piece.fields.len() {
        for j in 0..piece.fields[i].len() {
            if piece.fields[i][j] {
                if  fields[i+x_off][j+y_off] {
                    // overwrite the remaining board field elements with the piece
                    fields[i+x_off][j+y_off] = false;    
                } 
            } 
        }
    }        
}

fn offset_outside_fields(fields : &[[bool; 5]; 5], x_off: usize, y_off: usize) -> bool {
    if x_off >= fields.len() || y_off >= fields[0].len() {
        return true;
    } 
    false
}
/**
* Check if a piece fits into the remaining open fields of a given board
* Offset is needed to be able to move pieces around the fields
* 
**/
pub fn passing_piece(fields : &[[bool; 5]; 5], piece: &Piece, x_off: usize, y_off: usize) -> bool {

    // Return false if piece if larger than the board
    if offset_outside_fields(fields, x_off, y_off) {
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
                    // println!("Piece does not fit field at ({x})({y})");
                    // visualize_board(fields);
                    // visualize_piece(&piece.fields);

                    result = false;
                } 
            } 
        }
    }  
    result
}

// Board looks like this
// 1 1 1 1 0
// 1 1 1 1 0 
// 1 1 1 0 0 
// 0 0 0 0 0 
// 0 0 0 0 0 
pub fn create_board() -> Board {
    let mut board = Board {
        fields: [[false; 5]; 5],
        count: 0,
    };
    board.fields[0][0] = true;
    board.fields[1][0] = true;
    board.fields[2][0] = true;
    board.fields[3][0] = true;
    
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
pub fn create_red() -> Piece {
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
pub fn create_blue(horizontal: bool) -> Piece {
    let mut piece = Piece {
        fields : [[false; 4]; 4],
        count : 0
    };
    
    piece.count = 3;

    if horizontal {
        piece.fields[0][0] = true;
        piece.fields[1][0] = true;
        piece.fields[2][0] = true;
    } else {
        piece.fields[0][0] = true;
        piece.fields[0][1] = true;
        piece.fields[0][2] = true;
     }
    return piece;
}


pub fn create_green(orientation: Orientation) -> Piece {
    let mut piece = Piece {
        fields : [[false; 4]; 4],
        count : 0
    };

    // Orientation always related to the longest part of the piece
    match orientation {
        Orientation::North => {
            // 1 1 1 0  
            // 1 0 0 0  
            // 0 0 0 0  
            piece.fields[0][0] = true;
            piece.fields[1][0] = true;
            piece.fields[2][0] = true;
            piece.fields[0][1] = true;
        },
        Orientation::East =>  {              
            // 1 1 0 0  
            // 0 1 0 0  
            // 0 1 0 0  
            piece.fields[0][0] = true;
            piece.fields[1][0] = true;
            piece.fields[1][1] = true;
            piece.fields[1][2] = true;
        },
        Orientation::South => {
            // 0 0 1 0  
            // 1 1 1 0  
            // 0 0 0 0 
            piece.fields[2][0] = true;
            piece.fields[0][1] = true;
            piece.fields[1][1] = true;
            piece.fields[2][1] = true;
        },
        Orientation::West => {
            // 1 0 0 0  
            // 1 0 0 0  
            // 1 1 0 0         
            piece.fields[0][0] = true;
            piece.fields[0][1] = true;
            piece.fields[0][2] = true;
            piece.fields[1][2] = true;
        },
    }
    piece.count = 4;
    return piece;
}

// 1 1 0 0  
// 0 0 0 0  
// 0 0 0 0  
pub fn create_violet(horizontal: bool) -> Piece {
    let mut piece = Piece {
        fields : [[false; 4]; 4],
        count : 0
    };

    if horizontal == true {
        piece.fields[0][0] = true;
        piece.fields[1][0] = true;
    } else {
        piece.fields[0][0] = true;
        piece.fields[0][1] = true;
    }

    piece.count = 2;
    return piece;
}

pub fn check_board_complete(fields: &[[bool; 5]; 5]) -> Option<(usize, usize)> {
    for i in 0..fields.len() {
        for j in 0..fields[i].len() {
            if fields[i][j] {
                return Some((i,j));
            }
        }
    }  
    None
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

