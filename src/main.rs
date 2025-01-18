// this function should prompt the user who is up to take a turn. If this function will call validate move, if its an incorrect move than the user will be prompted
// to choose a different move. 
use std::{io, usize};
enum Direction {
    DiagonalTopRight,
    DiagonalTopLeft,
    DiagonalBottomLeft,
    DiagonalBottomRight,
    Forward,
    Backward,
    Left,
    Right
}

fn print_board(board:&[[Piece; 8]; 8]){
    for i in 0..8{
        let mut s = String::new();
        for j in 0..7{
            s.push(board[i][j].color);
            s.push(board[i][j].piece);
            s.push_str(" | ");

        }
        s.push(board[i][7].color);
        s.push(board[i][7].piece);
        s.push('\n');

        println!("{}",s);
        let break_line = " ";
        println!("{}",break_line);
    }
}




fn init_board(board: &mut [[Piece; 8]; 8]){
    let white_rook = Piece {
        color: 'w',
        piece: 'r'
    };
    let white_bishop = Piece {
        color: 'w',
        piece: 'b'
    };
    let white_queen = Piece{
        color: 'w',
        piece: 'Q'
    };
    let white_king = Piece{
        color: 'w',
        piece: 'K'
    };
    let white_pawn = Piece{
        color: 'w',
        piece: 'p'
    };
    let white_horse = Piece{

        color: 'w',
        piece: 'h'
    };


    let black_rook = Piece {
        color: 'b',
        piece: 'r'
    };
    let black_bishop = Piece {
        color: 'b',
        piece: 'b'
    };
    let black_queen = Piece{
        color: 'b',
        piece: 'Q'
    };
    let black_king = Piece{
        color: 'b',
        piece: 'K'
    };
    let black_pawn = Piece{
        color: 'b',
        piece: 'p'
    };
    let black_horse = Piece{

        color: 'b',
        piece: 'h'
    };
    board[0][0] = black_rook.clone();
    board[0][1] = black_horse.clone();
    board[0][2] = black_bishop.clone();
    board[0][3] = black_queen.clone();
    board[0][4] = black_king.clone();
    board[0][5] = black_bishop.clone();
    board[0][6] = black_horse.clone();
    board[0][7] = black_rook.clone();

    for i in 0..8 {
        board[1][i] = black_pawn.clone();
        board[6][i] = white_pawn.clone();
    }

    board[7][0] = white_rook.clone();
    board[7][1] = white_horse.clone();
    board[7][2] = white_bishop.clone();
    board[7][3] = white_queen.clone();
    board[7][4] = white_king.clone();
    board[7][5] = white_bishop.clone();
    board[7][6] = white_horse.clone();
    board[7][7] = white_rook.clone();    

}




#[derive(Copy, Clone)]
struct Piece {
    color: char,
    piece: char,
}
impl Default for Piece{
    fn default() -> Self {
        Piece {
            color: 'n',
            piece: 'n',
        }
    }
}

fn parse_input(input : String) -> (String, String){
    // let mut input = String::new();
    // io::stdin()
    // .read_line(&mut input) // Read input from the console
    // .expect("Failed to read line");

    println!("{}",input.trim());
    let parsed_inputs: Vec<&str> = input.split_terminator(',').collect();
    if parsed_inputs.len() > 2 || parsed_inputs.len() == 0{
        return ("bad".to_string(),"input".to_string())
    }
    let mut beg: String; 
    let mut end : String;
    match (parsed_inputs.get(0), parsed_inputs.get(1)) {
        (Some(first), Some(second)) => {
            beg = first.to_string();
            end = second.to_string();
            return (beg, end);
        },
        (Some(first), None) => {
            return ("bad".to_string(),"input".to_string());
        }
        _ => {
            return ("bad".to_string(),"input".to_string());
        }
    }
}

