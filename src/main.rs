mod bk2;
mod logger;
mod run;

use crate::bk2::{Bk2Writer, read_bk2_inputs};
use gambatte::*;
use log::{LevelFilter::*};
#[allow(unused_imports)] use montas::constants::*;
use montas::ftii;
#[allow(unused_imports)] use montas::gbexecutor::*;
use montas::rom::*;
#[allow(unused_imports)] use montas::segment::overworld::gen2;
#[allow(unused_imports)] use montas::util::*;
use montas::sdl::*;

fn main() {
  crate::logger::init_with_level(Info).unwrap();

  // let mut gbe = SingleGb::<Crystal>::with_screen();
  // // let mut gbe = GbPool::<Crystal>::no_screen();
  // let s = gbe.get_initial_state();
  // let sb = MoveSegment::new(A).with_max_skips(10).execute_split(&mut gbe, vec![s]).into_state_buffer();
  // let sb = MoveSegment::new(START).with_max_skips(10).execute_split(&mut gbe, sb).into_state_buffer();
  // let sb = MoveSegment::new(D).with_max_skips(10).execute_split(&mut gbe, sb).into_state_buffer();
  // let sb = MoveSegment::new(L|A).with_max_skips(10).execute_split(&mut gbe, sb).into_state_buffer();
  // let sb = MoveSegment::new(B).with_max_skips(10).execute_split(&mut gbe, sb).into_state_buffer();
  // let _sb = MoveSegment::new(A).with_max_skips(10).execute_split(&mut gbe, sb).into_state_buffer();
  // // let sb = gbe.execute_text_segment(sb, 1, A); // choose Boy
  // // let sb = gbe.execute_text_segment(sb, 3, B);
  // // let _sb = gbe.execute_text_segment(sb, 4, A); // time: 10:..
  // if true {return;}


  // create_gbi_inputs();
  // playback_inputs();
  // playback_test();
  // playback_demos();
  // convert_efl();

  crate::run::blue_glitchless::start();
  // crate::run::crystal_desync::start();
  // crate::run::crystal_glitchless::start();
  // crate::run::blue_testing::start();
  // crate::run::silver_testing::start();
  // crate::run::yellow_testing::start();
}


#[allow(dead_code)]
fn convert_efl() {
  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let (hi_inputs, lo_inputs) = {
    let gb = Gambatte::create("roms/gbc_bios.bin", "roms/crystal.gbc", false /* equal length frames */, 0 /* RTC divisor offset */, SdlScreen::new(sdl.clone(), 0 /* screen */));
    ftii::to_ftii::<Crystal>(gb, read_bk2_inputs("temp/crystal_glitchless_90.bk2").unwrap())
  };

  let inputs = {
    let gb = Gambatte::create("roms/gbc_bios.bin", "roms/crystal.gbc", true /* equal length frames */, 0 /* RTC divisor offset */, SdlScreen::new(sdl, 0 /* screen */));
    ftii::from_ftii::<Crystal>(gb, hi_inputs, lo_inputs)
  };
  Bk2Writer::new::<Crystal>().with_equal_length_frames(true).write_bk2("temp/crystal_glitchless_90_efl.bk2", &inputs).unwrap();
}


const CYCLE_OFFSET: u64 = 484500 + (4 << 9);

#[allow(dead_code)]
fn create_gbi_inputs() {
  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let input_map = {
    let gb = Gambatte::create("roms/gbc_bios.bin", "roms/crystal.gbc", false /* equal length frames */, -90 /* RTC divisor offset */, SdlScreen::new(sdl.clone(), 0 /* screen */));
    ftii::to_cycles::<Crystal>(gb, read_bk2_inputs("temp/crystal_glitchless_90.bk2").unwrap())
  };

  let mut cur_cycle = 0;
  let mut cur_input = Input::all();
  for (cycle, input) in input_map {
    if input != cur_input {
      let gbi_time = ((cycle + CYCLE_OFFSET + cur_cycle + CYCLE_OFFSET) * (1 << 21) / ((1 << 21) - 0) ) >> 10;
      eprintln!("{:08X} {:04X}", gbi_time, input.bits());
      cur_input = input;
    }
    cur_cycle = cycle;
  }
}

