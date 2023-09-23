mod engine;
mod game_tree;

extern crate chess_game;

use std::collections::hash_map::{DefaultHasher, RandomState};
use std::collections::HashMap;
use std::env;
use std::hash::BuildHasher;
use std::io::stdin;
use std::ptr::write;
use std::time::Instant;
use chess_game::board::board::MoveType;
use chess_game::board::board::MoveType::Standard;
use chess_game::game::game::Game;
use engine::engine::Engine;
use crate::engine::engine::Branch;
use crate::game_tree::game_tree::GameTree;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut root: GameTree = GameTree::new(None);
    let depth = if args.len() > 1 {args[1].parse::<usize>().unwrap()} else {4};

    /*let mut game = Game::new_from_string("r1b2r1k/4qp1p/p2ppb1Q/4nP2/1p1NP3/2N5/PPP4P/2KR1BR1".to_string(), true);
    //let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), true);
    //let mut game = Game::new_from_string("8/8/7p/8/8/P7/8/8".to_string(), true);
    println!("game eval at start: {}", game.evaluate_board());
    chess_game::debug::debug::print_board(&game);

    println!("Starting engine.");
    let (moves1, leafs1) = Engine::get_sorted_moves(&mut game, true, depth, true);
    let (moves2, leafs2) = Engine::get_sorted_moves(&mut game, true, depth, false);
    println!("Leaves after {} moves with sorting: {} without sorting: {}", depth, leafs1, leafs2);
    //println!("{:?}", moves);*/


    do_game(depth);
}

pub fn read_line() -> usize {
    let mut buffer1 = String::new();
    stdin()
        .read_line(&mut buffer1)
        .expect("Failed to read line");
    //println!("buffer: {}", buffer1);
    let num: usize = buffer1.trim().parse().expect("Input not an integer");
    (num)
}

pub fn print_moves(branch: &Vec<Branch>) {
    for (i, b) in branch.iter().enumerate() {

        print!("{:^4}: ", i);
        print!("{:^100}", b.m);
        println!("{:^15}", b.val)
    }
}

pub fn do_game(depth: usize) {
    let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), true);
    let mut leaves = 0;


    let hasher = RandomState::new();
    let mut map: HashMap<u64, i32> = HashMap::with_hasher(hasher);

    let start = Instant::now();
    loop {
    //for i in 1..11 {
        chess_game::debug::debug::print_board(&game);
        println!("map size: {}", map.len());
        let mut new_game = game.clone();
        let (moves, leafs) = Engine::get_sorted_moves(&mut game, &mut map,new_game.is_white_turn, depth, false);
        println!("Leaves after {} moves: {}", depth, leafs);
        leaves += leafs;
        print_moves(&moves);

        //let index = read_line();

        game.make_move(&moves[0].m);
    }

    println!("{}", start.elapsed().as_secs());

    println!("average leaves: {}", leaves / 10);

}