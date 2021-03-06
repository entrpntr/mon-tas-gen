// traits

/// Minimal set of defined addresses to be usable in the Gb<R> wrapper.
pub trait Rom: BasicRomInfo + JoypadAddresses + RngAddresses + Sync + 'static {}
impl<R: BasicRomInfo + JoypadAddresses + RngAddresses + Sync + 'static> Rom for R {} 

/// Minimal set of defined addresses to be usable in the multi::Gb<R> wrapper.
pub trait MultiRom: Rom + JoypadLowSensitivityAddresses + InputIdentificationAddresses {}
impl<R: Rom + JoypadLowSensitivityAddresses + InputIdentificationAddresses> MultiRom for R {} 

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Generation {
  Gen1,
  Gen2,
}

pub trait BasicRomInfo {
  const ROM_FILE_NAME: &'static str;
  const GAME_NAME: &'static str;
  const SHA1: &'static str;
  const BOARD_NAME: &'static str;
  const GENERATION: Generation;
  #[inline] fn is_gen1() -> bool { Self::GENERATION == Generation::Gen1 }
  #[inline] fn is_gen2() -> bool { Self::GENERATION == Generation::Gen2 }
}
pub trait JoypadAddresses {
  const JOYPAD_READ_HI_ADDRESS: i32; // address in VBlank reading the joypad hi nybble
  const JOYPAD_READ_LO_ADDRESS: i32; // address in VBlank reading the joypad lo nybble
  const JOYPAD_READ_FIRST_ADDRESS: i32; // address in VBlank reading the first joypad nybble
  const JOYPAD_READ_LAST_ADDRESS: i32; // address in VBlank reading the last joypad nybble
  const JOYPAD_READ_LOCKED_ADDRESS: i32; // address in VBlank after reading both nybbles is done and the values are locked. Assumed to be after reading hi/lo without any branches or interrupts
  const JOYPAD_INPUT_MEM_ADDRESS: u16; // address the input read in VBlank is stored in
  const JOYPAD_USE_ADDRESSES: &'static [i32]; // addresses of usages of joypad inputs, if none of these are hit inbetween VBlank reads, the input is assumed to be irrelevant
  const JOYPAD_USE_DISCARD_ADDRESSES: Option<(u16, u16, u8, i32, i32, i32)>; // Gen1 discard logic. (ignore_input_counter_mem_add, ignore_flag_mem_add, ignore_flag_bit, calc_joy_pressed_add, check_ignore_flag_add, discard_add)
  const JOYPAD_USE_IGNORE_MASK_MEM_ADDRESSES: &'static [(i32, u16, i32)]; // JOYPAD_USE_ADDRESSES which have a ignore mask option. (use add, ignored inputs, skip add)
}
pub trait HandleMenuInputAddresses {
  const MENU_WRAPPING_ENABLED_MEM_ADDRESS: u16; // wMenuWrappingEnabled
  const CURRENT_MENU_ITEM_MEM_ADDRESS: u16; // wCurrentMenuItem
  const MAX_MENU_ITEM_MEM_ADDRESS: u16; // wMaxMenuItem
  const MENU_WATCHED_KEYS_MEM_ADDRESS: u16; // wMenuWatchedKeys
  const MENU_WATCH_MOVING_OOB_MEM_ADDRESS: u16; // wMenuWatchMovingOutOfBounds
  const MENU_JOYPAD_POLL_COUNT_MEM_ADDRESS: u16; // wMenuJoypadPollCount
  const MENU_A_BUTTON_PRIORITY: bool; // whether A button is checked before directional buttons in HandleMenuInput_. True in Yellow.
  const BATTLE_START_SAVED_MENU_ITEM: u16; // wBattleAndStartSavedMenuItem
  const LIST_SCROLL_OFFSET_MEM_ADDRESS: u16; // wListScrollOffset
  const LIST_COUNT_MEM_ADDRESS: u16; // wListCount
  const MENU_ITEM_QUANTITY_MEM_ADDRESS: u16; // wItemQuantity
  const MENU_MAX_ITEM_QUANTITY_MEM_ADDRESS: u16; // wMaxItemQuantity
}
pub trait JoypadLowSensitivityAddresses {
  const JOYPAD_HJOY6_MEM_ADDRESS: u16; // hJoy6
  const JOYPAD_HJOY7_MEM_ADDRESS: u16; // hJoy7
  const JOYPAD_LAST_MEM_ADDRESS: u16; // hJoyLast
  const JOYPAD_FRAME_COUNTER_MEM_ADDRESS: u16; // hFrameCounter
  const JOYPAD_FRAME_COUNTER_CHECK_ADDRESS: i32; // JoypadLowSensitivity.noNewlyPressedButtons
}
pub trait JoypadOverworldAddresses {
  const FLAGS_D733_MEM_ADDRESS: u16; // wFlags_D733
  const CUR_MAP_MEM_ADDRESS: u16; // wCurMap
  const FLAGS_D730_MEM_ADDRESS: u16; // wd730
  const SIMULATED_JOYPAD_OVERRIDE_MASK_MEM_ADDRESS: u16; // wOverrideSimulatedJoypadStatesMask
  const SIMULATED_JOYPAD_STATES_INDEX_MEM_ADDRESS: u16; // wSimulatedJoypadStatesIndex
  const SIMULATED_JOYPAD_STATES_END_MEM_ADDRESS: u16; // wSimulatedJoypadStatesEnd
}
pub trait RngAddresses {
  const RNG_MEM_ADDRESS: u16;
}
pub trait TextAddresses {
  const TEXT_BEFORE_JOYPAD_ADDRESS: i32; // in PrintLetterDelay, before call to Joypad happened
  const TEXT_JOYPAD_ADDRESS: i32; // the element of JOYPAD_USE_ADDRESSES which is used in PrintLetterDelay
  const TEXT_AFTER_JOYPAD_ADDRESS: i32; // in PrintLetterDelay, after call to Joypad happened
  const TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS: i32; // in PrintLetterDelay, after wait loop is done
  const TEXT_END_NOINPUT_ADDRESSES: &'static [i32]; // List of addresses signifying that the text output has ended without user interaction. Must include any address that leads to a different input use, and happen before any metric is supposed to be collected.
  const TEXT_END_WITHINPUT_ADDRESSES: &'static [i32]; // List of addresses signifying that the text output has ended and the user is expected to press a button to continue.
  const TEXT_DELAY_FRAME_COUNTER_MEM_ADDRESS: u16; // variable which is checked in the busy-wait loop in PrintLetterDelay
  const TEXT_SAFE_CONFLICTING_INPUT_ADDRESSES: &'static [i32]; // addresses of usages of joypad inputs which are safe for PrintLetterDelay to spill into. Should generally be idempotent to neutral inputs without losing additional frames.
}
pub trait Gen1OverworldAddresses {
  const OVERWORLD_BEFORE_JOYPAD_ADDRESS: i32; // in JoypadOverworld, before call to Joypad happened
  const OVERWORLD_JOYPAD_ADDRESS: i32; // the element of JOYPAD_USE_ADDRESSES which is used in JoypadOverworld
  const OVERWORLD_AFTER_JOYPAD_ADDRESS: i32; // in JoypadOverworld, after call to Joypad happened
  const OVERWORLD_WARP_FOUND_ADDRESS: i32;
  const OVERWORLD_WARP_MAP_MEM_ADDRESS: u16;
  const OVERWORLD_WARP_ENTRANCE_MEM_ADDRESS: u16;
  const OVERWORLD_FLY_DUNGEON_WARP_FOUND_ADDRESS: i32;
  const OVERWORLD_HIDDEN_ITEM_FOUND_ADDRESS: i32; // CheckForHiddenObjectOrBookshelfOrCardKeyDoor.hiddenObjectNotFound - 7
  const OVERWORLD_CARD_KEY_DOOR_FOUND_ADDRESS: i32; // PrintCardKeyText.cardKeyDoorInFrontOfPlayer + 7
  const OVERWORLD_A_BUTTON_Z_CHECK_1: i32; // .displayDialogue - 12
  const OVERWORLD_A_BUTTON_Z_CHECK_2: i32; // .displayDialogue - 3
  const OVERWORLD_YELLOW_PIKACHU_INTERACTION_ADDRESS: i32; // 3f:4f20 InitializePikachuTextID
  const OVERWORLD_DISPLAY_TEXT_ADDRESS: i32;
  const OVERWORLD_DISPLAY_TEXT_ID_MEM_ADDRESS: u16;
  const OVERWORLD_DISPLAY_TEXT_FAILED_ADDRESS: i32; // .noDirectionButtonsPressed - 3
  const OVERWORLD_INIT_BATTLE_COMMON_ADDRESS: i32;
  const OVERWORLD_NEW_BATTLE_NO_BATTLE: i32; // .newBattle + 3
  const OVERWORLD_BATTLE_SPECIES_MEM_ADDRESS: u16;
  const OVERWORLD_BATTLE_LEVEL_MEM_ADDRESS: u16;
  const OVERWORLD_TURNING_DONE_ADDRESS: i32;
  const OVERWORLD_MOVING_DIRECTION_MEM_ADDRESS: u16;
  const OVERWORLD_JUMP_LEDGE_ADDRESS: i32; // HandleLedges.foundMatch + 4
  const OVERWORLD_LAND_COLLISION_ADDRESS: i32;
  const OVERWORLD_LAND_COLLISION_NO_WARP_ADDRESS: i32;
  const OVERWORLD_WATER_COLLISION_ADDRESS: i32;
  const OVERWORLD_HANDLE_BLACKOUT_ADDRESS: i32;
  const OVERWORLD_WALKED_PRE_ADDRESS: i32; // .notSafariZone
  const OVERWORLD_WALKED_ADDRESS: i32;
  const OVERWORLD_NO_ACTION_ADDRESS: i32;
  const OVERWORLD_TURNFRAME_DIRECTION_MEM_ADDRESS: u16; // wPlayerLastStopDirection
  const OVERWORLD_TURNFRAME_CHECK_MEM_ADDRESS: u16; // wCheckFor180DegreeTurn
}
pub trait Gen2MapEventsAddresses {
  const OVERWORLD_BEFORE_JOYPAD_ADDRESS: i32;
  const OVERWORLD_JOYPAD_ADDRESS: i32;
  const OVERWORLD_AFTER_JOYPAD_ADDRESS: i32;
  const PLAYER_EVENTS_ADDRESS: i32; // PlayerEvents
  const PLAYER_SCRIPT_RUNNING_MEM_ADDRESS: u16; // wScriptRunning
  const PLAYER_EVENTS_SEEN_BY_TRAINER_ADDRESS: i32; // in CheckTrainerBattle3
  const PLAYER_EVENTS_MAP_CONNECTION_ADDRESS: i32; // CheckTileEvent.map_connection
  const PLAYER_EVENTS_WARP_ADDRESS: i32; // CheckTileEvent.not_pit
  const PLAYER_EVENTS_FALL_ADDRESS: i32; // CheckTileEvent.pit
  const PLAYER_EVENTS_MAP_COORD_EVENT_ADDRESS: i32; // CheckTileEvent.coord_event
  const PLAYER_EVENTS_COUNT_STEP_EVENT_ADDRESS: i32; // CountStep.doscript
  const PLAYER_EVENTS_HATCH_ADDRESS: i32; // CountStep.hatch
  const PLAYER_EVENTS_RANDOM_ENCOUNTER_ADDRESS: i32; // RandomEncounter.done
  const PLAYER_EVENTS_RANDOM_ENCOUNTER_SPECIES_MEM_ADDRESS: u16; // wTempWildMonSpecies
  const PLAYER_EVENTS_RANDOM_ENCOUNTER_LEVEL_MEM_ADDRESS: u16; // wCurPartyLevel
  const PLAYER_EVENTS_REENTRY_SCRIPT_ADDRESS: i32; // RunMemScript.runScript
  const PLAYER_EVENTS_SCENE_SCRIPT_ADDRESS: i32; // RunSceneScript.runScript
  const PLAYER_EVENTS_END_BUG_CONTEST_ADDRESS: i32; // CheckTimeEvents.end_bug_contest
  const PLAYER_EVENTS_PHONE_CALL_ADDRESS: i32; // CheckPhoneCall.call
  const PLAYER_EVENTS_WHIRLPOOL_FORCED_MOVEMENT_ADDRESS: i32; // DoPlayerMovement.CheckTile_whirlpool
  const PLAYER_EVENTS_FORCED_MOVEMENT_ADDRESS: i32; // DoPlayerMovement.continue_walk
  const PLAYER_EVENTS_TURNING_ADDRESS: i32; // DoPlayerMovement.CheckTurning_turning
  const PLAYER_EVENTS_WALKING_DIRECTION_MEM_ADDRESS: u16; // wWalkingDirection
  const PLAYER_EVENTS_STEP_WALK_ADDRESS: i32; // DoPlayerMovement.walk
  const PLAYER_EVENTS_STEP_BIKE_ADDRESS: i32; // DoPlayerMovement.fast
  const PLAYER_EVENTS_STEP_BIKE_UPHILL_ADDRESS: i32; // DoPlayerMovement.bike_uphill
  const PLAYER_EVENTS_STEP_ICE_ADDRESS: i32; // DoPlayerMovement.ice
  const PLAYER_EVENTS_STEP_SURF_ADDRESS: i32; // DoPlayerMovement.surf_step
  const PLAYER_EVENTS_STEP_OUT_OF_WATER_ADDRESS: i32; // DoPlayerMovement.ExitWater
  const PLAYER_EVENTS_JUMP_LEDGE_ADDRESS: i32; // DoPlayerMovement.TryJump_jump
  const PLAYER_EVENTS_EDGE_WARP_ADDRESS: i32; // DoPlayerMovement.CheckWarp_warp
  const PLAYER_EVENTS_INTERACT_OBJECT_SCRIPT_ADDRESS: i32; // TryObjectEvent.script
  const PLAYER_EVENTS_INTERACT_OBJECT_ITEMBALL_ADDRESS: i32; // TryObjectEvent.itemball
  const PLAYER_EVENTS_INTERACT_OBJECT_TRAINER_ADDRESS: i32; // TryObjectEvent.trainer
  const PLAYER_EVENTS_INTERACT_BG_READ_ADDRESS: i32; // TryBGEvent.read
  const PLAYER_EVENTS_INTERACT_BG_HIDDEN_ITEM_ADDRESS: i32; // TryBGEvent.hiddenItem
  const PLAYER_EVENTS_INTERACT_BG_THENREAD_ADDRESS: i32; // TryBGEvent.thenread
  const PLAYER_EVENTS_INTERACT_TILE_COLLISION_ADDRESS: i32; // TryTileCollisionEvent.done
  const PLAYER_EVENTS_START_MENU_ADDRESS: i32; // CheckMenuOW.Start
  const PLAYER_EVENTS_SELECT_MENU_ADDRESS: i32; // CheckMenuOW.Select
  const PLAYER_EVENTS_NO_EVENTS_ADDRESS: i32; // PlayerEvents.noEvents
  const PLAYER_DIRECTION_MEM_ADDRESS: u16; // wPlayerDirection
}
pub trait InputIdentificationAddresses {
  const II_ADDRESSES: &'static [(i32, i32, i32, &'static str)];
}
pub trait Gen1MapAddresses {
  const SURF_STATE_MEM_ADDRESS: u16; // wWalkBikeSurfState (2 = surfing)
  const MAP_TILESET_MEM_ADDRESS: u16; // wCurMapTileset
  const TILE_PAIR_COLLISIONS_LAND_ADDRESS: i32; // TilePairCollisionsLand
  const TILE_PAIR_COLLISIONS_WATER_ADDRESS: i32; // TilePairCollisionsWater
  const MAP_TILESET_COLLISION_PTR_BANK_OFFSET: i32; // bank * 0x1_0000
  const MAP_TILESET_COLLISION_PTR_MEM_ADDRESS: u16; // wTilesetCollisionPtr
  const MAP_TILESET_BANK_MEM_ADDRESS: u16; // wTilesetBank
  const MAP_TILESET_BLOCKS_PTR_MEM_ADDRESS: u16; // wTilesetBlocksPtr
  const MAP_WIDTH_MEM_ADDRESS: u16; // wCurMapWidth
  const MAP_HEIGHT_MEM_ADDRESS: u16; // wCurMapHeight
  const OVERWORLD_MAP_MEM_ADDRESS: u16; // wOverworldMap
  const MAP_SPRITE_STATE_DATA_2_MEM_ADDRESS: u16; // wSpriteStateData2
  const MAP_SPRITE_DATA_MEM_ADDRESS: u16; // wMapSpriteData
  const MAP_MISSABLE_OBJECT_LIST_MEM_ADDRESS: u16; // wMissableObjectList
  const MAP_MISSABLE_OBJECT_FLAGS_MEM_ADDRESS: u16; // wMissableObjectFlags
  const MAP_INDEX_MEM_ADDRESS: u16; // wCurMap
  const MAP_HEADER_BANKS_ADDRESS: i32; // MapHeaderBanks
  const MAP_TEXT_PTR_MEM_ADDRESS: u16; // wMapTextPtr
  const TALK_TO_TRAINER_FUNCTION_ADDRESS: i32; // TalkToTrainer
  const PLAYER_X_POS_MEM_ADDRESS: u16; // wXCoord
  const PLAYER_Y_POS_MEM_ADDRESS: u16; // wYCoord
}
pub trait Gen2MapAddresses {
  const OVERWORLD_MAP_MEM_ADDRESS: u16; // wOverworldMap
  const MAP_WIDTH_MEM_ADDRESS: u16; // wMapWidth
  const MAP_HEIGHT_MEM_ADDRESS: u16; // wMapHeight
  const MAP_SCRIPTS_BANK_MEM_ADDRESS: u16; // wMapScriptsBank
  const TILESET_COLLISION_PTR_MEM_ADDRESS: u16; // wTilesetCollisionAddress
  const TILESET_COLLISION_BANK_MEM_ADDRESS: u16; // wTilesetCollisionBank
  const TILE_COLLISION_TABLE_ADDRESS: i32; // TileCollisionTable
  const MAP_OBJECTS_MEM_ADDRESS: u16; // wMapObjects
  const EVENT_FLAGS_MEM_ADDRESS: u16; // wEventFlags
  const PLAYER_X_MEM_ADDRESS: u16; // wPlayerStandingMapX
  const PLAYER_Y_MEM_ADDRESS: u16; // wPlayerStandingMapY
}
pub trait Gen1DVAddresses {
  const AFTER_DV_GENERATION_ADDRESSES: &'static [i32]; // addresses after DVs have been rolled into register a and b
}
pub trait Gen2DVAddresses {
  const AFTER_DV_GENERATION_ADDRESS: i32; // GeneratePartyMonStats.initializeDVs
  const AFTER_WILD_DV_GENERATION_ADDRESS: i32; // LoadEnemyMon.UpdateDVs
}
pub trait TrainerIDAddresses {
  const TRAINER_ID_AFTER_GENERATION_ADDRESS: i32; // after trainer ID is determined
  const TRAINER_ID_MEM_ADDRESS: u16; // wPlayerID
}
pub trait BattleDetermineMoveOrderAddresses {
  const DETERMINE_MOVE_ORDER_START_ADDRESS: i32; // Before the move order check starts
  const MOVE_ORDER_PLAYER_FIRST_ADDRESS: i32; // player goes first
  const MOVE_ORDER_ENEMY_FIRST_ADDRESS: i32; // enemy goes first
}
pub trait AIChooseMoveAddresses {
  const AFTER_AI_CHOOSE_MOVE_ADDRESS: i32; // BattleTurn.not_disconnected
  const CUR_ENEMY_MOVE_MEM_ADDRESS: u16; // wCurEnemyMove
}
pub trait BattleObedienceAddresses {
  const CHECK_OBEDIENCE_START_ADDRESS: i32; // Before the obedience check starts
  const CHECK_OBEDIENCE_OBEY_ADDRESS: i32; // Address reached when obeying
  const CHECK_OBEDIENCE_DISOBEY_ADDRESS: i32; // Address reached when disobeying
}
pub trait Gen1FightTurnAddresses {
  const CUR_MOVE_INDEX_MEM_ADDRESS: u16; // wPlayerMoveListIndex
  const CRITICAL_HIT_MEM_ADDRESS: u16; // wCriticalHitOrOHKO
  const AFTER_MAX_DAMAGE_CALC_ADDRESS: i32; // RandomizeDamage
  const CUR_DAMAGE_MEM_ADDRESS: u16; // wDamage
  const AFTER_PLAYER_HIT_CHECK_ADDRESS: i32; // handleIfPlayerMoveMissed
  const AFTER_ENEMY_HIT_CHECK_ADDRESS: i32; // handleIfEnemyMoveMissed
  const ATTACK_MISSED_MEM_ADDRESS: u16; // wMoveMissed
}
pub trait Gen1TrainerAIAddresses {
  const TRAINER_AI_START_ADDRESS: i32; // TrainerAI
  const TRAINER_AI_NO_ACTION_ADDRESS: i32; // ExecuteEnemyMove
  const TRAINER_AI_SWITCH_ADDRESS: i32; // SwitchEnemyMon
  const TRAINER_AI_XITEM_ADDRESS: i32; // AIIncreaseStat
  const TRAINER_AI_GUARD_SPEC_ADDRESS: i32; // AIUseGuardSpec
  const TRAINER_AI_FULL_HEAL_ADDRESS: i32; // AIUseFullHeal
  const TRAINER_AI_POTION_ADDRESS: i32; // AIRecoverHP
  const TRAINER_AI_FULL_RESTORE_ADDRESS: i32; // AIUseFullRestore
}
pub trait Gen1MoveEffectAddresses {
  const MOVE_EFFECT_START_ADDRESS: i32; // JumpMoveEffect
  const MOVE_EFFECT_NO_EFFECT_ADDRESS: i32; // JumpMoveEffect + 3
  const MOVE_EFFECT_SLEEP_SUCCESS_ADDRESS: i32; // SleepEffect.setSleepCounter + 7 (a contains sleep counter)
  const MOVE_EFFECT_SLEEP_FAILED_ADDRESS: i32; // SleepEffect.didntAffect
  const MOVE_EFFECT_POISON_SUCCESS_ADDRESS: i32; // PoisonEffect.inflictPoison
  const MOVE_EFFECT_POISON_FAILED_ADDRESS: i32; // PoisonEffect.didntAffect
  const MOVE_EFFECT_FREEZE_BURN_PARALYZE_PLAYER_SUCCESS_ADDRESS: i32; // FreezeBurnParalyzeEffect.next1 + 7
  const MOVE_EFFECT_FREEZE_BURN_PARALYZE_ENEMY_SUCCESS_ADDRESS: i32; // FreezeBurnParalyzeEffect.next2 + 7
  const MOVE_EFFECT_DEFROSTED_SUCCESS_ADDRESS: i32; // CheckDefrost.common
  const MOVE_EFFECT_STAT_UP_SUCCESS_ADDRESS: i32; // StatModifierUpEffect.ok
  const MOVE_EFFECT_STAT_UP_NOTHING_HAPPENED_ADDRESS: i32; // PrintNothingHappenedText
  const MOVE_EFFECT_STAT_DOWN_SUCCESS_ADDRESS: i32; // StatModifierDownEffect.recalculateStat
  const MOVE_EFFECT_STAT_DOWN_FAILED_ADDRESS: i32; // MoveMissed
  const MOVE_EFFECT_STAT_DOWN_NOTHING_HAPPENED_ADDRESS: i32; // CantLowerAnymore + 4
  const MOVE_EFFECT_BIDE_ADDRESS: i32; // ThrashPetalDanceEffect - 8 (a contains turn counter)
  const MOVE_EFFECT_THRASH_PETALDANCE_ADDRESS: i32; // SwitchAndTeleportEffect - 8 (a contains turn counter)
  const MOVE_EFFECT_MULTI_HIT_ADDRESS: i32; // TwoToFiveAttacksEffect.saveNumberOfHits (a contains number of hits)
  const MOVE_EFFECT_FLINCHED_ADDRESS: i32; // FlinchSideEffect.gotEffectChance + 5
  const MOVE_EFFECT_TRAPPING_ADDRESS: i32; // TrappingEffect.setTrappingCounter + 1 (a contains turn counter)
  const MOVE_EFFECT_CONFUSION_SUCCESS_ADDRESS: i32; // ConfusionSideEffectSuccess.confuseTarget + 14 (a contains turn counter)
  const MOVE_EFFECT_CONFUSION_FAILED_ADDRESS: i32; // ConfusionEffectFailed
  const MOVE_EFFECT_PARALYZE_SUCCESS_ADDRESS: i32; // ParalyzeEffect_.hitTest + 16
  const MOVE_EFFECT_PARALYZE_FAILED_ADDRESS: i32; // ParalyzeEffect_.didntAffect
  const MOVE_EFFECT_PARALYZE_NO_EFFECT_ADDRESS: i32; // ParalyzeEffect_.doesntAffect
  const MOVE_EFFECT_MIMIC_SUCCESS_ADDRESS: i32; // MimicEffect.playerTurn
  const MOVE_EFFECT_MIMIC_FAILED_ADDRESS: i32; // MimicEffect.mimicMissed
  const MOVE_EFFECT_DISABLE_SUCCESS_ADDRESS: i32; // DisableEffect.playerTurnNotLinkBattle
  const MOVE_EFFECT_DISABLE_FAILED_ADDRESS: i32; // DisableEffect.moveMissed
  const MOVE_EFFECT_HEAL_FAILED_ADDRESS: i32; // HealEffect_.failed
  const MOVE_EFFECT_TELEPORT_FAILED_ADDRESS: i32; // SwitchAndTeleportEffect.notWildBattle2
}
pub trait Gen2FightTurnAddresses {
  const NEXT_BATTLE_COMMAND_ADDRESS: i32; // DoMove.ReadMoveEffectCommand (for next command)
  const BATTLE_COMMAND_DOTURN_ADDRESS: i32; // BattleCommand_DoTurn
  const OUT_OF_PP_ADDRESS: i32; // BattleCommand_DoTurn.out_of_pp
  const BATTLE_COMMAND_DAMAGEVARIATION_ADDRESS: i32; // BattleCommand_DamageVariation
  const CUR_DAMAGE_MEM_ADDRESS: u16; // wCurDamage
  const BATTLE_COMMAND_LOWERSUB_ADDRESS: i32; // BattleCommand_LowerSub
  const BATTLE_COMMAND_MOVEANIMNOSUB_ADDRESS: i32; // BattleCommand_MoveAnimNoSub
  const ATTACK_MISSED_MEM_ADDRESS: u16; // wAttackMissed
  const EFFECT_FAILED_MEM_ADDRESS: u16; // wEffectFailed
  const CRITICAL_HIT_MEM_ADDRESS: u16; // wCriticalHit
  const CUR_MOVE_INDEX_MEM_ADDRESS: u16; // wCurMoveNum
  const FULLY_PARALYZED_ADDRESS: i32; // Address reached when mon is fully paralyzed
}
pub trait BattleMovesInfoAddresses {
  const MOVES_ADDRESS: i32; // Moves
  const MOVES_ENTRY_LENGTH: i32; // length of a single move

