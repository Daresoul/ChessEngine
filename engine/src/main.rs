mod engine;
mod move_gen;
mod magic;
mod game;
mod board;
mod debug;
mod eval_board;
mod utils;
mod move_list;

use std::{env, thread};

use std::io::stdin;
use std::ops::Index;
use std::str::Chars;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::time::{Duration, Instant};
use minifb::{Key, Window, WindowOptions};
use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use raylib::ffi;
use raylib::ffi::{MouseButton, Vector2};
use raylib::prelude::Image;
use crate::board::board::Move;
use crate::board::board::Move::{Promotion, Standard};
use crate::engine::engine::{Branch, Engine};
use crate::game::game::Game;


const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _depth = if args.len() > 1 {args[1].parse::<usize>().unwrap()} else {4};

    let (move_list_sender, move_list_reciever) = mpsc::channel();

    let (board_sender, board_reciever) = mpsc::channel();

    let (move_sender, move_reciever) = mpsc::channel();


    let handle = thread::spawn(move || {
        do_game_white(&move_list_sender, &board_sender, &move_reciever, 6);
    });

    graphics(&move_list_reciever, &board_reciever, &move_sender)


    //"r1b2r1k/4qp1p/p2ppb1Q/4nP2/1p1NP3/2N5/PPP4P/2KR1BR1"
    //"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    //"8/8/7p/8/8/P7/8/8"

    //do_game_white(6);
}

pub fn print_moves(m: &Vec<Move>) {
    for i in 0..m.len() {
        println!("{}: {}", i, m[i].to_printable())
    }
}

pub fn read_line() -> i32 {
    let mut buffer1 = String::new();
    stdin()
        .read_line(&mut buffer1)
        .expect("Failed to read line");
    //println!("buffer: {}", buffer1);
    let num: i32 = buffer1.trim().parse().expect("Input not an integer");
    num
}

pub fn print_branches(branches: &Vec<Branch>) -> () {
    for (i, branch) in branches.iter().enumerate() {
        println!("{}: {} with: {}", i, branch.m.to_printable(), branch.val);
    }
}

pub fn branch_to_moves(branches: &Vec<Branch>) -> Vec<(Move, Option<i32>)> {
    let mut moves = vec![];

    for branch in branches.iter() {
        moves.push((branch.m, Some(branch.val)))
    }

    return moves
}

pub fn move_list_to_val_move_list(m: &Vec<Move>) -> Vec<(Move, Option<i32>)> {
    let mut moves = vec![];

    for branch in m.iter() {
        moves.push((*branch, None))
    }

    return moves
}

pub fn do_game_white(move_list_sender: &Sender<Vec<(Move, Option<i32>)>>, board_sender: &Sender<[u64; 12]>, move_reciever: &Receiver<Move>, depth: usize) {

    // "r1bqkbnr/pppp1p1p/8/4P1p1/8/2N5/PPP1PPPP/R1BQKB1R"
    // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"

    let mut game = Game::new_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), true);

    let mut leaves = 0;

    loop {
        let boards = [
            game.board.white_pawn_board, game.board.white_knight_board, game.board.white_bishop_board, game.board.white_rook_board, game.board.white_queen_board, game.board.white_king_board,
            game.board.black_pawn_board, game.board.black_knight_board, game.board.black_bishop_board, game.board.black_rook_board, game.board.black_queen_board, game.board.black_king_board
        ];

        board_sender.send(boards);


        if game.is_white_turn {
            debug::debug::print_board(&game);
            let start = Instant::now();
            let (moves, leafs) = Engine::get_sorted_moves(&mut game, true, depth);
            println!("Leaves after {} moves: {}", depth, leafs);
            leaves += leafs;
            println!("miliseconds elapsed: {}", start.elapsed().as_millis());

            print_branches(&moves);



            move_list_sender.send(branch_to_moves(&moves));

            let m = move_reciever.recv().unwrap();

            if m == Move::None {
                game.undo_move()
            } else {
                game.make_move(&m);
            }
        } else {
            debug::debug::print_board(&game);
            let (mut moves, _, _) = game.get_all_moves();

            moves.sort_by(Engine::ordering_moves);

            println!("move_len: {}", moves.len());
            print_moves(&moves);

            
            
            move_list_sender.send(move_list_to_val_move_list(&moves));

            let m = move_reciever.recv().unwrap();

            if m == Move::None {
                game.undo_move()
            } else {
                game.make_move(&m);
            }
        }
    }
}

