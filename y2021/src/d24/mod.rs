use recap::Recap;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Index, IndexMut, Mul, Rem};

#[derive(Copy, Clone)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

impl<T> From<T> for Variable
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        match s.as_ref() {
            "w" => Variable::W,
            "x" => Variable::X,
            "y" => Variable::Y,
            "z" => Variable::Z,
            _ => panic!("Unknown variable {}", s.as_ref()),
        }
    }
}

enum Argument {
    Var(Variable),
    Num(i32),
}

impl<T> From<Option<T>> for Argument
where
    T: AsRef<str>,
{
    fn from(s: Option<T>) -> Self {
        let unwrapped = s.expect("Expected a non-None value");
        let s = unwrapped.as_ref();
        match s {
            "w" | "x" | "y" | "z" => Argument::Var(s.into()),
            _ => Argument::Num(
                s.parse()
                    .unwrap_or_else(|_| panic!("Expected variable or integer literal, got {}", s)),
            ),
        }
    }
}

#[derive(Deserialize, Recap)]
#[recap(regex = r"(?P<op>...) (?P<left>[^ ]+)( (?P<right>.+))?")]
struct InstructionRaw {
    op: String,
    left: String,
    right: Option<String>,
}

enum Instruction {
    Inp(Variable),
    Add(Variable, Argument),
    Mul(Variable, Argument),
    Div(Variable, Argument),
    Mod(Variable, Argument),
    Eql(Variable, Argument),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let raw: InstructionRaw = s.parse().unwrap();
        match raw.op.as_str() {
            "inp" => Instruction::Inp(raw.left.into()),
            "add" => Instruction::Add(raw.left.into(), raw.right.into()),
            "mul" => Instruction::Mul(raw.left.into(), raw.right.into()),
            "div" => Instruction::Div(raw.left.into(), raw.right.into()),
            "mod" => Instruction::Mod(raw.left.into(), raw.right.into()),
            "eql" => Instruction::Eql(raw.left.into(), raw.right.into()),
            _ => panic!("Unknown op {}", raw.op),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Expression {
    Num(i32),
    Input(u32),
    Rollup(usize),
    Operation(Box<Expression>, String, Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Num(num) => f.write_fmt(format_args!("{}", num)),
            Expression::Input(input_idx) => f.write_fmt(format_args!("i{}", input_idx)),
            Expression::Rollup(rollup_idx) => f.write_fmt(format_args!("r{}", rollup_idx)),
            Expression::Operation(left, op, right) => {
                f.write_fmt(format_args!("({} {} {})", left, op, right))
            }
        }
    }
}

const ZERO: Expression = Expression::Num(0);
const ONE: Expression = Expression::Num(1);

impl Expression {
    fn new_op(
        left: &Expression,
        op_str: &str,
        op_fn: impl FnOnce(i32, i32) -> i32,
        right: &Expression,
    ) -> Expression {
        if let Expression::Num(left) = left {
            if let Expression::Num(right) = right {
                return Expression::Num(op_fn(*left, *right));
            }
        }
        Expression::Operation(left.clone().into(), op_str.into(), right.clone().into())
    }
}

struct State {
    w: Expression,
    x: Expression,
    y: Expression,
    z: Expression,
    rollups: Vec<Expression>,
    next_input: u32,
}

impl Index<Variable> for State {
    type Output = Expression;

    fn index(&self, index: Variable) -> &Self::Output {
        match index {
            Variable::W => &self.w,
            Variable::X => &self.x,
            Variable::Y => &self.y,
            Variable::Z => &self.z,
        }
    }
}

impl IndexMut<Variable> for State {
    fn index_mut(&mut self, index: Variable) -> &mut Self::Output {
        match index {
            Variable::W => &mut self.w,
            Variable::X => &mut self.x,
            Variable::Y => &mut self.y,
            Variable::Z => &mut self.z,
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            w: ZERO.clone(),
            x: ZERO.clone(),
            y: ZERO.clone(),
            z: ZERO.clone(),
            rollups: Vec::new(),
            next_input: 0,
        }
    }

    fn resolve(&self, arg: Argument) -> Expression {
        match arg {
            Argument::Var(var) => self[var].clone(),
            Argument::Num(num) => Expression::Num(num),
        }
    }

