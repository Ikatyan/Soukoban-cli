use std::rc::Rc;
use stage::{Stage, StageObject};

pub trait MovableObj {
    fn move_to(&mut self, vec_x: i32, vec_y: i32) -> bool;
    fn move_by(&mut self, x: u32, y: u32) -> bool;
    fn is_possible_to_move(&self, x: u32, y: u32) -> bool;
}

#[derive(Debug)]
pub struct Player {
    x: u32,
    y: u32,
    parent: Rc<Stage>,
}

impl Player {
    pub fn new(x: u32, y: u32, parent: Rc<Stage>) -> Player {
        Player { x, y, parent }
    }
}

impl MovableObj for Player {
    fn move_to(&mut self, vec_x: i32, vec_y: i32) -> bool {
        let x = (vec_x + self.x as i32) as u32;
        let y = (vec_y + self.y as i32) as u32;
        let next_to = self.parent.crates.iter()
            .filter(|c| c.x() == x && c.y() == y).next();
        match next_to {
            Some(mut c) => {
                if !c.move_to(vec_x, vec_y) {
                    return false;
                } else {
                    return self.move_by(x, y);
                }
            }
            _ => return self.move_by(x, y)
        }
    }

    fn move_by(&mut self, x: u32, y: u32) -> bool {
        let is_possible = self.is_possible_to_move(x, y);
        if !is_possible {
            return is_possible;
        }
        self.x = x;
        self.y = y;
        true
    }

    fn is_possible_to_move(&self, x: u32, y: u32) -> bool {
        if x < 0 && x >= self.parent.width() {
            return false;
        }

        if y < 0 && y >= self.parent.height() {
            return false;
        }

        let stage_object = self.parent.get_object(x, y).unwrap();
        if stage_object == StageObject::WALL {
            return false;
        }

        return true;
    }
}

#[derive(Debug)]
pub struct Crate {
    x: u32,
    y: u32,
    goal_in: bool,
    stage: Rc<Stage>,
}

impl Crate {
    pub fn new(x: u32, y: u32, stage: Rc<Stage>) -> Crate {
        Crate { x, y, goal_in: false, stage }
    }
    pub fn x(&self) -> u32 { self.x }
    pub fn y(&self) -> u32 { self.y }
    pub fn goal_in(&self) -> bool { self.goal_in }
}

impl MovableObj for Crate {
    fn move_to(&mut self, vec_x: i32, vec_y: i32) -> bool {
        let x = (vec_x + self.x as i32) as u32;
        let y = (vec_y + self.y as i32) as u32;

        self.move_by(x, y)
    }

    fn move_by(&mut self, x: u32, y: u32) -> bool {
        let is_possible = self.is_possible_to_move(x, y);

        if !is_possible {
            return is_possible;
        }

        self.x = x;
        self.y = y;
        self.goal_in = self.stage.get_object(x, y).unwrap() == StageObject::CrateGoal;
        is_possible
    }

    fn is_possible_to_move(&self, x: u32, y: u32) -> bool {
        if x < 0 && x >= self.stage.width() {
            return false;
        }

        if y < 0 && y >= self.stage.height() {
            return false;
        }
        //ステージの範囲外だったら移動不可

        let stage_object = self.stage.get_object(x, y).unwrap();
        if stage_object == StageObject::WALL {
            return false;
        }
        //移動先が壁だったら移動不可

        if self.stage.crates.iter().all(|c| c.x == x && c.y == y) {
            return false;
        }
        //移動先にクレートがあったら移動不可

        true
    }
}


