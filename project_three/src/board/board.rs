use Vec;

#[derive(Debug)]
struct Board {
	grid: Vec<Vec<Tile>>
}

struct Tile{
	occupant: Box<dyn Occupant>,
	passive_percepts: Vec<Box<dyn Passive_Percept>>
}

trait Occupant {
	fn get_percept_produced(&self) -> Box<dyn Passive_Percept>;
}

trait Passive_Percept{
	fn describe(&self) -> String;
	fn 
}

