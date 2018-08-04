use std::rc::Rc;
use movable::Player;
use movable::Crate;


#[derive(Debug)]
pub struct Stage {
    width: u32,
    height: u32,
    stage_objects: Vec<StageObject>,
    players: Vec<Rc<Player>>,
    pub crates: Vec<Rc<Crate>>,
}

impl Stage {
    pub fn new(width: u32, height: u32, stage: &str) -> Stage {
        Stage {
            width,
            height,
            stage_objects: stage.chars().map(char_to_object).collect(),
            players: Vec::new(),
            crates: Vec::new(),
        }
    }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    pub fn add_player(&mut self, player: Rc<Player>) {
        self.players.push(player);
    }

    pub fn add_crate(&mut self, crate_obj: Rc<Crate>) {
        self.crates.push(crate_obj)
    }

    pub fn get_object(&self, x: u32, y: u32) -> Result<StageObject, ()> {
        if x < self.width && y < self.height {
            let index = self.width * y + x;
            return Ok(self.stage_objects[index as usize]);
        }
        Err(())
    }

    pub fn draw(&self) {}
}

fn char_to_object(c: char) -> StageObject {
    match c {
        '#' => StageObject::WALL,
        ' ' => StageObject::Floor,
        '.' => StageObject::CrateGoal,
        _ => StageObject::Floor,
    }
}

fn object_to_char(stage_object: StageObject) -> char {
    match stage_object {
        StageObject::WALL => '#',
        StageObject::CrateGoal => '.',
        StageObject::Floor => ' ',
    }
}

#[derive(PartialEq, Debug)]
pub enum StageObject {
    WALL,
    Floor,
    CrateGoal,
}