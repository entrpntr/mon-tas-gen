extern crate montas;

extern crate rand;

use montas::ftii;
use montas::gambatte::*;
use montas::gambatte::inputs::*;
use montas::gb::*;
#[allow(unused_imports)] use montas::gbexecutor::*;
use montas::rom::*;
use montas::segment::*;
#[allow(unused_imports)] use montas::segment::overworld::gen2;
#[allow(unused_imports)] use montas::util::*;
use montas::statebuffer::StateBuffer;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn main() {
  // test_gambattesdl();
  // test_gambattedll();
  // if true {return;}

  // let mut gbe = SingleGb::<Crystal>::with_screen();
  // // let mut gbe = GbPool::<Crystal>::no_screen();
  // let s = gbe.get_initial_state();
  // let sb = MoveSegment::new(A).with_max_skips(10).execute_parallel(&mut gbe, vec![s]).into_state_buffer();
  // let sb = MoveSegment::new(START).with_max_skips(10).execute_parallel(&mut gbe, sb).into_state_buffer();
  // let sb = MoveSegment::new(D).with_max_skips(10).execute_parallel(&mut gbe, sb).into_state_buffer();
  // let sb = MoveSegment::new(L|A).with_max_skips(10).execute_parallel(&mut gbe, sb).into_state_buffer();
  // let sb = MoveSegment::new(B).with_max_skips(10).execute_parallel(&mut gbe, sb).into_state_buffer();
  // let _sb = MoveSegment::new(A).with_max_skips(10).execute_parallel(&mut gbe, sb).into_state_buffer();
  // // let sb = gbe.execute_text_segment(sb, 1, A); // choose Boy
  // // let sb = gbe.execute_text_segment(sb, 3, B);
  // // let _sb = gbe.execute_text_segment(sb, 4, A); // time: 10:..
  // if true {return;}


  playback_inputs(load_inputs("temp/blue_NSC.txt"));
  // convert_efl();
  // if true {return;}

  // BlueTestSegment::run();
  // YellowTestSegment::run();
}

#[allow(dead_code)]
fn test_gambattesdl() {
  let sdl = montas::gambattesdl::Sdl::init_sdl(2, 3);
  let mut gambatte1 = montas::gambattesdl::Gambatte::create_on_screen(sdl.clone(), 0, false);
  gambatte1.load_gbc_bios("roms/gbc_bios.bin");
  gambatte1.load_rom("roms/yellow.gbc");
  let mut gambatte2 = montas::gambattesdl::Gambatte::create_on_screen(sdl, 1, false);
  gambatte2.load_gbc_bios("roms/gbc_bios.bin");
  gambatte2.load_rom("roms/crystal.gbc");
  for _ in 0..5000 {
    gambatte1.step();
    gambatte2.step();
    sleep(Duration::from_millis(0));
  }
  let state1 = gambatte1.save_state();
  let state2 = gambatte2.save_state();
  gambatte1.load_state(&state1);
  gambatte2.load_state(&state2);
  for _ in 0..5000 {
    gambatte1.step();
    gambatte2.step();
    sleep(Duration::from_millis(0));
  }
}

#[allow(dead_code)]
fn test_gambattedll() {
  let sdl = montas::gambattedll::Sdl::init_sdl(2, 3);
  let mut gambatte1 = montas::gambattedll::Gambatte::create_on_screen(sdl.clone(), 0, false);
  gambatte1.load_gbc_bios("roms/gbc_bios.bin");
  gambatte1.load_rom("roms/yellow.gbc");
  let mut gambatte2 = montas::gambattedll::Gambatte::create_on_screen(sdl, 1, false);
  gambatte2.load_gbc_bios("roms/gbc_bios.bin");
  gambatte2.load_rom("roms/crystal.gbc");
  for _ in 0..10000 {
    gambatte1.step();
    gambatte2.step();
    sleep(Duration::from_millis(2));
  }
}

