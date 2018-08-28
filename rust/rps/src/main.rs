use std::os::raw::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Move {
    m_me: c_int,
    m_you: c_int,
}

impl From<RMove> for Move {
    fn from(m: RMove) -> Self {
        let RMove{me, you} = m;
        Move {
            m_me: me as c_int,
            m_you: you as c_int,
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
enum Play {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl From<i32> for Play {
    fn from(i: i32) -> Self {
        match i {
            0 => Rock,
            1 => Paper,
            2 => Scissors,
            _ => unimplemented!(),
        }
    }
}

impl Play {
    fn beats(&self) -> Play {
        match *self {
            Paper => Rock,
            Scissors => Paper,
            Rock => Scissors,
        }
    }
    fn beaten(&self) -> Play {
        match *self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct RMove {
    me: Play,
    you: Play
}

impl From<Move> for RMove {
    fn from(m: Move) -> Self {
        let Move{m_me, m_you} = m;
        RMove {
            me: m_me.into(),
            you: m_you.into(),
        }
    }
}

use self::Play::*;

#[no_mangle]
pub extern fn c_move(move_history: *const Move, num_elems: usize) -> c_int {
    let mut moves = Vec::with_capacity(num_elems);

    for i in 0..num_elems as isize {
        unsafe {
            moves.push(RMove::from(*move_history.offset(i).as_ref().unwrap()));
        }
    }

    moves.last().map(|m| m.you.beaten()).unwrap_or(Scissors) as i32
}

fn main() {
    println!("Hello, world!");
}
