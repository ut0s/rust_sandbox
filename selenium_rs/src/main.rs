use rand::Rng;
use std::slice::Iter;
use std::time::Instant;
use thirtyfour_sync::prelude::*;
// use thirtyfour::prelude::*;
// use tokio;
use std::collections::HashMap;

const HEADLESS_MODE: bool = true;
const TILE_SIZE: usize = 4;
// const DEPTH_COUNT: u32 = 1;

const WEIGHT: [[i32; 4]; 4] = [
  [0, 1, 2 * 450, 8 * 650],
  [0, 1, 2 * 500, 8 * 700],
  [0, 1, 2 * 550, 8 * 750],
  [0, 1, 2 * 600, 8 * 800],
];

#[derive(Clone)]
struct Tile {
  pub number: u64,
  // pub state: TileState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
  Right,
  Left,
  Top,
  Bottom,
}

impl Direction {
  pub fn iterator() -> Iter<'static, Direction> {
    static DIRECTIONS: [Direction; 4] = [
      Direction::Right,
      Direction::Left,
      Direction::Top,
      Direction::Bottom,
    ];
    DIRECTIONS.iter()
  }
}

fn get_grid_info(tiles: &mut Vec<Vec<u64>>, line: String) {
  let tmp: Vec<&str> = line.split(" ").collect();
  // num
  let tmp_num: Vec<&str> = tmp[1].split("-").collect();
  let num: u64 = tmp_num[1].parse().unwrap();

  // position
  let tmp_pos: Vec<&str> = tmp[2].split("-").collect();
  let x_pos: usize = tmp_pos[2].parse().unwrap();
  let y_pos: usize = tmp_pos[3].parse().unwrap();

  // if tmp.len() == 4 {
  //   // state
  //   let tmp_state: Vec<&str> = tmp[3].split("-").collect();
  //   println!("{}", tmp_state[1]);
  // }

  // println!("tile num:{}", num);
  // println!("x:{},y:{}", x_pos, y_pos);
  tiles[y_pos - 1][x_pos - 1] = num;
}

fn predict(tiles: &Vec<Vec<u64>>) -> Direction {
  let mut ans = -1;
  let mut ret = Direction::Top;
  for dir in Direction::iterator() {
    let mut tiles_moved = tiles.clone();
    next_tiles(&mut tiles_moved, *dir);
    // add_random_tile(&mut tiles_moved);
    let tmp = eval_tiles(&tiles, &tiles_move4);

    if ans < tmp {
      ans = tmp;
      ret = *dir;
    }
  }
  // not update tiles
  if ans == -1 {
    ret = match rand::thread_rng().gen_range(0..4) {
      0 => Direction::Top,
      1 => Direction::Right,
      2 => Direction::Left,
      _ => Direction::Bottom,
    };
  }

  ret
}

fn eval_tiles(tiles_org: &Vec<Vec<u64>>, tiles_moved: &Vec<Vec<u64>>) -> i32 {
  if tiles_org == tiles_moved {
    -1
  } else {
    let mut ret: i32 = 0;

    for i in 0..TILE_SIZE {
      for j in 0..TILE_SIZE {
        if tiles_moved[i][j] == 0 {
          // empty tile
          ret += 100;
        }
        // weighted point
        ret += WEIGHT[i][j] * tiles_moved[i][j] as i32;

        if i > 1 && j != TILE_SIZE - 1 && tiles_moved[i][j] < tiles_moved[i][j + 1] {
          ret += 1000;
        }
        if i > 1 && j != TILE_SIZE - 1 && tiles_moved[i][j] == tiles_moved[i][j + 1] {
          ret += 5000;
        }
        if (i == 1 || i == 2) && j != TILE_SIZE - 1 && tiles_moved[i][j] < tiles_moved[i][j + 1] {
          ret += 5000;
        }
      }
    }
    ret
  }
}

