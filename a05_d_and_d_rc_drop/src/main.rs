use std::cell::RefCell;
use std::io::stdin;
use std::rc::Rc;

struct Player {
    name: String,
    health: i32,
    attack: i32,
}

struct Dungeon {
    rooms: Vec<Room>,
    current_room: usize,
}

struct Room {
    description: String,
    contents: RoomContent,
}

struct Monster {
    name: String,
    health: i32,
    attack: i32,
}

enum RoomContent {
    Monster(Rc<RefCell<Monster>>),
    Treasure(i32),
    Empty,
}

impl Player {
    fn new(name: String) -> Self {
        Player {
            name,
            health: 100,
            attack: 10,
        }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 0 {
            self.health = 0;
        }
    }

    fn attack_monster(&self, monster: &mut Monster) {
        monster.health -= self.attack;
        if monster.health < 0 {
            monster.health = 0;
        }
    }
}

impl Monster {
    fn new(name: &str, health: i32, attack: i32) -> Self {
        Monster {
            name: name.to_string(),
            health,
            attack,
        }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn attack_player(&self, player: &mut Player) {
        player.take_damage(self.attack);
    }
}

impl Room {
    fn new(description: &str, content: RoomContent) -> Self {
        Room {
            description: description.to_string(),
            contents: content,
        }
    }
}

impl Dungeon {
    fn new() -> Self {
        let rooms = vec![
            Room::new(
                "You are in a dark cave.",
                RoomContent::Monster(Rc::new(RefCell::new(Monster::new("Goblin", 30, 5)))),
            ),
            Room::new("You find a treasure chest.", RoomContent::Treasure(50)),
            Room::new("You are in an empty room.", RoomContent::Empty),
            Room::new(
                "You encounter a fierce dragon.",
                RoomContent::Monster(Rc::new(RefCell::new(Monster::new("Dragon", 100, 20)))),
            ),
        ];

        Dungeon {
            rooms,
            current_room: 0,
        }
    }

    fn current_room(&self) -> &Room {
        &self.rooms[self.current_room]
    }

    fn next_room(&mut self) {
        if self.current_room + 1 < self.rooms.len() {
            self.current_room += 1;
        }
    }
}

fn main() {
    let mut player = Player::new("Hero".to_string());
    let mut dungeon = Dungeon::new();

    println!("Welcome to the dungeon, {}!", player.name);

    let stdin = stdin();  // Create the stdin reader explicitly
    let mut input = String::new();

    while player.is_alive() {
        let room = dungeon.current_room();
        println!("{}", room.description);

        match &room.contents {
            RoomContent::Monster(monster_rc) => {
                let mut monster = monster_rc.borrow_mut();
                while player.is_alive() && monster.is_alive() {
                    println!("You encounter a {}!", monster.name);
                    println!("1. Attack");
                    println!("2. Run");

                    input.clear();
                    stdin.read_line(&mut input).expect("Failed to read line");
                    let choice = input.trim();

                    if choice == "1" {
                        player.attack_monster(&mut monster);
                        println!(
                            "You attack the {}. It now has {} health.",
                            monster.name, monster.health
                        );
                        if monster.is_alive() {
                            monster.attack_player(&mut player);
                            println!(
                                "The {} attacks you. You now have {} health.",
                                monster.name, player.health
                            );
                        } else {
                            println!("You have defeated the {}!", monster.name);
                        }
                    } else {
                        println!("You run away!");
                        break;
                    }
                }
            }
            RoomContent::Treasure(amount) => {
                println!("You find a treasure worth {} gold!", amount);
            }
            RoomContent::Empty => {
                println!("The room is empty.");
            }
        }

        if player.is_alive() {
            dungeon.next_room();
        } else {
            println!("You have died. Game over.");
        }

        if dungeon.current_room == dungeon.rooms.len() - 1 {
            println!("You have cleared the dungeon. Congratulations!");
            break;
        }
    }

    // Explicitly drop the stdin reader
    drop(stdin);

    // Ensure everything is dropped properly by the end of main
    println!("Game over.");
}
