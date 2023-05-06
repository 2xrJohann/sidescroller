const MAP_LENGTH: usize = 20;

struct Controller {
    x: usize,
    bag: Vec<Item>,
    world: World,
}

enum Item {
    JordationsSniperRifle,
    MatsesEzreal,
    JohannsDVA,
}

struct Block {
    item: Option<Item>,
}

impl Block {
    fn new(item: Option<Item>) -> Block {
        Block { item }
    }

    fn get(&mut self) -> Option<Item> {
        self.item.take()
    }
}

enum Direction {
    Left,
    Right,
}

impl Controller {
    fn new() -> Controller {
        Controller { x: 0, world: World::new(), bag: Vec::new() }
    }

    fn init(mut self) {
        println!("instructions: l | r");
        let mut buf = String::new();

        loop {
            let b1 = std::io::stdin().read_line(&mut buf).unwrap();
            match buf.as_str().trim() {
                "l" => {
                    self = self.move_player(Direction::Left);
                },
                "r" => {
                    self = self.move_player(Direction::Right);
                },
                _ => {
                    println!("invalid command");
                },
            }
            buf = "".to_string();
        
            self = self.print_map();
        }
    }

    fn move_player(mut self, direction: Direction) -> Controller {
        match direction {
            Direction::Left => self = self.move_left(),
            Direction::Right => self = self.move_right(),
        };

        self
    }

    fn print_bag(self) -> Controller {
        for item in &self.bag {
            match item {
                Item::JordationsSniperRifle => println!("JordationsSniperRifle😩 "),
                Item::MatsesEzreal => println!("MatsesEzreal😭 "),
                Item::JohannsDVA => println!("JohannsDVA🌈 "),
            }
        }

        self
    }

    fn move_left(self) -> Controller {
        match self.is_valid_move(Direction::Left) {
            true => Controller { x: self.x - 1, ..self },
            false => self,
        }
    }

    fn move_right(mut self) -> Controller {
        match self.is_valid_move(Direction::Right) {
            true => {
                self.x += 1;

                if let Some(item) = self.world.map[self.x].get() {
                    self.bag.push(item);
                }

                self
            },

            false => self,
        }
    }

    fn is_valid_move(&self, direction: Direction) -> bool {
        match direction {
            Direction::Left => {
                if self.x == 0 {
                    return false
                }
                
                return true
            },

            Direction::Right => {
                if self.x == MAP_LENGTH - 1 {
                   return false
                }
                
               return true
            },
        }
    }

    fn _print_position(self) -> Controller {
        println!("x: {}", self.x);
        
        self
    }

    fn print_map(self) -> Controller {
        for (index, value) in self.world.map.iter().enumerate() {
            let mut tile: String;

            match value.item {
                Some(Item::JordationsSniperRifle) |
                Some(Item::JohannsDVA)            |
                Some(Item::MatsesEzreal) 
                    => tile = String::from("[!] "),

                None => tile = String::from("[ ] "),
            }

             if index == self.x {
                tile = String::from("[x] ")
             }

            print!("{} ", tile);
        }

        println!();

        self
    }
}



struct World {
    map: Vec<Block>,
}

impl World {
    fn new() -> World {
        let mut blocks: Vec<Block> = Vec::new();
        for i in 0..MAP_LENGTH {
            match i {
                3 => blocks.push(Block::new(Some(Item::JordationsSniperRifle))),
                8 => blocks.push(Block::new(Some(Item::MatsesEzreal))),
                17 => blocks.push(Block::new(Some(Item::JohannsDVA))),
                _ => blocks.push(Block::new(None)),
            }
        }

        World { map: blocks }
    }
}

fn main() {
    let mut controller = Controller::new();
    
    controller.init()
}