#[allow(dead_code)]
fn playback_inputs() {
  let inputs = read_bk2_inputs("temp/crystal_glitchless_90.bk2").unwrap();
  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let mut gb = Gambatte::create("roms/gbc_bios.bin", "roms/crystal.gbc", false /* equal length frames */, -100 /* RTC divisor offset */, SdlScreen::new(sdl, 0 /* screen */));

  let i_offset = 0;

  // let mut last_hit_frame = 0;
  // let mut last_hit_tima = 0;
  // let mut last_hit_cycle = 0;
  for i in 0..inputs.len() {
    gb.set_input(inputs[i + i_offset]);
    if gb.step_until(&[/*0x0283, */0x3E93]).is_some() {
      // let tima_diff = (gb.get_cycle_count() - last_hit_cycle) >> 9;
      // let mut new_tima = u64::from(last_hit_tima) + tima_diff;
      // if new_tima == 255 { new_tima = 256; }
      // if new_tima < 256 {
      //   println!("non-cycling tima {} - {} ({}-{})", last_hit_frame, gb.frame(), last_hit_tima, new_tima);
      // } else if new_tima >= 512 {
      //   println!("{} - {} ({}-{})", last_hit_frame, gb.frame(), last_hit_tima, new_tima - 256);
      // } else if new_tima - 256 != u64::from(gb.read_memory(0xff05)) {
      //   // println!("non-matching tima {} - {} ({}-{})", last_hit_frame, gb.frame(), last_hit_tima, new_tima - 256);
      // }

      // last_hit_frame = gb.frame();
      // last_hit_tima = gb.read_memory(0xff05);
      // last_hit_cycle = gb.get_cycle_count();
      gb.step();
    }
    // if i == 118000 {
    //   let dvs = gb.read_memory_word_be(0xDD54);
    //   println!("DVs: {:x}, cycles: {}", dvs, gb.get_cycle_count());
    // }
    // if i == 394400 { // skip a frame
    //   gb.set_input(Input::empty());
    //   gb.step();
    // }
    // if i == 394449 { i_offset += 1; println!("skip: {:08X}", gb.get_cycle_count() >> 9); } // skip an input
    // if i == 394453 { i_offset -= 1; println!("unskip: {:08X}", gb.get_cycle_count() >> 9); } // skip an input

    if i > 604000 {
      std::thread::sleep(std::time::Duration::from_micros(15000));
    }
  }
}

#[allow(dead_code)]
fn playback_demos() {
  let sdl = Sdl::init_sdl(4 /* num screens */, 3 /* scale */);
  let mut gb1 = Gambatte::create("roms/gbc_bios.bin", "roms/blue.gb", false /* equal length frames */, 0 /* RTC divisor offset */, SdlScreen::new(sdl.clone(), 0 /* screen */));
  let mut gb2 = Gambatte::create("roms/gbc_bios.bin", "roms/yellow.gbc", false /* equal length frames */, 0 /* RTC divisor offset */, SdlScreen::new(sdl.clone(), 1 /* screen */));
  let mut gb3 = Gambatte::create("roms/gbc_bios.bin", "roms/gold.gbc", false /* equal length frames */, 0 /* RTC divisor offset */, SdlScreen::new(sdl.clone(), 2 /* screen */));
  let mut gb4 = Gambatte::create("roms/gbc_bios.bin", "roms/crystal.gbc", false /* equal length frames */, 0 /* RTC divisor offset */, SdlScreen::new(sdl.clone(), 3 /* screen */));
  let inputs1 = read_bk2_inputs("temp/blue_demo.bk2").unwrap();
  let inputs2 = read_bk2_inputs("temp/tikevin83-pokemonyellow-consoleverified.bk2").unwrap();
  let inputs3 = read_bk2_inputs("temp/gold_demo.bk2").unwrap();
  let inputs4 = read_bk2_inputs("temp/crystal_demo.bk2").unwrap();
  for i in 0..1_000_000 {
    let mut has_input = false;
    gb1.set_input(if inputs1.len() > i {has_input = true; inputs1[i]} else {Input::from_bits_truncate(1 << (rand::random::<u8>() & 7))}); gb1.step();
    gb2.set_input(if inputs2.len() > i {has_input = true; inputs2[i]} else {Input::from_bits_truncate(1 << (rand::random::<u8>() & 7))}); gb2.step();
    gb3.set_input(if inputs3.len() > i {has_input = true; inputs3[i]} else {Input::from_bits_truncate(1 << (rand::random::<u8>() & 7))}); gb3.step();
    gb4.set_input(if inputs4.len() > i {has_input = true; inputs4[i]} else {Input::from_bits_truncate(1 << (rand::random::<u8>() & 7))}); gb4.step();
    if !has_input { break; }
    if i == 8711 { gb1.write_memory(0xd179, 0x10); } // give XP
    if i == 11152 { gb3.write_memory(0xda32, 0x10); } // give XP
    if i == 47000 { gb4.write_memory(0xdce7, 0x20); } // give XP
  }
}

#[allow(dead_code)]
fn playback_test() {
  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let mut gb = Gambatte::create("roms/gbc_bios.bin", "roms/silver.gbc", false /* equal length frames */, 0 /* RTC divisor offset */, SdlScreen::new(sdl.clone(), 0 /* screen */));
  let inputs = read_bk2_inputs("temp/silver_test.bk2").unwrap();
  for input in inputs {
    gb.set_input(input); gb.step();
    std::thread::sleep(std::time::Duration::from_millis(1));
  }
}