  const GEN2_BADGES_MEM_ADDRESS: u16; // wJohtoBadges
  const TYPE_MATCHUPS_ADDRESS: i32; // TypeMatchups
}
pub trait BattleMonInfoAddresses {
  const BATTLE_MON_STRUCT_MEM_ADDRESS: u16; // wBattleMon
  const BATTLE_MON_STAT_LEVELS_MEM_ADDRESS: u16; // wPlayerStatLevels
  const GEN1_PARTY_MON_STATS_BASE_MEM_ADDRESS: u16; // wPartyMon1Attack
  const GEN1_PARTY_MON_STRUCT_LEN: u16; // wPartyMon2 - wPartyMon1
  const GEN1_PLAYER_MON_NUMBER_MEM_ADDRESS: u16; // wPlayerMonNumber
  const GEN2_BATTLE_MON_ORIG_STATS_MEM_ADDRESS: u16; // wPlayerStats

  const ENEMY_MON_STRUCT_MEM_ADDRESS: u16; // wEnemyMon
  const ENEMY_MON_STAT_LEVELS_MEM_ADDRESS: u16; // wEnemyStatLevels
  const GEN1_BASE_STATS_BASE_ADDRESS: i32; // BaseStats
  const GEN1_BASE_STATS_LEN: i32; // MonBaseStatsEnd - MonBaseStats
  const GEN2_ENEMY_MON_ORIG_STATS_MEM_ADDRESS: u16; // wEnemyStats
}
pub trait BattleCatchMonAddresses {
  const CATCH_SUCCESS_ADDRESS: i32; // Address reached when catching succeeded.
  const CATCH_FAIL_ADDRESS: i32; // Address reached when catching failed.
}
pub trait Gen2BattleSwitchMonAddresses {
  const SWITCH_DECIDED_ADDRESS: i32; // Address reached when mon to switch to is decided.
  const SWITCH_SPECIES_MEM_ADDRESS: u16; // Memory address containing the new mon's species.
  const SWITCH_LEVEL_MEM_ADDRESS: u16; // Memory address containing the new mon's level.
}
pub trait Gen2BattleSpiteAddresses {
  const SPITE_SUCCESS_ADDRESS: i32; // Address reached when spite succeeded.
  const SPITE_FAIL_ADDRESS: i32; // Address reached when spite failed.
}
pub trait Gen2BattleMultiHitAddresses {
  const MULTI_HIT_ADDRESS: i32; // Address reached when multi-hit loop count is determined.
}
pub trait RoamMonAddresses {
  const AFTER_ROAM_MON_UPDATE_ADDRESS: i32; // Address reached after roaming mon locations were updated.
  const ROAM_MON_DATA_MEM_ADDRESS: u16; // Memory address containing the roaming mon data.
}
pub trait HallOfFameAddresses {
  const HALL_OF_FAME_AFTER_SAVING_ADDRESS: i32; // After save is complete, safe to reset now.
}
pub trait VermilionTrashCanAddresses {
  const FIRST_TRASH_CAN_MEM_ADDRESS: u16; // wFirstLockTrashCanIndex
  const AFTER_FIRST_TRASH_CAN_ADDRESS: i32; // VermilionCity_Script.initCityScript - 1
  const SECOND_TRASH_CAN_MEM_ADDRESS: u16; // wSecondLockTrashCanIndex
  const AFTER_SECOND_TRASH_CAN_ADDRESS: i32; // GymTrashScript.trySecondLock - 2
}
pub trait FlyLocationsAddresses {
  const FLY_LOCATIONS_MEM_ADDRESS: u16; // wFlyLocationsList
}
pub trait PushBoulderAddresses {
  const AFTER_TRY_PUSHING_BOULDER_ADDRESS: i32; // RunMapScript + 11
  const BOULDER_FLAGS_MEM_ADDRESS: u16; // wFlags_0xcd60
}
// Gen 1
#[allow(dead_code)]
pub enum Red {}
impl BasicRomInfo for Red {
  const ROM_FILE_NAME: &'static str = "roms/red.gb";
  const GAME_NAME: &'static str = "Pokemon - Red Version (USA, Europe)";
  const SHA1: &'static str = "EA9BCAE617FDF159B045185467AE58B2E4A48B9A";
  const BOARD_NAME: &'static str = "MBC3 ROM+RAM+BATTERY";
  const GENERATION: Generation = Generation::Gen1;
}
#[allow(dead_code)]
pub enum Blue {}
impl BasicRomInfo for Blue {
  const ROM_FILE_NAME: &'static str = "roms/blue.gb";
  const GAME_NAME: &'static str = "Pokemon - Blue Version (USA, Europe)";
  const SHA1: &'static str = "D7037C83E1AE5B39BDE3C30787637BA1D4C48CE2";
  const BOARD_NAME: &'static str = "MBC3 ROM+RAM+BATTERY";
  const GENERATION: Generation = Generation::Gen1;
}
macro_rules! impl_red_blue_common_addresses {
  ($($t:ty),+) => {
    $(
    impl JoypadAddresses for $t {
      const JOYPAD_READ_HI_ADDRESS: i32 = 0x0_016F;
      const JOYPAD_READ_LO_ADDRESS: i32 = 0x0_018D;
      const JOYPAD_READ_FIRST_ADDRESS: i32 = Self::JOYPAD_READ_HI_ADDRESS;
      const JOYPAD_READ_LAST_ADDRESS: i32 = Self::JOYPAD_READ_LO_ADDRESS;
      const JOYPAD_READ_LOCKED_ADDRESS: i32 = 0x0_018F;
      const JOYPAD_INPUT_MEM_ADDRESS: u16 = 0xfff8; // hJoyInput
      const JOYPAD_USE_ADDRESSES: &'static [i32] = &[
        0x3_4000, // _Joypad
      ];
      const JOYPAD_USE_DISCARD_ADDRESSES: Option<(u16, u16, u8, i32, i32, i32)> = Some((0xd13a, 0xd730, 5, 0x3_4012, 0x3_4017, 0x3_4034)); // (ignore_input_counter_mem_add, ignore_flag_mem_add, ignore_flag_bit, calc_joy_pressed_add, check_ignore_flag_add, discard_add)
      const JOYPAD_USE_IGNORE_MASK_MEM_ADDRESSES: &'static [(i32, u16, i32)] = &[(0x3_4000, 0xCD6B, 0x3_4002)]; // wJoyIgnore
    }
    impl JoypadLowSensitivityAddresses for $t {
      const JOYPAD_HJOY6_MEM_ADDRESS: u16 = 0xffb6; // hJoy6
      const JOYPAD_HJOY7_MEM_ADDRESS: u16 = 0xffb7; // hJoy7
      const JOYPAD_LAST_MEM_ADDRESS: u16 = 0xffb1; // hJoyLast
      const JOYPAD_FRAME_COUNTER_MEM_ADDRESS: u16 = 0xffd5; // hFrameCounter
      const JOYPAD_FRAME_COUNTER_CHECK_ADDRESS: i32 = 0x0_3849; // JoypadLowSensitivity.noNewlyPressedButtons
    }
    impl HandleMenuInputAddresses for $t {
      const MENU_WRAPPING_ENABLED_MEM_ADDRESS: u16 = 0xcc4a; // wMenuWrappingEnabled
      const CURRENT_MENU_ITEM_MEM_ADDRESS: u16 = 0xcc26; // wCurrentMenuItem
      const MAX_MENU_ITEM_MEM_ADDRESS: u16 = 0xcc28; // wMaxMenuItem
      const MENU_WATCHED_KEYS_MEM_ADDRESS: u16 = 0xcc29; // wMenuWatchedKeys
      const MENU_WATCH_MOVING_OOB_MEM_ADDRESS: u16 = 0xcc37; // wMenuWatchMovingOutOfBounds
      const MENU_JOYPAD_POLL_COUNT_MEM_ADDRESS: u16 = 0xcc34; // wMenuJoypadPollCount
      const MENU_A_BUTTON_PRIORITY: bool = false; // whether A button is checked before directional buttons in HandleMenuInput_. True in Yellow.
      const BATTLE_START_SAVED_MENU_ITEM: u16 = 0xcc2d; // wBattleAndStartSavedMenuItem
      const LIST_SCROLL_OFFSET_MEM_ADDRESS: u16 = 0xcc36; // wListScrollOffset
      const LIST_COUNT_MEM_ADDRESS: u16 = 0xd12a; // wListCount
      const MENU_ITEM_QUANTITY_MEM_ADDRESS: u16 = 0xcf96; // wItemQuantity
      const MENU_MAX_ITEM_QUANTITY_MEM_ADDRESS: u16 = 0xcf97; // wMaxItemQuantity
    }
    impl JoypadOverworldAddresses for $t {
      const FLAGS_D733_MEM_ADDRESS: u16 = 0xd733; // wFlags_D733
      const CUR_MAP_MEM_ADDRESS: u16 = 0xd35e; // wCurMap
      const FLAGS_D730_MEM_ADDRESS: u16 = 0xd730; // wd730
      const SIMULATED_JOYPAD_OVERRIDE_MASK_MEM_ADDRESS: u16 = 0xcd3b; // wOverrideSimulatedJoypadStatesMask
      const SIMULATED_JOYPAD_STATES_INDEX_MEM_ADDRESS: u16 = 0xcd38; // wSimulatedJoypadStatesIndex
      const SIMULATED_JOYPAD_STATES_END_MEM_ADDRESS: u16 = 0xccd3; // wSimulatedJoypadStatesEnd
    }
    impl RngAddresses for $t {
      const RNG_MEM_ADDRESS: u16 = 0xffd3;
    }
    impl TextAddresses for $t {
      const TEXT_BEFORE_JOYPAD_ADDRESS: i32 = 0x0_38F6;
      const TEXT_JOYPAD_ADDRESS: i32 = 0x3_4000; // _Joypad
      const TEXT_AFTER_JOYPAD_ADDRESS: i32 = 0x0_38F9;
      const TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS: i32 = 0x0_390F;
      const TEXT_END_NOINPUT_ADDRESSES: &'static [i32] = &[
        // 0x0_1AAD, // <DONE> Char57; Address of the character $57 handler, used to end the text without any input required
        0x0_1B55, // NextTextCommand; called when the next text command is being processed.
      ];
      const TEXT_END_WITHINPUT_ADDRESSES: &'static [i32] = &[
        0x0_1AF8, // <_CONT> Char4B; Address of the character $4B handler, used to scroll text up after a button press
        0x0_1AB4, // <PARA> Char51; Address of the character $51 handler, used start a new paragraph of text
        0x0_1A95, // <PROMPT> Char58; Address of the character $58 handler, used to wait for a button press before ending the text
      ];
      const TEXT_DELAY_FRAME_COUNTER_MEM_ADDRESS: u16 = 0xffd5;
      const TEXT_SAFE_CONFLICTING_INPUT_ADDRESSES: &'static [i32] = &[
        0x0_3883, // WaitForTextScrollButtonPress
      ];
    }
    impl Gen1OverworldAddresses for $t {
      const OVERWORLD_BEFORE_JOYPAD_ADDRESS: i32 = 0x0_0F57;
      const OVERWORLD_JOYPAD_ADDRESS: i32 = 0x3_4000; // _Joypad
      const OVERWORLD_AFTER_JOYPAD_ADDRESS: i32 = 0x0_0F5A;
      const OVERWORLD_WARP_FOUND_ADDRESS: i32 = 0x0_073C; // WarpFound2
      const OVERWORLD_WARP_MAP_MEM_ADDRESS: u16 = 0xFF8B; // hWarpDestinationMap
      const OVERWORLD_WARP_ENTRANCE_MEM_ADDRESS: u16 = 0xD42F; // wDestinationWarpID
      const OVERWORLD_FLY_DUNGEON_WARP_FOUND_ADDRESS: i32 = 0x0_0965; // HandleFlyWarpOrDungeonWarp
      const OVERWORLD_HIDDEN_ITEM_FOUND_ADDRESS: i32 = 0x0_3edd - 4; // CheckForHiddenObjectOrBookshelfOrCardKeyDoor.hiddenObjectNotFound - 4
      const OVERWORLD_CARD_KEY_DOOR_FOUND_ADDRESS: i32 = 0x14_669c + 7; // PrintCardKeyText.cardKeyDoorInFrontOfPlayer + 7
      const OVERWORLD_A_BUTTON_Z_CHECK_1: i32 = 0x0_047d-12; // .displayDialogue - 12
      const OVERWORLD_A_BUTTON_Z_CHECK_2: i32 = 0x0_047d-3; // .displayDialogue - 3
      const OVERWORLD_YELLOW_PIKACHU_INTERACTION_ADDRESS: i32 = 0; // 3f:4f20 InitializePikachuTextID
      const OVERWORLD_DISPLAY_TEXT_ADDRESS: i32 = 0x0_0496; // at call DisplayTextID
      const OVERWORLD_DISPLAY_TEXT_ID_MEM_ADDRESS: u16 = 0xFF8C; // hSpriteIndexOrTextID
      const OVERWORLD_DISPLAY_TEXT_FAILED_ADDRESS: i32 = 0x0_04cd-3; // .noDirectionButtonsPressed - 3
      const OVERWORLD_INIT_BATTLE_COMMON_ADDRESS: i32 = 0xF_6F3D; // InitBattleCommon
      const OVERWORLD_NEW_BATTLE_NO_BATTLE: i32 = 0x0_062c+3; // .newBattle + 3
      const OVERWORLD_BATTLE_SPECIES_MEM_ADDRESS: u16 = 0xCFD8; // wEnemyMonSpecies2
      const OVERWORLD_BATTLE_LEVEL_MEM_ADDRESS: u16 = 0xD127; // wCurEnemyLVL
      const OVERWORLD_TURNING_DONE_ADDRESS: i32 = 0x0_057B; // at .holdIntermediateDirectionLoop -> jp OverworldLoop
      const OVERWORLD_MOVING_DIRECTION_MEM_ADDRESS: u16 = 0xD528; // wPlayerMovingDirection
      const OVERWORLD_JUMP_LEDGE_ADDRESS: i32 = 0x6_66a9+4; // HandleLedges.foundMatch + 4
      const OVERWORLD_LAND_COLLISION_ADDRESS: i32 = 0x0C0C; // CollisionCheckOnLand.setCarry
      const OVERWORLD_LAND_COLLISION_NO_WARP_ADDRESS: i32 = 0x0_03FF; // OverworldLoop
      const OVERWORLD_WATER_COLLISION_ADDRESS: i32 = 0x0_1001; // CollisionCheckOnWater.setCarry
      const OVERWORLD_HANDLE_BLACKOUT_ADDRESS: i32 = 0x0_0931; // HandleBlackOut
      const OVERWORLD_WALKED_PRE_ADDRESS: i32 = 0x0_0619; // .notSafariZone
      const OVERWORLD_WALKED_ADDRESS: i32 = 0x0_06B4; // CheckWarpsNoCollision
      const OVERWORLD_NO_ACTION_ADDRESS: i32 = 0x0_04CD; // OverworldLoopLessDelay.noDirectionButtonsPressed
      const OVERWORLD_TURNFRAME_DIRECTION_MEM_ADDRESS: u16 = 0xd529; // wPlayerLastStopDirection
      const OVERWORLD_TURNFRAME_CHECK_MEM_ADDRESS: u16 = 0xcc4b; // wCheckFor180DegreeTurn
    }
    impl Gen1DVAddresses for $t {
      const AFTER_DV_GENERATION_ADDRESSES: &'static [i32] = &[
        0x03_73B3, // _AddPartyMon.next4
        0x0F_6B33, // LoadEnemyMonData.storeDVs
      ];
    }
    impl TrainerIDAddresses for $t {
      const TRAINER_ID_AFTER_GENERATION_ADDRESS: i32 = 0x03_7860; // in InitPlayerData2
      const TRAINER_ID_MEM_ADDRESS: u16 = 0xD359; // wPlayerID
    }
    impl InputIdentificationAddresses for $t {
      const II_ADDRESSES: &'static [(i32, i32, i32, &'static str)] = &[
        (0x0_12FC, 0x3_4000, 0x0_12FF, "CheckForUserInterruption"),
        (0x0_06F9, 0x3_4000, 0x0_06FC, "CheckWarpsNoCollisionLoop"),
        (0x0_2B70, 0x3_4000, 0x0_2B73, "CloseStartMenu"),
        (0x0_2D89, 0x3_4000, 0x0_2D8C, "DisplayChooseQuantityMenu"),
        (0x1_660F, 0x3_4000, 0x1_6612, "DisplayNamingScreen"),
        (0x1_5EF5, 0x3_4000, 0x1_5EF8, "DisplayOptionMenu"),
        (0x1C_4ECB, 0x3_4000, 0x1C_4ECE, "DisplayTownMap"),
        (0x1E_7EFE, 0x3_4000, 0x1E_7F01, "Evolution_CheckForCancel"),
        (0x0_3AE9, 0x3_4000, 0x0_3AEC, "HandleMenuInput_"),
        (0x0_29DF, 0x3_4000, 0x0_29E2, "HoldTextDisplayOpen"),
        (0x0_0F57, 0x3_4000, 0x0_0F5A, "JoypadOverworld"),
        (0x1C_5008, 0x3_4000, 0x1C_500B, "LoadTownMap_Fly"),
        (0x1_5BC1, 0x3_4000, 0x1_5BC4, "MainMenu"),
        (0x0_38F6, 0x3_4000, 0x0_38F9, "PrintLetterDelay"),
        (0x10_4423, 0x3_4000, 0x10_4426, "ShowPokedexDataInternal"),
        (0xD_7885, 0x3_4000, 0xD_7888, "SlotMachine_HandleInputWhileWheelsSpin"),
        (0x0_1C1E, 0x3_4000, 0x0_1C21, "TextCommand0A"),
        (0x0_1C82, 0x3_4000, 0x0_1C85, "TextCommand0C"),
        (0x1_574A, 0x3_4000, 0x1_574D, "TradeCenter_SelectMon"),
        (0x0_3883, 0x3_4000, 0x0_3886, "WaitForTextScrollButtonPress"),
      ];
    }
    impl BattleDetermineMoveOrderAddresses for $t {
      const DETERMINE_MOVE_ORDER_START_ADDRESS: i32 = 0x0F_42E5; // MainInBattleLoop.noLinkBattle
      const MOVE_ORDER_PLAYER_FIRST_ADDRESS: i32 = 0x0F_437D; // MainInBattleLoop.playerMovesFirst
      const MOVE_ORDER_ENEMY_FIRST_ADDRESS: i32 = 0x0F_433D; // MainInBattleLoop.enemyMovesFirst
    }
    impl BattleObedienceAddresses for $t {
      const CHECK_OBEDIENCE_START_ADDRESS: i32 = 0x0F_5C88; // CheckForDisobedience
      const CHECK_OBEDIENCE_OBEY_ADDRESS: i32 = 0x0F_569A; // CheckIfPlayerNeedsToChargeUp
      const CHECK_OBEDIENCE_DISOBEY_ADDRESS: i32 = 0x0F_5CEB; // CheckForDisobedience.loop2
    }
    impl BattleCatchMonAddresses for $t {
      const CATCH_SUCCESS_ADDRESS: i32 = 0x03_578B; // PokeBallEffect.captured
      const CATCH_FAIL_ADDRESS: i32 = 0x03_578D; // PokeBallEffect.failedToCapture
    }
    impl BattleMovesInfoAddresses for $t {
      const MOVES_ADDRESS: i32 = 0xe_4000; // Moves
      const MOVES_ENTRY_LENGTH: i32 = 6; // length of a single move

      const GEN2_BADGES_MEM_ADDRESS: u16 = 0; // unused in gen 1
      const TYPE_MATCHUPS_ADDRESS: i32 = 0x0f_6474; // TypeEffects
    }
    impl BattleMonInfoAddresses for $t {
      const BATTLE_MON_STRUCT_MEM_ADDRESS: u16 = 0xd014; // wBattleMon
      const BATTLE_MON_STAT_LEVELS_MEM_ADDRESS: u16 = 0xcd1a; // wPlayerMonStatMods
      const GEN1_PARTY_MON_STATS_BASE_MEM_ADDRESS: u16 = 0xd18f; // wPartyMon1Attack
      const GEN1_PARTY_MON_STRUCT_LEN: u16 = 44; // wPartyMon2 - wPartyMon1
      const GEN1_PLAYER_MON_NUMBER_MEM_ADDRESS: u16 = 0xcc2f; // wPlayerMonNumber
      const GEN2_BATTLE_MON_ORIG_STATS_MEM_ADDRESS: u16 = 0; // unused

      const ENEMY_MON_STRUCT_MEM_ADDRESS: u16 = 0xcfe5; // wEnemyMon
      const ENEMY_MON_STAT_LEVELS_MEM_ADDRESS: u16 = 0xcd2e; // wEnemyMonStatMods
      const GEN1_BASE_STATS_BASE_ADDRESS: i32 = 0xe_43de; // BaseStats
      const GEN1_BASE_STATS_LEN: i32 = 0x1c; // MonBaseStatsEnd - MonBaseStats
      const GEN2_ENEMY_MON_ORIG_STATS_MEM_ADDRESS: u16 = 0; // unused
    }
    impl AIChooseMoveAddresses for $t {
      const AFTER_AI_CHOOSE_MOVE_ADDRESS: i32 = 0xf_42a9; // MainInBattleLoop.selectEnemyMove + 3
      const CUR_ENEMY_MOVE_MEM_ADDRESS: u16 = 0xccdd; // wEnemySelectedMove
    }
    impl Gen1FightTurnAddresses for $t {
      const CUR_MOVE_INDEX_MEM_ADDRESS: u16 = 0xcc2e; // wPlayerMoveListIndex
      const CRITICAL_HIT_MEM_ADDRESS: u16 = 0xd05e; // wCriticalHitOrOHKO
      const AFTER_MAX_DAMAGE_CALC_ADDRESS: i32 = 0xf_6687; // RandomizeDamage
      const CUR_DAMAGE_MEM_ADDRESS: u16 = 0xd0d7; // wDamage
      const AFTER_PLAYER_HIT_CHECK_ADDRESS: i32 = 0xf_5705; // handleIfPlayerMoveMissed
      const AFTER_ENEMY_HIT_CHECK_ADDRESS: i32 = 0xf_6782; // handleIfEnemyMoveMissed
      const ATTACK_MISSED_MEM_ADDRESS: u16 = 0xd05f; // wMoveMissed
    }
    impl Gen1TrainerAIAddresses for $t {
      const TRAINER_AI_START_ADDRESS: i32 = 0xe_652e; // TrainerAI
      const TRAINER_AI_NO_ACTION_ADDRESS: i32 = 0xf_66bc; // ExecuteEnemyMove
      const TRAINER_AI_SWITCH_ADDRESS: i32 = 0xe_674b; // SwitchEnemyMon
      const TRAINER_AI_XITEM_ADDRESS: i32 = 0xe_6808; // AIIncreaseStat
      const TRAINER_AI_GUARD_SPEC_ADDRESS: i32 = 0xe_67b5; // AIUseGuardSpec
      const TRAINER_AI_FULL_HEAL_ADDRESS: i32 = 0xe_6786; // AIUseFullHeal
      const TRAINER_AI_POTION_ADDRESS: i32 = 0xe_66da; // AIRecoverHP
      const TRAINER_AI_FULL_RESTORE_ADDRESS: i32 = 0xe_66a0; // AIUseFullRestore
    }
    impl Gen1MoveEffectAddresses for $t {
      const MOVE_EFFECT_START_ADDRESS: i32 = 0xf_7132; // JumpMoveEffect
      const MOVE_EFFECT_NO_EFFECT_ADDRESS: i32 = 0xf_7135; // JumpMoveEffect + 3
      const MOVE_EFFECT_SLEEP_SUCCESS_ADDRESS: i32 = 0xf_7231+7; // SleepEffect.setSleepCounter + 7 (a contains sleep counter)
      const MOVE_EFFECT_SLEEP_FAILED_ADDRESS: i32 = 0xf_7242; // SleepEffect.didntAffect
      const MOVE_EFFECT_POISON_SUCCESS_ADDRESS: i32 = 0xf_7295; // PoisonEffect.inflictPoison
      const MOVE_EFFECT_POISON_FAILED_ADDRESS: i32 = 0xf_72d7; // PoisonEffect.didntAffect
      const MOVE_EFFECT_FREEZE_BURN_PARALYZE_PLAYER_SUCCESS_ADDRESS: i32 = 0xf_733c+7; // FreezeBurnParalyzeEffect.next1 + 7
      const MOVE_EFFECT_FREEZE_BURN_PARALYZE_ENEMY_SUCCESS_ADDRESS: i32 = 0xf_73bf+7; // FreezeBurnParalyzeEffect.next2 + 7
      const MOVE_EFFECT_DEFROSTED_SUCCESS_ADDRESS: i32 = 0xf_7420; // CheckDefrost.common
      const MOVE_EFFECT_STAT_UP_SUCCESS_ADDRESS: i32 = 0xf_745a; // StatModifierUpEffect.ok
      const MOVE_EFFECT_STAT_UP_NOTHING_HAPPENED_ADDRESS: i32 = 0xf_7522; // PrintNothingHappenedText
      const MOVE_EFFECT_STAT_DOWN_SUCCESS_ADDRESS: i32 = 0xf_75ef; // StatModifierDownEffect.recalculateStat
      const MOVE_EFFECT_STAT_DOWN_FAILED_ADDRESS: i32 = 0xf_765a; // MoveMissed
      const MOVE_EFFECT_STAT_DOWN_NOTHING_HAPPENED_ADDRESS: i32 = 0xf_7650+4; // CantLowerAnymore + 4
      const MOVE_EFFECT_BIDE_ADDRESS: i32 = 0xf_7717-8; // ThrashPetalDanceEffect - 8 (a contains turn counter)
      const MOVE_EFFECT_THRASH_PETALDANCE_ADDRESS: i32 = 0xf_7739-8; // SwitchAndTeleportEffect - 8 (a contains turn counter)
      const MOVE_EFFECT_MULTI_HIT_ADDRESS: i32 = 0xf_7853; // TwoToFiveAttacksEffect.saveNumberOfHits (a contains number of hits)
      const MOVE_EFFECT_FLINCHED_ADDRESS: i32 = 0xf_7879+5; // FlinchSideEffect.gotEffectChance + 5
      const MOVE_EFFECT_TRAPPING_ADDRESS: i32 = 0xf_793e+1; // TrappingEffect.setTrappingCounter + 1 (a contains turn counter)
      const MOVE_EFFECT_CONFUSION_SUCCESS_ADDRESS: i32 = 0xf_7986+14; // ConfusionSideEffectSuccess.confuseTarget + 14 (a contains turn counter)
      const MOVE_EFFECT_CONFUSION_FAILED_ADDRESS: i32 = 0xf_79a6; // ConfusionEffectFailed
      const MOVE_EFFECT_PARALYZE_SUCCESS_ADDRESS: i32 = 0x14_662a+16; // ParalyzeEffect_.hitTest + 16
      const MOVE_EFFECT_PARALYZE_FAILED_ADDRESS: i32 = 0x14_6659; // ParalyzeEffect_.didntAffect
      const MOVE_EFFECT_PARALYZE_NO_EFFECT_ADDRESS: i32 = 0x14_6666; // ParalyzeEffect_.doesntAffect
      const MOVE_EFFECT_MIMIC_SUCCESS_ADDRESS: i32 = 0xf_7a5f; // MimicEffect.playerTurn
      const MOVE_EFFECT_MIMIC_FAILED_ADDRESS: i32 = 0xf_7a74; // MimicEffect.mimicMissed
      const MOVE_EFFECT_DISABLE_SUCCESS_ADDRESS: i32 = 0xf_7ae1; // DisableEffect.playerTurnNotLinkBattle
      const MOVE_EFFECT_DISABLE_FAILED_ADDRESS: i32 = 0xf_7b06; // DisableEffect.moveMissed
      const MOVE_EFFECT_HEAL_FAILED_ADDRESS: i32 = 0x0e_7a97; // HealEffect_.failed
      const MOVE_EFFECT_TELEPORT_FAILED_ADDRESS: i32 = 0x0F_77D1; // SwitchAndTeleportEffect.notWildBattle2
    }
    impl Gen1MapAddresses for $t {
      const SURF_STATE_MEM_ADDRESS: u16 = 0xd700; // wWalkBikeSurfState (2 = surfing)
      const MAP_TILESET_MEM_ADDRESS: u16 = 0xd367; // wCurMapTileset
      const TILE_PAIR_COLLISIONS_LAND_ADDRESS: i32 = 0x0_0c7e; // TilePairCollisionsLand
      const TILE_PAIR_COLLISIONS_WATER_ADDRESS: i32 = 0x0_0ca0; // TilePairCollisionsWater
      const MAP_TILESET_COLLISION_PTR_BANK_OFFSET: i32 = 0; // bank * 0x1_0000
      const MAP_TILESET_COLLISION_PTR_MEM_ADDRESS: u16 = 0xd530; // wTilesetCollisionPtr
      const MAP_TILESET_BANK_MEM_ADDRESS: u16 = 0xd52b; // wTilesetBank
      const MAP_TILESET_BLOCKS_PTR_MEM_ADDRESS: u16 = 0xd52c; // wTilesetBlocksPtr
      const MAP_WIDTH_MEM_ADDRESS: u16 = 0xd369; // wCurMapWidth
      const MAP_HEIGHT_MEM_ADDRESS: u16 = 0xd368; // wCurMapHeight
      const OVERWORLD_MAP_MEM_ADDRESS: u16 = 0xc6e8; // wOverworldMap
      const MAP_SPRITE_STATE_DATA_2_MEM_ADDRESS: u16 = 0xc200; // wSpriteStateData2
      const MAP_SPRITE_DATA_MEM_ADDRESS: u16 = 0xd4e4; // wMapSpriteData
      const MAP_MISSABLE_OBJECT_LIST_MEM_ADDRESS: u16 = 0xd5ce; // wMissableObjectList
      const MAP_MISSABLE_OBJECT_FLAGS_MEM_ADDRESS: u16 = 0xd5a6; // wMissableObjectFlags
      const MAP_INDEX_MEM_ADDRESS: u16 = 0xd35e; // wCurMap
      const MAP_HEADER_BANKS_ADDRESS: i32 = 0x03_423d; // MapHeaderBanks
      const MAP_TEXT_PTR_MEM_ADDRESS: u16 = 0xd36c; // wMapTextPtr
      const TALK_TO_TRAINER_FUNCTION_ADDRESS: i32 = 0x31cc; // TalkToTrainer
      const PLAYER_X_POS_MEM_ADDRESS: u16 = 0xd362; // wXCoord
      const PLAYER_Y_POS_MEM_ADDRESS: u16 = 0xd361; // wYCoord
    }
    impl VermilionTrashCanAddresses for $t {
      const FIRST_TRASH_CAN_MEM_ADDRESS: u16 = 0xd743; // wFirstLockTrashCanIndex
      const AFTER_FIRST_TRASH_CAN_ADDRESS: i32 = 0x6_57cb-1; // VermilionCity_Script.initCityScript - 1
      const SECOND_TRASH_CAN_MEM_ADDRESS: u16 = 0xd744; // wSecondLockTrashCanIndex
      const AFTER_SECOND_TRASH_CAN_ADDRESS: i32 = 0x17_5e53-2; // GymTrashScript.trySecondLock - 2
    }
    impl FlyLocationsAddresses for $t {
      const FLY_LOCATIONS_MEM_ADDRESS: u16 = 0xcd3e; // wFlyLocationsList
    }
    impl PushBoulderAddresses for $t {
      const AFTER_TRY_PUSHING_BOULDER_ADDRESS: i32 = 0x00_101b+11; // RunMapScript + 11
      const BOULDER_FLAGS_MEM_ADDRESS: u16 = 0xcd60; // wFlags_0xcd60
    }
    )+
  }
}
impl_red_blue_common_addresses!(Red, Blue);


