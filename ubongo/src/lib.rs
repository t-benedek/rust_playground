
pub struct Board {
    pub fields : [[bool; 5]; 5],
    pub count : u8
}

#[cfg(test)]
mod tests;

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
