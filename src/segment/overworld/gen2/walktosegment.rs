use gambatte::Input;
use gb::*;
use rom::*;
use segment::WithDebugOutput;
use statebuffer::StateBuffer;
use std::collections::VecDeque;
use std::marker::PhantomData;
use super::OverworldInteractionResult;

const MAX_WALK_STEP_SKIPS: u32 = 1;

pub struct WalkToSegment<T> {
  dest_x: usize,
  dest_y: usize,
  into_result: OverworldInteractionResult,
  debug_output: bool,
  _rom: PhantomData<T>,
}
impl <T: JoypadAddresses + RngAddresses + Gen2MapAddresses + Gen2MapEventsAddresses> WalkToSegment<T> {
  #[allow(dead_code)]
  pub fn new(dest_x: isize, dest_y: isize) -> Self {
    Self {
      dest_x: (dest_x + 6) as usize,
      dest_y: (dest_y + 6) as usize,
      into_result: OverworldInteractionResult::NoEvents,
      debug_output: false,
      _rom: PhantomData,
    }
  }
  #[allow(dead_code)]
  pub fn into(mut self, into_result: OverworldInteractionResult) -> Self { self.into_result = into_result; self }
}
impl<T: JoypadAddresses + RngAddresses + Gen2MapAddresses + Gen2MapEventsAddresses> WithDebugOutput for WalkToSegment<T> {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}

impl<T: JoypadAddresses + RngAddresses + Gen2MapAddresses + Gen2MapEventsAddresses> ::segment::Segment for WalkToSegment<T> {
  type Rom = T;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    let initial_states: Vec<_> = iter.into_iter().collect();
    assert!(!initial_states.is_empty());
    gb.restore(&initial_states[0]);
    let map = super::map::Map::new().load_gen2_map(gb);
    if self.debug_output {
      println!("WalkToSegment navigate to ({}, {})", self.dest_x as isize-6, self.dest_y as isize-6);
      println!("tile collisions:");
      map.print_tile_collision();
      println!("allowed movements:");
      map.print_tile_allowed_movements();
    }    

    // calculate distances
    let distances = {
      let mut distances = vec![];
      distances.resize(map.width * map.height, -1);
      distances[map.width * self.dest_y + self.dest_x] = 0;
      let mut queue = VecDeque::new();
      queue.push_back((self.dest_x, self.dest_y));
      while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        for &(dx, dy, from_input) in &[(0, -1, Input::DOWN), (0, 1, Input::UP), (-1, 0, Input::RIGHT), (1, 0, Input::LEFT)] {
          if y as isize + dy < 0 || y as isize + dy >= map.height as isize { continue; } // out of bounds
          if x as isize + dx < 0 || x as isize + dx >= map.width as isize { continue; } // out of bounds
          let from_x = (x as isize + dx) as usize;
          let from_y = (y as isize + dy) as usize;
          if distances[map.width * from_y + from_x] != -1 { continue; } // already calculated
          if !map.tile_allowed_movements[map.width * from_y + from_x].contains(from_input) { continue; } // step not allowed
          distances[map.width * from_y + from_x] = distances[map.width * y + x] + 1;
          queue.push_back((from_x, from_y));
        }
      }
      distances
    };

    if self.debug_output {
      println!("distance map:");
      for y in 0..map.height {
        for x in 0..map.width {
          print!(" {:2}", distances[map.width * y + x]);
        }
        println!();
      }
    }

    // initialize intermediate buffers
    let mut buffers = Vec::<StateBuffer>::new();
    for _ in 0..map.width * map.height { buffers.push(StateBuffer::new()); }
    let mut max_dist = -1;
    for s in initial_states.into_iter() {
      gb.restore(&s);
      let x = gb.gb.read_memory(T::PLAYER_X_MEM_ADDRESS) as usize + 2;
      let y = gb.gb.read_memory(T::PLAYER_Y_MEM_ADDRESS) as usize + 2;
      buffers[map.width * y + x].add_state(s);
      if max_dist < distances[map.width * y + x] { max_dist = distances[map.width * y + x]; }
    }
    assert!(max_dist >= 0, "destination unreachable!");

    let mut cur_dist = max_dist;
    while cur_dist > 0 {
      let into_result = if cur_dist == 1 { &self.into_result } else { &super::OverworldInteractionResult::NoEvents };
      for x in 0..map.width {
        for y in 0..map.height {
          if distances[map.width * y + x] == cur_dist {
            let sb = ::std::mem::replace(buffers.get_mut(map.width * y + x).unwrap(), StateBuffer::new());
            if sb.is_empty() { continue; }
            if self.debug_output { println!("processing states at ({},{}) with dist {}, statebuffer {}", x as isize-6, y as isize-6, cur_dist, sb); }
            for s in sb.into_iter() {
              for &(dx, dy, input) in &[(0, 1, Input::DOWN), (0, -1, Input::UP), (1, 0, Input::RIGHT), (-1, 0, Input::LEFT)] {
                if !map.tile_allowed_movements[map.width * y + x].contains(input) { continue; } // step not allowed
                if y as isize + dy < 0 || y as isize + dy >= map.height as isize { continue; } // out of bounds
                if x as isize + dx < 0 || x as isize + dx >= map.width as isize { continue; } // out of bounds
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;
                if distances[map.width * ny + nx] != cur_dist - 1 { continue; } // not on any shortest path

                gb.restore(&s);
                let mut s = gb.save();
                for _skips in 0..MAX_WALK_STEP_SKIPS+1 {
                  let facing_dir = match gb.gb.read_memory(T::PLAYER_DIRECTION_MEM_ADDRESS) & 0b1100 {
                    0x0 => Input::DOWN,
                    0x4 => Input::UP,
                    0x8 => Input::LEFT,
                    0xc => Input::RIGHT,
                    _ => panic!("got invalid direction"),
                  };
                  gb.input(input);
                  if super::walkstepsegment::walk_step_check(gb, into_result) {
                    gb.restore(&s);
                    gb.input(input);
                    gb.step();
                    buffers[map.width * ny + nx].add_state(gb.save());
                  }
                  if facing_dir != input { break; }
                  gb.restore(&s);
                  gb.input(Input::empty());
                  gb.step();
                  s = gb.save();
                }
              }
            }
          }
        }
      }
      cur_dist -= 1;
    }

    buffers.into_iter().nth(map.width * self.dest_y + self.dest_x).unwrap()
  }
}