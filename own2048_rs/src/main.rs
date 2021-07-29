mod game;
mod solver;

use itertools::Itertools;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};

macro_rules! measure {
  ( $x:expr) => {{
    let start = Instant::now();
    let result = $x;
    let end = start.elapsed();
    println!(
      "{}.{:04} elaspled",
      end.as_secs(),
      end.subsec_nanos() / 1_000_000
    );
    result
  }};
}

fn show_result(res: &HashMap<u64, u32>) {
  print!("{}[2J", 27 as char);
  let mut sum = 0;
  for (_, v) in res {
    sum += v;
  }

  println!("Count:{} times", sum);
  for k in res.keys().sorted() {
    println!("{}: {}[%]", k, 100.0 * (res[k] as f64) / (sum as f64));
  }
}

fn main() {
  let mut g = game::Game { board: 0, score: 0 };
  g.new();

  g.spawn_tile();
  g.spawn_tile();
  g.show_matrix();

  let mut res = HashMap::new();

  // play game
  loop {
    // g.move_to(game::Direction::random_dir()); //random move
    let mut s = solver::Solver {
      game: g,
      max_value: 0,
    };

    let start = Instant::now(); //start timer

    // g.move_to(s.next_dir(2)); //dfs move
    g.move_to(s.next_dir2(100)); //random search move
    g.spawn_tile();

    let end = start.elapsed(); // end timer

    // print!("{}[2J", 27 as char);
    // println!(
    //   "{}.{:04} elaspled",
    //   end.as_secs(),
    //   end.subsec_nanos() / 1_000_000
    // );
    // g.show_matrix();

    if g.count_empty() == 0 && g.is_end() {
      let max_val = g.get_max_value();
      if res.contains_key(&max_val) {
        *res.get_mut(&max_val).unwrap() += 1;
      } else {
        res.insert(max_val, 1);
      }
    }
    show_result(&res);
  }

  // println!("board: {:#16X}", g.board);
  // println!("score: {}", g.score);
}