#[allow(dead_code)]
pub enum Yellow {}
impl BasicRomInfo for Yellow {
  const ROM_FILE_NAME: &'static str = "roms/yellow.gbc";
  const GAME_NAME: &'static str = "Pokemon - Yellow Version (USA, Europe)";
  const SHA1: &'static str = "CC7D03262EBFAF2F06772C1A480C7D9D5F4A38E1";
  const BOARD_NAME: &'static str = "MBC5 ROM+RAM+BATTERY";
  const GENERATION: Generation = Generation::Gen1;
}
impl JoypadAddresses for Yellow {
  const JOYPAD_READ_HI_ADDRESS: i32 = 0x3_400A;
  const JOYPAD_READ_LO_ADDRESS: i32 = 0x3_4020;
  const JOYPAD_READ_FIRST_ADDRESS: i32 = Self::JOYPAD_READ_HI_ADDRESS;
  const JOYPAD_READ_LAST_ADDRESS: i32 = Self::JOYPAD_READ_LO_ADDRESS;
  const JOYPAD_READ_LOCKED_ADDRESS: i32 = 0x3_4022;
  const JOYPAD_INPUT_MEM_ADDRESS: u16 = 0xfff5; // hJoyInput
  const JOYPAD_USE_ADDRESSES: &'static [i32] = &[
    0x3_402D, // _Joypad
  ];
  const JOYPAD_USE_DISCARD_ADDRESSES: Option<(u16, u16, u8, i32, i32, i32)> = Some((0xd139, 0xd72f, 5, 0x3_4041, 0x3_4046, 0x3_4063)); // (ignore_input_counter_mem_add, ignore_flag_mem_add, ignore_flag_bit, calc_joy_pressed_add, check_ignore_flag_add, discard_add)
  const JOYPAD_USE_IGNORE_MASK_MEM_ADDRESSES: &'static [(i32, u16, i32)] = &[(0x3_402D, 0xCD6B, 0x3_402F)]; // wJoyIgnore
}
impl JoypadLowSensitivityAddresses for Yellow {
  const JOYPAD_HJOY6_MEM_ADDRESS: u16 = 0xffb6; // hJoy6
  const JOYPAD_HJOY7_MEM_ADDRESS: u16 = 0xffb7; // hJoy7
  const JOYPAD_LAST_MEM_ADDRESS: u16 = 0xffb1; // hJoyLast
  const JOYPAD_FRAME_COUNTER_MEM_ADDRESS: u16 = 0xffd5; // hFrameCounter
  const JOYPAD_FRAME_COUNTER_CHECK_ADDRESS: i32 = 0x0_3836; // JoypadLowSensitivity.noNewlyPressedButtons
}
impl HandleMenuInputAddresses for Yellow {
  const MENU_WRAPPING_ENABLED_MEM_ADDRESS: u16 = 0xcc4a; // wMenuWrappingEnabled
  const CURRENT_MENU_ITEM_MEM_ADDRESS: u16 = 0xcc26; // wCurrentMenuItem
  const MAX_MENU_ITEM_MEM_ADDRESS: u16 = 0xcc28; // wMaxMenuItem
  const MENU_WATCHED_KEYS_MEM_ADDRESS: u16 = 0xcc29; // wMenuWatchedKeys
  const MENU_WATCH_MOVING_OOB_MEM_ADDRESS: u16 = 0xcc37; // wMenuWatchMovingOutOfBounds
  const MENU_JOYPAD_POLL_COUNT_MEM_ADDRESS: u16 = 0xcc34; // wMenuJoypadPollCount
  const MENU_A_BUTTON_PRIORITY: bool = true; // whether A button is checked before directional buttons in HandleMenuInput_. True in Yellow.
  const BATTLE_START_SAVED_MENU_ITEM: u16 = 0xcc2d; // wBattleAndStartSavedMenuItem
  const LIST_SCROLL_OFFSET_MEM_ADDRESS: u16 = 0xcc36; // wListScrollOffset
  const LIST_COUNT_MEM_ADDRESS: u16 = 0xd129; // wListCount
  const MENU_ITEM_QUANTITY_MEM_ADDRESS: u16 = 0xcf95; // wItemQuantity
  const MENU_MAX_ITEM_QUANTITY_MEM_ADDRESS: u16 = 0xcf96; // wMaxItemQuantity
}
impl JoypadOverworldAddresses for Yellow {
  const FLAGS_D733_MEM_ADDRESS: u16 = 0xd732; // wFlags_D733
  const CUR_MAP_MEM_ADDRESS: u16 = 0xd35d; // wCurMap
  const FLAGS_D730_MEM_ADDRESS: u16 = 0xd72f; // wd730
  const SIMULATED_JOYPAD_OVERRIDE_MASK_MEM_ADDRESS: u16 = 0xcd3b; // wOverrideSimulatedJoypadStatesMask
  const SIMULATED_JOYPAD_STATES_INDEX_MEM_ADDRESS: u16 = 0xcd38; // wSimulatedJoypadStatesIndex
  const SIMULATED_JOYPAD_STATES_END_MEM_ADDRESS: u16 = 0xccd3; // wSimulatedJoypadStatesEnd
}
impl RngAddresses for Yellow {
  const RNG_MEM_ADDRESS: u16 = 0xffd3;
}
impl TextAddresses for Yellow {
  const TEXT_BEFORE_JOYPAD_ADDRESS: i32 = 0x0_38EB;
  const TEXT_JOYPAD_ADDRESS: i32 = 0x3_402D; // _Joypad
  const TEXT_AFTER_JOYPAD_ADDRESS: i32 = 0x0_38EE;
  const TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS: i32 = 0x0_3904;
  const TEXT_END_NOINPUT_ADDRESSES: &'static [i32] = &[
    // 0x0_187B, // Char57; Address of the character $57 handler, used to end the text without any input required
    0x0_192E, // NextTextCommand; called when the next text command is being processed.
  ];
  const TEXT_END_WITHINPUT_ADDRESSES: &'static [i32] = &[
    0x0_18D1, // Char4B; Address of the character $4B handler, used to scroll text up after a button press
    0x0_1882, // Char51; Address of the character $51 handler, used start a new paragraph of text
    0x0_1863, // Char58; Address of the character $58 handler, used to wait for a button press before ending the text
  ];
  const TEXT_DELAY_FRAME_COUNTER_MEM_ADDRESS: u16 = 0xffd5;
  const TEXT_SAFE_CONFLICTING_INPUT_ADDRESSES: &'static [i32] = &[
    0x0_3879, // WaitForTextScrollButtonPress
  ];
}
impl Gen1OverworldAddresses for Yellow {
  const OVERWORLD_BEFORE_JOYPAD_ADDRESS: i32 = 0x0_0C5B;
  const OVERWORLD_JOYPAD_ADDRESS: i32 = 0x3_402D; // _Joypad
  const OVERWORLD_AFTER_JOYPAD_ADDRESS: i32 = 0x0_0C5E;
  const OVERWORLD_WARP_FOUND_ADDRESS: i32 = 0x0_054A; // WarpFound2
  const OVERWORLD_WARP_MAP_MEM_ADDRESS: u16 = 0xFF8B; // hWarpDestinationMap
  const OVERWORLD_WARP_ENTRANCE_MEM_ADDRESS: u16 = 0xD42E; // wDestinationWarpID
  const OVERWORLD_FLY_DUNGEON_WARP_FOUND_ADDRESS: i32 = 0x0_0794; // HandleFlyWarpOrDungeonWarp
  const OVERWORLD_HIDDEN_ITEM_FOUND_ADDRESS: i32 = 0x0_3f1f - 7; // CheckForHiddenObjectOrBookshelfOrCardKeyDoor.hiddenObjectNotFound - 7
  const OVERWORLD_CARD_KEY_DOOR_FOUND_ADDRESS: i32 = 0x14_65fc + 7; // PrintCardKeyText.cardKeyDoorInFrontOfPlayer + 7
  const OVERWORLD_A_BUTTON_Z_CHECK_1: i32 = 0x0_02b2; // different in Yellow
  const OVERWORLD_A_BUTTON_Z_CHECK_2: i32 = 0x0_02c5-3; // .displayDialogue - 3
  const OVERWORLD_YELLOW_PIKACHU_INTERACTION_ADDRESS: i32 = 0x3f_4f20; // InitializePikachuTextID
  const OVERWORLD_DISPLAY_TEXT_ADDRESS: i32 = 0x0_02DE; // at call DisplayTextID
  const OVERWORLD_DISPLAY_TEXT_ID_MEM_ADDRESS: u16 = 0xFF8C; // hSpriteIndexOrTextID
  const OVERWORLD_DISPLAY_TEXT_FAILED_ADDRESS: i32 = 0x0_02f8-3; // .noDirectionButtonsPressed - 3
  const OVERWORLD_INIT_BATTLE_COMMON_ADDRESS: i32 = 0x3D_601D; // asm_f601d
  const OVERWORLD_NEW_BATTLE_NO_BATTLE: i32 = 0x0_040b+3; // .newBattle + 3
  const OVERWORLD_BATTLE_SPECIES_MEM_ADDRESS: u16 = 0xCFD7; // wEnemyMonSpecies2
  const OVERWORLD_BATTLE_LEVEL_MEM_ADDRESS: u16 = 0xD126; // wCurEnemyLVL
  const OVERWORLD_TURNING_DONE_ADDRESS: i32 = 0x0_0381; // at .handleDirectionButtonPress -> jp OverworldLoop
  const OVERWORLD_MOVING_DIRECTION_MEM_ADDRESS: u16 = 0xD527; // wPlayerMovingDirection
  const OVERWORLD_JUMP_LEDGE_ADDRESS: i32 = 0x6_682b+4; // HandleLedges.foundMatch + 4
  const OVERWORLD_LAND_COLLISION_ADDRESS: i32 = 0x0_0A75; // CollisionCheckOnLand.setCarry
  const OVERWORLD_LAND_COLLISION_NO_WARP_ADDRESS: i32 = 0x0_0242; // OverworldLoop
  const OVERWORLD_WATER_COLLISION_ADDRESS: i32 = 0x0_0D08; // CollisionCheckOnWater.setCarry
  const OVERWORLD_HANDLE_BLACKOUT_ADDRESS: i32 = 0x0_0762; // HandleBlackOut
  const OVERWORLD_WALKED_PRE_ADDRESS: i32 = 0x0_03f8; // .notSafariZone
  const OVERWORLD_WALKED_ADDRESS: i32 = 0x0_04BD; // CheckWarpsNoCollision
  const OVERWORLD_NO_ACTION_ADDRESS: i32 = 0x0_02F8; // OverworldLoopLessDelay.noDirectionButtonsPressed
  const OVERWORLD_TURNFRAME_DIRECTION_MEM_ADDRESS: u16 = 0xd528; // wPlayerLastStopDirection
  const OVERWORLD_TURNFRAME_CHECK_MEM_ADDRESS: u16 = 0xcc4b; // wCheckFor180DegreeTurn
}
impl Gen1DVAddresses for Yellow {
      const AFTER_DV_GENERATION_ADDRESSES: &'static [i32] = &[
        0x03_722F, // _AddPartyMon.next4
        0x0F_6CB9, // LoadEnemyMonData.storeDVs
      ];
}
impl TrainerIDAddresses for Yellow {
  const TRAINER_ID_AFTER_GENERATION_ADDRESS: i32 = 0x03_76e6; // in InitPlayerData2
  const TRAINER_ID_MEM_ADDRESS: u16 = 0xD358; // wPlayerID
}
impl InputIdentificationAddresses for Yellow {
  const II_ADDRESSES: &'static [(i32, i32, i32, &'static str)] = &[
    (0x0_38EB, 0x3_402D, 0x0_38EE, "PrintLetterDelay"),
    (0x0_2A72, 0x3_402D, 0x0_2A75, "CloseStartMenu"),
    (0x17_5B7C, 0x3_402D, 0x17_5B7F, "Evolution_CheckForCancel"),
    (0x3A_4C0F, 0x3_402D, 0x3A_4C12, "PrintPokedexEntry"),
    (0x3A_4C7D, 0x3_402D, 0x3A_4C80, "PrintSurfingMinigameHighScore"),
    (0x3A_4D14, 0x3_402D, 0x3A_4D17, "Print_Func_e8d11"),
    (0x3A_4DFE, 0x3_402D, 0x3A_4E01, "Print_Func_e8dfb"),
    (0x3A_4E45, 0x3_402D, 0x3A_4E48, "PrintFanClubPortrait"),
    (0x0_19F7, 0x3_402D, 0x0_19FA, "TextCommand0A"),
    (0x0_1A5B, 0x3_402D, 0x0_1A5E, "TextCommand0C"),
    (0x1_5C73, 0x3_402D, 0x1_5C76, "MainMenu"),
    (0x1C_4F41, 0x3_402D, 0x1C_4F44, "DisplayTownMap"),
    (0x1C_5091, 0x3_402D, 0x1C_5094, "LoadTownMap_Fly"),
    (0x10_5C73, 0x3_402D, 0x10_5C76, "DisplayOptionMenu_"),
    (0x1_42AD, 0x3_402D, 0x1_42B0, "DisplayTitleScreen"),
    (0x1_637F, 0x3_402D, 0x1_6382, "DisplayNamingScreen"),
    (0x3D_5A58, 0x3_402D, 0x3D_5A5B, "HandleMenuInputPokemonSelectionDouble"),
    (0x10_434B, 0x3_402D, 0x10_434E, "ShowPokedexDataInternal"),
    (0x0_10BE, 0x3_402D, 0x0_10C1, "CheckForUserInterruption"),
    (0x0_28CF, 0x3_402D, 0x0_28D2, "HoldTextDisplayOpen"),
    (0x0_2C81, 0x3_402D, 0x0_2C84, "DisplayChooseQuantityMenu"),
    (0x0_3879, 0x3_402D, 0x0_387C, "WaitForTextScrollButtonPress"),
    (0x0_3AD6, 0x3_402D, 0x0_3AD9, "HandleMenuInput_"),
    (0x0_0502, 0x3_402D, 0x0_0505, "CheckWarpsNoCollisionLoop"),
    (0x0_0C5B, 0x3_402D, 0x0_0C5E, "JoypadOverworld"),
    (0x3F_5ABC, 0x3_402D, 0x3F_5ABF, "PikaPicAnimTimerAndJoypad"),
    (0x1_57EB, 0x3_402D, 0x1_57EE, "TradeCenter_SelectMon"),
    (0xD_7AB5, 0x3_402D, 0xD_7AB8, "SlotMachine_HandleInputWhileWheelsSpin"),
    (0x3E_5848, 0x3_402D, 0x3E_584B, "PlayIntroScene"),
    (0x3E_523F, 0x3_402D, 0x3E_5242, "SurfingPikachu_GetJoypad_3FrameBuffer"),
  ];
}
impl BattleDetermineMoveOrderAddresses for Yellow {
  const DETERMINE_MOVE_ORDER_START_ADDRESS: i32 = 0x0F_42FB; // MainInBattleLoop.noLinkBattle
  const MOVE_ORDER_PLAYER_FIRST_ADDRESS: i32 = 0x0F_4393; // MainInBattleLoop.playerMovesFirst
  const MOVE_ORDER_ENEMY_FIRST_ADDRESS: i32 = 0x0F_4353; // MainInBattleLoop.enemyMovesFirst
}
impl BattleObedienceAddresses for Yellow {
  const CHECK_OBEDIENCE_START_ADDRESS: i32 = 0x0F_5DFA; // CheckForDisobedience
  const CHECK_OBEDIENCE_OBEY_ADDRESS: i32 = 0x0F_580C; // CheckIfPlayerNeedsToChargeUp
  const CHECK_OBEDIENCE_DISOBEY_ADDRESS: i32 = 0x0F_5E5D; // CheckForDisobedience.loop2
}
impl BattleCatchMonAddresses for Yellow {
  const CATCH_SUCCESS_ADDRESS: i32 = 0x03_54D4; // PokeBallEffect.captured
  const CATCH_FAIL_ADDRESS: i32 = 0x03_54D6; // PokeBallEffect.failedToCapture
}
impl BattleMovesInfoAddresses for Yellow {
  const MOVES_ADDRESS: i32 = 0xe_4000; // Moves
  const MOVES_ENTRY_LENGTH: i32 = 6; // length of a single move

