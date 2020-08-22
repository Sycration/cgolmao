use rand::Rng;

#[derive(Debug)]
pub struct Game {
    pub board: Vec<Vec<bool>>,
    pub rounds: i64,
}

//OUTSIDE IS X, INSIDE IS Y
impl Game {
    pub fn new(xcoord: i32, ycoord: i32, percent: i32) -> Game {
        Game {
            board: {
                //OUTSIDE IS X, INSIDE IS Y
                let mut board: Vec<Vec<bool>> = vec![vec![false; ycoord as usize]; xcoord as usize];
                let mut rng = rand::thread_rng();
                for o in 0..xcoord {
                    for i in 0..ycoord {
                        if percent <= 99 {
                            board[o as usize][i as usize] = true;
                        }
                        let val: i32 = rng.gen::<i32>() % 100;
                        if val < percent {
                            board[o as usize][i as usize] = true;
                        } else {
                            board[o as usize][i as usize] = false;
                        }
                    }
                }
                board
            },
            rounds: 0,
        }
    }

    pub fn update(&mut self) {
        let old_board = self.board.clone();

        for o in 0..self.board.len() {
            for i in 0..self.board[0].len() {
                let o = o + self.board.len();
                let i = i + self.board[0].len();
                let mut count: i32 = 0;
                for dout in [-1, 0, 1].iter() {
                    for din in [-1, 0, 1].iter() {
                        if *dout == 0 && *din == 0 {
                            continue;
                        }

                        if old_board[((o as i32 + *dout as i32) % self.board.len() as i32) as usize]
                            [((i as i32 + *din as i32) % self.board[0].len() as i32) as usize]
                        {
                            count += 1;
                        }
                    }
                }
                if !old_board[o % self.board.len()][i % self.board[0].len()] {
                    if count == 3 {
                        self.board[o % old_board.len()][i % old_board[0].len()] = true;
                    }
                } else if count != 2 && count != 3 {
                    self.board[o % old_board.len()][i % old_board[0].len()] = false;
                }
            }
        }
    }
    pub fn print(&self) {
        for o in 0..self.board.len() {
            for i in 0..self.board[0].len() {
                match self.board[o][i] {
                    true => print!("#"),
                    _ => print!(" "),
                }
            }
            println!()
        }
    }
}