    fn save_rollup(&mut self, var: Variable) {
        if let Expression::Operation(_, _, _) = self[var] {
            let mut replacement = Expression::Rollup(self.rollups.len());
            std::mem::swap(&mut replacement, &mut self[var]);
            self.rollups.push(replacement);
        }
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Inp(var) => {
                self.save_rollup(Variable::W);
                self.save_rollup(Variable::X);
                self.save_rollup(Variable::Y);
                self.save_rollup(Variable::Z);
                self[var] = Expression::Input(self.next_input);
                self.next_input += 1;
            }
            Instruction::Add(var, arg) => {
                let resolved = self.resolve(arg);
                if self[var] == ZERO {
                    self[var] = resolved;
                } else if resolved != ZERO {
                    self[var] = Expression::new_op(&self[var], "+", i32::add, &resolved);
                }
            }
            Instruction::Mul(var, arg) => {
                let resolved = self.resolve(arg);
                if self[var] == ZERO || resolved == ZERO {
                    self[var] = ZERO.clone();
                } else if self[var] == ONE {
                    self[var] = resolved;
                } else if resolved != ONE {
                    self[var] = Expression::new_op(&self[var], "*", i32::mul, &resolved);
                }
            }
            Instruction::Div(var, arg) => {
                let resolved = self.resolve(arg);
                if self[var] == ZERO {
                } else if resolved == ZERO {
                    panic!("Divide by zero");
                } else if resolved != ONE {
                    self[var] = Expression::new_op(&self[var], "/", i32::div, &resolved);
                }
            }
            Instruction::Mod(var, arg) => {
                let resolved = self.resolve(arg);
                if self[var] == ZERO {
                } else if resolved == ZERO {
                    panic!("Mod by zero");
                } else if resolved != ONE {
                    self[var] = Expression::new_op(&self[var], "%", i32::rem, &resolved)
                }
            }
            Instruction::Eql(var, arg) => {
                let resolved = self.resolve(arg);
                if self[var] == resolved {
                    self[var] = ONE.clone();
                } else {
                    self[var] =
                        Expression::new_op(&self[var], "==", |l, r| i32::from(l == r), &resolved)
                }
            }
        }
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let _ = input;
    // symbolically_execute_program(input);