  const GEN2_BADGES_MEM_ADDRESS: u16 = 0; // unused in gen 1
  const TYPE_MATCHUPS_ADDRESS: i32 = 0x0f_65fa; // TypeEffects
}
impl BattleMonInfoAddresses for Yellow {
  const BATTLE_MON_STRUCT_MEM_ADDRESS: u16 = 0xd013; // wBattleMon
  const BATTLE_MON_STAT_LEVELS_MEM_ADDRESS: u16 = 0xcd1a; // wPlayerMonStatMods
  const GEN1_PARTY_MON_STATS_BASE_MEM_ADDRESS: u16 = 0xd18e; // wPartyMon1Attack
  const GEN1_PARTY_MON_STRUCT_LEN: u16 = 44; // wPartyMon2 - wPartyMon1
  const GEN1_PLAYER_MON_NUMBER_MEM_ADDRESS: u16 = 0xcc2f; // wPlayerMonNumber
  const GEN2_BATTLE_MON_ORIG_STATS_MEM_ADDRESS: u16 = 0; // unused

  const ENEMY_MON_STRUCT_MEM_ADDRESS: u16 = 0xcfe4; // wEnemyMon
  const ENEMY_MON_STAT_LEVELS_MEM_ADDRESS: u16 = 0xcd2e; // wEnemyMonStatMods
  const GEN1_BASE_STATS_BASE_ADDRESS: i32 = 0xe_43de; // BaseStats
  const GEN1_BASE_STATS_LEN: i32 = 0x1c; // MonBaseStatsEnd - MonBaseStats
  const GEN2_ENEMY_MON_ORIG_STATS_MEM_ADDRESS: u16 = 0; // unused
}
impl AIChooseMoveAddresses for Yellow {
  const AFTER_AI_CHOOSE_MOVE_ADDRESS: i32 = 0xf_42bf; // MainInBattleLoop.selectEnemyMove + 3
  const CUR_ENEMY_MOVE_MEM_ADDRESS: u16 = 0xccdd; // wEnemySelectedMove
}
impl Gen1TrainerAIAddresses for Yellow {
  const TRAINER_AI_START_ADDRESS: i32 = 0xe_65b2; // TrainerAI
  const TRAINER_AI_NO_ACTION_ADDRESS: i32 = 0xf_6842; // ExecuteEnemyMove
  const TRAINER_AI_SWITCH_ADDRESS: i32 = 0xe_67e1; // SwitchEnemyMon
  const TRAINER_AI_XITEM_ADDRESS: i32 = 0xe_689e; // AIIncreaseStat
  const TRAINER_AI_GUARD_SPEC_ADDRESS: i32 = 0xe_684b; // AIUseGuardSpec
  const TRAINER_AI_FULL_HEAL_ADDRESS: i32 = 0xe_681c; // AIUseFullHeal
  const TRAINER_AI_POTION_ADDRESS: i32 = 0xe_6770; // AIRecoverHP
  const TRAINER_AI_FULL_RESTORE_ADDRESS: i32 = 0xe_6736; // AIUseFullRestore
}
impl Gen1FightTurnAddresses for Yellow {
  const CUR_MOVE_INDEX_MEM_ADDRESS: u16 = 0xcc2e; // wPlayerMoveListIndex
  const CRITICAL_HIT_MEM_ADDRESS: u16 = 0xd05d; // wCriticalHitOrOHKO
  const AFTER_MAX_DAMAGE_CALC_ADDRESS: i32 = 0xf_680d; // RandomizeDamage
  const CUR_DAMAGE_MEM_ADDRESS: u16 = 0xd0d6; // wDamage
  const AFTER_PLAYER_HIT_CHECK_ADDRESS: i32 = 0xf_5877; // handleIfPlayerMoveMissed
  const AFTER_ENEMY_HIT_CHECK_ADDRESS: i32 = 0xf_6908; // handleIfEnemyMoveMissed
  const ATTACK_MISSED_MEM_ADDRESS: u16 = 0xd05e; // wMoveMissed
}
impl Gen1MoveEffectAddresses for Yellow {
  const MOVE_EFFECT_START_ADDRESS: i32 = 0xf_70a7; // JumpMoveEffect
  const MOVE_EFFECT_NO_EFFECT_ADDRESS: i32 = 0xf_70a7+3; // JumpMoveEffect + 3
  const MOVE_EFFECT_SLEEP_SUCCESS_ADDRESS: i32 = 0xf_71a6+7; // SleepEffect.setSleepCounter + 7 (a contains sleep counter)
  const MOVE_EFFECT_SLEEP_FAILED_ADDRESS: i32 = 0xf_71c5; // SleepEffect.didntAffect
  const MOVE_EFFECT_POISON_SUCCESS_ADDRESS: i32 = 0xf_7218; // PoisonEffect.inflictPoison
  const MOVE_EFFECT_POISON_FAILED_ADDRESS: i32 = 0xf_725a; // PoisonEffect.didntAffect
  const MOVE_EFFECT_FREEZE_BURN_PARALYZE_PLAYER_SUCCESS_ADDRESS: i32 = 0xf_72d1+7; // FreezeBurnParalyzeEffect.next1 + 7
  const MOVE_EFFECT_FREEZE_BURN_PARALYZE_ENEMY_SUCCESS_ADDRESS: i32 = 0xf_734b+7; // FreezeBurnParalyzeEffect.next2 + 7
  const MOVE_EFFECT_DEFROSTED_SUCCESS_ADDRESS: i32 = 0xf_73d6; // CheckDefrost.common
  const MOVE_EFFECT_STAT_UP_SUCCESS_ADDRESS: i32 = 0xf_7410; // StatModifierUpEffect.ok
  const MOVE_EFFECT_STAT_UP_NOTHING_HAPPENED_ADDRESS: i32 = 0xf_74d8; // PrintNothingHappenedText
  const MOVE_EFFECT_STAT_DOWN_SUCCESS_ADDRESS: i32 = 0xf_75a5; // StatModifierDownEffect.recalculateStat
  const MOVE_EFFECT_STAT_DOWN_FAILED_ADDRESS: i32 = 0xf_7610; // MoveMissed
  const MOVE_EFFECT_STAT_DOWN_NOTHING_HAPPENED_ADDRESS: i32 = 0xf_7606+4; // CantLowerAnymore + 4
  const MOVE_EFFECT_BIDE_ADDRESS: i32 = 0xf_76cd-8; // ThrashPetalDanceEffect - 8 (a contains turn counter)
  const MOVE_EFFECT_THRASH_PETALDANCE_ADDRESS: i32 = 0xf_76ef-8; // SwitchAndTeleportEffect - 8 (a contains turn counter)
  const MOVE_EFFECT_MULTI_HIT_ADDRESS: i32 = 0xf_7809; // TwoToFiveAttacksEffect.saveNumberOfHits (a contains number of hits)
  const MOVE_EFFECT_FLINCHED_ADDRESS: i32 = 0xf_7837+5; // FlinchSideEffect.gotEffectChance + 5
  const MOVE_EFFECT_TRAPPING_ADDRESS: i32 = 0xf_7919+1; // TrappingEffect.setTrappingCounter + 1 (a contains turn counter)
  const MOVE_EFFECT_CONFUSION_SUCCESS_ADDRESS: i32 = 0xf_7961+14; // ConfusionSideEffectSuccess.confuseTarget + 14 (a contains turn counter)
  const MOVE_EFFECT_CONFUSION_FAILED_ADDRESS: i32 = 0xf_7981; // ConfusionEffectFailed
  const MOVE_EFFECT_PARALYZE_SUCCESS_ADDRESS: i32 = 0x14_658b+16; // ParalyzeEffect_.hitTest + 16
  const MOVE_EFFECT_PARALYZE_FAILED_ADDRESS: i32 = 0x14_65ba; // ParalyzeEffect_.didntAffect
  const MOVE_EFFECT_PARALYZE_NO_EFFECT_ADDRESS: i32 = 0x14_65c7; // ParalyzeEffect_.doesntAffect
  const MOVE_EFFECT_MIMIC_SUCCESS_ADDRESS: i32 = 0xf_7a3a; // MimicEffect.playerTurn
  const MOVE_EFFECT_MIMIC_FAILED_ADDRESS: i32 = 0xf_7a4f; // MimicEffect.mimicMissed
  const MOVE_EFFECT_DISABLE_SUCCESS_ADDRESS: i32 = 0xf_7abc; // DisableEffect.playerTurnNotLinkBattle
  const MOVE_EFFECT_DISABLE_FAILED_ADDRESS: i32 = 0xf_7ae1; // DisableEffect.moveMissed
  const MOVE_EFFECT_HEAL_FAILED_ADDRESS: i32 = 0x3d_6365; // HealEffect_.failed
  const MOVE_EFFECT_TELEPORT_FAILED_ADDRESS: i32 = 0x0f_7787; // SwitchAndTeleportEffect.notWildBattle2
}
impl Gen1MapAddresses for Yellow {
  const SURF_STATE_MEM_ADDRESS: u16 = 0xd6ff; // wWalkBikeSurfState (2 = surfing)
  const MAP_TILESET_MEM_ADDRESS: u16 = 0xd366; // wCurMapTileset
  const TILE_PAIR_COLLISIONS_LAND_ADDRESS: i32 = 0x0_0ada; // TilePairCollisionsLand
  const TILE_PAIR_COLLISIONS_WATER_ADDRESS: i32 = 0x0_0afc; // TilePairCollisionsWater
  const MAP_TILESET_COLLISION_PTR_BANK_OFFSET: i32 = 0x1_0000; // bank * 0x1_0000
  const MAP_TILESET_COLLISION_PTR_MEM_ADDRESS: u16 = 0xd52f; // wTilesetCollisionPtr
  const MAP_TILESET_BANK_MEM_ADDRESS: u16 = 0xd52a; // wTilesetBank
  const MAP_TILESET_BLOCKS_PTR_MEM_ADDRESS: u16 = 0xd52b; // wTilesetBlocksPtr
  const MAP_WIDTH_MEM_ADDRESS: u16 = 0xd368; // wCurMapWidth
  const MAP_HEIGHT_MEM_ADDRESS: u16 = 0xd367; // wCurMapHeight
  const OVERWORLD_MAP_MEM_ADDRESS: u16 = 0xc6e8; // wOverworldMap
  const MAP_SPRITE_STATE_DATA_2_MEM_ADDRESS: u16 = 0xc200; // wSpriteStateData2
  const MAP_SPRITE_DATA_MEM_ADDRESS: u16 = 0xd4e3; // wMapSpriteData
  const MAP_MISSABLE_OBJECT_LIST_MEM_ADDRESS: u16 = 0xd5cd; // wMissableObjectList
  const MAP_MISSABLE_OBJECT_FLAGS_MEM_ADDRESS: u16 = 0xd5a5; // wMissableObjectFlags
  const MAP_INDEX_MEM_ADDRESS: u16 = 0xd35d; // wCurMap
  const MAP_HEADER_BANKS_ADDRESS: i32 = 0x3f_43e4; // MapHeaderBanks
  const MAP_TEXT_PTR_MEM_ADDRESS: u16 = 0xd36b; // wMapTextPtr
  const TALK_TO_TRAINER_FUNCTION_ADDRESS: i32 = 0x3168; // TalkToTrainer
  const PLAYER_X_POS_MEM_ADDRESS: u16 = 0xd361; // wXCoord
  const PLAYER_Y_POS_MEM_ADDRESS: u16 = 0xd360; // wYCoord
}
impl VermilionTrashCanAddresses for Yellow {
  const FIRST_TRASH_CAN_MEM_ADDRESS: u16 = 0xd742; // wFirstLockTrashCanIndex
  const AFTER_FIRST_TRASH_CAN_ADDRESS: i32 = 0x6_5884-1; // VermilionCity_Script.initCityScript - 1
  const SECOND_TRASH_CAN_MEM_ADDRESS: u16 = 0xd743; // wSecondLockTrashCanIndex
  const AFTER_SECOND_TRASH_CAN_ADDRESS: i32 = 0x17_5e98-2; // GymTrashScript.trySecondLock - 2
}
impl FlyLocationsAddresses for Yellow {
  const FLY_LOCATIONS_MEM_ADDRESS: u16 = 0xcd3e; // wFlyLocationsList
}
impl PushBoulderAddresses for Yellow {
  const AFTER_TRY_PUSHING_BOULDER_ADDRESS: i32 = 0x00_0d2c+11; // RunMapScript + 11
  const BOULDER_FLAGS_MEM_ADDRESS: u16 = 0xcd60; // wFlags_0xcd60
}

