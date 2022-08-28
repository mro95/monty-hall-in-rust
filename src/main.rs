use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Clone)]
enum Door {
    Door1, Door2, Door3,
}

impl Door {
    fn random_door() -> Door {
        match rand::thread_rng().gen_range(1..=3) {
            1 => Door::Door1, 2 => Door::Door2, 3 => Door::Door3,
            i32::MIN..=0_i32 | 4_i32..=i32::MAX => panic!("invalid random nummer"),
        }
    }
}

#[derive(Debug)]
struct Game {
    winning_door: Door,
    choosen_door: Option<Door>,
    revealed_door: Option<Door>,
}

impl Game {
    fn new() -> Game {
        Game {
            winning_door: Door::random_door(),
            choosen_door: None,
            revealed_door: None,
        }
    }

    fn player_choose_door(&mut self) {
        self.choosen_door = Some(Door::random_door());
    }

    fn host_reveal_door(&mut self) {
        let all_doors = vec![ Door::Door1, Door::Door2, Door::Door3, ];
        self.revealed_door = Some(all_doors
            .into_iter()
            .filter(|x| Some(x) != self.choosen_door.as_ref())
            .filter(|x| *x != self.winning_door)
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
            .expect("slice is empty")
            .clone()
        );
    }

    fn player_switches_door(&mut self) {
        let all_doors = vec![ Door::Door1, Door::Door2, Door::Door3, ];
        self.choosen_door = Some(all_doors
            .into_iter()
            .filter(|x| Some(x) != self.choosen_door.as_ref())
            .filter(|x| Some(x) != self.revealed_door.as_ref())
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
            .expect("slice is empty")
            .clone()
        );
    }

    fn player_won(&self) -> bool {
        return &self.winning_door == self.choosen_door.as_ref().unwrap() 
    }
}

fn main() {
    let mut winning = 0;
    let count = 5000;

    for _ in 0..count {
        let mut game = Game::new();
        game.player_choose_door();
        game.host_reveal_door();
        game.player_switches_door();
        if game.player_won() {
            winning += 1;
        }
    }

    println!("winning {} = {}%", winning, winning as f32 / count as f32 * 100.0);
    println!("loosing {} = {}%", count-winning, (count-winning) as f32 / count as f32 * 100.0);
}