fn main() {

    let mut board: [[Piece; 8]; 8] = Default::default();

    init_board(&mut board);
    let mut turn = true;
    let mut gameOver = false;
    while !gameOver{
        println!("{}, it is your turn, please enter your move", turn);
        print_board(&board);
        let mut valid_move_bool = false; 
        
        while !valid_move_bool{
            let mut input = String::new();
            io::stdin()
            .read_line(&mut input) // Read input from the console
            .expect("Failed to read line");        
            valid_move_bool = valid_move(&board,input,turn);
            
        }
    }

    





    // the board will be the current state of the game, where all the pieces 

    /* 
    some sort of while loop that iterates through the game
    */ 

    //allows a user to take a turn on the board. This 
    /*
    take_turn()
    checkState()
    */ 
    println!("Hello world");
    
}

/* 
Black King (0,4) Black Queen (0,3)
i = first number 
j = second number 

Black is on i = 0 
moving right on the board is j+1
moving left on the board is j-1

*/
fn valid_move(board:&[[Piece; 8]; 8], input: String, turn : bool ) -> bool {
    //here we need to take the position that the user has given us ie. a1 and determine what the end position is. 
    //lets create a function that takes the user input in the form of chess lingo (a1) to (b1) and return a pair that is the coordinates of the starting and ending positions


    
    //call is_inbounds for both move positiona and starting position. The function should return 
    let input_parsed = parse_input(input);
            if input_parsed.0 == "bad" || input_parsed.1 == "input"{
                println!("Please enter a valid input");
                return false;

            }
    
    let proposed_move = find_coordinates(input_parsed.0);
    let initial_position = find_coordinates(input_parsed.1);
    
    println!("{}{}",proposed_move.0,proposed_move.1);
    if !is_inbounds(proposed_move)  || !is_inbounds(initial_position){
        println!("Your input is not within the board range");
        return false;
    }
    let proposed_move_usize = as_usize(proposed_move);
    let initial_position_usize = as_usize(initial_position);
    if !validate_turn(board,turn,initial_position_usize){
        return false;
    }



    return true;

}

// validate_move_for_piece(&board:[[Piece; 8]; 8], row: i32, col:i32, piece: Piece ) -> bool {

// }


// this function returns either a input is not correct message or the string that has the cooredinates of the input
// this function should only take in one argument from the users input: if the user inputs b1, c3 this function should only validate and parse the first argument (b1)

fn as_usize (coordinates: (i32,i32)) -> (usize,usize){
    if coordinates.0 < 0 || coordinates.1 < 0{
        return (100,100);
    }
    let x : usize = coordinates.0 as usize;
    let y : usize = coordinates.1 as usize;

    return (x,y);
}


fn find_coordinates(position: String) -> (i32, i32) {
    let mut chars = position.chars();
    let mut letter;
    let mut number;
    if let Some(letter_position) = chars.next(){
        letter = letter_position;
    }
    else{
        return (100,100);
    }
    if let Some(number_position) = chars.next(){
        number = number_position;
    }
    else{
        return (100,100);
    }
    let x_value : i32 = if letter.is_ascii_alphabetic(){
        (letter.to_ascii_lowercase() as i32) - 97
    }else{
        return (100,100)
    };
    let numeric_value: i32 = (number as i32) - ('0' as i32) -1;
    return (x_value,numeric_value)


}
fn is_inbounds(coordinates : (i32,i32))-> bool{
    if coordinates.0 < 0 || coordinates.0  >=8 || coordinates.1 <0 || coordinates.1 >=8{
        return false
    }
    return true

}

fn validate_turn(board:&[[Piece;8];8],turn: bool, starting_position : (usize,usize)) -> bool{
    let row = starting_position.0;
    let col = starting_position.1;

    if (board[row][col].color == 'w' && !turn) || (board[row][col].color == 'b' && turn){
        return false;
    }
    // if in_check(board:&[[Piece;8];8],turn: bool){
    //     return false; 
    // }
    return true;
}

fn validate_piece_move (board:&[[Piece;8];8],starting_position : (i32,i32), ending_position : (i32,i32))-> bool{
    return true;
}


