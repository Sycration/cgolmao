use crate::game::Game;
mod game;
extern crate clap;
use clap::Clap;
fn main() {
    #[derive(Clap)]
    #[clap(
        version = "1.2",
        author = "Sycration <sycration@gmail.com>",
        about = "Terminal implementation of Conway's game of life. Use the '-h' or '--help' flag for help.\nDefault born and live settings is Conway's game."
    )]
    struct Opts {
        //make commandline options
        ///What percent of the board starts alive
        #[clap(short, default_value = "15")]
        percent_alive: String,
        ///How many cells wide is the board
        #[clap(short)]
        xlength: i32,
        ///How many cells tall in the board
        #[clap(short)]
        ylength: i32,
        ///How many milliseconds between refreshes
        #[clap(short, default_value = "65")]
        time: String,
        ///What amount of neighbo!rs will keep a cell alive
        #[clap(short, default_value = "23")]
        live: String,
        ///What amount of neighbors will make a cell be born
        #[clap(short, default_value = "3")]
        born: String,
    }

    let opts: Opts = Opts::parse(); //get commandline flags
    let time: i32 = opts.time.parse::<i32>().unwrap_or(65);
    let live: String = opts.live.chars().map(|x| {
        if x.is_numeric() { x } else {' '}
    }).collect();
    let born: String = opts.born.chars().map(|x| {
        if x.is_numeric() { x } else {' '}
    }).collect();
    let mut game = Game::new(
        opts.ylength,
        opts.xlength,
        opts.percent_alive.parse::<i32>().unwrap_or(15),
        live,
        born,
    );
    loop {
        game.print();
        game.update();
        let time = std::time::Duration::from_millis(time as u64);
        std::thread::sleep(time);
        print!("\x1B[2J\x1B[1;1H"); //unix escape sequence to clear screen
    }
}
