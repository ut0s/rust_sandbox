use rand::Rng;

pub enum Direction {
  Left,
  Right,
  Up,
  Down,
}

impl Direction {
  pub fn random_dir() -> Direction {
    match rand::thread_rng().gen_range(0..4) {
      0 => Direction::Left,
      1 => Direction::Right,
      2 => Direction::Up,
      3 => Direction::Down,
      _ => Direction::Down,
    }
  }
}

#[derive(Clone, Copy)]
pub struct Game {
  pub board: u64,
  pub score: u64,
}

impl Game {
  pub fn new(&mut self) {
    self.board = 0x0000_0000_0000_0000_u64;
    self.score = 0;
  }

  pub fn count_empty(&self) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..16 {
      if self.board & (0xF_u64 << i * 0x4) == 0 {
        ret += 1;
      }
    }
    ret
  }

  pub fn empty_tile_idx(&self) -> Vec<u32> {
    let mut vec = Vec::new();
    for i in 0..16 {
      if (self.board >> (0x4 * (16 - i - 1))) & 0xF_u64 == 0 {
        vec.push(i);
      }
    }
    vec
  }

  pub fn tile(self) -> u64 {
    match rand::thread_rng().gen_range(0..10) {
      0 => 2,
      _ => 1,
    }
  }

  pub fn spawn_tile(&mut self) {
    let ids = self.empty_tile_idx();
    if ids.len() != 0 {
      // println!("not empty");
      let i = rand::thread_rng().gen_range(0..ids.len());
      self.set_value(ids[i] as usize, self.tile());
    } else {
      println!("empty");
      self.board = 0x0000_0000_0000_0000_u64;
    }
  }

  pub fn get_value(&mut self, idx: usize) -> u64 {
    let tmp = self.board >> (0x4 * (16 - idx - 1));
    tmp & 0x0000_0000_0000_000F_u64
  }

  pub fn set_value(&mut self, idx: usize, num: u64) {
    let mut head = 0;
    if idx != 0 {
      head = self.board >> (0x4 * (16 - idx));
      head = head << (0x4 * (16 - idx));
      head = head & 0xFFFF_FFFF_FFFF_FFFF_u64;
    }
    // println!("head:\t{:16X}", head);
    let setter = num * 0x1000_0000_0000_0000_u64 >> (0x4 * (idx));
    // println!("setter:\t{:16X}", setter);
    let mut tail = 0;
    if idx != 15 {
      tail = self.board << (0x4 * (idx + 1));
      tail = tail & 0xFFFF_FFFF_FFFF_FFFF_u64;
      tail = tail >> (0x4 * (idx + 1));
    }
    // println!("tail:\t{:16X}", tail);
    // println!("goal:\t{:16X}", head | setter | tail);
    self.board = head | setter | tail;
  }

  pub fn transpose(&mut self) {
    let a1 = self.board & 0xF0F0_0F0F_F0F0_0F0F_u64;
    let a2 = self.board & 0x0000_F0F0_0000_F0F0_u64;
    let a3 = self.board & 0x0F0F_0000_0F0F_0000_u64;
    let a = a1 | (a2 << 0x4 * 3) | (a3 >> 0x4 * 3);
    let b1 = a & 0xFF00_FF00_00FF_00FF_u64;
    let b2 = a & 0x00FF_00FF_0000_0000_u64;
    let b3 = a & 0x0000_0000_FF00_FF00_u64;
    self.board = b1 | (b2 >> 0x4 * 6) | (b3 << 0x4 * 6);
  }

  pub fn rows(self) -> [u16; 4] {
    let row1 = ((self.board & 0xFFFF_0000_0000_0000) >> 0x4 * 12) as u16;
    let row2 = ((self.board & 0x0000_FFFF_0000_0000) >> 0x4 * 8) as u16;
    let row3 = ((self.board & 0x0000_0000_FFFF_0000) >> 0x4 * 4) as u16;
    let row4 = ((self.board & 0x0000_0000_0000_FFFF) >> 0x4 * 0) as u16;
    [row1, row2, row3, row4]
  }

  pub fn cols(&mut self) -> [u16; 4] {
    self.transpose();
    self.rows()
  }

  pub fn row2vec(self, row: u16) -> Vec<u16> {
    let v1 = (row & 0xF000) >> 0x4 * 3;
    let v2 = (row & 0x0F00) >> 0x4 * 2;
    let v3 = (row & 0x00F0) >> 0x4 * 1;
    let v4 = (row & 0x000F) >> 0x4 * 0;
    vec![v1, v2, v3, v4]
  }

  pub fn vec2row(self, vec: Vec<u16>) -> u64 {
    let mut ret = 0;
    for i in 0..4 {
      ret |= (0x0001 << 0x4 * (3 - i as u16)) * vec[i];
    }
    ret as u64
  }

  pub fn row2left(&mut self, mut l: Vec<u16>) -> u64 {
    // remove zero
    l.retain(|&x| x != 0);
    for _ in 0..(4 - l.len()) {
      l.push(0);
    }

    if l[0] == l[1] {
      l.remove(0);
      self.score += l[0] as u64;
      l[0] += 1;
      l.push(0);
    }
    if l[1] != 0 && l[1] == l[2] {
      l.remove(1);
      self.score += l[1] as u64;
      l[1] += 1;
      l.push(0);
    }
    if l[2] != 0 && l[2] == l[3] {
      l.remove(2);
      self.score += l[2] as u64;
      l[2] += 1;
      l.push(0);
    }

    self.vec2row(l)
  }

  pub fn rotate90deg(&mut self) {
    self.transpose();
    let mut c = 0;
    let mut ret = 0;
    for row in self.rows().iter() {
      let mut vec = self.row2vec(*row);
      vec.reverse();
      let tmp = self.vec2row(vec);
      ret |= tmp << (0x4 * 4 * (3 - c));
      c += 1;
    }
    self.board = ret;
  }

  pub fn into_left(&mut self) {
    let mut c = 0;
    let mut ret = 0;
    for row in self.rows().iter() {
      let tmp = self.row2left(self.row2vec(*row));
      ret |= tmp << (0x4 * 4 * (3 - c));
      // println!("{:16X}", ret);
      c += 1;
    }
    self.board = ret;
  }

  pub fn move_to(&mut self, dir: Direction) {
    let rot_count: u32 = match dir {
      Direction::Right => 2,
      Direction::Left => 0,
      Direction::Up => 3,
      Direction::Down => 1,
    };
    for _ in 0..rot_count {
      self.rotate90deg();
    }
    self.into_left();
    for _ in 0..((4 - rot_count) % 4) {
      self.rotate90deg();
    }
  }

  pub fn show_matrix(self) {
    for row in self.rows().iter() {
      println!("{:04X}", *row);
      // for e in self.row2vec(*row) {
      //   print!("{}\t", 0b1 << e);
      // }
    }
    println!("");
  }

  pub fn get_max_value(self) -> u64 {
    let mut ret = 0;
    for row in self.rows().iter() {
      for e in self.row2vec(*row) {
        if ret <= e {
          ret = e;
        };
      }
    }
    ret as u64
  }

  pub fn is_end(&mut self) -> bool {
    let board_org = self.board;
    if self.count_empty() != 0 {
      return false;
    }

    self.move_to(Direction::Down);
    self.move_to(Direction::Left);
    self.move_to(Direction::Right);
    self.move_to(Direction::Up);

    println!("self: {:16X}", self.board);
    println!("org:  {:16X}", board_org);

    if self.board == board_org {
      return true;
    } else {
      self.board = board_org;
      return false;
    }
  }
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test00_count_empty() {
    let g = Game {
      board: 0x0010_0100_0000_1000_u64,
      score: 0,
    };
    assert_eq!(13, g.count_empty());
  }

  #[test]
  fn test01_count_empty() {
    let g = Game {
      board: 0x0010_0100_0000_1000_u64,
      score: 0,
    };
    assert_eq!(13, g.count_empty());
  }

  #[test]
  fn test02_count_empty() {
    let g = Game {
      board: 0x1000_0000_0000_0001_u64,
      score: 0,
    };
    assert_eq!(14, g.count_empty());
  }

  #[test]
  fn test03_count_empty() {
    let g = Game {
      board: 0x0000_0000_0000_0000_u64,
      score: 0,
    };
    assert_eq!(16, g.count_empty());
  }

  #[test]
  fn test04_count_empty() {
    let g = Game {
      board: 0xFFFF_FFFF_FFFF_FFFF_u64,
      score: 0,
    };
    assert_eq!(0, g.count_empty());
  }

  #[test]
  fn test05_count_empty() {
    let g = Game {
      board: 0x9123_4567_89AB_CDEF_u64,
      score: 0,
    };
    assert_eq!(0, g.count_empty());
  }

  #[test]
  fn test00_empty_tile_idx() {
    let g = Game {
      board: 0x1000_0000_0000_0001_u64,
      score: 0,
    };
    assert_eq!(
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
      g.empty_tile_idx()
    );
  }

  #[test]
  fn test01_empty_tile_idx() {
    let g = Game {
      board: 0x0566_0100_0000_0001_u64,
      score: 0,
    };
    assert_eq!(
      vec![0, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14],
      g.empty_tile_idx()
    );
  }

  #[test]
  fn test02_empty_tile_idx() {
    let g = Game {
      board: 0x0566_3431_5555_6666_u64,
      score: 0,
    };
    assert_eq!(vec![0], g.empty_tile_idx());
  }

  #[test]
  fn test03_empty_tile_idx() {
    let g = Game {
      board: 0x1566_3431_5555_6666_u64,
      score: 0,
    };
    let mut vec = Vec::<u32>::new();
    assert_eq!(vec, g.empty_tile_idx());
  }

  #[test]
  fn test00_spawn_tile() {
    let mut g = Game {
      board: 0x0000_0000_0000_0000_u64,
      score: 0,
    };
    for _ in 0..16 {
      g.spawn_tile();
    }
    println!("{:16X}", g.board);
    g.spawn_tile();
    assert_eq!(0x0000_0000_0000_0000_u64, g.board);
  }

  #[test]
  fn test00_get_value() {
    let mut g = Game {
      board: 0x0123_4567_89AB_CDEF_u64,
      score: 0,
    };
    assert_eq!(0x0, g.get_value(0));
    assert_eq!(0x1, g.get_value(1));
    assert_eq!(0x2, g.get_value(2));
    assert_eq!(0x3, g.get_value(3));
    assert_eq!(0x4, g.get_value(4));
    assert_eq!(0x5, g.get_value(5));
    assert_eq!(0x6, g.get_value(6));
    assert_eq!(0x7, g.get_value(7));
    assert_eq!(0x8, g.get_value(8));
    assert_eq!(0x9, g.get_value(9));
    assert_eq!(0xA, g.get_value(10));
    assert_eq!(0xB, g.get_value(11));
    assert_eq!(0xC, g.get_value(12));
    assert_eq!(0xD, g.get_value(13));
    assert_eq!(0xE, g.get_value(14));
    assert_eq!(0xF, g.get_value(15));
  }

  #[test]
  fn test00_set_value() {
    let mut g = Game {
      board: 0x0123_4567_89AB_CDEF_u64,
      score: 0,
    };
    g.set_value(5, 0xA);
    g.set_value(10, 0x5);
    assert_eq!(0x0123_4A67_895B_CDEF_u64, g.board);
  }

  #[test]
  fn test01_set_value() {
    let mut g = Game {
      board: 0x0123_4567_89AB_CDEF_u64,
      score: 0,
    };
    g.set_value(0, 0xF);
    g.set_value(15, 0x0);
    assert_eq!(0xF123_4567_89AB_CDE0_u64, g.board);
  }

  #[test]
  fn test00_transpose() {
    let mut g = Game {
      board: 0x0123_4567_89AB_CDEF_u64,
      score: 0,
    };
    g.transpose();
    println!("{:16X}", g.board);
    assert_eq!(0x048C_159D_26AE_37BF_u64, g.board);
  }

  #[test]
  fn test00_rows() {
    let mut g = Game {
      board: 0x0123_4567_89AB_CDEF_u64,
      score: 0,
    };
    assert_eq!([0x0123, 0x4567, 0x89AB, 0xCDEF], g.rows());
  }

  #[test]
  fn test00_cols() {
    let mut g = Game {
      board: 0x0123_4567_89AB_CDEF_u64,
      score: 0,
    };
    assert_eq!([0x048C, 0x159D, 0x26AE, 0x37BF], g.cols());
  }

  #[test]
  fn test00_row2vec() {
    let mut g = Game {
      board: 0x0123_4567_89AB_CDEF_u64,
      score: 0,
    };
    let rows = g.rows();
    assert_eq!(vec![0, 1, 2, 3], g.row2vec(rows[0]));
  }

  #[test]
  fn test00_row2left() {
    let mut g = Game {
      board: 0x0123_4567_89AB_CDEF_u64,
      score: 0,
    };

    let tmp = g.row2left(vec![1, 0, 0, 1]);
    println!("{:16X}", tmp);
    assert_eq!(0x2000, tmp);
    assert_eq!(1, g.score);
  }

  #[test]
  fn test01_row2left() {
    let mut g = Game {
      board: 0x0123_4567_89AB_CDEF_u64,
      score: 0,
    };

    let tmp = g.row2left(vec![2, 0, 0, 1]);
    println!("{:16X}", tmp);
    assert_eq!(0x2100, tmp);
    assert_eq!(0, g.score);
  }

  #[test]
  fn test02_row2left() {
    let mut g = Game {
      board: 0x0123_4567_89AB_CDEF_u64,
      score: 0,
    };

    let tmp = g.row2left(vec![2, 2, 1, 1]);
    println!("{:16X}", tmp);
    assert_eq!(0x3200, tmp);
    assert_eq!(3, g.score);
  }

  #[test]
  fn test00_into_left() {
    let mut g = Game {
      board: 0x0101_0001_1100_2002_u64,
      score: 0,
    };
    g.into_left();
    println!("{:16X}", g.board);
    assert_eq!(0x2000_1000_2000_3000_u64, g.board);
  }

  #[test]
  fn test01_into_left() {
    let mut g = Game {
      board: 0x1121_4014_4023_2111_u64,
      score: 0,
    };
    g.into_left();
    println!("{:16X}", g.board);
    assert_eq!(0x2210_4140_4230_2210_u64, g.board);
  }

  #[test]
  fn test00_rotate90deg() {
    let mut g = Game {
      board: 0x0123_4567_89AB_CDEF_u64,
      score: 0,
    };
    g.rotate90deg();
    println!("{:16X}", g.board);
    assert_eq!(0xC840_D951_EA62_FB73_u64, g.board);
  }

  #[test]
  fn test00_move_to() {
    let mut g = Game {
      board: 0x1121_4014_4023_2111_u64,
      score: 0,
    };
    g.move_to(Direction::Right);
    println!("{:16X}", g.board);
    assert_eq!(0x0221_0414_0423_0212_u64, g.board);
  }

  #[test]
  fn test01_move_to() {
    let mut g = Game {
      board: 0x1121_4014_4023_2111_u64,
      score: 0,
    };
    g.move_to(Direction::Down);
    println!("{:16X}", g.board);
    assert_eq!(0x0021_1014_5023_2211_u64, g.board);
  }

  #[test]
  fn test00_get_max_value() {
    let mut g = Game {
      board: 0x1121_4014_4523_2111_u64,
      score: 0,
    };
    assert_eq!(5, g.get_max_value());
  }

  #[test]
  fn test00_is_end() {
    let mut g = Game {
      board: 0x9123_4567_89AB_CDEF_u64,
      score: 0,
    };
    assert_eq!(true, g.is_end());
  }

  #[test]
  fn test01_is_end() {
    let mut g = Game {
      board: 0x1123_4567_89AB_CDEF_u64,
      score: 0,
    };
    assert_eq!(false, g.is_end());
  }

  #[test]
  fn test02_is_end() {
    let mut g = Game {
      board: 0x0000_4567_89AB_CDEF_u64,
      score: 0,
    };
    assert_eq!(false, g.is_end());
  }
}
