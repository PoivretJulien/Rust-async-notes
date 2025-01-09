

pub mod video_game_case {
    #[derive(Debug)]
    pub struct Monster {
        pub health: i16,
        pub alive: bool,
        pub attack_taken: u64,
    }
    impl Monster {
        pub fn new(health: i16) -> Self {
            Self {
                health,
                alive: true,
                attack_taken: 0,
            }
        }
        pub fn take_damage(&mut self, amount: i16) {
            if self.alive {
                self.health -= amount;
                self.attack_taken += 1;
                if self.health <= 0 {
                    self.alive = false;
                    println!("Monster is dead {:?}", self);
                }
            } else {
                eprintln!("(Error monster already dead !")
            }
        }

        pub fn attack(&self, player: &mut Character, amount: i16) {
            player.take_damage(amount);
            println!("Slap! ({amount})")
        }
    }
    impl Default for Monster {
        fn default() -> Self {
            Self {
                health: 1000,
                alive: true,
                attack_taken: 0,
            }
        }
    }
    #[derive(Debug)]
    pub struct Character {
        pub name:String,
        pub health: i16,
        pub alive: bool,
        pub attack_taken: i16,
    }
    impl Character {
        pub fn new(name:&str,health: i16) -> Self {
            Self {
                name: name.to_owned(),
                health,
                alive: true,
                attack_taken: 0,
            }
        }
        pub fn take_damage(&mut self, amount: i16) {
            if self.alive {
                self.health -= amount;
                self.attack_taken += 1;
                println!("Slap! damages:{amount}");
                if self.health <= 0 {
                    self.alive = false;
                    println!("Player is unfortunately  already dead.. status:{:?}", self);
                }
            } else {
                eprintln!("Crazy monster... Player ({0}) is already dead -_-'...",self.name);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
use rand::*;
use raw_pointer_study::raw_pointer;
use std::sync::Arc;
use tokio::sync::Mutex;
use video_game_case::{Character, Monster};
////////////////////////////////////////////////////////////////////////////////
#[tokio::main]
async fn main() {
    /*
     *  Async Task test in Rust.
     *  memo on how to manage asynchronously and safely some game entity 
     *  sharing mutable memory state for in game interactions.
     */
    // Create an horde of monsters.
    let mut monster_cluster = (0..10)
        .map(|_| Monster::default())
        .collect::<Vec<Monster>>();
    // Create a Player sadly shared target across attack Task.
    let mut player = Arc::new(
        Mutex::new(
            Character::new("Milton",3000)
            ));
    // Create one task by monster and attack the player.
    let mut handles = Vec::new(); // handles stack.
    monster_cluster.iter().for_each(|monster| {
        // Make an atomic pointer.
        let mut target_player = player.clone();
        // Stack handles
        let handle = tokio::task::spawn(async move {
            // monster.attack(player, amount); // template. (target ,amount)
            // Async Task    ////////////////////////////////////////////////
            let mut target = target_player.lock().await;
            // Generate random number. //////////////////////////////////////
            let damages = rand::thread_rng().gen_range(400..808);
            target.take_damage(damages);
            if target.health < 0 {
                target.alive = false;
            }
            /////////////////////////////////////////////////////////////////
        });
        handles.push(handle);
    });
    for handle in handles {
        handle.await.unwrap();
    }
    ///////////////////////////////////////////////////////////////////////////
    // Raw Pointer Test.         (for computer enthusiast)
    ///////////////////////////////////////////////////////////////////////////
    raw_pointer(); 
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
}
/*
*                Rust Raw Pointer base offset technic.
*/
mod raw_pointer_study {
    pub fn raw_pointer() {
        let mut array: [f32; 5] = [0.0; 5];

        // Compute how many bytes in an f32 type.
        let base = std::mem::size_of::<f32>();
        println!("Size of an f32:({base}) bytes");
        println!("Initial State of Array: {:?}", array);

        // Then make a raw pointer on the first elements of the
        // array and offset the memory block by an offset multiple
        // of the array base elements
        //                      (sized in bytes multiples as usize).

        let ptr_on_array = &mut array as *mut f32; // make raw pointer.
        let address = ptr_on_array as usize; //Extract address.

        // then Mutate the raw pointer offset of your choice
        // from the raw components available.
        // for mutating the 3 elements (index 2):
        unsafe {
            *((address + (base * 2)) as *mut f32) = 9.999;
        }
        // for mutating the 4 elements index(3):
        unsafe {
            *ptr_on_array.offset(3) = 3.3333;
        }
        // Print the result on console:
        println!("Mutated state of the Array: {:?}", array);
        /*
         * This provide fast access on data without any safety measure
         * it's equivalent of pure C where fast processing is paramount
         * and the safety context carefully measured.
         * it's tool in the tools box. rust forbid nothing it's just
         * the way how to do it safely if you want to be stupid then you
         * should have your reason i guess i love that tools spirit noting
         * less nothing more just right.
         */
    }
}
