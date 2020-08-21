use rand::Rng;
use rand::distributions::{Distribution, Standard};
#[derive(Debug)]
pub struct Game {
    Board: Vec<Vec<bool>>,
    Rounds: i64,
}
//OUTSIDE IS X, INSIDE IS Y
impl Game {
    pub fn new(xcoord: i32, ycoord: i32, percent: i8) -> Game {
        Game {
            Board: {
                //OUTSIDE IS X, INSIDE IS Y
                let mut board: Vec<Vec<bool>> = vec![vec![false; *&ycoord as usize]; *&xcoord as usize];
                let mut rng = rand::thread_rng();
                for o in 0.. *&xcoord {
                    for i in 0.. *&ycoord {
                        if percent <= 99{
                            board[o as usize][i as usize] = true;
                        }
                        let val: i8 = (rng.gen::<i8>() % 100);
                        if val <= *&percent{
                            board[o as usize][i as usize] = true;
                        }
                        else {
                            board[o as usize][i as usize] = false;
                        }
                    }
                }
                board
            },
            Rounds: 0,
        }
    }
}

fn main() {
    let game = Game::new(5, 5, 5);
    println!("{:#?}", game);
}