fn add_random_tile(tiles: &mut Vec<Vec<u64>>) {
  let mut cells_available: Vec<(usize, usize)> = Vec::new();
  for i in 0..tiles.len() {
    for j in 0..tiles[i].len() {
      if tiles[i][j] == 0 {
        cells_available.push((i, j));
      }
    }
  }

  let (i, j) = cells_available[rand::thread_rng().gen_range(0..cells_available.len())];
  let prob: f32 = rand::thread_rng().gen_range(0.0..1.0f32);
  if prob < 0.9 {
    tiles[i][j] = 2;
  } else {
    tiles[i][j] = 4;
  }
}

fn next_tiles(tiles: &mut Vec<Vec<u64>>, dir: Direction) {
  let rot_count: u32 = match dir {
    Direction::Right => 2,
    Direction::Left => 0,
    Direction::Top => 3,
    Direction::Bottom => 1,
  };
  rotate(tiles, rot_count);
  tiles_to_left(tiles);
  rotate(tiles, (4 - rot_count) % 4);
}

fn rotate(tiles: &mut Vec<Vec<u64>>, rot_count: u32) {
  for _ in 0..rot_count {
    tiles.reverse();
    for i in 1..tiles.len() {
      let (left, right) = tiles.split_at_mut(i);
      for j in 0..i {
        std::mem::swap(&mut left[j][i], &mut right[0][j]);
      }
    }
  }
}

fn tiles_to_left(tiles: &mut Vec<Vec<u64>>) {
  for row in 0..TILE_SIZE {
    line_to_left(&mut tiles[row]);
  }
}

fn line_to_left(l: &mut Vec<u64>) {
  // remove zero
  l.retain(|x| *x != 0);
  for _ in 0..(TILE_SIZE - l.len()) {
    l.push(0);
  }

  if l[0] == l[1] {
    l.remove(0);
    l[0] *= 2;
    l.push(0);
  }
  if l[1] == l[2] {
    l.remove(1);
    l[1] *= 2;
    l.push(0);
  }
  if l[2] == l[3] {
    l.remove(2);
    l[2] *= 2;
    l.push(0);
  }
}