fn test_segment_end<R: Rom>(gb: &mut Gb<R>, sb: &StateBuffer, file_name: &str) {
  {
    println!("Creating sample input file...");
    gb.restore(sb.into_iter().next().unwrap());
    let inputs = gb.create_inputs();
    save_inputs(file_name, inputs);
  }
  println!("Rendering end states of {}", sb);
  for s in sb {
    gb.restore(s);
    for _ in 0..1000 {
      gb.input(Input::empty());
      gb.step();
    }
    std::thread::sleep(std::time::Duration::from_millis(100));
  }
}

pub struct BlueTestSegment {}
impl BlueTestSegment {
  #[allow(dead_code)]
  fn run() {
    let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
    let mut gb = Gb::<Blue>::create(Gambatte::create_on_screen(sdl, 0 /* screen */, false /* equal length frames */));
    let states = vec![gb.save()];
    let sb = BlueTestSegment{}.execute(&mut gb, states);
    test_segment_end(&mut gb, &sb, "temp/blue_test.txt");
  }
}
impl Segment<Blue> for BlueTestSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<Blue>, _iter: I) -> StateBuffer {
//     let sb = DelaySegment::new(MoveSegment::with_metric_fn(START, |_gb| Some(()))).execute(gb, _iter);
//     println!("{}", sb);
//     let sb = MoveSegment::new(A).with_max_skips(10).execute(gb, sb);
//     println!("{}", sb);
//     let sb = MoveSegment::new(START).with_max_skips(10).execute(gb, sb);
//     println!("{}", sb);
// //    let sb = MoveSegment::new(A).with_max_skips(10).execute(gb, sb);
//     let sb = MoveSegment::new(D|A).execute(gb, sb); // options
//     let sb = MoveSegment::new(L).execute(gb, sb); // text speed fast
//     let sb = MoveSegment::new(D).execute(gb, sb); // battle animations...
//     let sb = MoveSegment::new(L).execute(gb, sb); // ...off
//     let sb = MoveSegment::new(D).execute(gb, sb); // battle style...
//     let sb = MoveSegment::new(L).execute(gb, sb); // ...set
//     let sb = MoveSegment::new(B).execute(gb, sb); // back
//     let sb = MoveSegment::new(A).execute(gb, sb); // new game
//     println!("{}", sb);
//     let sb = SkipTextsSegment::new(13, B).execute(gb, sb); // skip texts until player name
//     let sb = MoveSegment::new(D|A).execute(gb, sb); // Name: Blue
//     let sb = SkipTextsSegment::new(5, B).execute(gb, sb); // skip texts until rival name
//     let sb = MoveSegment::new(D|A).execute(gb, sb); // Name: Red
//     let sb = SkipTextsSegment::new(7, B).execute(gb, sb); // skip texts until game start
//     let sb = TextSegment::new(A).execute(gb, sb); // ...awaits, let's go
//     println!("{}", sb);
//     sb.save("blue_test");
    let sb = StateBuffer::load("blue_test");
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);

    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = IdentifyInputSegment::new().execute(gb, sb);
    // let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);

    // let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);

    // let sb = MoveSegment::with_metric(A, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // sb.save("blue_test");
    // let sb = StateBuffer::load("blue_test");
    // let sb = SkipTextsSegment::new(6, B).execute(gb, sb); // it's dangerous to go outside, take this
    // let sb = TextSegment::new(A).expect_conflicting_inputs().execute(gb, sb); // come with me
    // let sb = MoveSegment::new(B).execute(gb, sb); // come with me
    // let sb = IdentifyInputSegment::new().execute(gb, sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    // let sb = IdentifyInputSegment::new().execute(gb, sb);
    sleep(Duration::from_millis(1000));
    sb
  }
}