// Gen 2
#[allow(dead_code)]
pub enum Gold {}
impl BasicRomInfo for Gold {
  const ROM_FILE_NAME: &'static str = "roms/gold.gbc";
  const GAME_NAME: &'static str = "Pokemon - Gold Version (USA, Europe)";
  const SHA1: &'static str = "D8B8A3600A465308C9953DFA04F0081C05BDCB94";
  const BOARD_NAME: &'static str = "MBC3 ROM+TIMER+RAM+BATTERY";
  const GENERATION: Generation = Generation::Gen2;
}
#[allow(dead_code)]
pub enum Silver {}
impl BasicRomInfo for Silver {
  const ROM_FILE_NAME: &'static str = "roms/silver.gbc";
  const GAME_NAME: &'static str = "Pokemon - Silver Version (USA, Europe)";
  const SHA1: &'static str = "49B163F7E57702BC939D642A18F591DE55D92DAE";
  const BOARD_NAME: &'static str = "MBC3 ROM+TIMER+RAM+BATTERY";
  const GENERATION: Generation = Generation::Gen2;
}
macro_rules! impl_gold_silver_common_addresses {
  ($($t:ty),+) => {
    $(
    impl JoypadAddresses for $t {
      const JOYPAD_READ_HI_ADDRESS: i32 = 0x0_08F7;
      const JOYPAD_READ_LO_ADDRESS: i32 = 0x0_090D;
      const JOYPAD_READ_FIRST_ADDRESS: i32 = Self::JOYPAD_READ_HI_ADDRESS;
      const JOYPAD_READ_LAST_ADDRESS: i32 = Self::JOYPAD_READ_LO_ADDRESS;
      const JOYPAD_READ_LOCKED_ADDRESS: i32 = 0x0_090F;
      const JOYPAD_INPUT_MEM_ADDRESS: u16 = 0xffa6; // hJoypadDown
      const JOYPAD_USE_ADDRESSES: &'static [i32] = &[
        0x0_0940, // in GetJoypad
      ];
      const JOYPAD_USE_DISCARD_ADDRESSES: Option<(u16, u16, u8, i32, i32, i32)> = None;
      const JOYPAD_USE_IGNORE_MASK_MEM_ADDRESSES: &'static [(i32, u16, i32)] = &[];
    }
    impl RngAddresses for $t {
      const RNG_MEM_ADDRESS: u16 = 0xffe3;
    }
    impl TextAddresses for $t {
      const TEXT_BEFORE_JOYPAD_ADDRESS: i32 = 0x0_320A;
      const TEXT_JOYPAD_ADDRESS: i32 = 0x0_0940; // in GetJoypad
      const TEXT_AFTER_JOYPAD_ADDRESS: i32 = 0x0_320D;
      const TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS: i32 = 0x0_322A;
      const TEXT_END_NOINPUT_ADDRESSES: &'static [i32] = &[
        // 0x0_1205, // DoneText; Address of the character $57 handler, used to end the text without any input required
        0x0_1283, // DoTextUntilTerminator; called when the next text command is being processed.
      ];
      const TEXT_END_WITHINPUT_ADDRESSES: &'static [i32] = &[
        0x0_11B0, // Char4B; Address of the character $4B handler, used to scroll text up after a button press
        0x0_1187, // Paragraph; Address of the character $51 handler, used start a new paragraph of text
        0x0_11EB, // PromptText; Address of the character $58 handler, used to wait for a button press before ending the text
      ];
      const TEXT_DELAY_FRAME_COUNTER_MEM_ADDRESS: u16 = 0xCEE9;
      const TEXT_SAFE_CONFLICTING_INPUT_ADDRESSES: &'static [i32] = &[];
    }
    impl InputIdentificationAddresses for $t {
      const II_ADDRESSES: &'static [(i32, i32, i32, &'static str)] = &[
        // GetJoypad
        (0x0_320A, 0x0_0940, 0x0_320D, "PrintLetterDelay"),
        (0x0_09EA, 0x0_0940, 0x0_09ED, "JoyWaitAorB"),
        (0x0_136F, 0x0_0940, 0x0_1372, "Text_TX_EXIT"),
        (0x0_13CC, 0x0_0940, 0x0_13CF, "Text_TX_DOTS"),
        (0x1_6442, 0x0_0940, 0x1_6445, "TitleScreenMain_Function6434"),
        (0x1_5E5E, 0x0_0940, 0x1_5E61, "ConfirmContinue"),
        // JoyTextDelay
        (0x00_0A8D, 0x0_0940, 0x00_0A90, "JoyWaitInput"),
        (0x00_379A, 0x0_0940, 0x00_379D, "ScrollingMenuJoyTextDelay"),
        (0x00_0A46, 0x0_0940, 0x00_0A49, "WaitPressAorB_BlinkCursor"),
        (0x00_0A56, 0x0_0940, 0x00_0A59, "SimpleWaitPressAorB"),
        (0x00_09D0, 0x0_0940, 0x00_09D3, "JoyTitleScreenInput"),
        (0x04_4438, 0x0_0940, 0x04_443B, "Pack"),
        (0x04_48CB, 0x0_0940, 0x04_48CE, "BattlePack"),
        (0x04_5CD4, 0x0_0940, 0x04_5CD7, "NamingScreenJoypadLoop"),
        (0x04_638D, 0x0_0940, 0x04_6390, "_ComposeMailMessage"),
      ];
    }
    impl Gen2MapEventsAddresses for $t {
      const OVERWORLD_BEFORE_JOYPAD_ADDRESS: i32 = 0x25_670f; // in HandleMapTimeAndJoypad
      const OVERWORLD_JOYPAD_ADDRESS: i32 = 0x0_0940;
      const OVERWORLD_AFTER_JOYPAD_ADDRESS: i32 = 0x25_6712; // in HandleMapTimeAndJoypad
      const PLAYER_EVENTS_ADDRESS: i32 = 0x25_675e; // PlayerEvents
      const PLAYER_SCRIPT_RUNNING_MEM_ADDRESS: u16 = 0xd15f; // wScriptRunning
      const PLAYER_EVENTS_SEEN_BY_TRAINER_ADDRESS: i32 = 0x25_679e; // in CheckTrainerBattle_GetPlayerEvent
      const PLAYER_EVENTS_MAP_CONNECTION_ADDRESS: i32 = 0x25_67d3; // CheckTileEvent.map_connection
      const PLAYER_EVENTS_WARP_ADDRESS: i32 = 0x25_67e3; // CheckTileEvent.not_pit
      const PLAYER_EVENTS_FALL_ADDRESS: i32 = 0x25_67df; // CheckTileEvent.pit
      const PLAYER_EVENTS_MAP_COORD_EVENT_ADDRESS: i32 = 0x25_67e7; // CheckTileEvent.coord_event
      const PLAYER_EVENTS_COUNT_STEP_EVENT_ADDRESS: i32 = 0x25_6afe; // CountStep.doscript
      const PLAYER_EVENTS_HATCH_ADDRESS: i32 = 0x25_6b02; // CountStep.hatch
      const PLAYER_EVENTS_RANDOM_ENCOUNTER_ADDRESS: i32 = 0x25_7b25; // RandomEncounter.done
      const PLAYER_EVENTS_RANDOM_ENCOUNTER_SPECIES_MEM_ADDRESS: u16 = 0xd117; // wTempWildMonSpecies
      const PLAYER_EVENTS_RANDOM_ENCOUNTER_LEVEL_MEM_ADDRESS: u16 = 0xD040; // wCurPartyLevel
      const PLAYER_EVENTS_REENTRY_SCRIPT_ADDRESS: i32 = 0x25_7a61+5; // in RunMemScript
      const PLAYER_EVENTS_SCENE_SCRIPT_ADDRESS: i32 = 0x25_6857; // in RunSceneScript
      const PLAYER_EVENTS_END_BUG_CONTEST_ADDRESS: i32 = 0x25_6899; // CheckTimeEvents.end_bug_contest
      const PLAYER_EVENTS_PHONE_CALL_ADDRESS: i32 = 0x24_40A2; // CheckPhoneCall.call
      const PLAYER_EVENTS_WHIRLPOOL_FORCED_MOVEMENT_ADDRESS: i32 = 0x4_40C2; // DoPlayerMovement.CheckTile_whirlpool
      const PLAYER_EVENTS_FORCED_MOVEMENT_ADDRESS: i32 = 0x04_413e; // DoPlayerMovement.continue_walk
      const PLAYER_EVENTS_TURNING_ADDRESS: i32 = 0x04_4167; // DoPlayerMovement.CheckTurning_turning
      const PLAYER_EVENTS_WALKING_DIRECTION_MEM_ADDRESS: u16 = 0xcf2e; // wWalkingDirection
      const PLAYER_EVENTS_STEP_WALK_ADDRESS: i32 = 0x04_41AE; // DoPlayerMovement.walk
      const PLAYER_EVENTS_STEP_BIKE_ADDRESS: i32 = 0x04_41A7; // DoPlayerMovement.fast
      const PLAYER_EVENTS_STEP_BIKE_UPHILL_ADDRESS: i32 = 0x04_41A0; // DoPlayerMovement.bike_uphill
      const PLAYER_EVENTS_STEP_ICE_ADDRESS: i32 = 0x04_41B5; // DoPlayerMovement.ice
      const PLAYER_EVENTS_STEP_SURF_ADDRESS: i32 = 0x04_41DB; // DoPlayerMovement.surf_step
      const PLAYER_EVENTS_STEP_OUT_OF_WATER_ADDRESS: i32 = 0x04_41E2; // DoPlayerMovement.ExitWater
      const PLAYER_EVENTS_JUMP_LEDGE_ADDRESS: i32 = 0x04_421A; // DoPlayerMovement.TryJump_jump
      const PLAYER_EVENTS_EDGE_WARP_ADDRESS: i32 = 0x04_4255; // DoPlayerMovement.CheckWarp_warp
      const PLAYER_EVENTS_INTERACT_OBJECT_SCRIPT_ADDRESS: i32 = 0x25_6937; // TryObjectEvent.script
      const PLAYER_EVENTS_INTERACT_OBJECT_ITEMBALL_ADDRESS: i32 = 0x25_6945; // TryObjectEvent.itemball
      const PLAYER_EVENTS_INTERACT_OBJECT_TRAINER_ADDRESS: i32 = 0x25_695c; // TryObjectEvent.trainer
      const PLAYER_EVENTS_INTERACT_BG_READ_ADDRESS: i32 = 0x25_69a5; // TryBGEvent.read
      const PLAYER_EVENTS_INTERACT_BG_HIDDEN_ITEM_ADDRESS: i32 = 0x25_69d3; // TryBGEvent.hiddenItem
      const PLAYER_EVENTS_INTERACT_BG_THENREAD_ADDRESS: i32 = 0x25_69f4; // TryBGEvent.thenread
      const PLAYER_EVENTS_INTERACT_TILE_COLLISION_ADDRESS: i32 = 0x25_7aea; // TryTileCollisionEvent.done
      const PLAYER_EVENTS_START_MENU_ADDRESS: i32 = 0x25_6a7a; // CheckMenuOW.Start
      const PLAYER_EVENTS_SELECT_MENU_ADDRESS: i32 = 0x25_6a7e; // CheckMenuOW.Select
      const PLAYER_EVENTS_NO_EVENTS_ADDRESS: i32 = 0x25_6785; // PlayerEvents.noEvents
      const PLAYER_DIRECTION_MEM_ADDRESS: u16 = 0xD205; // wPlayerDirection
    }
    impl Gen2MapAddresses for $t {
      const OVERWORLD_MAP_MEM_ADDRESS: u16 = 0xc700; // wOverworldMap
      const MAP_WIDTH_MEM_ADDRESS: u16 = 0xD088; // wMapWidth
      const MAP_HEIGHT_MEM_ADDRESS: u16 = 0xd087; // wMapHeight
      const MAP_SCRIPTS_BANK_MEM_ADDRESS: u16 = 0xd08c; // wMapScriptsBank
      const TILESET_COLLISION_PTR_MEM_ADDRESS: u16 = 0xd0c9; // wTilesetCollisionAddress
      const TILESET_COLLISION_BANK_MEM_ADDRESS: u16 = 0xd0c8; // wTilesetCollisionBank
      const TILE_COLLISION_TABLE_ADDRESS: i32 = 0x3e_74be; // TileCollisionTable
      const MAP_OBJECTS_MEM_ADDRESS: u16 = 0xD445; // wMapObjects
      const EVENT_FLAGS_MEM_ADDRESS: u16 = 0xd7b7; // wEventFlags
      const PLAYER_X_MEM_ADDRESS: u16 = 0xd20d; // wPlayerStandingMapX
      const PLAYER_Y_MEM_ADDRESS: u16 = 0xd20e; // wPlayerStandingMapY
    }
    impl Gen2DVAddresses for $t {
      const AFTER_DV_GENERATION_ADDRESS: i32 = 0x03_59bb; // GeneratePartyMonStats.initializeDVs
      const AFTER_WILD_DV_GENERATION_ADDRESS: i32 = 0x0f_6800; // LoadEnemyMon.UpdateDVs
    }
    impl BattleDetermineMoveOrderAddresses for $t {
      const DETERMINE_MOVE_ORDER_START_ADDRESS: i32 = 0x0F_42cb; // DetermineMoveOrder
      const MOVE_ORDER_PLAYER_FIRST_ADDRESS: i32 = 0x0F_43a8; // DetermineMoveOrder.player_first
      const MOVE_ORDER_ENEMY_FIRST_ADDRESS: i32 = 0x0F_43aa; // DetermineMoveOrder.enemy_first
    }
    impl AIChooseMoveAddresses for $t {
      const AFTER_AI_CHOOSE_MOVE_ADDRESS: i32 = 0x0F_4148; // BattleTurn.not_disconnected
      const CUR_ENEMY_MOVE_MEM_ADDRESS: u16 = 0xCBC2; // wCurEnemyMove
    }
    impl BattleObedienceAddresses for $t {
      const CHECK_OBEDIENCE_START_ADDRESS: i32 = 0x0D_43EA; // BattleCommand_CheckObedience
      const CHECK_OBEDIENCE_OBEY_ADDRESS: i32 = 0x0D_4067; // DoMove.ReadMoveEffectCommand (for next command)
      const CHECK_OBEDIENCE_DISOBEY_ADDRESS: i32 = 0x0D_4529; // IgnoreSleepOnly (at this point disobey is certain)
    }
    impl Gen2FightTurnAddresses for $t {
      const NEXT_BATTLE_COMMAND_ADDRESS: i32 = 0x0D_4067; // DoMove.ReadMoveEffectCommand (for next command)
      const BATTLE_COMMAND_DOTURN_ADDRESS: i32 = 0x0d_4699; // BattleCommand_DoTurn
      const OUT_OF_PP_ADDRESS: i32 = 0x0D_4727; // BattleCommand_DoTurn.out_of_pp
      const BATTLE_COMMAND_DAMAGEVARIATION_ADDRESS: i32 = 0x0D_4e4d; // BattleCommand_DamageVariation
      const CUR_DAMAGE_MEM_ADDRESS: u16 = 0xd141; // wCurDamage
      const BATTLE_COMMAND_LOWERSUB_ADDRESS: i32 = 0x0D_503e; // BattleCommand_LowerSub
      const BATTLE_COMMAND_MOVEANIMNOSUB_ADDRESS: i32 = 0x0D_50b1; // BattleCommand_MoveAnimNoSub
      const ATTACK_MISSED_MEM_ADDRESS: u16 = 0xcb45; // wAttackMissed
      const EFFECT_FAILED_MEM_ADDRESS: u16 = 0xcbeb; // wEffectFailed
      const CRITICAL_HIT_MEM_ADDRESS: u16 = 0xcb44; // wCriticalHit
      const CUR_MOVE_INDEX_MEM_ADDRESS: u16 = 0xcfc7; // wCurMoveNum
      const FULLY_PARALYZED_ADDRESS: i32 = 0x0D_439B; // in CheckEnemyTurn.no_disabled_move
    }
    impl BattleMovesInfoAddresses for $t {
      const MOVES_ADDRESS: i32 = 0x10_5AFE; // Moves
      const MOVES_ENTRY_LENGTH: i32 = 7; // length of a single move

      const GEN2_BADGES_MEM_ADDRESS: u16 = 0xd57c; // wJohtoBadges
      const TYPE_MATCHUPS_ADDRESS: i32 = 0x0D_4d01; // TypeMatchups
    }
    impl BattleMonInfoAddresses for $t {
      const BATTLE_MON_STRUCT_MEM_ADDRESS: u16 = 0xCB0C; // wBattleMon
      const BATTLE_MON_STAT_LEVELS_MEM_ADDRESS: u16 = 0xcbaa; // wPlayerStatLevels
      const GEN1_PARTY_MON_STATS_BASE_MEM_ADDRESS: u16 = 0; // unused
      const GEN1_PARTY_MON_STRUCT_LEN: u16 = 0; // unused
      const GEN1_PLAYER_MON_NUMBER_MEM_ADDRESS: u16 = 0; // unused
      const GEN2_BATTLE_MON_ORIG_STATS_MEM_ADDRESS: u16 = 0xcb94; // wPlayerStats

      const ENEMY_MON_STRUCT_MEM_ADDRESS: u16 = 0xD0EF; // wEnemyMon
      const ENEMY_MON_STAT_LEVELS_MEM_ADDRESS: u16 = 0xcbb2; // wEnemyStatLevels
      const GEN1_BASE_STATS_BASE_ADDRESS: i32 = 0; // unused
      const GEN1_BASE_STATS_LEN: i32 = 0; // unused
      const GEN2_ENEMY_MON_ORIG_STATS_MEM_ADDRESS: u16 = 0xcb9f; // wEnemyStats
    }
    impl Gen2BattleSwitchMonAddresses for $t {
      const SWITCH_DECIDED_ADDRESS: i32 = 0x0f_55c3; // LoadEnemyMonToSwitchTo.skip_unown
      const SWITCH_SPECIES_MEM_ADDRESS: u16 = 0xd004; // wCurPartySpecies
      const SWITCH_LEVEL_MEM_ADDRESS: u16 = 0xd040; // wCurPartyLevel
    }
    impl Gen2BattleSpiteAddresses for $t {
      const SPITE_SUCCESS_ADDRESS: i32 = 0x0d_5d97; // BattleCommand_Spite.deplete_pp
      const SPITE_FAIL_ADDRESS: i32 = 0x0d_5dcd; // BattleCommand_Spite.failed
    }
    impl Gen2BattleMultiHitAddresses for $t {
      const MULTI_HIT_ADDRESS: i32 = 0x0D_6b48; // BattleCommand_EndLoop.double_hit
    }
    impl RoamMonAddresses for $t {
      const AFTER_ROAM_MON_UPDATE_ADDRESS: i32 = 0x0a_6942; // _BackUpMapIndices
      const ROAM_MON_DATA_MEM_ADDRESS: u16 = 0xdd1a; // wRoamMon1
    }
    impl HallOfFameAddresses for $t {
      const HALL_OF_FAME_AFTER_SAVING_ADDRESS: i32 = 0x21_64af; // AnimateHallOfFame
    }
    )+
  }
}
impl_gold_silver_common_addresses!(Gold, Silver);
impl BattleCatchMonAddresses for Gold {
  const CATCH_SUCCESS_ADDRESS: i32 = 0x03_6A20; // PokeBallEffect.catch_without_fail
  const CATCH_FAIL_ADDRESS: i32 = 0x03_6A23; // PokeBallEffect.fail_to_catch
}
impl BattleCatchMonAddresses for Silver {
  const CATCH_SUCCESS_ADDRESS: i32 = 0x03_6A1E; // PokeBallEffect.catch_without_fail
  const CATCH_FAIL_ADDRESS: i32 = 0x03_6A21; // PokeBallEffect.fail_to_catch
}