fn show_res(res: &HashMap<i32, u32>) {
  print!("{}[2J", 27 as char);
  let mut sum = 0;
  for (k, v) in res {
    sum += v;
  }

  println!("Count:{} times", sum);
  for (k, v) in res {
    println!("{}: {}[%]", k, 100.0 * (*v as f64) / (sum as f64));
  }
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test00_line_to_left() {
    let mut vec = vec![0, 0, 0, 0];
    line_to_left(&mut vec);
    assert_eq!(vec, [0, 0, 0, 0]);
  }
  #[test]
  fn test01_line_to_left() {
    let mut vec = vec![2, 2, 0, 0];
    line_to_left(&mut vec);
    assert_eq!(vec, [4, 0, 0, 0]);
  }
  #[test]
  fn test02_line_to_left() {
    let mut vec = vec![0, 2, 2, 0];
    line_to_left(&mut vec);
    assert_eq!(vec, [4, 0, 0, 0]);
  }
  #[test]
  fn test03_line_to_left() {
    let mut vec = vec![2, 0, 2, 0];
    line_to_left(&mut vec);
    assert_eq!(vec, [4, 0, 0, 0]);
  }
  #[test]
  fn test04_line_to_left() {
    let mut vec = vec![0, 0, 2, 2];
    line_to_left(&mut vec);
    assert_eq!(vec, [4, 0, 0, 0]);
  }
  #[test]
  fn test05_line_to_left() {
    let mut vec = vec![2, 4, 8, 8];
    line_to_left(&mut vec);
    assert_eq!(vec, [2, 4, 16, 0]);
  }

  #[test]
  fn test00_rotate() {
    let mut vec2d = vec![
      vec![0, 1, 0, 0],
      vec![0, 1, 0, 0],
      vec![0, 1, 1, 1],
      vec![0, 0, 0, 0],
    ];
    rotate(&mut vec2d, 0);
    #[rustfmt::skip]
    assert_eq!(
      vec2d,
      [[0, 1, 0, 0],
       [0, 1, 0, 0],
       [0, 1, 1, 1],
       [0, 0, 0, 0]]
    );
  }
  #[test]
  fn test01_rotate() {
    let mut vec2d = vec![
      vec![0, 1, 0, 0],
      vec![0, 1, 0, 0],
      vec![0, 1, 1, 1],
      vec![0, 0, 0, 0],
    ];
    rotate(&mut vec2d, 1);
    #[rustfmt::skip]
    assert_eq!(
      vec2d,
      [[0, 0, 0, 0],
       [0, 1, 1, 1],
       [0, 1, 0, 0],
       [0, 1, 0, 0]]
    );
  }
  #[test]
  fn test02_rotate() {
    let mut vec2d = vec![
      vec![0, 1, 0, 0],
      vec![0, 1, 0, 0],
      vec![0, 1, 1, 1],
      vec![0, 0, 0, 0],
    ];
    rotate(&mut vec2d, 4);
    #[rustfmt::skip]
    assert_eq!(
      vec2d,
      [[0, 1, 0, 0],
       [0, 1, 0, 0],
       [0, 1, 1, 1],
       [0, 0, 0, 0]]
    );
  }

  #[test]
  fn test00_next_tiles() {
    let mut vec2d = vec![
      vec![0, 2, 0, 0],
      vec![0, 2, 0, 0],
      vec![0, 0, 2, 2],
      vec![0, 0, 0, 0],
    ];
    next_tiles(&mut vec2d, Direction::Left);
    #[rustfmt::skip]
    assert_eq!(
      vec2d,
      [[2, 0, 0, 0],
       [2, 0, 0, 0],
       [4, 0, 0, 0],
       [0, 0, 0, 0]]
    );
  }
  #[test]
  fn test01_next_tiles() {
    let mut vec2d = vec![
      vec![0, 2, 0, 0],
      vec![0, 2, 0, 0],
      vec![0, 0, 2, 2],
      vec![0, 0, 0, 0],
    ];
    next_tiles(&mut vec2d, Direction::Right);
    #[rustfmt::skip]
    assert_eq!(
      vec2d,
      [[0, 0, 0, 2],
       [0, 0, 0, 2],
       [0, 0, 0, 4],
       [0, 0, 0, 0]]
    );
  }
  #[test]
  fn test02_next_tiles() {
    let mut vec2d = vec![
      vec![0, 2, 0, 0],
      vec![0, 2, 0, 0],
      vec![0, 0, 2, 2],
      vec![0, 0, 0, 0],
    ];
    next_tiles(&mut vec2d, Direction::Top);
    #[rustfmt::skip]
    assert_eq!(
      vec2d,
      [[0, 4, 2, 2],
       [0, 0, 0, 0],
       [0, 0, 0, 0],
       [0, 0, 0, 0]]
    );
  }
  #[test]
  fn test03_next_tiles() {
    let mut vec2d = vec![
      vec![0, 2, 0, 0],
      vec![0, 2, 0, 0],
      vec![0, 0, 2, 2],
      vec![0, 0, 0, 0],
    ];
    next_tiles(&mut vec2d, Direction::Bottom);
    #[rustfmt::skip]
    assert_eq!(
      vec2d,
      [[0, 0, 0, 0],
       [0, 0, 0, 0],
       [0, 0, 0, 0],
       [0, 4, 2, 2]]
    );
  }

  #[test]
  fn test00_eval_tiles() {
    let vec2d = vec![
      vec![0, 2, 0, 0],
      vec![0, 2, 0, 0],
      vec![0, 0, 2, 2],
      vec![0, 0, 0, 0],
    ];
    assert_eq!(eval_tiles(&vec2d), 4);
  }

  #[test]
  fn test00_predict() {
    let vec2d = vec![
      vec![0, 2, 0, 0],
      vec![0, 2, 0, 0],
      vec![0, 0, 4, 2],
      vec![0, 0, 0, 0],
    ];
    assert_eq!(predict(&vec2d), Direction::Top);
  }

  #[test]
  fn test01_predict() {
    let vec2d = vec![
      vec![0, 2, 0, 0],
      vec![0, 0, 0, 0],
      vec![0, 0, 4, 2],
      vec![0, 0, 0, 0],
    ];
    assert_eq!(predict(&vec2d), Direction::Right);
  }
}

