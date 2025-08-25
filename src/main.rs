use std::*;

pub struct Player {
    id: u8,
    name: Option<String>,
    marker: Option<String>,
}

/*pub struct PlayerState {
    player1: Vec<Vec<u8>>,
    player2: Vec<Vec<u8>>,
}*/



fn marker_validate(marker: Option<String>) -> bool {
    let mut isValid: bool = true;

    if marker != Some("X") || "x" || "O" || "o" {
        return !isValid;
    }

    return isValid;
}



fn main() {

    let mut player_1 = Player {
       id: 1,
       marker: None,
       name: None,
    };

    let mut player_2 = Player {
       id: 2,
       marker: None,
       name: None,
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

    while !winner {
        let mut x_coord: u8 = 0;
        let mut y_coord: u8 = 0;

        for i in &grid{
            println!("{:?}", i);
        }

        
        if player_1.name == None || player_2.name == None {

            //--- Request Player 1 information
            println!("Please enter the name for Player 1: \n");
            player_1.name = Some(String::new());
            
            //--- Request Player 2 information
            println!("Please enter the name for Player 2: \n");
            player_2.name = Some(String::new());

            println!("Hello {:?} & {:?}! Ready to play?\n", player_1.name, player_2.name);
            println!("\nNow we have to decide who wants X's and who wants O's...\n");

            println!("\n{:?}, please type in X or O: ", Some(player_1.name));
            player_1.marker = Some(String::new());

            let mut is_valid: bool = marker_validate(player_1.marker);

            if is_valid {
                println!("\n{:?} chose {:?} !\n", Some(player_1.name), Some(player_1.marker));
            }




        }




        
    }

    println!("The loop ended!");

}