    // is_valid(9, 9, 1, 9, 6, 9, 9, 7, 9, 8, 5, 9, 4, 2);
    println!("Part 1: {}", try_find_largest_number());
    println!("Part 2: {}", try_find_smallest_number());
}

#[allow(dead_code)]
fn symbolically_execute_program(input: String) {
    let mut state = State::new();
    for instr_str in input.split('\n') {
        println!("Running instr '{}'", instr_str);
        state.run_instruction(Instruction::from_str(instr_str));
        // println!("w={}", state.w);
        // println!("x={}", state.x);
        // println!("y={}", state.y);
        // println!("z={}", state.z);
    }
    println!("z={}", state.z);
    for (idx, r) in state.rollups.iter().enumerate() {
        println!("r{}={}", idx, r);
    }
}

fn try_find_largest_number() -> String {
    for i0 in (1..=9).rev() {
        for i1 in (1..=9).rev() {
            for i2 in (1..=9).rev() {
                for i3 in (1..=9).rev() {
                    for i4 in (1..=9).rev() {
                        println!("{}{},{}{}{},000,000,000", i0, i1, i2, i3, i4);
                        for i5 in (1..=9).rev() {
                            for i6 in (1..=9).rev() {
                                for i7 in (1..=9).rev() {
                                    for i8 in (1..=9).rev() {
                                        for i9 in (1..=9).rev() {
                                            for i10 in (1..=9).rev() {
                                                for i11 in (1..=9).rev() {
                                                    for i12 in (1..=9).rev() {
                                                        for i13 in (1..=9).rev() {
                                                            if is_valid(
                                                                i0, i1, i2, i3, i4, i5, i6, i7, i8,
                                                                i9, i10, i11, i12, i13,
                                                            ) {
                                                                return format!(
                                                                    "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                                                                    i0,
                                                                    i1,
                                                                    i2,
                                                                    i3,
                                                                    i4,
                                                                    i5,
                                                                    i6,
                                                                    i7,
                                                                    i8,
                                                                    i9,
                                                                    i10,
                                                                    i11,
                                                                    i12,
                                                                    i13,
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("Unable to find valid number");
}

fn try_find_smallest_number() -> String {
    // Not below 21,111,000,000,000
    // Checked fully from 8xxx to 81,775,000,000,000

    for i0 in 8..=9 {
        for i1 in 1..=9 {
            for i2 in 1..=9 {
                for i3 in 1..=9 {
                    println!("{}{},{}{}0,000,000,000", i0, i1, i2, i3);
                    for i4 in 1..=9 {
                        for i5 in 1..=9 {
                            for i6 in 1..=9 {
                                for i7 in 1..=9 {
                                    for i8 in 1..=9 {
                                        for i9 in 1..=9 {
                                            for i10 in 1..=9 {
                                                for i11 in 1..=9 {
                                                    for i12 in 1..=9 {
                                                        // for i13 in 1..=9 {
                                                        let i13 = 1;
                                                        if is_valid(
                                                            i0, i1, i2, i3, i4, i5, i6, i7, i8, i9,
                                                            i10, i11, i12, i13,
                                                        ) {
                                                            return format!(
                                                                "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                                                                i0,
                                                                i1,
                                                                i2,
                                                                i3,
                                                                i4,
                                                                i5,
                                                                i6,
                                                                i7,
                                                                i8,
                                                                i9,
                                                                i10,
                                                                i11,
                                                                i12,
                                                                i13,
                                                            );
                                                            // }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("Unable to find valid number");
}

fn neq(left: i32, right: i32) -> i32 {
    i32::from(left != right)
}

#[allow(clippy::too_many_arguments)]
fn is_valid(
    i0: i32,
    i1: i32,
    i2: i32,
    i3: i32,
    i4: i32,
    i5: i32,
    i6: i32,
    i7: i32,
    i8: i32,
    i9: i32,
    i10: i32,
    i11: i32,
    i12: i32,
    i13: i32,
) -> bool {
    let r2 = (i0 + 1) * neq(12, i0);
    // dbg!(r2);
    let r5v = neq((r2 % 26) + 12, i1);
    // dbg!(r5v);
    let r5 = (r2 * ((25 * r5v) + 1)) + ((i1 + 1) * r5v);
    // dbg!(r5);
    let r8v = neq((r5 % 26) + 15, i2);
    // dbg!(r8v);
    let r8 = (r5 * ((25 * r8v) + 1)) + ((i2 + 16) * r8v);
    // dbg!(r8);
    let r11v = neq((r8 % 26) + -8, i3);
    // dbg!(r11v);
    let r11 = ((r8 / 26) * ((25 * r11v) + 1)) + ((i3 + 5) * r11v);
    // dbg!(r11);
    let r14v = neq((r11 % 26) + -4, i4);
    // dbg!(r14v);
    let r14 = ((r11 / 26) * ((25 * r14v) + 1)) + ((i4 + 9) * r14v);
    // dbg!(r14);
    let r17v = neq((r14 % 26) + 15, i5);
    // dbg!(r17v);
    let r17 = (r14 * ((25 * r17v) + 1)) + ((i5 + 3) * r17v);
    // dbg!(r17);
    let r20v = neq((r17 % 26) + 14, i6);
    // dbg!(r20v);
    let r20 = (r17 * ((25 * r20v) + 1)) + ((i6 + 2) * r20v);
    // dbg!(r20);
    let r23v = neq((r20 % 26) + 14, i7);
    // dbg!(r23v);
    let r23 = (r20 * ((25 * r23v) + 1)) + ((i7 + 15) * r23v);
    // dbg!(r23);
    let r26v = neq((r23 % 26) + -13, i8);
    // dbg!(r26v);
    let r26 = ((r23 / 26) * ((25 * r26v) + 1)) + ((i8 + 5) * r26v);
    // dbg!(r26);
    let r29v = neq((r26 % 26) + -3, i9);
    // dbg!(r29v);
    let r29 = ((r26 / 26) * ((25 * r29v) + 1)) + ((i9 + 11) * r29v);
    // dbg!(r29);
    let r32v = neq((r29 % 26) + -7, i10);
    // dbg!(r32v);
    let r32 = ((r29 / 26) * ((25 * r32v) + 1)) + ((i10 + 7) * r32v);
    // dbg!(r32);
    let r35v = neq((r32 % 26) + 10, i11);
    // dbg!(r35v);
    let r35 = (r32 * ((25 * r35v) + 1)) + ((i11 + 1) * r35v);
    // dbg!(r35);
    let r38v = neq((r35 % 26) + -6, i12);
    // dbg!(r38v);
    let r38 = ((r35 / 26) * ((25 * r38v) + 1)) + ((i12 + 10) * r38v);
    // dbg!(r38);
    let zv = neq((r38 % 26) + -8, i13);
    // dbg!(zv);
    let z = ((r38 / 26) * ((25 * zv) + 1)) + ((i13 + 3) * zv);
    // dbg!(z);
    z == 0
}

#[allow(dead_code)]
fn foo() {
    let r38 = 10;
    let i13 = 2;
    let v = neq((r38 % 26) + -8, i13);
    let left = (r38 / 26) * ((25 * v) + 1);
    let right = (i13 + 3) * v;
    let _ = left + right;

    // v == 0
    // 0 <= r38 <= 25
    // r38 == i13 + 8
    // r38 == [1,9] + 8
    // r38 == [9,17]

    // r2 = (i0 + 1)
    // r2 = [2, 10]

    // r2 == r38
    // r2, r38 == [9,10]

    // let v = neq((r35 % 26) + -6, i12);
    // let left = (r35 / 26) * ((25 * v) + 1);
    // let right = (i12 + 10) * v;
    // let a = left + right;
}

/*
inputs must be in [1, 9]

left + right = 0
left = -right
(r35 / 26) * (25v + 1) = -v(i12+10)
r35 * (25v + 1) / 26 = -v(i12+10)

# v=1

r35 * (25 + 1) / 26 = -(i12+10)
r35 = -i12 - 10
i12 = -(r35 + 10)
    -> r35 must be in [-19, -11]

r35 % 26 - 6 != i12
    -> r35 must be >= 0




# v=0 (can't happen)
r35 / 26 = 0
r35 = 0

r35 % 26 - 6 == i12
i12 = -6 (inputs must be 1-9 inclusive)

*/

/*

// 99196997985942

[src/y2021/d24/mod.rs:448] r2 = 10
[src/y2021/d24/mod.rs:450] r5v = 1
[src/y2021/d24/mod.rs:452] r5 = 270
[src/y2021/d24/mod.rs:454] r8v = 1
[src/y2021/d24/mod.rs:456] r8 = 7037
[src/y2021/d24/mod.rs:458] r11v = 0
[src/y2021/d24/mod.rs:460] r11 = 270
[src/y2021/d24/mod.rs:462] r14v = 0
[src/y2021/d24/mod.rs:464] r14 = 10
[src/y2021/d24/mod.rs:466] r17v = 1
[src/y2021/d24/mod.rs:468] r17 = 272
[src/y2021/d24/mod.rs:470] r20v = 1
[src/y2021/d24/mod.rs:472] r20 = 7083
[src/y2021/d24/mod.rs:474] r23v = 1
[src/y2021/d24/mod.rs:476] r23 = 184180
[src/y2021/d24/mod.rs:478] r26v = 0
[src/y2021/d24/mod.rs:480] r26 = 7083
[src/y2021/d24/mod.rs:482] r29v = 0
[src/y2021/d24/mod.rs:484] r29 = 272
[src/y2021/d24/mod.rs:486] r32v = 0
[src/y2021/d24/mod.rs:488] r32 = 10
[src/y2021/d24/mod.rs:490] r35v = 1
[src/y2021/d24/mod.rs:492] r35 = 270
[src/y2021/d24/mod.rs:494] r38v = 0
[src/y2021/d24/mod.rs:496] r38 = 10
[src/y2021/d24/mod.rs:498] zv = 0
[src/y2021/d24/mod.rs:500] z = 0

*/
