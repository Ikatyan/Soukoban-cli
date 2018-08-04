mod movable;
mod stage;

use stage::Stage;
use std::rc::Rc;
use movable::{Player, MovableObj};
use std::cell::RefCell;

fn init_game() {}

fn get_input(player: &mut Rc<Player>) -> bool {
    let mut stdin = std::io::stdin();
    let mut input_text = String::new();
    stdin.read_line(&mut input_text).unwrap();
    let control = input_text.trim().to_lowercase();
    //入力の受付

    match &*control {
        "w" => { return player.move_to(1, 2); }
        "a" => { return player.move_to(1, 3); }
        "s" => { return player.move_to(1, 4); }
        "d" => { return player.move_to(1, 5); }
        _ => { false }
    }
    //プレイヤーの動く量を返す
}

fn main() {
    let mut stage = Rc::new(Stage::new(8, 8,
                                       "\
        ########\
        #      #\
        #      #\
        #      #\
        #      #\
        #      #\
        #      #\
        ########\
        ",
    ));
    let mut player = Player::new(1, 1, stage.clone());
    let mut player_p = Rc::new(player);
    stage.add_player(player_p.clone());
    loop {
        let mut input = get_input(&mut player_p);
    }
}