fn in_check(board:&[[Piece;8];8],turn: bool) -> bool{
    // first we need to find the position of the king 
    let mut king_position : (i32, i32) = (100,100);
    let color: char = if turn{
        'w'
    }else {'b'};
    for i in 0..8{
        for j in 0..8 {
            if board[i as usize][j as usize].color == color && board[i as usize][j as usize].piece == 'k'{
                king_position = (i,j);
            }
        }
    }

    return in_check_helper(&board,turn,(-1,-1),king_position,(king_position.0-1,king_position.1-1)) ||
    in_check_helper(&board,turn,(0,-1),king_position,(king_position.0,king_position.1-1)) ||
    in_check_helper(&board,turn,(-1,0),king_position,(king_position.0-1,king_position.1)) ||
    in_check_helper(&board,turn,(1,1),king_position,(king_position.0+1,king_position.1+1)) ||
    in_check_helper(&board,turn,(1,0),king_position,(king_position.0+1,king_position.1)) ||
    in_check_helper(&board,turn,(0,1),king_position,(king_position.0,king_position.1+1)) ||
    in_check_helper(&board,turn,(-1,1),king_position,(king_position.0-1,king_position.1+1)) ||
    in_check_helper(&board,turn,(1,-1),king_position,(king_position.0+1,king_position.1-1));







    // now we need to check if the king is in check. We will need to iterate over all of the possible 
    // ways that the king could be in check, first checking all of the 

}
fn in_check_helper(board:&[[Piece;8];8],turn:bool,direction: (i32,i32), king_position: (i32,i32),current_position:(i32,i32))-> bool{
    if(!is_inbounds(current_position)){
        return false;
    }
    let position = as_usize(current_position);
    //check if 
    if board[position.0][position.1].color == 'w' && turn || board[position.0][position.1].color == 'b' && !turn {
        return false;
    }
    if board[position.0][position.1].color == 'w' && !turn || board[position.0][position.1].color == 'b' && turn {
       return can_attack(board, as_usize(king_position),position, turn);
    }

    return in_check_helper(board,turn,direction,king_position, (current_position.0+direction.0,current_position.1+direction.1));

}

fn can_attack(board:&[[Piece;8];8],attack_position: (usize,usize), victim: (usize,usize),is_clear: bool) -> bool {
    
    if board[attack_position.0][attack_position.1].piece == 'p' {
        return pawn_movement(board, attack_position, victim,is_clear);
    }
    else{
        return false;
    }
}
fn pawn_movement(board:&[[Piece;8];8],starting_position : (usize,usize), ending_position : (usize, usize),is_clear:bool) -> bool {
    let moving_piece = board[starting_position.0][starting_position.1].clone();
    let ending_piece: Piece = board[ending_position.0][ending_position.1].clone();
    let mut move_direction;

    if moving_piece.color =='w'{
        move_direction = -1;
    }
    else{
        move_direction = 1;
    }

    /* 
Black King (0,4) Black Queen (0,3)
i = first number 
j = second number 

Black is on i = 0 
moving right on the board is j+1
moving left on the board is j-1

*/

// check the case where we are moving diagonally 
    let moving_correct_i_direction : bool = ending_position.0 as i32 == starting_position.0 as i32 + move_direction;
    let opposite_colors : bool = moving_piece.color == 'w' && ending_piece.color == 'b' || (moving_piece.color =='b'  && ending_piece.color == 'w');
    let attacking : bool = (starting_position.1 as i32 == ending_position.1 as i32 -1 || starting_position.1 as i32 == ending_position.1 as i32 + 1 );
    // checking if we are attacking and they are opposite colors 
    if moving_correct_i_direction && (attacking) && opposite_colors{
        return true;
    }
    //checking if we arent attacking and the piece in front of us is unoccupied
    if moving_correct_i_direction && starting_position.1 == ending_position.1 && ending_piece.color == 'n' {
        return true;
    }

    //all other cases are false 
    return false; 


}

