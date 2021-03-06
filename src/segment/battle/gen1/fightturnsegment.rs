use std::cmp::min;

use crate::constants::*;
use crate::metric::*;
use crate::metric::battle::*;
use crate::metric::battle::gen1::*;
use crate::rom::*;
use crate::segment::*;
use crate::segment::battle::gen1::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;


pub struct FightTurnSegment {
  player_move: Move,
  expect_crit: bool,
  expected_effect: Option<MoveEffectResult>,
  min_acceptable_damage: u16,
  exact_damage: bool,
  damage_slack: u16,
  enemy_attack: EnemyAttack,
  buffer_size: usize,
}
impl FightTurnSegment {
  #[allow(dead_code)]
  pub fn new(player_move: Move, expect_crit: bool, min_acceptable_damage: u16, enemy_attack: EnemyAttack) -> Self {
    Self {
      player_move,
      expect_crit,
      expected_effect: None,
      min_acceptable_damage,
      exact_damage: false,
      damage_slack: 0,
      enemy_attack,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  #[allow(dead_code)]
  pub fn with_expected_effect(self, expected_effect: Option<MoveEffectResult>) -> Self { Self { expected_effect, ..self } }
  #[allow(dead_code)]
  pub fn with_exact_damage(self) -> Self { Self { exact_damage: true, ..self } }
}
impl WithOutputBufferSize for FightTurnSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + Gen1FightTurnAddresses + Gen1MoveEffectAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + TextAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + AIChooseMoveAddresses + JoypadLowSensitivityAddresses + Gen1TrainerAIAddresses> Segment<R> for FightTurnSegment {
  type Key = u16;

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let move_infos = gbe.execute_state(&sb, MoveInfosFn::new(Who::Player)).get_value_assert_all_equal();
    let move_info = move_infos.into_iter().find(|move_info| move_info.mov == self.player_move).expect("move not found");
    assert!(move_info.max_damage > 0, "selected move does not do any damage");
    let max_damage = if self.expect_crit { move_info.max_crit_damage } else { move_info.max_damage };
    assert!(max_damage >= self.min_acceptable_damage, "min damage {} higher than max damage {}", self.min_acceptable_damage, max_damage);

    let enemy_move_infos = gbe.execute_state(&sb, MoveInfosFn::new(Who::Enemy)).get_value_assert_all_equal();
    let enemy_move_info = enemy_move_infos.into_iter().find(|move_info| move_info.mov == self.enemy_attack.mov).expect("enemy move not found");

    let player_mon_info = gbe.execute_state(&sb, BattleMonInfoFn::new(Who::Player)).get_value_assert_all_equal();
    let enemy_mon_info = gbe.execute_state(&sb, BattleMonInfoFn::new(Who::Enemy)).get_value_assert_all_equal();

    let is_ko = max_damage >= enemy_mon_info.hp;
    assert!(!is_ko || self.min_acceptable_damage == enemy_mon_info.hp);
    let primary_damage = if is_ko { enemy_mon_info.hp } else { if self.exact_damage { self.min_acceptable_damage } else { max_damage - self.damage_slack } };

    let player_moves_first = player_mon_info.stats.spd >= enemy_mon_info.stats.spd && (self.player_move == Move::QuickAttack || self.enemy_attack.mov != Move::QuickAttack);

    let has_after_attack_texts = self.expect_crit || move_info.is_effective; // side effect texts not included here
    let player_after_attack_texts_segment = if !has_after_attack_texts && self.expected_effect.unwrap_or(MoveEffectResult::NoEffect) == MoveEffectResult::NoEffect { None } else {
      Some(PlayerAfterAttackTextsSegment {
        crit: self.expect_crit,
        effective: move_info.is_effective,
        expected_effect: self.expected_effect,
        buffer_size: self.buffer_size,
      })};
    let has_player_after_attack_texts_segment = player_after_attack_texts_segment.is_some();
    let enemy_after_attack_texts_segment = self.enemy_attack.attack_type.get_after_attack_texts_segment(enemy_move_info.is_effective, self.buffer_size);

    let player_after_attack_metric = PlayerAfterAttackMetric::new(self.expected_effect, has_player_after_attack_texts_segment);
    let enemy_after_attack_metric = EnemyAfterAttackMetric::new(self.enemy_attack.attack_type, enemy_move_info.is_effective);

    let player_attack_metric = Gen1NormalHitMetric::with_expected_max_damage(move_info.max_damage, move_info.max_crit_damage);
    let player_attack_metric = player_attack_metric
      .map(|v| match v {
        FightTurnResult::Hit { damage }                      => if !self.expect_crit { damage } else { 0 },
        FightTurnResult::CriticalHit { damage }              => if self.expect_crit { damage } else { 0 },
        _ => 0 }) // filter unwanted result types
      .map(|damage| min(damage, enemy_mon_info.hp)) // cap damage at enemy hp
      .filter(|&damage| damage >= self.min_acceptable_damage && (!self.exact_damage || damage == self.min_acceptable_damage)) // filter damage that's too low (or high)
      .and_then_split(FnMetric::new(|gb| if !has_after_attack_texts && self.expected_effect.is_some() { MoveEffectMetric {}.expect(self.expected_effect.unwrap()).evaluate(gb) } else { Some(()) }))
      .and_then_split(FnMetric::new(|gb| if player_moves_first && !has_player_after_attack_texts_segment { TrainerAIMetric {}.expect(TrainerAIAction::NoAction).evaluate(gb) } else { Some(()) }));
    let player_attack_segment = TextSegment::with_metric(player_attack_metric).with_skip_ends(4).with_unbounded_buffer(); // mon used move
    let enemy_attack_metric = EnemyAttackMetric::new(self.enemy_attack.attack_type, enemy_move_info.max_damage, enemy_move_info.max_crit_damage, enemy_move_info.is_effective);
    let enemy_attack_segment = TextSegment::with_metric(enemy_attack_metric).with_skip_ends(4).with_unbounded_buffer(); // enemy mon used move

    let sb = SelectMoveSegment::new(self.player_move).with_buffer_size(self.buffer_size).execute(gbe, sb); // Select desired move

    if player_moves_first && is_ko {
      log::info!("FightTurnSegment: player moves first, ko");
      let mut sb_map = DelaySegment::new(
        MoveSegment::with_metric(Input::A,
          BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey))).with_buffer_size(self.buffer_size)
        .seq(player_attack_segment)
      ).with_primary_key(primary_damage).with_buffer_size(self.buffer_size).execute_split(gbe, sb);
      if let Some(player_after_attack_texts_segment) = player_after_attack_texts_segment {
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, player_after_attack_texts_segment.execute(gbe, sb))).collect();
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, DelaySegment::new(MoveSegment::with_metric(Input::B, player_after_attack_metric).with_buffer_size(self.buffer_size)).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
      }
      sb_map
    } else if player_moves_first {
      let mut sb_map = if let Some(player_after_attack_texts_segment) = player_after_attack_texts_segment {
        log::info!("FightTurnSegment: player moves first, separate moves");
        DelaySegment::new(
          MoveSegment::with_metric(Input::A,
            ExpectedAIChooseMoveMetric { expected_move: Some(self.enemy_attack.mov) }.and_then(BattleMoveOrderMetric {}).debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey))).with_buffer_size(self.buffer_size)
          .seq(player_attack_segment)
        ).with_primary_key(primary_damage).with_buffer_size(self.buffer_size)
        .seq_split(player_after_attack_texts_segment)
        .seq_split(DelaySegment::new(
          MoveSegment::with_metric(Input::B, player_after_attack_metric.and_then(TrainerAIMetric {}).expect(TrainerAIAction::NoAction)).with_buffer_size(self.buffer_size)
          .seq(enemy_attack_segment)
        ).with_buffer_size(self.buffer_size)).execute_split(gbe, sb)
      } else {
        log::info!("FightTurnSegment: player moves first, combined moves");
        DelaySegment::new(
          MoveSegment::with_metric(Input::A,
            ExpectedAIChooseMoveMetric { expected_move: Some(self.enemy_attack.mov) }.and_then(BattleMoveOrderMetric {}).expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey))).with_buffer_size(self.buffer_size)
          .seq(player_attack_segment)
          .seq_split(enemy_attack_segment.with_allowed_end_inputs(Input::B))
        ).with_primary_key(primary_damage).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
      };
      if let Some(enemy_after_attack_texts_segment) = enemy_after_attack_texts_segment {
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, enemy_after_attack_texts_segment.execute(gbe, sb))).collect();
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, DelaySegment::new(MoveSegment::with_metric(Input::B, enemy_after_attack_metric).with_buffer_size(self.buffer_size)).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
      }
      sb_map
    } else {
      let mut sb_map = if let Some(enemy_after_attack_texts_segment) = enemy_after_attack_texts_segment {
        log::info!("FightTurnSegment: enemy moves first, separate moves");
        DelaySegment::new(
          MoveSegment::with_metric(Input::A,
            ExpectedAIChooseMoveMetric { expected_move: Some(self.enemy_attack.mov) }.and_then(BattleMoveOrderMetric {}).debug_print().expect(MoveOrder::EnemyFirst).and_then(TrainerAIMetric {}).expect(TrainerAIAction::NoAction)).with_buffer_size(self.buffer_size)
          .seq(enemy_attack_segment)
        ).with_buffer_size(self.buffer_size)
        .seq(enemy_after_attack_texts_segment)
        .seq(DelaySegment::new(
          MoveSegment::with_metric(Input::A, enemy_after_attack_metric.and_then(BattleObedienceMetric {}).expect(BattleObedience::Obey)).with_buffer_size(self.buffer_size)
          .seq(player_attack_segment)
        ).with_primary_key(primary_damage).with_buffer_size(self.buffer_size)).execute_split(gbe, sb)
      } else {
        log::info!("FightTurnSegment: enemy moves first, combined moves");
        DelaySegment::new(
          MoveSegment::with_metric(Input::A,
            ExpectedAIChooseMoveMetric { expected_move: Some(self.enemy_attack.mov) }.and_then(BattleMoveOrderMetric {}).debug_print().expect(MoveOrder::EnemyFirst).and_then(TrainerAIMetric {}).expect(TrainerAIAction::NoAction)).with_buffer_size(self.buffer_size)
          .seq(enemy_attack_segment)
          .seq(player_attack_segment.with_allowed_end_inputs(Input::B))
        ).with_primary_key(primary_damage).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
      };
      if let Some(player_after_attack_texts_segment) = player_after_attack_texts_segment {
        let player_after_attack_texts_segment = player_after_attack_texts_segment.with_unbounded_buffer();
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, player_after_attack_texts_segment.execute(gbe, sb))).collect();
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, DelaySegment::new(MoveSegment::with_metric(Input::B, player_after_attack_metric).with_buffer_size(self.buffer_size)).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
      }
      sb_map
    }
  }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PlayerAfterAttackMetric {
  expected_effect: Option<MoveEffectResult>,
  has_after_attack_text_segment: bool,
}
impl PlayerAfterAttackMetric {
  fn new(expected_effect: Option<MoveEffectResult>, has_after_attack_text_segment: bool) -> Self {
    PlayerAfterAttackMetric {
      expected_effect,
      has_after_attack_text_segment,
    }
  }
}
impl<R: Rom + Gen1FightTurnAddresses + Gen1MoveEffectAddresses> Metric<R> for PlayerAfterAttackMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if let Some(effect) = self.expected_effect {
      if effect == MoveEffectResult::NoEffect || !self.has_after_attack_text_segment { // effects with texts are handled in the PlayerAfterAttackTextsSegment instead
        MoveEffectMetric {}.expect(effect).evaluate(gb)
      } else { Some(()) }
    } else { Some(()) }
  }
}
struct PlayerAfterAttackTextsSegment {
  crit: bool,
  effective: bool,
  expected_effect: Option<MoveEffectResult>,
  buffer_size: usize,
}
impl WithOutputBufferSize for PlayerAfterAttackTextsSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl<R: Rom + Gen1MoveEffectAddresses + TextAddresses> Segment<R> for PlayerAfterAttackTextsSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, mut sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {

