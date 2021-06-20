use crate::game;
use rand::Rng;
use std::cmp;

#[derive(Clone, Copy)]
pub struct Solver {
  pub game: game::Game,
  pub max_value: u64,
}

impl Solver {
  // pub fn new(&mut self) {
  //   self.board_org = 0x0000_0000_0000_0000_u64;
  //   self.max_value = 0;
  // }

  pub fn dfs(
    &mut self,
    mut board_org: game::Game,
    dir: game::Direction,
    pos: Vec<u32>,
    mut depth: u32,
  ) -> u64 {
    let mut ret = 0;
    let mut tmp_score = 0;
    if depth == 0 {
      let mut board_trg = board_org.clone();

      for i in pos.iter() {
        board_trg.set_value(*i as usize, board_trg.tile());
        let mut board_pos = board_trg.clone();
        for d in game::Direction::iterator() {
          let tmp = board_pos.move_to(*d);
          if tmp_score < tmp {
            ret = tmp;
          }
          //reset board
          board_trg = board_org.clone();
        }
      }

      return ret;
    } else {
      depth -= 1;
      let mut s = Solver {
        game: board_org,
        max_value: 0,
      };

      let mut ret = 0;
      for dir in game::Direction::iterator() {
        let score = self.dfs(self.game, *dir, self.game.empty_tile_idx(), depth);
        if ret < score {
          ret = score;
        };
      }
      return ret;
    }
  }

  pub fn next_dir(&mut self, depth: u32) -> game::Direction {
    let mut ret_dir = game::Direction::Down;
    let mut tmp = 0;
    for dir in game::Direction::iterator() {
      let score = self.dfs(self.game, *dir, self.game.empty_tile_idx(), depth);
      if tmp < score {
        tmp = score;
        ret_dir = *dir;
      };
    }
    return ret_dir;
  }
}