pub struct YellowTestSegment {}
impl YellowTestSegment {
  #[allow(dead_code)]
  fn run() {
    let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
    let mut gb = Gb::<Yellow>::create(Gambatte::create_on_screen(sdl, 0 /* screen */, false /* equal length frames */));
    let states = vec![gb.save()];
    let sb = YellowTestSegment{}.execute(&mut gb, states);
    test_segment_end(&mut gb, &sb, "temp/yellow_test.txt");
  }
}
impl Segment<Yellow> for YellowTestSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<Yellow>, _iter: I) -> StateBuffer {
    // let sb = MoveSegment::new(START).with_max_skips(15).with_buffer_size(4096).execute(gb, _iter);
    // println!("{}", sb);
    // let sb = MoveSegment::new(A).with_max_skips(15).with_buffer_size(4096).execute(gb, sb);
    // println!("{}", sb);
    // let sb = MoveSegment::new(START).with_max_skips(15).with_buffer_size(4096).execute(gb, sb);
    // println!("{}", sb);
    // let sb = DelaySegment::new(MoveSegment::with_metric(A, TrainerIDMetric{}.filter(|v| { v == &0x26F1 }))).with_debug_output(true).execute(gb, sb); // new game
    // sb.save("yellow_after_tid");
    // let sb = StateBuffer::load("yellow_after_tid");
    // println!("{}", sb);
    // let sb = SkipTextsSegment::new(13, B).execute(gb, sb); // skip texts until player name
    // let sb = MoveSegment::new(D).execute(gb, sb); // Name: Yellow
    // let sb = MoveSegment::new(A).execute(gb, sb); // Name: Yellow
    // let sb = SkipTextsSegment::new(5, B).execute(gb, sb); // skip texts until rival name
    // let sb = MoveSegment::new(D).execute(gb, sb); // Name: Blue
    // let sb = MoveSegment::new(A).execute(gb, sb); // Name: Blue
    // let sb = SkipTextsSegment::new(7, B).execute(gb, sb); // skip texts until game start
    // let sb = TextSegment::new(A).execute(gb, sb); // ...awaits, let's go
    // println!("{}", sb);
    // let sb = MoveSegment::new(START).execute(gb, sb);
    // let sb = MoveSegment::new(U).execute(gb, sb);
    // let sb = MoveSegment::new(NIL).execute(gb, sb);
    // let sb = MoveSegment::new(U).execute(gb, sb);
    // let sb = MoveSegment::new(L|A).execute(gb, sb);
    // let sb = MoveSegment::new(B).execute(gb, sb);
    // let sb = MoveSegment::new(START).execute(gb, sb);
    // sb.save("yellow_after_intro_256_div");
    let sb = StateBuffer::load("yellow_after_intro_256_div");
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
  let sb = MoveSegment::new(NIL).execute(gb, sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);

    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);

    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    sb.save("yellow_grass_256_wait1_");
    let sb = StateBuffer::load("yellow_grass_256_wait1_");
    let sb = SkipTextsSegment::new(3, B).execute(gb, sb); // oak in grass
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = SkipTextsSegment::new(3, B).execute(gb, sb); // oak vs pikachu, catch pikachu
    let sb = SkipTextsSegment::new(7, B).execute(gb, sb); // oak in grass
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = SkipTextsSegment::new(15, B).execute(gb, sb); // oak speech
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(A, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    sb.save("yellow_before_collect_pikachu_256_wait1_");
    let sb = StateBuffer::load("yellow_before_collect_pikachu_256_wait1_");
    let sb = MoveSegment::new(NIL).execute(gb, sb); // Release A button to start sequence
    let sb = SkipTextsSegment::new(10, B).execute(gb, sb); // Rival snatches
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gb, sb); println!("{}", sb);
    let sb = SkipTextsSegment::new(4, B).execute(gb, sb); // get Pikachu
    sb.save("yellow_before_collect_pikachu_2_256_wait1_");
    let sb = StateBuffer::load("yellow_before_collect_pikachu_2_256_wait1_");
    let sb = SkipTextsSegment::new(2, B).with_buffer_size(256).execute(gb, sb); // get Pikachu
    let sb = TextSegment::new(A).with_buffer_size(256).execute(gb, sb); // Nickname?
    let sb = DelaySegment::new(MoveSegment::with_metric(B, Gen1DVMetric {}.filter(|v| {
      let dvs = u16::from(v.atk) << 12 | u16::from(v.def) << 8 | u16::from(v.spd) << 4 | u16::from(v.spc);
      let div = ((dvs >> 8) as u8).wrapping_sub(dvs as u8);
      if ::rand::random::<u8>() < 0x04 {
        print!("{:X}", div);
        // print!("/{:X}", v.div_state >> 6);
        // print!("/{}", to_human_readable_time(v.cycle_count));
        print!(" ");
      }
      if v.atk != 7 { return false; }
      if v.spc != 8 && v.spc != 9 { return false; }
      if v.def != 0 && v.def != 1 { return false; }
      if v.spd != 12 && v.spd != 13 { return false; }
      println!("Chosen DVs: {:?}", v); true
    }))).with_debug_output(true).execute(gb, sb); println!("{}", sb);
    // sb.save("yellow_after_collect_pikachu_256_div");
    // let sb = StateBuffer::load("yellow_after_collect_pikachu_256_div");
    // let sb = IdentifyInputSegment::new().execute(gb, sb);
    sleep(Duration::from_millis(1000));
    sb
  }
}

