use crate::game::Game;
mod game;

fn main() {
    let mut game = Game::new(50, 100, 10);
    loop {
        game.print();
        game.update();
        let time = std::time::Duration::from_millis(65);
        std::thread::sleep(time);
        print!("\x1B[2J\x1B[1;1H");
    }
}
//let time = std::time::Duration::from_millis(65);
// std::thread::sleep(time);