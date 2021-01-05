use montas::multi::*;
use montas::rom::*;
use montas::sdl::*;

const EQUAL_LENGTH_FRAMES: bool = false;
const RTC_DIVISOR_OFFSET: i32 = 0;

#[allow(dead_code)]
pub fn start() {
  log::set_max_level(log::LevelFilter::Debug);

  let sdl = Sdl::init_sdl(2 /* num screens */, 1 /* scale */);
  let blue_gb = Gb::<Blue>::create(EQUAL_LENGTH_FRAMES, RTC_DIVISOR_OFFSET, SdlScreen::new(sdl.clone(), 0));
  let _blue_executor = MultiGbExecutor::new(blue_gb, blue_plan());
  let red_gb = Gb::<Red>::create(EQUAL_LENGTH_FRAMES, RTC_DIVISOR_OFFSET, SdlScreen::new(sdl, 1));
  let _red_executor = MultiGbExecutor::new(red_gb, red_plan());
  let mut r = MultiGbRunner::new([
    Box::new(_blue_executor),
    // Box::new(_red_executor),
  ]);

  // r.load("multi_test2");
  r.run();
  r.save("multi_test");
  std::thread::sleep(std::time::Duration::from_millis(1000));

  r.debug_segment_end("temp/multi_testing");
}

fn blue_plan() -> ListPlan<Blue> {
  ListPlan::new(vec![
    Box::new(SkipIntroPlan::new().with_auto_pass_after(214)), // Logo
    Box::new(SkipIntroPlan::new().with_auto_pass_after(322)), // Intro cutscene
    Box::new(SkipIntroPlan::new().with_no_up_select_b()), // main menu
    Box::new(MainMenuPlan::new()), // main menu
    Box::new(SkipTextsPlan::new(13)), // oak speech
  ])
}

fn red_plan() -> ListPlan<Red> {
  ListPlan::new(vec![
    Box::new(SkipIntroPlan::new().with_auto_pass_after(214)), // Logo
    Box::new(SkipIntroPlan::new().with_auto_pass_after(322)), // Intro cutscene
    Box::new(SkipIntroPlan::new().with_no_up_select_b()), // main menu
    Box::new(MainMenuPlan::new()), // main menu
    Box::new(SkipTextsPlan::new(13)), // oak speech
  ])
}
