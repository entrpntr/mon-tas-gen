use gambatte::Input;
use gb::*;
use rom::*;
use statebuffer::StateBuffer;
use std::collections::BTreeMap;
use std::marker::PhantomData;

// intermediate buffers are larger by default so the goal buffer ends up with enough (varied) states.
const TEXT_SEGMENT_DEFAULT_INTERMEDIATE_BUFFER_SIZE: usize = ::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE << 2;

pub struct TextSegment<R: JoypadAddresses + RngAddresses + TextAddresses> {
  skip_input: Input,
  debug_output: bool,
  expect_conflicting_inputs: bool,
  _rom: PhantomData<R>,
}
impl<R: JoypadAddresses + RngAddresses + TextAddresses> super::WithDebugOutput for TextSegment<R> {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}
impl<R: JoypadAddresses + RngAddresses + TextAddresses> TextSegment<R> {
  pub fn new(skip_input: Input) -> Self {
    TextSegment {
      skip_input: skip_input,
      debug_output: false,
      expect_conflicting_inputs: false,
      _rom: PhantomData,
    }
  }
  // conflicting future inputs are expected, and the default behavior of dropping these states is employed without warning.
  pub fn expect_conflicting_inputs(mut self) -> Self { self.expect_conflicting_inputs = true; self }

  fn is_print_letter_delay_frame(gb: &mut Gb<R>) -> bool {
    gb.input(Input::empty());
    super::is_correct_input_use(gb, R::TEXT_BEFORE_JOYPAD_ADDRESS, R::TEXT_JOYPAD_ADDRESS, R::TEXT_AFTER_JOYPAD_ADDRESS)
  }
  fn progress_print_letter_delay_frame(&self, gb: &mut Gb<R>, s: State) -> Vec<(State, u32)> {
    let mut result = vec![];
    let mut num_done = 0;

    gb.restore(&s);
    gb.input(self.skip_input);
    if gb.step_until(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS, R::TEXT_JOYPAD_ADDRESS]) == R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS {
      num_done += 1;
      gb.step_until(&[R::TEXT_JOYPAD_ADDRESS]); // This is guaranteed to hit by is_print_letter_delay_frame
    }
    let delay = gb.gb.read_memory(R::TEXT_DELAY_FRAME_COUNTER_MEM_ADDRESS);
    assert!(gb.step_until_or_any_vblank(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS]) == 0); // this should never leave the function
    result.push((gb.save(), num_done));
    if delay >= 1 { // there's no way this input has an affect beyond PrintLetterDelay, there will always be a VBlank before leaving.
      gb.restore(&s);
      gb.input(Input::empty());
      gb.step_until(&[R::TEXT_JOYPAD_ADDRESS]); // This is guaranteed to hit by is_print_letter_delay_frame
      assert!(gb.step_until_or_any_vblank(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS]) == 0); // this should never leave the function
      result.push((gb.save(), num_done));
    } else if delay == 0 { // not pressing anything may conflict with future inputs, check whether it does
      gb.restore(&s);
      gb.input(Input::empty());
      gb.step_until(&[R::TEXT_JOYPAD_ADDRESS]); // This is guaranteed to hit by is_print_letter_delay_frame
      let (stayed, num_cycles) = Self::check_stays_within_print_letter_delay(gb);
      if stayed { result.push((gb.save(), num_done + num_cycles)); } else {
        if !self.expect_conflicting_inputs { println!("WARNING: TextSegment found state with conflicting inputs between PrintLetterDelay and future inputs [{}].", gb.get_stack_trace_string()); }
      }
    }
    result
  }
  fn check_stays_within_print_letter_delay(gb: &mut Gb<R>) -> (bool, u32) {
    let mut num_cycles = 0;
    loop {
      if gb.step_until(&[R::TEXT_AFTER_JOYPAD_ADDRESS]) == 0 { return (true, num_cycles); }
      let mut hit = gb.step_until(&[&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS, R::TEXT_BEFORE_JOYPAD_ADDRESS], R::JOYPAD_USE_ADDRESSES].concat());
      if hit == R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS {
        num_cycles += 1;
        hit = gb.step_until(&[&[R::TEXT_BEFORE_JOYPAD_ADDRESS], R::JOYPAD_USE_ADDRESSES].concat());
      }
      if hit == 0 { return (true, num_cycles); }
      if hit != R::TEXT_BEFORE_JOYPAD_ADDRESS {return (false, num_cycles); }
      if gb.step_until(&[R::TEXT_JOYPAD_ADDRESS]) == 0 { return (true, num_cycles); }
      if gb.gb.read_memory(R::TEXT_DELAY_FRAME_COUNTER_MEM_ADDRESS) > 0 {
        assert!(gb.step_until_or_any_vblank(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS]) == 0); // this should never leave the function
        return (true, num_cycles);
      }
    }
  }
}
impl<R: JoypadAddresses + RngAddresses + TextAddresses> super::Segment for TextSegment<R> {
  type Rom = R;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer {
    let mut goal_buffer = StateBuffer::new();
    let mut active_states: BTreeMap<u32, StateBuffer> = BTreeMap::new();
    for s in iter.into_iter() {
      gb.restore(&s);
      if !Self::is_print_letter_delay_frame(gb) {
        println!("WARNING: found State not at PrintLetterDelay initially, maybe there's another input before. Dropping state.");
      } else {
        active_states.entry(0).or_insert(StateBuffer::with_max_size(TEXT_SEGMENT_DEFAULT_INTERMEDIATE_BUFFER_SIZE)).add_state(s);
      }
    }
    while !active_states.is_empty() {
      let min_cycles: u32 = *active_states.keys().next().unwrap();
      let max_cycles: u32 = *active_states.keys().next_back().unwrap();
      let sb = active_states.remove(&min_cycles).unwrap();
      if self.debug_output { println!("TextSegment loop cycles {}-{}, min cycle size {}, goal_buffer size {}", min_cycles, max_cycles, sb.len(), goal_buffer.len()); }
      for s in sb.into_iter() {
        for (s, num_cycles) in self.progress_print_letter_delay_frame(gb, s) {
          gb.restore(&s);
          if Self::is_print_letter_delay_frame(gb) {
            active_states.entry(min_cycles + num_cycles).or_insert(StateBuffer::with_max_size(TEXT_SEGMENT_DEFAULT_INTERMEDIATE_BUFFER_SIZE)).add_state(s);
          } else {
            goal_buffer.add_state(s);
          }
        }
      }
    }
    goal_buffer
  }
}