fn bishop_movement(board:&[[Piece;8];8],starting_position : (usize,usize), ending_position : (usize, usize),is_clear:bool) -> bool {
    let moving_piece = board[starting_position.0][starting_position.1].clone();
    let ending_piece: Piece = board[ending_position.0][ending_position.1].clone();

    return true 

}
fn knight_movement(board:&[[Piece;8];8],starting_position : (usize,usize), ending_position : (usize, usize),is_clear:bool) -> bool {
    return true 

}
fn queen_movement(board:&[[Piece;8];8],starting_position : (usize,usize), ending_position : (usize, usize),is_clear:bool) -> bool {
    return true 

}
fn king_movement(board:&[[Piece;8];8],starting_position : (usize,usize), ending_position : (usize, usize),is_clear:bool) -> bool {
    return true 

}
fn rook_movement(board:&[[Piece;8];8],starting_position : (usize,usize), ending_position : (usize, usize),is_clear:bool) -> bool {
    return true 
}






/*

the following are just test cases, we will have a test case for each function that is being build for this project
 */
fn create_test_board() ->  [[Piece; 8]; 8] {
    let empty_piece = Piece::default();

    [
        [
            Piece { color: 'b', piece: 'r' }, // Rook
            Piece { color: 'b', piece: 'h' }, // Knight
            Piece { color: 'b', piece: 'b' }, // Bishop
            Piece { color: 'b', piece: 'Q' }, // Queen
            Piece { color: 'b', piece: 'K' }, // King
            Piece { color: 'b', piece: 'b' }, // Bishop
            Piece { color: 'b', piece: 'h' }, // Knight
            Piece { color: 'b', piece: 'r' }, // Rook

        ],
        [
            Piece { color: 'b', piece: 'p' }; 8 // Pawns
        ],
        [empty_piece; 8],                     // Empty row
        [empty_piece; 8],                     // Empty row
        [empty_piece; 8],                     // Empty row
        [empty_piece; 8],                     // Empty row
        [
            Piece { color: 'w', piece: 'p' }; 8 // Pawns
        ],
        [
            Piece { color: 'w', piece: 'r' }, // Rook
            Piece { color: 'w', piece: 'h' }, // Knight
            Piece { color: 'w', piece: 'b' }, // Bishop
            Piece { color: 'w', piece: 'Q' }, // Queen
            Piece { color: 'w', piece: 'K' }, // King
            Piece { color: 'w', piece: 'b' }, // Bishop
            Piece { color: 'w', piece: 'h' }, // Knight
            Piece { color: 'w', piece: 'r' }, // Rook
        ],
    ]
}
 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_inbounds_true() {
        assert_eq!(is_inbounds((0,0)), true);
        
    }
    #[test]
    fn test_is_inbounds_false_letter(){
        assert_eq!(is_inbounds((0,1)),true);

    }
    #[test]
    fn test_is_inbounds_false_number_under(){
        assert_eq!(is_inbounds((9,1)),false);
    }
    #[test]
    fn test_is_inbounds_false_number_over(){
        assert_eq!(is_inbounds((115,0)),false);
    }
    #[test]
    fn test_read_and_parse_input_three_arguments(){
        assert_eq!(parse_input("a1,b2,c3".to_string()), ("bad".to_string(), "input".to_string()));
    }
    #[test]
    fn test_parse_input_one_argument(){
        assert_eq!(parse_input("a1".to_string()),("bad".to_string(), "input".to_string()));
    }
    #[test]
    fn test_pawn_correct(){
        let board = create_test_board();
        assert_eq!(pawn_movement(&board,as_usize((1,1)),as_usize((2,1)),false),true);
    }
    #[test]
    fn test_pawn_false(){
        let board = create_test_board();
        assert_eq!(pawn_movement(&board,as_usize((1,1)),as_usize((2,3)),false),false);
    }
}