pub struct SilverTestSegment {}
impl Segment<Silver> for SilverTestSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<Silver>, iter: I) -> StateBuffer {
    let sb = DelaySegment::new(MoveSegment::with_metric_fn(A, |_gb| Some(()))).execute(gb, iter);
    println!("{}", sb);
    let sb = MoveSegment::new(START).with_max_skips(10).execute(gb, sb);
    println!("{}", sb);
    let sb = MoveSegment::new(D).execute(gb, sb); // options
    let sb = MoveSegment::new(L|A).execute(gb, sb); // fast options
    let sb = MoveSegment::new(B).execute(gb, sb); // back
    let sb = MoveSegment::new(A).execute(gb, sb); // new game
    println!("{}", sb);
    let sb = SkipTextsSegment::new(3, B).execute(gb, sb);
    let sb = SkipTextsSegment::new(4, A).execute(gb, sb); // time: 10:..
    let sb = TextSegment::new(A).execute(gb, sb); // overslept
    let sb = MoveSegment::new(B).execute(gb, sb); // overslept
    let sb = SkipTextsSegment::new(17, B).execute(gb, sb); // oak speech
    let sb = MoveSegment::new(D).execute(gb, sb); // Name: Silver
    let sb = MoveSegment::new(A).execute(gb, sb); // Name: Silver
    let sb = SkipTextsSegment::new(7, B).execute(gb, sb); // skip texts until game start
    let sb = TextSegment::new(A).execute(gb, sb); // ... seeing you later
    let sb = IdentifyInputSegment::new().execute(gb, sb);
    println!("{}", sb);
    sleep(Duration::from_millis(1000));
    sb
  }
}