    assert!(self.crit || self.effective || self.expected_effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect);
    if self.crit && self.effective { // if both crit/eff texts are present, skip one of them
      sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb);
    }
    if self.crit || self.effective {
      if self.expected_effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect {
        // crit/eff and side effect text
        sb = TextSegment::new().with_allowed_end_inputs(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb);
        sb = DelaySegment::new(MoveSegment::with_metric(Input::B, MoveEffectMetric {}.expect(self.expected_effect.unwrap()))).execute(gbe, sb);
        TextSegment::new().with_allowed_end_inputs(Input::A).with_skip_ends(2).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
      } else {
        // only crit/eff texts
        TextSegment::new().with_allowed_end_inputs(Input::A).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
      }
    } else {
      // only side effect text, trigger already checked in attack metric
      TextSegment::new().with_allowed_end_inputs(Input::A).with_skip_ends(2).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnemyAttack {
  pub mov: Move,
  pub attack_type: EnemyAttackType,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnemyAttackType {
  CriticalHit { damage: u16 },
  Hit { damage: u16 },
  HitNoEffect { damage: u16 },
  HitFailed,
  StatUpDown,
  EffectFailed,
}
impl EnemyAttackType {
  fn get_after_attack_texts_segment(&self, is_effective: bool, buffer_size: usize) -> Option<EnemyAfterAttackTextsSegment> {
    match self {
      EnemyAttackType::Hit { .. }
          | EnemyAttackType::HitNoEffect { .. } => if is_effective { Some(EnemyAfterAttackTextsSegment { attack_type: *self, is_effective, buffer_size, }) } else { None },
      EnemyAttackType::CriticalHit { .. }
          | EnemyAttackType::HitFailed
          | EnemyAttackType::StatUpDown
          | EnemyAttackType::EffectFailed => Some(EnemyAfterAttackTextsSegment { attack_type: *self, is_effective, buffer_size, }),
    }
  }
}
struct EnemyAttackMetric {
  enemy_attack_type: EnemyAttackType,
  expected_max_damage: u16,
  expected_max_crit_damage: u16,
  is_effective: bool,
}
impl EnemyAttackMetric {
  fn new(enemy_attack_type: EnemyAttackType, expected_max_damage: u16, expected_max_crit_damage: u16, is_effective: bool) -> Self {
    EnemyAttackMetric {
      enemy_attack_type,
      expected_max_damage,
      expected_max_crit_damage,
      is_effective,
    }
  }
}
impl<R: Rom + Gen1FightTurnAddresses + Gen1MoveEffectAddresses> Metric<R> for EnemyAttackMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    match self.enemy_attack_type {
      EnemyAttackType::CriticalHit { damage } => Gen1NormalHitMetric::with_expected_max_damage(self.expected_max_damage, self.expected_max_crit_damage).expect(FightTurnResult::CriticalHit { damage }).evaluate(gb),
      EnemyAttackType::Hit { damage } => Gen1NormalHitMetric::with_expected_max_damage(self.expected_max_damage, self.expected_max_crit_damage).expect(FightTurnResult::Hit { damage }).evaluate(gb),
      EnemyAttackType::HitNoEffect { damage } => if self.is_effective { 
          Gen1NormalHitMetric::with_expected_max_damage(self.expected_max_damage, self.expected_max_crit_damage).expect(FightTurnResult::Hit { damage }).evaluate(gb)
        } else {
          Gen1NormalHitMetric::with_expected_max_damage(self.expected_max_damage, self.expected_max_crit_damage).expect(FightTurnResult::Hit { damage }).and_then(MoveEffectMetric {}).expect(MoveEffectResult::NoEffect).evaluate(gb)
        },
      EnemyAttackType::HitFailed => Gen1NormalHitMetric::with_expected_max_damage(self.expected_max_damage, self.expected_max_crit_damage).expect(FightTurnResult::Failed).evaluate(gb),
      EnemyAttackType::StatUpDown => MoveEffectMetric {}.expect(MoveEffectResult::Success).evaluate(gb),
      EnemyAttackType::EffectFailed => MoveEffectMetric {}.expect(MoveEffectResult::Failed).evaluate(gb),
    }
  }
}
struct EnemyAfterAttackTextsSegment {
  attack_type: EnemyAttackType,
  is_effective: bool,
  buffer_size: usize,
}
impl WithOutputBufferSize for EnemyAfterAttackTextsSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl<R: Rom + TextAddresses> Segment<R> for EnemyAfterAttackTextsSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, mut sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    match self.attack_type {
      EnemyAttackType::CriticalHit { .. } => {
        if self.is_effective {
          sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb);
        }
        sb = TextSegment::new().with_allowed_end_inputs(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb);
      },
      EnemyAttackType::Hit { .. } | EnemyAttackType::HitNoEffect { .. } => {
        assert!(self.is_effective);
        sb = TextSegment::new().with_allowed_end_inputs(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb);
      },
      EnemyAttackType::EffectFailed | EnemyAttackType::HitFailed => {
        sb = TextSegment::new().with_allowed_end_inputs(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb);
      },
      EnemyAttackType::StatUpDown => {
        sb = TextSegment::new().with_allowed_end_inputs(Input::A).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb);
      },
    };
    Some(((), sb)).into_iter().collect()
  }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EnemyAfterAttackMetric {
  enemy_attack_type: EnemyAttackType,
  is_effective: bool,
}
impl EnemyAfterAttackMetric {
  fn new(  enemy_attack_type: EnemyAttackType, is_effective: bool) -> Self {
    EnemyAfterAttackMetric {
      enemy_attack_type,
      is_effective,
    }
  }
}
impl<R: Rom + Gen1FightTurnAddresses + Gen1MoveEffectAddresses> Metric<R> for EnemyAfterAttackMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if let EnemyAttackType::HitNoEffect { .. } = self.enemy_attack_type {
      if self.is_effective { MoveEffectMetric {}.expect(MoveEffectResult::NoEffect).evaluate(gb) } else { Some(()) } // already handled in EnemyAttackMetric
    } else { Some(()) }
  }
}
