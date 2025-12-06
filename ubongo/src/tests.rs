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
        assert!(passing_piece(&board.fields, &blue_piece, 0, 1));
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