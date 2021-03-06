use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::metric::*;
use crate::metric::overworld::gen1::*;
use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverworldInteractPlanState {
  joypad_overworld_state: JoypadOverworldState,
}
impl PartialOrd for OverworldInteractPlanState {
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    Some(Ordering::Equal)
  }
}
impl PartialEq for OverworldInteractPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress JoypadOverworld inputs
pub struct OverworldInteractPlan<M> {
  // instance state
  joypad_overworld_state: JoypadOverworldState,

  // config state
  id: u8,
  metric: M,
}
impl OverworldInteractPlan<NullMetric> {
  pub fn with(id: u8) -> Self {
    Self::with_metric(id, NullMetric)
  }
  pub fn with_hidden_item() -> Self {
    Self::with(0xff)
  }
  pub fn with_card_key_door() -> Self {
    Self::with(0xfe)
  }
}
impl<M> OverworldInteractPlan<M> {
  pub fn with_metric(id: u8, metric: M) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      joypad_overworld_state: JoypadOverworldState::unknown(),

      // Default config state.
      id,
      metric,
    }
  }
  pub fn with_hidden_item_metric(metric: M) -> Self {
    Self::with_metric(0xff, metric)
  }
}
impl<R: MultiRom + JoypadOverworldAddresses + Gen1OverworldAddresses + Gen1DVAddresses, M: Metric<R>> Plan<R> for OverworldInteractPlan<M> {
  type Value = M::ValueType;

  fn save(&self) -> PlanState {
    PlanState::OverworldInteractState(OverworldInteractPlanState { joypad_overworld_state: self.joypad_overworld_state.clone() })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::OverworldInteractState(OverworldInteractPlanState { joypad_overworld_state, }) = state {
      self.joypad_overworld_state = joypad_overworld_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.joypad_overworld_state = JoypadOverworldState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { A }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    let (_, pressed) = self.joypad_overworld_state.get_input(input);
    if pressed.intersects(START) { return None; } // Opening start menu is not allowed
    Some(pressed & A)
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)> {
    gb.restore(s);
    gb.input(input);
    match get_overworld_interaction_result(gb) {
      OverworldInteractionResult::DisplayText { id } => {
        if id != self.id { return None; }
        if let Some(metric_value) = self.metric.evaluate(gb) {
          if !gb.is_at_input() { gb.step(); }
          Some((gb.save(), Some(metric_value)))
        } else { None }
      },
      OverworldInteractionResult::HiddenItem => {
        if 0xff != self.id { return None; }
        if let Some(metric_value) = self.metric.evaluate(gb) {
          if !gb.is_at_input() { gb.step(); }
          Some((gb.save(), Some(metric_value)))
        } else { None }
      },
      OverworldInteractionResult::CardKeyDoor => {
        if 0xfe != self.id { return None; }
        if let Some(metric_value) = self.metric.evaluate(gb) {
          if !gb.is_at_input() { gb.step(); }
          Some((gb.save(), Some(metric_value)))
        } else { None }
      },
      OverworldInteractionResult::NoAction => {
        gb.delay_step();
        let new_state = gb.save();
        self.joypad_overworld_state = JoypadOverworldState::from_gb(gb);
        Some((new_state, None))
      }
      _ => None,
    }
  }
}