fn worker(is_headless: bool, res: &mut HashMap<i32, u32>) -> WebDriverResult<()> {
  //start timer
  let start = Instant::now();

  let mut caps = DesiredCapabilities::chrome();
  if is_headless {
    let _ = caps.set_headless();
  }
  let driver = WebDriver::new("http://localhost:4444", &caps)?;

  driver.get("https://play2048.co")?;
  let html_body = driver.find_element(By::Tag("body"))?;

  // restart
  // html_body.send_keys('r')?;

  let grid = driver.find_element(By::ClassName("tile-container"))?;
  std::thread::sleep(std::time::Duration::from_millis(500));

  let mut check_stuck = 0;
  let mut old_tiles = vec![vec![0; TILE_SIZE]; TILE_SIZE];
  for step in 0..1000 {
    // println!("\nStep:{}", step);

    // init tile
    let mut tiles = vec![vec![0; TILE_SIZE]; TILE_SIZE];

    let tile_list = grid.find_elements(By::ClassName("tile"))?;
    // std::thread::sleep(std::time::Duration::from_millis(50));
    for tile in tile_list {
      let oneline = tile.class_name()?.unwrap();
      // println!("{:?}", oneline);
      get_grid_info(&mut tiles, oneline);
    }
    if tiles == old_tiles {
      check_stuck += 1;
      if check_stuck > 10 {
        // for i in 0..tiles.len() {
        //   println!("{:?}", tiles[i]);
        // }
        break;
      }
    } else {
      check_stuck = 0;
    }

    let key: Direction = predict(&tiles);
    let _ = match key {
      Direction::Right => html_body.send_keys(Keys::Right),
      Direction::Left => html_body.send_keys(Keys::Left),
      Direction::Top => html_body.send_keys(Keys::Up),
      Direction::Bottom => html_body.send_keys(Keys::Down),
    };
    std::thread::sleep(std::time::Duration::from_millis(15));
    old_tiles = tiles.clone();

    for i in 0..tiles.len() {
      println!("{:?}", tiles[i]);
    }
    println!("{:?}", key);
    // println!("Stuck Count:{}", check_stuck);
  }

  //end timer
  let end = start.elapsed();
  // println!(
  //   "Send keys :{}.{:03}[s]",
  //   end.as_secs(),
  //   end.subsec_nanos() / 1_000_000
  // );

  //start timer
  let start = Instant::now();

  let mut tiles_vec = Vec::new();
  let grid = driver.find_element(By::ClassName("tile-container"))?;
  for tile in grid.find_elements(By::ClassName("tile"))? {
    // println!("{:?}", tile.class_name()?);
    // println!("{}", tile.text()?);
    // tiles_vec.push(tile.text()?);
    let tmp_num: i32 = tile.text()?.parse().unwrap();
    tiles_vec.push(tmp_num);
  }
  //end timer
  let end = start.elapsed();
  // println!(
  //   "Get tile info :{}.{:03}[s]",
  //   end.as_secs(),
  //   end.subsec_nanos() / 1_000_000
  // );

  tiles_vec.sort_unstable();
  let max_tile_num = tiles_vec[tiles_vec.len() - 1];
  // println!("{:?}", tiles_vec);
  // println!("max num of tile: {}", max_tile_num);

  if res.contains_key(&max_tile_num) {
    *res.get_mut(&max_tile_num).unwrap() += 1;
  } else {
    res.insert(max_tile_num, 1);
  }

  // let score = driver.find_element(By::ClassName("score-container"))?;
  // println!("{}", score.text()?);

  if max_tile_num > 1000 {
    let _ = driver.screenshot_as_png();
  }

  Ok(())
}

fn main() {
  let mut res = HashMap::new();
  loop {
    let _1 = worker(HEADLESS_MODE, &mut res);
    show_res(&res);
  }
}
