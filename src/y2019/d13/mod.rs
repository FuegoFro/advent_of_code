use std::cmp::Ordering;

use itertools::Either;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::util::point::{get_bounding_box, Point};
use crate::y2019::computer::{Computer, ComputerExitStatus, Word};

#[derive(FromPrimitive, Clone, Eq, PartialEq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl Tile {
    fn print_self(&self) {
        let char = match self {
            Tile::Empty => " ",
            Tile::Wall => "#",
            Tile::Block => "@",
            Tile::Paddle => "-",
            Tile::Ball => "o",
        };
        print!("{}", char);
    }
}

struct DrawInstruction {
    coordinate: Point,
    value: Either<Tile, Word>,
}

fn get_draw_instructions(computer: &mut Computer) -> Vec<DrawInstruction> {
    let mut draw_instructions = Vec::new();
    let outputs = computer.drain_outputs();
    let mut outputs = outputs.as_slice();
    while !outputs.is_empty() {
        assert!(outputs.len() >= 3);
        let (next, rest) = outputs.split_at(3);
        let coordinate = Point::new(next[0] as i32, next[1] as i32);
        let value = if coordinate == SCORE_COORDINATE {
            Either::Right(next[2])
        } else {
            Either::Left(Tile::from_i64(next[2]).unwrap())
        };
        draw_instructions.push(DrawInstruction { coordinate, value });
        outputs = rest;
    }
    draw_instructions
}

const SCORE_COORDINATE: Point = Point::new(-1, 0);

fn render_screen(computer: &mut Computer, screen: &mut Vec<Vec<Tile>>, do_print: bool) {
    let mut score = 0;
    for instruction in get_draw_instructions(computer).into_iter() {
        match instruction.value {
            Either::Left(tile) => {
                screen[instruction.coordinate.y as usize][instruction.coordinate.x as usize] = tile;
            }
            Either::Right(new_score) => score = new_score,
        }
    }

    if do_print {
        println!("SCORE = {}", score);
        for row in screen.iter() {
            for tile in row.iter() {
                tile.print_self();
            }
            println!();
        }
    }
}

fn get_move(screen: &Vec<Vec<Tile>>) -> Word {
    let mut ball_coords: Option<Point> = None;
    let mut paddle_coords: Option<Point> = None;
    for (y, row) in screen.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            match tile {
                Tile::Ball => {
                    assert!(ball_coords.is_none());
                    ball_coords = Some(Point::new(x as i32, y as i32));
                }
                Tile::Paddle => {
                    assert!(paddle_coords.is_none());
                    paddle_coords = Some(Point::new(x as i32, y as i32));
                }
                _ => {}
            }
        }
    }
    let ball_coords = ball_coords.unwrap();
    let paddle_coords = paddle_coords.unwrap();
    match ball_coords.x.cmp(&paddle_coords.x) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

pub fn main() {
    let input = include_str!("actual_input.txt").trim();
    let mut computer = Computer::from_packed(input);
    computer.run().assert_finished();

    let draw_instructions = get_draw_instructions(&mut computer);
    let num_blocks = draw_instructions
        .iter()
        .filter(|di| di.value == Either::Left(Tile::Block))
        .count();
    println!("{}", num_blocks);
    let (origin, outer_edge) = get_bounding_box(draw_instructions.iter().map(|di| &di.coordinate));
    assert_eq!(origin, Point::new(0, 0));
    let outer_edge = outer_edge + Point::new(1, 1);
    let mut screen = vec![vec![Tile::Empty; outer_edge.x as usize]; outer_edge.y as usize];
    println!("{:?}", (origin, outer_edge));

    let mut computer = Computer::from_packed(input);
    computer.write_memory(0, 2);
    while let ComputerExitStatus::WaitingForInput = computer.run() {
        render_screen(&mut computer, &mut screen, false);
        let game_move = get_move(&screen);
        computer.send_as_input(game_move);
    }
    render_screen(&mut computer, &mut screen, true);
}