pub fn graphics(move_list_reciever: &Receiver<Vec<(Move, Option<i32>)>>, board_reciever: &Receiver<[u64; 12]>, move_sender: &Sender<Move>) {
    let (mut rl, thread) = raylib::init()
        .size(400, 400)
        .resizable()
        .title("Hello, World")
        .build();

    let arial = rl.load_font_ex(&thread, "resources/Montserrat.ttf", 64, None).unwrap();

    let black_pawn_img = Image::load_image("resources/black_pawn.png").unwrap();
    let black_pawn = rl.load_texture_from_image(&thread, &black_pawn_img).unwrap();
    let black_knight_img = Image::load_image("resources/black_knight.png").unwrap();
    let black_knight = rl.load_texture_from_image(&thread, &black_knight_img).unwrap();
    let black_bishop_img = Image::load_image("resources/black_bishop.png").unwrap();
    let black_bishop = rl.load_texture_from_image(&thread, &black_bishop_img).unwrap();
    let black_rook_img = Image::load_image("resources/black_rook.png").unwrap();
    let black_rook = rl.load_texture_from_image(&thread, &black_rook_img).unwrap();
    let black_queen_img = Image::load_image("resources/black_queen.png").unwrap();
    let black_queen = rl.load_texture_from_image(&thread, &black_queen_img).unwrap();
    let black_king_img = Image::load_image("resources/black_king.png").unwrap();
    let black_king = rl.load_texture_from_image(&thread, &black_king_img).unwrap();

    let white_pawn_img = Image::load_image("resources/white_pawn.png").unwrap();
    let white_pawn = rl.load_texture_from_image(&thread, &white_pawn_img).unwrap();
    let white_knight_img = Image::load_image("resources/white_knight.png").unwrap();
    let white_knight = rl.load_texture_from_image(&thread, &white_knight_img).unwrap();
    let white_bishop_img = Image::load_image("resources/white_bishop.png").unwrap();
    let white_bishop = rl.load_texture_from_image(&thread, &white_bishop_img).unwrap();
    let white_rook_img = Image::load_image("resources/white_rook.png").unwrap();
    let white_rook = rl.load_texture_from_image(&thread, &white_rook_img).unwrap();
    let white_queen_img = Image::load_image("resources/white_queen.png").unwrap();
    let white_queen = rl.load_texture_from_image(&thread, &white_queen_img).unwrap();
    let white_king_img = Image::load_image("resources/white_king.png").unwrap();
    let white_king = rl.load_texture_from_image(&thread, &white_king_img).unwrap();

    const SQUARE_SIZE: i32 = 50;
    const OFFSET_FOR_NUMBERING: i32 = 50;

    let mut moves = vec![];
    let mut boards: [u64; 12] = [0; 12];

    let mut files = "ABCDEFGH";

    while !rl.window_should_close() {

        let a = move_list_reciever.try_recv();
        match a {
            Ok(ms) => moves = ms,
            Err(_) => (),
        }

        let b = board_reciever.try_recv();

        match b {
            Ok(bs) => boards = bs,
            Err(_) => ()
        }

        let used_boards = boards.clone();

        let mut d = rl.begin_drawing(&thread);


        d.clear_background(Color::WHITE);

        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_position = d.get_mouse_position();
            if mouse_position.x > 495_f32 {
                let index = (mouse_position.y as i32 - 20) / 25;
                if (index as usize) < moves.len() {
                    let (m, val) = moves[index as usize];
                    move_sender.send(m);
                    moves.clear();
                }
                println!("{}", index);
            }
        }


        for i in 0..64 {

            if i % 8 == 0 {
                d.draw_text_ex(&arial, &(8 - (i/8)).to_string(), Vector2 { x: 20_f32, y: 15_f32 + (50 * i/8) as f32 }, 20_f32, 0_f32, Color::BLACK)
            }

            let row = i / 8;

            let color: Color = if row % 2 == 0  {

                if i % 2 == 0 {
                    Color::LIGHTGRAY
                } else {
                    Color::DARKGREEN
                }

            } else {
                if i % 2 == 0 {
                    Color::DARKGREEN
                } else {
                    Color::LIGHTGRAY
                }
            };

            d.draw_rectangle(OFFSET_FOR_NUMBERING + (i%8) * SQUARE_SIZE, (i / 8) * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE, color);
            d.draw_text_ex(
                &arial,
                &format!("{}{}", files.chars().nth((i%8) as usize).unwrap(), 8 - (i / 8)),
                Vector2 { x: (OFFSET_FOR_NUMBERING + (i%8) * SQUARE_SIZE) as f32, y: ((i / 8) * SQUARE_SIZE) as f32 },
                20_f32,
                0_f32,
                Color::BLACK
            )
        }

        for i in 0..8 {
            d.draw_text_ex(&arial, &files.chars().nth(i).unwrap().to_string(), Vector2 { x: OFFSET_FOR_NUMBERING as f32 + 15_f32 + (50 * i) as f32, y: 420_f32 }, 20_f32, 0_f32, Color::BLACK)
        }

        d.draw_text_ex(&arial, &format!("Moves count: {}", moves.len()), Vector2 { x: OFFSET_FOR_NUMBERING as f32, y: 470_f32 }, 20_f32, 0_f32, Color::BLACK);

        //20 + (20 * i) as i32
        for (i, (m, val)) in moves.iter().enumerate() {
            let mut str = m.to_printable();
            match val {
                None => {}
                Some(v) => str += &format!(" ---->>>> {}", v)
            }
            d.draw_text_ex(&arial, &str, Vector2 { x: 500_f32, y: 20_f32 + (25 * i) as f32 }, 20_f32, 0_f32, Color::BLACK)
        }



        let mut white_pawn_board = used_boards[0];
        for _ in 0..white_pawn_board.count_ones() {
            let index = utils::utils::pop_lsb(&mut white_pawn_board);
            d.draw_texture(
                &white_pawn,
                (OFFSET_FOR_NUMBERING - 6) + (index as i32%8_i32) * SQUARE_SIZE,
                -5 + (index as i32 / 8) * SQUARE_SIZE,
                Color::WHITE
            )
        }

        let mut white_knight_board = used_boards[1];
        for _ in 0..white_knight_board.count_ones() {
            let index = utils::utils::pop_lsb(&mut white_knight_board);
            d.draw_texture(
                &white_knight,
                (OFFSET_FOR_NUMBERING - 6) + (index as i32%8_i32) * SQUARE_SIZE,
                -5 + (index as i32 / 8) * SQUARE_SIZE,
                Color::WHITE
            )
        }

        let mut white_bishop_board = used_boards[2];
        for _ in 0..white_bishop_board.count_ones() {
            let index = utils::utils::pop_lsb(&mut white_bishop_board);
            d.draw_texture(
                &white_bishop,
                (OFFSET_FOR_NUMBERING - 6) + (index as i32%8_i32) * SQUARE_SIZE,
                -5 + (index as i32 / 8) * SQUARE_SIZE,
                Color::WHITE
            )
        }

        let mut white_rook_board = used_boards[3];
        for _ in 0..white_rook_board.count_ones() {
            let index = utils::utils::pop_lsb(&mut white_rook_board);
            d.draw_texture(
                &white_rook,
                (OFFSET_FOR_NUMBERING - 6) + (index as i32%8_i32) * SQUARE_SIZE,
                -5 + (index as i32 / 8) * SQUARE_SIZE,
                Color::WHITE
            )
        }

        let mut white_queen_board = used_boards[4];
        for _ in 0..white_queen_board.count_ones() {
            let index = utils::utils::pop_lsb(&mut white_queen_board);
            d.draw_texture(
                &white_queen,
                (OFFSET_FOR_NUMBERING - 6) + (index as i32%8_i32) * SQUARE_SIZE,
                -5 + (index as i32 / 8) * SQUARE_SIZE,
                Color::WHITE
            )
        }

        let mut white_king_board = used_boards[5];
        for _ in 0..white_king_board.count_ones() {
            let index = utils::utils::pop_lsb(&mut white_king_board);
            d.draw_texture(
                &white_king,
                (OFFSET_FOR_NUMBERING - 6) + (index as i32%8_i32) * SQUARE_SIZE,
                -5 + (index as i32 / 8) * SQUARE_SIZE,
                Color::WHITE
            )
        }


        let mut black_pawn_board = used_boards[6];
        for _ in 0..black_pawn_board.count_ones() {
            let index = utils::utils::pop_lsb(&mut black_pawn_board);
            d.draw_texture(
                &black_pawn,
                (OFFSET_FOR_NUMBERING - 6) + (index as i32%8_i32) * SQUARE_SIZE,
                -5 + (index as i32 / 8) * SQUARE_SIZE,
                Color::WHITE
            )
        }

        let mut black_knight_board = used_boards[7];
        for _ in 0..black_knight_board.count_ones() {
            let index = utils::utils::pop_lsb(&mut black_knight_board);
            d.draw_texture(
                &black_knight,
                (OFFSET_FOR_NUMBERING - 6) + (index as i32%8_i32) * SQUARE_SIZE,
                -5 + (index as i32 / 8) * SQUARE_SIZE,
                Color::WHITE
            )
        }

        let mut black_bishop_board = used_boards[8];
        for _ in 0..black_bishop_board.count_ones() {
            let index = utils::utils::pop_lsb(&mut black_bishop_board);
            d.draw_texture(
                &black_bishop,
                (OFFSET_FOR_NUMBERING - 6) + (index as i32%8_i32) * SQUARE_SIZE,
                -5 + (index as i32 / 8) * SQUARE_SIZE,
                Color::WHITE
            )
        }

        let mut black_rook_board = used_boards[9];
        for _ in 0..black_rook_board.count_ones() {
            let index = utils::utils::pop_lsb(&mut black_rook_board);
            d.draw_texture(
                &black_rook,
                (OFFSET_FOR_NUMBERING - 6) + (index as i32%8_i32) * SQUARE_SIZE,
                -5 + (index as i32 / 8) * SQUARE_SIZE,
                Color::WHITE
            )
        }

        let mut black_queen_board = used_boards[10];
        for _ in 0..black_queen_board.count_ones() {
            let index = utils::utils::pop_lsb(&mut black_queen_board);
            d.draw_texture(
                &black_queen,
                (OFFSET_FOR_NUMBERING - 6) + (index as i32%8_i32) * SQUARE_SIZE,
                -5 + (index as i32 / 8) * SQUARE_SIZE,
                Color::WHITE
            )
        }

        let mut black_king_board = used_boards[11];
        for _ in 0..black_king_board.count_ones() {
            let index = utils::utils::pop_lsb(&mut black_king_board);
            d.draw_texture(
                &black_king,
                (OFFSET_FOR_NUMBERING - 6) + (index as i32%8_i32) * SQUARE_SIZE,
                -5 + (index as i32 / 8) * SQUARE_SIZE,
                Color::WHITE
            )
        }

    }
}
