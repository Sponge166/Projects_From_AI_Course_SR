use core::cell::RefCell;
use std::rc::Rc;

pub struct Tile{
	pub occupant: Option<Occupant>,
	pub passive_percepts: Vec<Rc<PassivePercept>>
}

impl std::fmt::Debug for Tile{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>{
		write!(f, "{:?}", self.occupant)
	}
}


impl Tile{
	pub fn empty() -> Tile{
		Tile {occupant: None, passive_percepts: Vec::new(), }
	}

	pub fn from_char(c: &str) -> Option<Tile>{
		Some(Tile { occupant: match c {
			"P" => Some(Occupant::Pit(Pit(PassivePercept::Breeze(Breeze)))),
			"G" => Some(Occupant::Gold(Gold(PassivePercept::Glitter(Glitter)))),
			"W" => Some(Occupant::Wampus(Wampus(PassivePercept::Stink(Stink), RefCell::new(true)))),
			_ => None
		}, passive_percepts: Vec::new()})
	}

	pub fn add(&mut self, percept: Rc<PassivePercept>){
		self.passive_percepts.push(percept);
	}
}

pub trait OccupantTrait {
	fn get_percept_produced(&self) -> Rc<PassivePercept>;
}

pub trait PassivePerceptTrait: std::fmt::Debug{
	fn describe(&self) -> String;
	fn effect(&self) -> Effect;
}

pub enum Effect{
	Neighbors,
	NotNeighbors
}

#[derive(Debug)]
pub enum Occupant {
	Pit(Pit),
	Wampus(Wampus),
	Gold(Gold)
}

impl OccupantTrait for Occupant{
	fn get_percept_produced(&self) -> Rc<PassivePercept> {
		match self {
			Occupant::Pit(x) => x.get_percept_produced(),
			Occupant::Wampus(x) => x.get_percept_produced(),
			Occupant::Gold(x) => x.get_percept_produced()
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub enum PassivePercept {
	Stink(Stink),
	Glitter(Glitter),
	Breeze(Breeze)
}

impl PassivePerceptTrait for PassivePercept{
	fn describe(&self) -> String {
		match self {
			PassivePercept::Breeze(x) => x.describe(),
			PassivePercept::Stink(x) => x.describe(),
			PassivePercept::Glitter(x) => x.describe()
		}
	}

	fn effect(&self) -> Effect {
		match self {
			PassivePercept::Breeze(x) => x.effect(),
			PassivePercept::Stink(x) => x.effect(),
			PassivePercept::Glitter(x) => x.effect()
		}
	}
}


// percept and occupant defs


#[derive(Debug, Copy, Clone)]
pub struct Stink;

impl PassivePerceptTrait for Stink{
	fn describe(&self) -> String{
		String::from("You smell something foul")
	}

	fn effect(&self) -> Effect{
		Effect::Neighbors
	}
}

#[derive(Debug)]
pub struct Wampus(PassivePercept, RefCell<bool>);

impl OccupantTrait for Wampus{
	fn get_percept_produced(&self) -> Rc<PassivePercept>{
		Rc::new(self.0)
	}
}

impl Wampus {
	pub fn is_alive(&self) -> bool{
		if *self.1.borrow(){ return true; }
		false
	}

	pub fn frag_it(&self) {
		self.1.swap(&RefCell::new(false));
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Glitter;

impl PassivePerceptTrait for Glitter{
	fn describe(&self) -> String{
		String::from("You see a bright Gitter! You've found the Gold")
	}

	fn effect(&self) -> Effect{
		Effect::NotNeighbors
	}
}

#[derive(Debug)]
pub struct Gold(PassivePercept);

impl OccupantTrait for Gold{
	fn get_percept_produced(&self) -> Rc<PassivePercept>{
		Rc::new(self.0)
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Breeze;

impl PassivePerceptTrait for Breeze{
	fn describe(&self) -> String{
		String::from("You feel a strong Breeze ")
	}

	fn effect(&self) -> Effect{
		Effect::Neighbors
	}
}

#[derive(Debug)]
pub struct Pit(PassivePercept);

impl OccupantTrait for Pit{
	fn get_percept_produced(&self) -> Rc<PassivePercept>{
		Rc::new(self.0)
	}
}
