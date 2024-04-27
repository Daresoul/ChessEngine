mod engine;
mod move_gen;
mod magic;
mod game;
mod board;
mod debug;
mod eval_board;
mod utils;
mod move_list;

use std::env;

use std::io::stdin;
use std::time::Instant;
use crate::board::board::Move;
use crate::board::board::Move::{Promotion, Standard};
use crate::engine::engine::{Branch, Engine};
use crate::game::game::Game;


fn main() {
    let args: Vec<String> = env::args().collect();

    let _depth = if args.len() > 1 {args[1].parse::<usize>().unwrap()} else {4};


    //"r1b2r1k/4qp1p/p2ppb1Q/4nP2/1p1NP3/2N5/PPP4P/2KR1BR1"
    //"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    //"8/8/7p/8/8/P7/8/8"


//    do_game_white(depth);
    do_game_white(2);
}

pub fn print_moves(m: &Vec<Move>) {
    for i in 0..m.len() {
        match m[i] {
            Standard(pos, to, ptype, is_white) => println!("{}: {} -> {} as {:?}", i, pos, to, ptype),
            Promotion(pos, to, ptype, cp, is_white) => println!("{}: {} -> {} to {:?}", i, pos, to, ptype),
            _ => ()
        }

    }
}

pub fn read_line() -> usize {
    let mut buffer1 = String::new();
    stdin()
        .read_line(&mut buffer1)
        .expect("Failed to read line");
    //println!("buffer: {}", buffer1);
    let num: usize = buffer1.trim().parse().expect("Input not an integer");
    num
}

pub fn print_branches(branches: &Vec<Branch>) -> () {
    for (i, branch) in branches.iter().enumerate() {
        match branch.m {
            Standard(pos, to, ptype, is_white) => println!("{}: {} -> {} as {:?} with: {}", i, pos, to, ptype, branch.val),
            Promotion(pos, to, ptype, cp, is_white) => println!("{}: {} -> {} to {:?} with: {}", i, pos, to, ptype, branch.val),
            _ => ()
        }
    }
}

pub fn do_game_white(depth: usize) {

    let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), true);

    let mut leaves = 0;

    loop {
        if game.is_white_turn {
            debug::debug::print_board(&game);
            let start = Instant::now();
            let (moves, leafs) = Engine::get_sorted_moves(&mut game, true, depth);
            println!("Leaves after {} moves: {}", depth, leafs);
            leaves += leafs;
            println!("miliseconds elapsed: {}", start.elapsed().as_millis());

            print_branches(&moves);

            let index = read_line();

            game.make_move(&moves[index].m);
        } else {
            debug::debug::print_board(&game);
            let moves = game.get_all_moves();
            println!("move_len: {}", moves.len());
            print_moves(&moves);

            let index = read_line();

            game.make_move(&moves[index]);
        }
    }
}