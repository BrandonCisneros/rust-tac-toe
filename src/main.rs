use std::vec;
use std::io;

struct Player {
    name: String,
    marker: String,
}

/*pub struct PlayerState {
    player1: Vec<Vec<u8>>,
    player2: Vec<Vec<u8>>,
}*/



fn marker_validate(marker: &String) -> bool {
    let is_valid: bool = true;

    if marker != "X" || marker != "x" || marker != "O" || marker != "o"{
        return !is_valid;
    }

    return is_valid;
}



fn main() {

    let mut player_1 = Player {
       name: String::from(""),
       marker: String::from(""),
    };

    let mut player_2 = Player {
       marker: "".to_string(),
       name: "".to_string(),
    };

    let grid: Vec<Vec<&str>> = vec![
        vec!["*", "|", "*", "|", "*"],
        vec!["-", "|", "-", "|", "-"],
        vec!["*", "|", "*", "|", "*"],
        vec!["-", "|", "-", "|", "-"],
        vec!["*", "|", "*", "|", "*"],
    ];
    
    let mut winner: bool = false;
        
    println!("Welcome to RUST-TAC-TOE!!!\n");

    //--- Player 1 Information
    print!("Please enter the name for Player 1: ");
    let mut player_name1 = String::new();
    io::stdin().read_line(&mut player_name1).unwrap();
    player_1.name = player_name1;
    println!("\nPlayer 1: {}\n", player_1.name);
    
    //--- Player 2 information
    print!("Please enter the name for Player 2: ");
    let mut player_name2 = String::new();
    io::stdin().read_line(&mut player_name2).unwrap();
    player_2.name = player_name2;
    println!("\nPlayer 2: {}\n", player_2.name);

    println!("Hello {} & {}! Ready to play?\n", player_1.name, player_2.name);
    println!("\nNow we have to decide who wants X's and who wants O's...\n");

    println!("\n{:?}, please type in X or O: ", player_1.name);
    player_1.marker = String::new();

    let is_valid: bool = marker_validate(&player_1.marker);

    if is_valid {
       println!("\n{:?} chose {:?} !\n", player_1.name, player_1.marker);
    }


    while !winner {
        let mut x_coord: u8 = 0;
        let mut y_coord: u8 = 0;

        x_coord = 1;
        y_coord = 1;

        print!("X: {} | Y: {}\n", x_coord,y_coord);

        for i in &grid{
            println!("{:?}", i);
        }

        winner = true;
        
    }

    println!("The loop ended!");

}