pub struct CrystalTestSegment {}
impl Segment<Crystal> for CrystalTestSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<Crystal>, _iter: I) -> StateBuffer {
    // let sb = DelaySegment::new(MoveSegment::with_metric(A, NullMetric::new())).with_debug_output(true).execute(gb, _iter);
    // println!("{}", sb);
    // let sb = MoveSegment::new(START).with_max_skips(10).execute(gb, sb);
    // println!("{}", sb);
    // let sb = MoveSegment::new(D).execute(gb, sb); // options
    // let sb = MoveSegment::new(L|A).execute(gb, sb); // fast options
    // let sb = MoveSegment::new(B).execute(gb, sb); // back
    // let sb = MoveSegment::new(A).execute(gb, sb); // new game
    // println!("{}", sb);
    // let sb = SkipTextsSegment::new(1, A).execute(gb, sb); // choose Boy
    // let sb = SkipTextsSegment::new(3, B).execute(gb, sb);
    // let sb = SkipTextsSegment::new(4, A).execute(gb, sb); // time: 10:..
    // let sb = TextSegment::new(A).expect_conflicting_inputs().execute(gb, sb); // overslept
    // let sb = MoveSegment::new(B).execute(gb, sb); // overslept
    // let sb = SkipTextsSegment::new(17, B).execute(gb, sb); // oak speech
    // let sb = MoveSegment::new(D).execute(gb, sb); // Name: Chris
    // let sb = MoveSegment::new(A).execute(gb, sb); // Name: Chris
    // let sb = SkipTextsSegment::new(7, B).execute(gb, sb); // skip texts until game start
    // let sb = TextSegment::new(A).execute(gb, sb); // ... seeing you later
    // println!("{}", sb);
    // sb.save("crystal_test");
    // let sb = StateBuffer::load("crystal_test");
    // let sb = gen2::TurnSegment::new(R).execute(gb, sb); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(7, 0).into(gen2::OverworldInteractionResult::Warped).with_debug_output(true).execute(gb, sb);
    // let sb = gen2::WarpSegment::new().execute(gb, sb); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(7, 2).with_debug_output(true).execute(gb, sb);
    // let sb = gen2::WalkStepSegment::new(D).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(A, gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v == &gen2::OverworldInteractionResult::Interact})).execute(gb, sb); println!("{}", sb);
    // let sb = SkipTextsSegment::new(12, B).execute(gb, sb); // mom speech
    // let sb = SkipTextsSegment::new(2, A).execute(gb, sb); // choose Sunday
    // let sb = SkipTextsSegment::new(1, B).execute(gb, sb); // no DST
    // let sb = SkipTextsSegment::new(1, A).execute(gb, sb); // confirm time
    // let sb = SkipTextsSegment::new(3, B).execute(gb, sb); // mom speech
    // let sb = SkipTextsSegment::new(1, A).execute(gb, sb); // know phone
    // let sb = SkipTextsSegment::new(5, B).execute(gb, sb); // mom speech
    // let sb = gen2::TurnSegment::new(R).execute(gb, sb); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(7, 7).with_debug_output(true).execute(gb, sb);
    // let sb = gen2::WarpSegment::new().with_input(D).execute(gb, sb); println!("{}", sb);
    // sb.save("crystal_left_house");
    // let sb = StateBuffer::load("crystal_left_house");
    // let sb = gen2::TurnSegment::new(L).execute(gb, sb); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(6, 3).into(gen2::OverworldInteractionResult::Warped).with_debug_output(true).execute(gb, sb);
    // let sb = gen2::WarpSegment::new().execute(gb, sb); println!("{}", sb);
    // let sb = gen2::SkipScriptSegment::new().execute(gb, sb); println!("{}", sb);
    // let sb = SkipTextsSegment::new(12, B).execute(gb, sb); // elm speech
    // let sb = SkipTextsSegment::new(1, A).execute(gb, sb); // choose to help
    // let sb = SkipTextsSegment::new(6, B).execute(gb, sb); // elm speech
    // let sb = gen2::SkipScriptSegment::new().execute(gb, sb); println!("{}", sb);
    // let sb = SkipTextsSegment::new(15, B).execute(gb, sb); // elm speech
    // let sb = gen2::SkipScriptSegment::new().execute(gb, sb); println!("{}", sb);
    // let sb = SkipTextsSegment::new(5, B).execute(gb, sb); // elm speech
    // let sb = gen2::WalkToSegment::new(7, 4).with_debug_output(true).execute(gb, sb);
    // let sb = MoveSegment::with_metric(NIL, gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v == &gen2::OverworldInteractionResult::NoEvents})).execute(gb, sb); println!("{}", sb);
    // let sb = gen2::TurnSegment::new(U).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(A, gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v == &gen2::OverworldInteractionResult::Interact})).execute(gb, sb); println!("{}", sb);
    // let sb = MoveSegment::new(B).execute(gb, sb); // close picture
    // let sb = SkipTextsSegment::new(2, A).execute(gb, sb); // choose Totodile
    // sb.save("crystal_choose_starter");
    // let sb = StateBuffer::load("crystal_choose_starter");
    // let sb = SkipTextsSegment::new(2, B).with_buffer_size(4096).execute(gb, sb); println!("{}", sb); // elm speech
    // let sb = TextSegment::new(A).with_buffer_size(4096).execute(gb, sb); println!("{}", sb); // elm speech
    // sb.save("crystal_choose_starter_unbounded");
    // let sb = StateBuffer::load("crystal_choose_starter_unbounded");
    // let sb = DelaySegment::new(MoveSegment::with_metric(B, Gen2DVMetric {}.filter(|v| {
    //   if v.atk < 15 || v.spc < 15 || v.spd < 15 { return false; }
    //   println!("Chosen DVs: {:?}", v); true
    // }))).with_debug_output(true).execute(gb, sb); println!("{}", sb);
    // let sb = SkipTextsSegment::new(2, B).execute(gb, sb); println!("{}", sb); // no nickname
    // let sb = gen2::SkipScriptSegment::new().execute(gb, sb); println!("{}", sb);
    // let sb = SkipTextsSegment::new(11, B).execute(gb, sb); // elm speech
    // let sb = gen2::TurnSegment::new(D).execute(gb, sb); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(4, 7).with_debug_output(true).execute(gb, sb);
    // let sb = gen2::WalkStepSegment::new(D).into(gen2::OverworldInteractionResult::MapCoordEvent).execute(gb, sb); println!("{}", sb);
    // let sb = gen2::SkipScriptSegment::new().execute(gb, sb); println!("{}", sb);
    // let sb = SkipTextsSegment::new(7, B).execute(gb, sb); // aide speech
    // let sb = gen2::SkipScriptSegment::new().execute(gb, sb); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(4, 11).with_debug_output(true).execute(gb, sb);
    // let sb = gen2::WarpSegment::new().with_input(D).execute(gb, sb); println!("{}", sb);
    // sb.save("crystal_test_after_elm");
    let sb = StateBuffer::load("crystal_test_after_elm");
    let sb = gen2::WalkToSegment::new(-1, 8).into(gen2::OverworldInteractionResult::MapConnection).with_debug_output(true).execute(gb, sb);
    let sb = MoveSegment::new(NIL).execute(gb, sb); println!("{}", sb); // MapConnection
    let sb = gen2::WalkToSegment::new(9, 6).with_debug_output(true).execute(gb, sb);
    let sb = gen2::JumpLedgeSegment::new(L).execute(gb, sb); println!("{}", sb);
    let sb = gen2::WalkToSegment::new(-1, 7).into(gen2::OverworldInteractionResult::MapConnection).with_debug_output(true).execute(gb, sb);
    let sb = MoveSegment::new(NIL).execute(gb, sb); println!("{}", sb); // MapConnection
    let sb = gen2::WalkToSegment::new(17, -1).into(gen2::OverworldInteractionResult::MapConnection).with_debug_output(true).execute(gb, sb);
    let sb = MoveSegment::new(NIL).execute(gb, sb); println!("{}", sb); // MapConnection
    let sb = gen2::WalkToSegment::new(17, 5).into(gen2::OverworldInteractionResult::Warped).with_debug_output(true).execute(gb, sb);
    let sb = gen2::WarpSegment::new().execute(gb, sb); println!("{}", sb);
    sb.save("crystal_test_entered_mr_pokemon_house");
    let sb = StateBuffer::load("crystal_test_entered_mr_pokemon_house");
    let sb = gen2::SkipScriptSegment::new().execute(gb, sb); println!("{}", sb);
    let sb = SkipTextsSegment::new(2, B).execute(gb, sb); // Mr.Pokemon speech
    let sb = gen2::SkipScriptSegment::new().execute(gb, sb); println!("{}", sb);
    let sb = SkipTextsSegment::new(14, B).execute(gb, sb); // Mr.Pokemon speech
    let sb = gen2::SkipScriptSegment::new().execute(gb, sb); println!("{}", sb);
    let sb = SkipTextsSegment::new(29, B).execute(gb, sb); // Oak speech
    let sb = gen2::SkipScriptSegment::new().execute(gb, sb); println!("{}", sb);
    let sb = SkipTextsSegment::new(4, B).execute(gb, sb); // Mr.Pokemon speech
    let sb = gen2::TurnSegment::new(D).execute(gb, sb); println!("{}", sb);
    let sb = gen2::WalkStepSegment::new(D).execute(gb, sb); println!("{}", sb);
    let sb = gen2::WarpSegment::new().with_input(D).execute(gb, sb); println!("{}", sb);
    sb.save("crystal_test_after_mr_pokemon_house");
    let sb = StateBuffer::load("crystal_test_after_mr_pokemon_house");
    let sb = MoveLoopSegment::new(gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v != &gen2::OverworldInteractionResult::CountStepEvent})).execute(gb, sb); println!("{}", sb);
    let sb = SkipTextsSegment::new(4, B).execute(gb, sb); // Elm phone call
    let sb = TextSegment::new(A).execute(gb, sb); // Elm phone call ends
    let sb = gen2::WalkToSegment::new(7, 54).into(gen2::OverworldInteractionResult::MapConnection).with_debug_output(true).execute(gb, sb);
    let sb = MoveSegment::new(NIL).execute(gb, sb); println!("{}", sb); // MapConnection
    let sb = gen2::WalkToSegment::new(33, 7).into(gen2::OverworldInteractionResult::MapCoordEvent).with_debug_output(true).execute(gb, sb);
    let sb = gen2::SkipScriptSegment::new().execute(gb, sb); println!("{}", sb);
    sb.save("crystal_test_before_rival1");
    let sb = StateBuffer::load("crystal_test_before_rival1");
    let sb = SkipTextsSegment::new(7, B).execute(gb, sb); // pre-battle texts
    let sb = SkipTextsSegment::new(1, B).execute(gb, sb); // trainer wants to battle
    let sb = SkipTextsSegment::new(1, B).execute(gb, sb); // trainer sent out ...
    // let sb = TextSegment::new(A).execute(gb, sb); // mon! / Go! mon!

    let sb = IdentifyInputSegment::new().execute(gb, sb);
    sleep(Duration::from_millis(1000));
    sb
  }
}





