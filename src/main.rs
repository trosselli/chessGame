// this function should prompt the user who is up to take a turn. If this function will call validate move, if its an incorrect move than the user will be prompted
// to choose a different move. 
use std::io;

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




#[derive(Clone)]
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


fn valid_move(board:&[[Piece; 8]; 8], input: String, turn : bool ) -> bool {
    //here we need to take the position that the user has given us ie. a1 and determine what the end position is. 
    //lets create a function that takes the user input in the form of chess lingo (a1) to (b1) and return a pair that is the coordinates of the starting and ending positions


    
    //call is_inbounds for both move positiona and starting position. The function should return 
    let input_parsed = parse_input(input);
            if input_parsed.0 == "bad" || input_parsed.1 == "input"{
                println!("Please enter a valid input");
                return false;

            }
    let proposed_move = is_inbounds(input_parsed.0);
    let initial_position = is_inbounds(input_parsed.1);

    println!("{}{}",proposed_move.0,proposed_move.1);
    if proposed_move == (-1,-1) || initial_position == (-1,-1){
        println!("Your input is not within the board range");
        return false;
    }
    if !validate_turn(board,turn,initial_position){
        return false;
    }

    return true;

}

// validate_move_for_piece(&board:[[Piece; 8]; 8], row: i32, col:i32, piece: Piece ) -> bool {

// }


// this function returns either a input is not correct message or the string that has the cooredinates of the input
// this function should only take in one argument from the users input: if the user inputs b1, c3 this function should only validate and parse the first argument (b1)

fn is_inbounds(position: String) -> (i32, i32) {
    let mut chars = position.chars();
    let mut letter;
    let mut number;
    if let Some(letter_position) = chars.next(){
        letter = letter_position;
    }
    else{
        println!("inside of is_inbounds");
        return (-1,-1);
    }
    if let Some(number_position) = chars.next(){
        number = number_position;
    }
    else{
        println!("inside of is_inbounds");
        return (-1,-1);
    }
    let x_value : i32 = if letter.is_ascii_alphabetic(){
        (letter.to_ascii_lowercase() as i32) - 97
    }else{
        return (-1,-1)
    };

    let numeric_value: i32 = (number as i32) - ('0' as i32) -1;
    let y_coor = numeric_value;

    println!("{}{}",x_value,y_coor);
    if x_value < 0 || x_value >=8{
        println!("inside of checking x coor range");
        return (-1,-1)
    }
    if y_coor < 0 || y_coor >=8{
        println!("inside of checking y coor range");
        return (-1,-1)
    }
    return (x_value, y_coor)


}

fn validate_turn(board:&[[Piece;8];8],turn: bool, starting_position : (i32,i32)) -> bool{
    let row = starting_position.0 as usize;
    let col = starting_position.1 as usize;

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
    let mut king_position : (usize, usize);
    let color: char = if turn{
        'w'
    }else {'b'};
    for i in 0..8{
        for j in 0..8 {
            if board[i][j].color == color && board[i][j].piece == 'k'{
                king_position = (i,j);
            }
        }
    }
    // now we need to check if the king is in check. We will need to iterate over all of the possible 
    // ways that the king could be in check, first checking all of the 
    
    return true;
}

fn pawn_movement(board:&[[Piece;8];8],starting_position : (i32,i32), ending_position : (i32,i32)) -> bool {
    return true 

}

fn rook_movement(board:&[[Piece;8];8],starting_position : (i32,i32), ending_position : (i32,i32)) -> bool{
    return true
}

fn bishop_movement(board:&[[Piece;8];8],starting_position : (i32,i32), ending_position : (i32,i32)) -> bool{
    return true
}
fn king_movement(board:&[[Piece;8];8],starting_position : (i32,i32), ending_position : (i32,i32)) -> bool{
    return true
}
fn queen_movement(board:&[[Piece;8];8],starting_position : (i32,i32), ending_position : (i32,i32)) -> bool{
    return true
}
fn knight_movement(board:&[[Piece;8];8],starting_position : (i32,i32), ending_position : (i32,i32)) -> bool{
    return true
}




/*

the following are just test cases, we will have a test case for each function that is being build for this project
 */

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_inbounds_true() {
        assert_eq!(is_inbounds("a1".to_string()), (0,0));
        
    }
    #[test]
    fn test_is_inbounds_false_letter(){
        assert_eq!(is_inbounds("i2".to_string()),(-1,-1));

    }
    #[test]
    fn test_is_inbounds_false_number_under(){
        assert_eq!(is_inbounds("i0".to_string()),(-1,-1));
    }
    #[test]
    fn test_is_inbounds_false_number_over(){
        assert_eq!(is_inbounds("i9".to_string()),(-1,-1));
    }
    #[test]
    fn test_read_and_parse_input_three_arguments(){
        assert_eq!(parse_input("a1,b2,c3".to_string()), ("bad".to_string(), "input".to_string()));
    }
    #[test]
    fn test_parse_input_one_argument(){
        assert_eq!(parse_input("a1".to_string()),("bad".to_string(), "input".to_string()));
    }
}