#[allow(dead_code)]
pub enum Crystal {}
impl BasicRomInfo for Crystal {
  const ROM_FILE_NAME: &'static str = "roms/crystal.gbc";
  const GAME_NAME: &'static str = "Pokemon - Crystal Version (USA, Europe)";
  const SHA1: &'static str = "F4CD194BDEE0D04CA4EAC29E09B8E4E9D818C133";
  const BOARD_NAME: &'static str = "MBC3 ROM+TIMER+RAM+BATTERY";
  const GENERATION: Generation = Generation::Gen2;
}
impl JoypadAddresses for Crystal {
  const JOYPAD_READ_HI_ADDRESS: i32 = 0x0_0946;
  const JOYPAD_READ_LO_ADDRESS: i32 = 0x0_095C;
  const JOYPAD_READ_FIRST_ADDRESS: i32 = Self::JOYPAD_READ_HI_ADDRESS;
  const JOYPAD_READ_LAST_ADDRESS: i32 = Self::JOYPAD_READ_LO_ADDRESS;
  const JOYPAD_READ_LOCKED_ADDRESS: i32 = 0x0_095E;
  const JOYPAD_INPUT_MEM_ADDRESS: u16 = 0xffa4; // hJoypadDown
  const JOYPAD_USE_ADDRESSES: &'static [i32] = &[
    0x0_098F, // in GetJoypad
    0x42_58FD, // Credits_HandleAButton
    0x42_5908, // Credits_HandleBButton
    0x24_5D79, // in Pokedex_GetArea
    0x38_5FCC, // in _DummyGame
    0x38_61AB, // in DummyGame_InterpretJoypad_AnimateCursor
    0x24_68D9, // in SlotsAction_WaitReel1
    0x24_6903, // in SlotsAction_WaitReel2
    0x24_692D, // in SlotsAction_WaitReel3
  ];
  const JOYPAD_USE_DISCARD_ADDRESSES: Option<(u16, u16, u8, i32, i32, i32)> = None;
  const JOYPAD_USE_IGNORE_MASK_MEM_ADDRESSES: &'static [(i32, u16, i32)] = &[];
}
impl RngAddresses for Crystal {
  const RNG_MEM_ADDRESS: u16 = 0xffe1;
}
impl TextAddresses for Crystal {
  const TEXT_BEFORE_JOYPAD_ADDRESS: i32 = 0x0_3165;
  const TEXT_JOYPAD_ADDRESS: i32 = 0x0_098F; // in GetJoypad
  const TEXT_AFTER_JOYPAD_ADDRESS: i32 = 0x0_3168;
  const TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS: i32 = 0x0_3185;
  const TEXT_END_NOINPUT_ADDRESSES: &'static [i32] = &[
    // 0x0_137C, // DoneText; Address of the character $57 handler, used to end the text without any input required
    0x0_13F6, // DoTextUntilTerminator; called when the next text command is being processed.
  ];
  const TEXT_END_WITHINPUT_ADDRESSES: &'static [i32] = &[
    0x0_131F, // _ContText; Address of the character $4B handler, used to scroll text up after a button press
    0x0_12F2, // Paragraph; Address of the character $51 handler, used start a new paragraph of text
    0x0_135A, // PromptText; Address of the character $58 handler, used to wait for a button press before ending the text
  ];
  const TEXT_DELAY_FRAME_COUNTER_MEM_ADDRESS: u16 = 0xCFB2;
  const TEXT_SAFE_CONFLICTING_INPUT_ADDRESSES: &'static [i32] = &[];
}
impl InputIdentificationAddresses for Crystal {
  const II_ADDRESSES: &'static [(i32, i32, i32, &'static str)] = &[
    // GetJoypad
    (0x0_3165, 0x0_098F, 0x0_3168, "PrintLetterDelay"),
    (0x0_14EF, 0x0_098F, 0x0_14F2, "Text_TX_EXIT"),
    (0x0_154C, 0x0_098F, 0x0_154F, "Text_TX_DOTS"),
    (0x1_5E37, 0x0_098F, 0x1_5E3A, "ConfirmContinue"),
    (0x1_6312, 0x0_098F, 0x1_6312, "TitleScreenMain"),
    (0x25_67CA, 0x0_098F, 0x25_67CD, "HandleMapTimeAndJoypad"),
    (0x2E_529A, 0x0_098F, 0x2E_529D, "ReadAnyMail"),
    (0x40_4A95, 0x0_098F, 0x40_4A98, "_LinkBattleSendReceiveAction"),
    (0x40_5418, 0x0_098F, 0x40_541B, "_LinkBattle_Function101418"),
    (0x40_54B7, 0x0_098F, 0x40_54BA, "_LinkBattle_Function1014b7"),
    (0x40_64AF, 0x0_098F, 0x40_64B2, "_LinkBattle_Function1024af"),
    (0x40_66C8, 0x0_098F, 0x40_66CB, "_LinkBattle_Function1026c8"),
    (0x40_6754, 0x0_098F, 0x40_6757, "_LinkBattle_Function102754"),
    (0x40_68FC, 0x0_098F, 0x40_68FF, "_LinkBattle_Function1028fc"),
    (0x40_73AF, 0x0_098F, 0x40_73B2, "_LinkBattle_Function1033af"),
    (0x13_5E2C, 0x0_098F, 0x13_5E2F, "StatsScreen_GetJoypad"),
    (0x0_0A39, 0x0_098F, 0x0_0A3C, "JoyWaitAorB"),
    // JoyTextDelay
    (0x00_0A1F, 0x0_098F, 0x00_0A22, "JoyTitleScreenInput"),
    (0x00_0A95, 0x0_098F, 0x00_0A98, "WaitPressAorB_BlinkCursor"),
    (0x00_0AA5, 0x0_098F, 0x00_0AA8, "SimpleWaitPressAorB"),
    (0x00_0ADC, 0x0_098F, 0x00_0ADF, "ButtonSound"),
    (0x00_3555, 0x0_098F, 0x00_3558, "JoyTextDelay_ForcehJoyDown"),
    (0x04_4008, 0x0_098F, 0x04_400B, "Pack"),
    (0x04_449B, 0x0_098F, 0x04_449E, "BattlePack"),
    (0x04_5915, 0x0_098F, 0x04_5918, "NamingScreenJoypadLoop"),
    (0x04_5FC0, 0x0_098F, 0x04_5FC3, "_ComposeMailMessage"),
    (0x05_6571, 0x0_098F, 0x05_6574, "Mom_WithdrawDepositMenuJoypad"),
    (0x05_6C6B, 0x0_098F, 0x05_6C6E, "_UnownPrinter"),
    (0x08_40BA, 0x0_098F, 0x08_40BD, "RestartClock"),
    (0x09_4266, 0x0_098F, 0x09_4269, "Menu_WasButtonPressed"),
    (0x09_511A, 0x0_098F, 0x09_511D, "TrainerCard"),
    (0x0A_4AE3, 0x0_098F, 0x0A_4AE6, "link_Function28ade"),
    (0x10_4029, 0x0_098F, 0x10_402C, "Pokedex"),
    (0x12_47C6, 0x0_098F, 0x12_47C9, "mobile_Function4876f"),
    (0x12_4923, 0x0_098F, 0x12_4926, "mobile_asm_48922"),
    (0x13_543A, 0x0_098F, 0x13_543D, "ClockResetPassword"),
    (0x13_677D, 0x0_098F, 0x13_6780, "EvolutionAnimation"),
    (0x20_5A74, 0x0_098F, 0x20_5A77, "debug_Function81a74"),
    (0x21_43F0, 0x0_098F, 0x21_43F3, "SendScreenToPrinter"),
    (0x21_4597, 0x0_098F, 0x21_459A, "PrintUnownStamp"),
    (0x21_666E, 0x0_098F, 0x21_6671, "_HallOfFamePC"),
    (0x22_67B3, 0x0_098F, 0x22_67B6, "mobile_Function8a78c"),
    (0x24_470D, 0x0_098F, 0x24_4710, "InitClock_hour"),
    (0x24_4752, 0x0_098F, 0x24_4755, "InitClock_minute"),
    (0x24_496A, 0x0_098F, 0x24_496A, "SetDayOfWeek"),
    (0x24_4BAC, 0x0_098F, 0x24_4BAF, "PokeGear"),
    (0x24_5399, 0x0_098F, 0x24_539C, "PokegearPhoneContactSubmenu"),
    (0x24_59B0, 0x0_098F, 0x24_59B3, "_TownMap"),
    (0x24_5A62, 0x0_098F, 0x24_5A65, "PlayRadio"),
    (0x24_5B29, 0x0_098F, 0x24_5B2C, "_FlyMap"),
    (0x24_5D6E, 0x0_098F, 0x24_5D71, "Pokedex_GetArea"),
    (0x24_636E, 0x0_098F, 0x24_6371, "Unreferenced_Function92311"),
    (0x38_425C, 0x0_098F, 0x38_425F, "_CardFlip_1"),
    (0x38_42BD, 0x0_098F, 0x38_42C0, "_CardFlip_2"),
    (0x38_5217, 0x0_098F, 0x38_521A, "_UnownPuzzle"),
    (0x38_61A8, 0x0_098F, 0x38_61AB, "DummyGame_InterpretJoypad_AnimateCursor"),
    (0x38_63B4, 0x0_098F, 0x38_63B7, "_DepositPKMN"),
    (0x38_65A7, 0x0_098F, 0x38_65AA, "_WithdrawPKMN"),
    (0x38_6781, 0x0_098F, 0x38_6784, "_MovePKMNWithoutMail"),
    (0x39_4217, 0x0_098F, 0x39_421A, "_OptionsMenu"),
    (0x39_45C0, 0x0_098F, 0x39_45C3, "Copyright_GFPresents"),
    (0x39_48BC, 0x0_098F, 0x39_48BF, "CrystalIntro"),
    (0x43_6D54, 0x0_098F, 0x43_6D57, "unused_title_Function10ed51"),
    (0x45_76EE, 0x0_098F, 0x45_76F1, "mobile_Function1176ee"),
    (0x45_7ACD, 0x0_098F, 0x45_7AD0, "mobile_Function117acd"),
    (0x46_4024, 0x0_098F, 0x46_4027, "mobile_asm_11800b"),
    (0x46_407D, 0x0_098F, 0x46_4080, "mobile_Function11805f"),
    (0x46_40D1, 0x0_098F, 0x46_40D4, "mobile_Function1180b8"),
    (0x46_413E, 0x0_098F, 0x46_4141, "mobile_Function118125"),
    (0x46_41F8, 0x0_098F, 0x46_41FB, "mobile_Function1181da"),
    (0x46_424C, 0x0_098F, 0x46_424F, "mobile_Function118233"),
    (0x46_429D, 0x0_098F, 0x46_42A0, "mobile_Function118284"),
    (0x46_42EE, 0x0_098F, 0x46_42F1, "mobile_Function1182d5"),
    (0x46_4342, 0x0_098F, 0x46_4345, "mobile_Function118329"),
    (0x46_4393, 0x0_098F, 0x46_4396, "mobile_Function11837a"),
    (0x46_6C82, 0x0_098F, 0x46_6C85, "mobile_Function11ac51"),
    (0x47_4283, 0x0_098F, 0x47_4286, "EZChat_MasterLoop"),
    (0x5B_57F4, 0x0_098F, 0x5B_57F7, "LinkTradeMenu"),
    (0x5C_44FA, 0x0_098F, 0x5C_44FD, "mobile_Function1704e1"),
    (0x5C_5A11, 0x0_098F, 0x5C_5A14, "mobile_Function171a11"),
    (0x5E_6788, 0x0_098F, 0x5E_678B, "mobile_Function17a781"),
    (0x5F_545A, 0x0_098F, 0x5F_545D, "mobile_Function17d45a"),
    (0x5F_63F6, 0x0_098F, 0x5F_63F9, "mobile_Function17e3f0"),
    (0x5F_7555, 0x0_098F, 0x5F_7558, "DisplayMobileError"),
    // misc. direct input uses
    (0x42_58FD, 0x42_58FD, 0x42_58FD, "Credits_HandleAButton"),
    (0x42_5908, 0x42_5908, 0x42_5908, "Credits_HandleBButton"),
    (0x24_5D79, 0x24_5D79, 0x24_5D79, "Pokedex_GetArea"),
    (0x38_5FCC, 0x38_5FCC, 0x38_5FCC, "_DummyGame"),
    (0x38_61AB, 0x38_61AB, 0x38_61AB, "DummyGame_InterpretJoypad_AnimateCursor_direct"),
    (0x24_68D9, 0x24_68D9, 0x24_68D9, "SlotsAction_WaitReel1"),
    (0x24_6903, 0x24_6903, 0x24_6903, "SlotsAction_WaitReel2"),
    (0x24_692D, 0x24_692D, 0x24_692D, "SlotsAction_WaitReel3"),
  ];
}
impl Gen2MapEventsAddresses for Crystal {
  const OVERWORLD_BEFORE_JOYPAD_ADDRESS: i32 = 0x25_67CA; // in HandleMapTimeAndJoypad
  const OVERWORLD_JOYPAD_ADDRESS: i32 = 0x0_098F;
  const OVERWORLD_AFTER_JOYPAD_ADDRESS: i32 = 0x25_67CD; // in HandleMapTimeAndJoypad
  const PLAYER_EVENTS_ADDRESS: i32 = 0x25_681F; // PlayerEvents
  const PLAYER_SCRIPT_RUNNING_MEM_ADDRESS: u16 = 0xd438; // wScriptRunning
  const PLAYER_EVENTS_SEEN_BY_TRAINER_ADDRESS: i32 = 0x25_686E; // in CheckTrainerBattle_GetPlayerEvent
  const PLAYER_EVENTS_MAP_CONNECTION_ADDRESS: i32 = 0x25_68A6; // CheckTileEvent.map_connection
  const PLAYER_EVENTS_WARP_ADDRESS: i32 = 0x25_68B6; // CheckTileEvent.not_pit
  const PLAYER_EVENTS_FALL_ADDRESS: i32 = 0x25_68B2; // CheckTileEvent.pit
  const PLAYER_EVENTS_MAP_COORD_EVENT_ADDRESS: i32 = 0x25_68BA; // CheckTileEvent.coord_event
  const PLAYER_EVENTS_COUNT_STEP_EVENT_ADDRESS: i32 = 0x25_6BCB; // CountStep.doscript
  const PLAYER_EVENTS_HATCH_ADDRESS: i32 = 0x25_6BCF; // CountStep.hatch
  const PLAYER_EVENTS_RANDOM_ENCOUNTER_ADDRESS: i32 = 0x25_7CF4; // RandomEncounter.done
  const PLAYER_EVENTS_RANDOM_ENCOUNTER_SPECIES_MEM_ADDRESS: u16 = 0xD22E; // wTempWildMonSpecies
  const PLAYER_EVENTS_RANDOM_ENCOUNTER_LEVEL_MEM_ADDRESS: u16 = 0xD143; // wCurPartyLevel
  const PLAYER_EVENTS_REENTRY_SCRIPT_ADDRESS: i32 = 0x25_7C35; // in RunMemScript
  const PLAYER_EVENTS_SCENE_SCRIPT_ADDRESS: i32 = 0x25_692a; // in RunSceneScript
  const PLAYER_EVENTS_END_BUG_CONTEST_ADDRESS: i32 = 0x25_6966; // CheckTimeEvents.end_bug_contest
  const PLAYER_EVENTS_PHONE_CALL_ADDRESS: i32 = 0x24_40A2; // CheckPhoneCall.call
  const PLAYER_EVENTS_WHIRLPOOL_FORCED_MOVEMENT_ADDRESS: i32 = 0x20_40C2; // DoPlayerMovement.CheckTile_whirlpool
  const PLAYER_EVENTS_FORCED_MOVEMENT_ADDRESS: i32 = 0x20_413E; // DoPlayerMovement.continue_walk
  const PLAYER_EVENTS_TURNING_ADDRESS: i32 = 0x20_4167; // DoPlayerMovement.CheckTurning_turning
  const PLAYER_EVENTS_WALKING_DIRECTION_MEM_ADDRESS: u16 = 0xD043; // wWalkingDirection
  const PLAYER_EVENTS_STEP_WALK_ADDRESS: i32 = 0x20_41AE; // DoPlayerMovement.walk
  const PLAYER_EVENTS_STEP_BIKE_ADDRESS: i32 = 0x20_41A7; // DoPlayerMovement.fast
  const PLAYER_EVENTS_STEP_BIKE_UPHILL_ADDRESS: i32 = 0x20_41A0; // DoPlayerMovement.bike_uphill
  const PLAYER_EVENTS_STEP_ICE_ADDRESS: i32 = 0x20_41B5; // DoPlayerMovement.ice
  const PLAYER_EVENTS_STEP_SURF_ADDRESS: i32 = 0x20_41DB; // DoPlayerMovement.surf_step
  const PLAYER_EVENTS_STEP_OUT_OF_WATER_ADDRESS: i32 = 0x20_41E2; // DoPlayerMovement.ExitWater
  const PLAYER_EVENTS_JUMP_LEDGE_ADDRESS: i32 = 0x20_421A; // DoPlayerMovement.TryJump_jump
  const PLAYER_EVENTS_EDGE_WARP_ADDRESS: i32 = 0x20_4255; // DoPlayerMovement.CheckWarp_warp
  const PLAYER_EVENTS_INTERACT_OBJECT_SCRIPT_ADDRESS: i32 = 0x25_6A04; // TryObjectEvent.script
  const PLAYER_EVENTS_INTERACT_OBJECT_ITEMBALL_ADDRESS: i32 = 0x25_6A12; // TryObjectEvent.itemball
  const PLAYER_EVENTS_INTERACT_OBJECT_TRAINER_ADDRESS: i32 = 0x25_6A29; // TryObjectEvent.trainer
  const PLAYER_EVENTS_INTERACT_BG_READ_ADDRESS: i32 = 0x25_6A72; // TryBGEvent.read
  const PLAYER_EVENTS_INTERACT_BG_HIDDEN_ITEM_ADDRESS: i32 = 0x25_6AA0; // TryBGEvent.hiddenItem
  const PLAYER_EVENTS_INTERACT_BG_THENREAD_ADDRESS: i32 = 0x25_6AC1; // TryBGEvent.thenread
  const PLAYER_EVENTS_INTERACT_TILE_COLLISION_ADDRESS: i32 = 0x25_7CB9; // TryTileCollisionEvent.done
  const PLAYER_EVENTS_START_MENU_ADDRESS: i32 = 0x25_6B47; // CheckMenuOW.Start
  const PLAYER_EVENTS_SELECT_MENU_ADDRESS: i32 = 0x25_6B4B; // CheckMenuOW.Select
  const PLAYER_EVENTS_NO_EVENTS_ADDRESS: i32 = 0x25_6846; // PlayerEvents.noEvents
  const PLAYER_DIRECTION_MEM_ADDRESS: u16 = 0xD4DE; // wPlayerDirection
}
impl Gen2MapAddresses for Crystal {
  const OVERWORLD_MAP_MEM_ADDRESS: u16 = 0xc800; // wOverworldMap
  const MAP_WIDTH_MEM_ADDRESS: u16 = 0xd19f; // wMapWidth
  const MAP_HEIGHT_MEM_ADDRESS: u16 = 0xd19e; // wMapHeight
  const MAP_SCRIPTS_BANK_MEM_ADDRESS: u16 = 0xd1a3; // wMapScriptsBank
  const TILESET_COLLISION_PTR_MEM_ADDRESS: u16 = 0xd1e0; // wTilesetCollisionAddress
  const TILESET_COLLISION_BANK_MEM_ADDRESS: u16 = 0xd1df; // wTilesetCollisionBank
  const TILE_COLLISION_TABLE_ADDRESS: i32 = 0x13_4E1F; // TileCollisionTable
  const MAP_OBJECTS_MEM_ADDRESS: u16 = 0xd71e; // wMapObjects
  const EVENT_FLAGS_MEM_ADDRESS: u16 = 0xda72; // wEventFlags
  const PLAYER_X_MEM_ADDRESS: u16 = 0xD4E6; // wPlayerStandingMapX
  const PLAYER_Y_MEM_ADDRESS: u16 = 0xD4E7; // wPlayerStandingMapY
}
impl Gen2DVAddresses for Crystal {
  const AFTER_DV_GENERATION_ADDRESS: i32 = 0x03_59B5; // GeneratePartyMonStats.initializeDVs
  const AFTER_WILD_DV_GENERATION_ADDRESS: i32 = 0x0F_69A8; // LoadEnemyMon.UpdateDVs
}
impl BattleDetermineMoveOrderAddresses for Crystal {
  const DETERMINE_MOVE_ORDER_START_ADDRESS: i32 = 0x0F_4314; // DetermineMoveOrder
  const MOVE_ORDER_PLAYER_FIRST_ADDRESS: i32 = 0x0F_43F1; // DetermineMoveOrder.player_first
  const MOVE_ORDER_ENEMY_FIRST_ADDRESS: i32 = 0x0F_43F3; // DetermineMoveOrder.enemy_first
}
impl AIChooseMoveAddresses for Crystal {
  const AFTER_AI_CHOOSE_MOVE_ADDRESS: i32 = 0x0F_4174; // BattleTurn.not_disconnected
  const CUR_ENEMY_MOVE_MEM_ADDRESS: u16 = 0xC6E4; // wCurEnemyMove
}
impl BattleObedienceAddresses for Crystal {
  const CHECK_OBEDIENCE_START_ADDRESS: i32 = 0x0D_43DB; // BattleCommand_CheckObedience
  const CHECK_OBEDIENCE_OBEY_ADDRESS: i32 = 0x0D_4058; // DoMove.ReadMoveEffectCommand (for next command)
  const CHECK_OBEDIENCE_DISOBEY_ADDRESS: i32 = 0x0D_451F; // IgnoreSleepOnly (at this point disobey is certain)
}
impl Gen2FightTurnAddresses for Crystal {
  const NEXT_BATTLE_COMMAND_ADDRESS: i32 = 0x0D_4058; // DoMove.ReadMoveEffectCommand (for next command)
  const BATTLE_COMMAND_DOTURN_ADDRESS: i32 = 0x0D_4555; // BattleCommand_DoTurn
  const OUT_OF_PP_ADDRESS: i32 = 0x0D_45E3; // BattleCommand_DoTurn.out_of_pp
  const BATTLE_COMMAND_DAMAGEVARIATION_ADDRESS: i32 = 0x0D_4CFD; // BattleCommand_DamageVariation
  const CUR_DAMAGE_MEM_ADDRESS: u16 = 0xD256; // wCurDamage
  const BATTLE_COMMAND_LOWERSUB_ADDRESS: i32 = 0x0D_4EEE; // BattleCommand_LowerSub
  const BATTLE_COMMAND_MOVEANIMNOSUB_ADDRESS: i32 = 0x0D_4F60; // BattleCommand_MoveAnimNoSub
  const ATTACK_MISSED_MEM_ADDRESS: u16 = 0xC667; // wAttackMissed
  const EFFECT_FAILED_MEM_ADDRESS: u16 = 0xC70D; // wEffectFailed
  const CRITICAL_HIT_MEM_ADDRESS: u16 = 0xC666; // wCriticalHit
  const CUR_MOVE_INDEX_MEM_ADDRESS: u16 = 0xD0D5; // wCurMoveNum
  const FULLY_PARALYZED_ADDRESS: i32 = 0x0D_437C; // in CheckEnemyTurn.no_disabled_move
}
impl BattleMovesInfoAddresses for Crystal {
  const MOVES_ADDRESS: i32 = 0x10_5AFB; // Moves
  const MOVES_ENTRY_LENGTH: i32 = 7; // length of a single move