#[allow(dead_code)]
fn convert_efl() {
  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let (hi_inputs, lo_inputs) = {
    let mut gb = Gambatte::create_on_screen(sdl.clone(), 0 /* screen */, false /* equal length frames */);
    gb.load_gbc_bios("roms/gbc_bios.bin");
    gb.load_rom("roms/crystal.gbc");
    ftii::to_ftii::<Crystal>(gb, load_inputs("temp/crystal_test.txt"))
  };

  let inputs = {
    let mut gb = Gambatte::create_on_screen(sdl, 0 /* screen */, true /* equal length frames */);
    gb.load_gbc_bios("roms/gbc_bios.bin");
    gb.load_rom("roms/crystal.gbc");
    ftii::from_ftii::<Crystal>(gb, hi_inputs, lo_inputs)
  };
  save_inputs("temp/crystal_test_efl.txt", inputs);
}

fn load_inputs(file_name: &str) -> Vec<Input> {
  let mut result: Vec<Input> = vec![];
  let f = File::open(file_name).expect("file not found");
  let file = BufReader::new(&f);
  for line in file.lines() {
    let l = line.unwrap();
    let mut input = Input::empty();
    if l.contains('D') { input |= Input::DOWN; }
    if l.contains('U') { input |= Input::UP; }
    if l.contains('L') { input |= Input::LEFT; }
    if l.contains('R') { input |= Input::RIGHT; }
    if l.contains('S') { input |= Input::START; }
    if l.contains('s') { input |= Input::SELECT; }
    if l.contains('B') { input |= Input::B; }
    if l.contains('A') { input |= Input::A; }
    result.push(input);
  }
  result
}

#[allow(dead_code)]
fn playback_inputs(inputs: Vec<Input>) {
  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let mut gb = Gambatte::create_on_screen(sdl, 0 /* screen */, false /* equal length frames */);
  gb.load_gbc_bios("roms/gbc_bios.bin");
  gb.load_rom("roms/blue.gb");
  for input in inputs {
    gb.set_input(input);
    gb.step();
    sleep(Duration::from_millis(1));
  }
}

fn save_inputs(file_name: &str, inputs: Vec<Input>) {
  let mut f = File::create(file_name).expect("creating file failed");
  for i in inputs {
    f.write_all(format!("|{}{}{}{}{}{}{}{}.|\n",
        if i.contains(Input::UP) {"U"} else {"."},
        if i.contains(Input::DOWN) {"D"} else {"."},
        if i.contains(Input::LEFT) {"L"} else {"."},
        if i.contains(Input::RIGHT) {"R"} else {"."},
        if i.contains(Input::START) {"S"} else {"."},
        if i.contains(Input::SELECT) {"s"} else {"."},
        if i.contains(Input::B) {"B"} else {"."},
        if i.contains(Input::A) {"A"} else {"."}).as_bytes()).unwrap();
  }
}