  const GEN2_BADGES_MEM_ADDRESS: u16 = 0xD857; // wJohtoBadges
  const TYPE_MATCHUPS_ADDRESS: i32 = 0x0D_4BB1; // TypeMatchups
}
impl BattleMonInfoAddresses for Crystal {
  const BATTLE_MON_STRUCT_MEM_ADDRESS: u16 = 0xC62C; // wBattleMon
  const BATTLE_MON_STAT_LEVELS_MEM_ADDRESS: u16 = 0xC6CC; // wPlayerStatLevels
  const GEN1_PARTY_MON_STATS_BASE_MEM_ADDRESS: u16 = 0; // unused
  const GEN1_PARTY_MON_STRUCT_LEN: u16 = 0; // unused
  const GEN1_PLAYER_MON_NUMBER_MEM_ADDRESS: u16 = 0; // unused
  const GEN2_BATTLE_MON_ORIG_STATS_MEM_ADDRESS: u16 = 0xC6B6; // wPlayerStats

  const ENEMY_MON_STRUCT_MEM_ADDRESS: u16 = 0xD206; // wEnemyMon
  const ENEMY_MON_STAT_LEVELS_MEM_ADDRESS: u16 = 0xC6D4; // wEnemyStatLevels
  const GEN1_BASE_STATS_BASE_ADDRESS: i32 = 0; // unused
  const GEN1_BASE_STATS_LEN: i32 = 0; // unused
  const GEN2_ENEMY_MON_ORIG_STATS_MEM_ADDRESS: u16 = 0xC6C1; // wEnemyStats
}
impl BattleCatchMonAddresses for Crystal {
  const CATCH_SUCCESS_ADDRESS: i32 = 0x03_699C; // PokeBallEffect.catch_without_fail
  const CATCH_FAIL_ADDRESS: i32 = 0x03_699F; // PokeBallEffect.fail_to_catch
}
impl Gen2BattleSwitchMonAddresses for Crystal {
  const SWITCH_DECIDED_ADDRESS: i32 = 0x0F_5708; // LoadEnemyMonToSwitchTo.skip_unown
  const SWITCH_SPECIES_MEM_ADDRESS: u16 = 0xD108; // wCurPartySpecies
  const SWITCH_LEVEL_MEM_ADDRESS: u16 = 0xD143; // wCurPartyLevel
}
impl Gen2BattleSpiteAddresses for Crystal {
  const SPITE_SUCCESS_ADDRESS: i32 = 0x0D_5C5B; // BattleCommand_Spite.deplete_pp
  const SPITE_FAIL_ADDRESS: i32 = 0x0D_5C91; // BattleCommand_Spite.failed
}
impl Gen2BattleMultiHitAddresses for Crystal {
  const MULTI_HIT_ADDRESS: i32 = 0x0D_6A3A; // BattleCommand_EndLoop.double_hit
}
impl RoamMonAddresses for Crystal {
  const AFTER_ROAM_MON_UPDATE_ADDRESS: i32 = 0x0A_63F6; // _BackUpMapIndices
  const ROAM_MON_DATA_MEM_ADDRESS: u16 = 0xDFCF; // wRoamMon1
}
impl HallOfFameAddresses for Crystal {
  const HALL_OF_FAME_AFTER_SAVING_ADDRESS: i32 = 0x21_64C3; // AnimateHallOfFame
}
