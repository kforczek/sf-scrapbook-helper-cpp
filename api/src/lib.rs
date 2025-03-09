#![warn(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::print_stdout,
    clippy::print_stderr,
    missing_debug_implementations,
    clippy::pedantic,
)]
#![allow(
    clippy::redundant_closure_for_method_calls,
    clippy::wildcard_imports,
    clippy::too_many_lines,
    clippy::field_reassign_with_default,
    clippy::match_bool
)]
#![allow(unsafe_code)] // Explicitly allowing unsafe code for FFI

pub mod command;
pub mod error;
pub mod gamestate;
pub mod misc;
pub mod response;
#[cfg(feature = "session")]
pub mod session;
pub mod simulate;
#[cfg(feature = "sso")]
pub mod sso;

/// Represents the numerical ID of a player on a server.
pub type PlayerId = u32;


pub use crate::error::SFError;
pub use crate::session::{ServerConnection, Session};
//pub use crate::command::Command;
pub use crate::response::Response;
// pub use crate::command::{
//     ExpeditionSetting, BlacksmithAction, FortunePayment, RollDicePrice, DiceType, DiceReward, AttributeType,
//     ShopType, TimeSkip, GuildPortal, BattlesJoined, GuildMemberData,
//     GuildRank, GuildSkill, Mount,
// };
pub use crate::command::*;
//pub use crate::command::Command::*;
//pub use crate::unlockables::Unlockable;
pub use crate::{
    gamestate::{
        character::*,
        dungeons::{CompanionClass, Dungeon},
        fortress::*,
        guild::{Emblem, GuildSkill},
        idle::IdleBuildingType,
        items::*,
        social::Relationship,
        underworld::*,
        unlockables::{
            EnchantmentIdent, HabitatType, HellevatorTreatType, Unlockable,
        },
    },
};

use std::ffi::{CStr, CString};
use std::ptr;
use tokio::runtime::Runtime;


// ##############################################
// #                 SESSION                    #
// ##############################################

/// Creates a new session instance
#[no_mangle]
pub extern "C" fn init_session(username: *const i8, password: *const i8, server_url: *const i8) -> *mut Session {
    let user = unsafe { CStr::from_ptr(username).to_str().unwrap_or("").to_string() };
    let pass = unsafe { CStr::from_ptr(password).to_str().unwrap_or("").to_string() };
    let server = unsafe { CStr::from_ptr(server_url).to_str().unwrap_or("").to_string() };

    let server_connection = match ServerConnection::new(&server) {
        Some(conn) => conn,
        None => return ptr::null_mut(),
    };

    let session = Session::new(&user, &pass, server_connection);
    Box::into_raw(Box::new(session))
}

/// Frees a session instance
#[no_mangle]
pub extern "C" fn destr_session(session: *mut Session) {
    if !session.is_null() {
        unsafe { drop(Box::from_raw(session)) };
    }
}

/// Logs in using a session
#[no_mangle]
pub extern "C" fn login(session: *mut Session) -> bool {
    if session.is_null() {
        return false;
    }
    let session = unsafe { &mut *session };
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(session.login()).is_ok()
}


// #######################################################
// #                 RESPONSE FREEING                    #
// #######################################################

/// Frees a response object
#[no_mangle]
pub extern "C" fn destr_response(response: *mut Response) {
    if !response.is_null() {
        unsafe { drop(Box::from_raw(response)) };
    }
}


// #####################################################
// #                 RESPONSE VALUE                    #
// #####################################################

/// Retrieves a value from a parsed response
#[no_mangle]
pub extern "C" fn response_get_value(response: *mut Response, key: *const i8) -> *mut i8 {
    if response.is_null() || key.is_null() {
        return ptr::null_mut();
    }

    let key_cstr = unsafe { CStr::from_ptr(key) };
    let key_str = key_cstr.to_str().unwrap_or("");

    let response = unsafe { &*response };

    if let Some(value) = response.values().get(key_str) {
        let c_string = CString::new(value.as_str()).unwrap();
        return c_string.into_raw();  // Now returning a mutable pointer
    }
    ptr::null_mut()
}

/// Frees a C string allocated by `sf_response_get_value`
#[no_mangle]
pub extern "C" fn destr_response_value(value: *mut i8) {
    if value.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(value)); // Reclaim and drop the CString
    }
}


// ####################################################
// #                 RESPONSE KEYS                    #
// ####################################################

/// Retrieves the keys from a parsed response
#[no_mangle]
pub extern "C" fn response_get_keys(response: *mut Response, out_len: *mut usize) -> *mut *const i8 {
    if response.is_null() || out_len.is_null() {
        return ptr::null_mut();
    }

    let response = unsafe { &*response };
    let keys: Vec<CString> = response.values().keys().map(|&key| CString::new(key).unwrap()).collect();
    
    // Convert CString to *const i8 (C-compatible)
    let mut c_keys: Vec<*const i8> = keys.iter().map(|s| s.as_ptr()).collect();
    
    // Store length in out_len
    unsafe { *out_len = c_keys.len(); }

    // Leak memory (C++ must call sf_response_free_keys() later)
    let ptr = c_keys.as_mut_ptr();
    std::mem::forget(keys);
    std::mem::forget(c_keys);
    ptr
}

/// Frees the memory allocated by sf_response_get_keys
#[no_mangle]
pub extern "C" fn destr_response_keys(keys: *mut *const i8, len: usize) {
    if keys.is_null() {
        return;
    }
    unsafe {
        let _ = Vec::from_raw_parts(keys, len, len); // Reclaim ownership, drop it
    }
}


// ###########################################################
// #                 ALL COMMAND WRAPPERS                    #
// ###########################################################

/// Common function to execute a command and return a response
fn execute_command(session: *mut Session, command: Command) -> *mut Response {
    if session.is_null() {
        return ptr::null_mut();
    }

    let session = unsafe { &mut *session };
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");

    match runtime.block_on(session.send_command(command)) {
        Ok(response) => Box::into_raw(Box::new(response)),
        Err(e) => {
            eprintln!("execute_command: Failed to execute command: {:?}", e);
            ptr::null_mut()
        }
    }
}

// Macro to generate command functions
// macro_rules! generate_command_function {
//     ($name:ident, $command:expr) => {
//         #[no_mangle]
//         pub extern "C" fn $name(session: *mut Session) -> *mut Response {
//             execute_command(session, $command)
//         }
//     };
// }

// Generate functions for simple commands (no arguments)
// generate_command_function!(exec_Update, Command::Update);
// generate_command_function!(exec_BuyBeer, Command::BuyBeer);
// generate_command_function!(exec_CancelQuest, Command::CancelQuest);
// generate_command_function!(exec_FinishWork, Command::FinishWork);
// generate_command_function!(exec_CheckArena, Command::CheckArena);
// generate_command_function!(exec_CollectCalendar, Command::CollectCalendar);
// generate_command_function!(exec_ToiletFlush, Command::ToiletFlush);
// generate_command_function!(exec_ToiletOpen, Command::ToiletOpen);
// generate_command_function!(exec_CancelWork, Command::CancelWork);
// generate_command_function!(exec_GuildLoadMushrooms, Command::GuildLoadMushrooms);
// generate_command_function!(exec_GuildJoinAttack, Command::GuildJoinAttack);
// generate_command_function!(exec_GuildJoinDefense, Command::GuildJoinDefense);
// generate_command_function!(exec_GuildRaid, Command::GuildRaid);
// generate_command_function!(exec_GuildPortalBattle, Command::GuildPortalBattle);
// generate_command_function!(exec_GuildGetFightableTargets, Command::GuildGetFightableTargets);
// generate_command_function!(exec_ViewScrapbook, Command::ViewScrapbook);
// generate_command_function!(exec_FightPortal, Command::FightPortal);
// generate_command_function!(exec_SwapManequin, Command::SwapManequin);
// generate_command_function!(exec_IdleSacrifice, Command::IdleSacrifice);
// generate_command_function!(exec_HellevatorEnter, Command::HellevatorEnter);
// generate_command_function!(exec_HellevatorViewGuildRanking, Command::HellevatorViewGuildRanking);
// generate_command_function!(exec_HellevatorRefreshShop, Command::HellevatorRefreshShop);
// generate_command_function!(exec_HellevatorClaimDaily, Command::HellevatorClaimDaily);
// generate_command_function!(exec_HellevatorClaimDailyYesterday, Command::HellevatorClaimDailyYesterday);
// generate_command_function!(exec_HellevatorClaimFinal, Command::HellevatorClaimFinal);
// generate_command_function!(exec_HellevatorPreviewRewards, Command::HellevatorPreviewRewards);
// generate_command_function!(exec_BuyGoldFrame, Command::BuyGoldFrame);

// generate_command_function!(exec_GuildLoadMushrooms, Command::GuildLoadMushrooms);
// generate_command_function!(exec_GuildJoinAttack, Command::GuildJoinAttack);
// generate_command_function!(exec_GuildJoinDefense, Command::GuildJoinDefense);
// generate_command_function!(exec_GuildRaid, Command::GuildRaid);
// generate_command_function!(exec_FightPortal, Command::FightPortal);
// generate_command_function!(exec_GuildPortalBattle, Command::GuildPortalBattle);
// generate_command_function!(exec_GuildGetFightableTargets, Command::GuildGetFightableTargets);
// generate_command_function!(exec_FortressGemStoneSearch, Command::FortressGemStoneSearch);
// generate_command_function!(exec_FortressGemStoneSearchCancel, Command::FortressGemStoneSearchCancel);
// generate_command_function!(exec_FortressUpgradeHallOfKnights, Command::FortressUpgradeHallOfKnights);
// generate_command_function!(exec_IdleSacrifice, Command::IdleSacrifice);
// generate_command_function!(exec_SwapManequin, Command::SwapManequin);
// generate_command_function!(exec_HellevatorEnter, Command::HellevatorEnter);
// generate_command_function!(exec_HellevatorViewGuildRanking, Command::HellevatorViewGuildRanking);
// generate_command_function!(exec_HellevatorRefreshShop, Command::HellevatorRefreshShop);
// generate_command_function!(exec_HellevatorClaimDaily, Command::HellevatorClaimDaily);
// generate_command_function!(exec_HellevatorClaimDailyYesterday, Command::HellevatorClaimDailyYesterday);
// generate_command_function!(exec_HellevatorClaimFinal, Command::HellevatorClaimFinal);
// generate_command_function!(exec_HellevatorPreviewRewards, Command::HellevatorPreviewRewards);
// generate_command_function!(exec_ViewScrapbook, Command::ViewScrapbook);
// generate_command_function!(exec_BuyGoldFrame, Command::BuyGoldFrame);
// generate_command_function!(exec_FortressUpgradeHallOfKnights, Command::FortressUpgradeHallOfKnights);
// generate_command_function!(exec_FortressBuild, Command::FortressBuild);
// generate_command_function!(exec_FortressBuildCancel, Command::FortressBuildCancel);
// generate_command_function!(exec_FortressGather, Command::FortressGather);
// generate_command_function!(exec_FortressBuildFinish, Command::FortressBuildFinish);
// generate_command_function!(exec_FortressBuildUnit, Command::FortressBuildUnit);
// generate_command_function!(exec_FortressNewEnemy, Command::FortressNewEnemy);
// generate_command_function!(exec_FortressSetCAEnemy, Command::FortressSetCAEnemy);
// generate_command_function!(exec_UnderworldCollect, Command::UnderworldCollect);
// generate_command_function!(exec_UnderworldUnitUpgrade, Command::UnderworldUnitUpgrade);
// generate_command_function!(exec_UnderworldUpgradeCancel, Command::UnderworldUpgradeCancel);
// generate_command_function!(exec_UnderworldUpgradeFinish, Command::UnderworldUpgradeFinish);
// generate_command_function!(exec_GuildPetBattle, Command::GuildPetBattle);
// generate_command_function!(exec_SpinWheelOfFortune, Command::SpinWheelOfFortune);
//generate_command_function!(exec_HallOfFameGroupPage, Command::HallOfFameGroupPage);
//generate_command_function!(exec_HallOfFameUnderworldPage, Command::HallOfFameUnderworldPage);
//generate_command_function!(exec_HallOfFamePetsPage, Command::HallOfFamePetsPage);
//generate_command_function!(exec_BlockGuildInvites, Command::BlockGuildInvites);
//generate_command_function!(exec_ShowTips, Command::ShowTips);


#[no_mangle]
pub extern "C" fn exec_Update(session: *mut Session) -> *mut Response {
    execute_command(session, Command::Update)
}

#[no_mangle]
pub extern "C" fn exec_BuyBeer(session: *mut Session) -> *mut Response {
    execute_command(session, Command::BuyBeer)
}

#[no_mangle]
pub extern "C" fn exec_CancelQuest(session: *mut Session) -> *mut Response {
    execute_command(session, Command::CancelQuest)
}

#[no_mangle]
pub extern "C" fn exec_FinishWork(session: *mut Session) -> *mut Response {
    execute_command(session, Command::FinishWork)
}

#[no_mangle]
pub extern "C" fn exec_CheckArena(session: *mut Session) -> *mut Response {
    execute_command(session, Command::CheckArena)
}

#[no_mangle]
pub extern "C" fn exec_CollectCalendar(session: *mut Session) -> *mut Response {
    execute_command(session, Command::CollectCalendar)
}

#[no_mangle]
pub extern "C" fn exec_ToiletFlush(session: *mut Session) -> *mut Response {
    execute_command(session, Command::ToiletFlush)
}

#[no_mangle]
pub extern "C" fn exec_ToiletOpen(session: *mut Session) -> *mut Response {
    execute_command(session, Command::ToiletOpen)
}

#[no_mangle]
pub extern "C" fn exec_CancelWork(session: *mut Session) -> *mut Response {
    execute_command(session, Command::CancelWork)
}

#[no_mangle]
pub extern "C" fn exec_GuildLoadMushrooms(session: *mut Session) -> *mut Response {
    execute_command(session, Command::GuildLoadMushrooms)
}

#[no_mangle]
pub extern "C" fn exec_GuildJoinAttack(session: *mut Session) -> *mut Response {
    execute_command(session, Command::GuildJoinAttack)
}

#[no_mangle]
pub extern "C" fn exec_GuildJoinDefense(session: *mut Session) -> *mut Response {
    execute_command(session, Command::GuildJoinDefense)
}

#[no_mangle]
pub extern "C" fn exec_GuildRaid(session: *mut Session) -> *mut Response {
    execute_command(session, Command::GuildRaid)
}

#[no_mangle]
pub extern "C" fn exec_GuildPortalBattle(session: *mut Session) -> *mut Response {
    execute_command(session, Command::GuildPortalBattle)
}

#[no_mangle]
pub extern "C" fn exec_GuildGetFightableTargets(session: *mut Session) -> *mut Response {
    execute_command(session, Command::GuildGetFightableTargets)
}

#[no_mangle]
pub extern "C" fn exec_ViewScrapbook(session: *mut Session) -> *mut Response {
    execute_command(session, Command::ViewScrapbook)
}

#[no_mangle]
pub extern "C" fn exec_FightPortal(session: *mut Session) -> *mut Response {
    execute_command(session, Command::FightPortal)
}

#[no_mangle]
pub extern "C" fn exec_SwapManequin(session: *mut Session) -> *mut Response {
    execute_command(session, Command::SwapManequin)
}

#[no_mangle]
pub extern "C" fn exec_IdleSacrifice(session: *mut Session) -> *mut Response {
    execute_command(session, Command::IdleSacrifice)
}

#[no_mangle]
pub extern "C" fn exec_HellevatorEnter(session: *mut Session) -> *mut Response {
    execute_command(session, Command::HellevatorEnter)
}

#[no_mangle]
pub extern "C" fn exec_HellevatorViewGuildRanking(session: *mut Session) -> *mut Response {
    execute_command(session, Command::HellevatorViewGuildRanking)
}

#[no_mangle]
pub extern "C" fn exec_HellevatorRefreshShop(session: *mut Session) -> *mut Response {
    execute_command(session, Command::HellevatorRefreshShop)
}

#[no_mangle]
pub extern "C" fn exec_HellevatorClaimDaily(session: *mut Session) -> *mut Response {
    execute_command(session, Command::HellevatorClaimDaily)
}

#[no_mangle]
pub extern "C" fn exec_HellevatorClaimDailyYesterday(session: *mut Session) -> *mut Response {
    execute_command(session, Command::HellevatorClaimDailyYesterday)
}

#[no_mangle]
pub extern "C" fn exec_HellevatorClaimFinal(session: *mut Session) -> *mut Response {
    execute_command(session, Command::HellevatorClaimFinal)
}

#[no_mangle]
pub extern "C" fn exec_HellevatorPreviewRewards(session: *mut Session) -> *mut Response {
    execute_command(session, Command::HellevatorPreviewRewards)
}

#[no_mangle]
pub extern "C" fn exec_BuyGoldFrame(session: *mut Session) -> *mut Response {
    execute_command(session, Command::BuyGoldFrame)
}


// Generate functions for commands with arguments
#[no_mangle]
pub extern "C" fn exec_HallOfFamePage(session: *mut Session, page: usize) -> *mut Response {
    execute_command(session, Command::HallOfFamePage { page })
}

#[no_mangle]
pub extern "C" fn exec_HallOfFameFortressPage(session: *mut Session, page: usize) -> *mut Response {
    execute_command(session, Command::HallOfFameFortressPage { page })
}

#[no_mangle]
pub extern "C" fn exec_ViewPlayer(session: *mut Session, ident: *const i8) -> *mut Response {
    if ident.is_null() {
        return ptr::null_mut();
    }
    let ident_cstr = unsafe { CStr::from_ptr(ident) };
    let ident_str = ident_cstr.to_str().unwrap_or("").to_string();
    execute_command(session, Command::ViewPlayer { ident: ident_str })
}

#[no_mangle]
pub extern "C" fn exec_StartQuest(session: *mut Session, quest_pos: usize, overwrite_inv: bool) -> *mut Response {
    execute_command(session, Command::StartQuest { quest_pos, overwrite_inv })
}

#[no_mangle]
pub extern "C" fn exec_StartWork(session: *mut Session, hours: u8) -> *mut Response {
    execute_command(session, Command::StartWork { hours })
}

// #[no_mangle]
// pub extern "C" fn exec_BuyMount(session: *mut Session, mount_type: u8) -> *mut Response {
//     let mount = match mount_type {
//         1 => crate::command::Mount::Horse,
//         2 => crate::command::Mount::Tiger,
//         3 => crate::command::Mount::Griffon,
//         _ => return ptr::null_mut(), // Invalid mount type
//     };
//     execute_command(session, Command::BuyMount { mount })
// }

// #[no_mangle]
// pub extern "C" fn exec_IncreaseAttribute(session: *mut Session, attribute: u32, increase_to: u32) -> *mut Response {
//     let attr = match attribute {
//         1 => crate::command::AttributeType::Strength,
//         2 => crate::command::AttributeType::Dexterity,
//         3 => crate::command::AttributeType::Intelligence,
//         4 => crate::command::AttributeType::Constitution,
//         5 => crate::command::AttributeType::Luck,
//         _ => return ptr::null_mut(), // Invalid attribute
//     };
//     execute_command(session, Command::IncreaseAttribute { attribute: attr, increase_to })
// }

#[no_mangle]
pub extern "C" fn exec_RemovePotion(session: *mut Session, pos: usize) -> *mut Response {
    execute_command(session, Command::RemovePotion { pos })
}

#[no_mangle]
pub extern "C" fn exec_Fight(session: *mut Session, name: *const i8, use_mushroom: bool) -> *mut Response {
    if name.is_null() {
        return ptr::null_mut();
    }
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_str = name_cstr.to_str().unwrap_or("").to_string();
    execute_command(session, Command::Fight { name: name_str, use_mushroom })
}

#[no_mangle]
pub extern "C" fn exec_GuildAttack(session: *mut Session, guild: *const i8) -> *mut Response {
    if guild.is_null() {
        return ptr::null_mut();
    }
    let guild_cstr = unsafe { CStr::from_ptr(guild) };
    let guild_str = guild_cstr.to_str().unwrap_or("").to_string();
    execute_command(session, Command::GuildAttack { guild: guild_str })
}

// #[no_mangle]
// pub extern "C" fn exec_GuildIncreaseSkill(session: *mut Session, skill: u16, current: u16) -> *mut Response {
//     let skill_enum = match skill {
//         0 => crate::command::GuildSkill::Instructor,
//         1 => crate::command::GuildSkill::Treasure,
//         _ => return ptr::null_mut(),
//     };
//     execute_command(session, Command::GuildIncreaseSkill { skill: skill_enum, current })
// }

// Functions for commands requiring arguments

#[no_mangle]
pub extern "C" fn exec_FinishQuest(session: *mut Session, skip: bool) -> *mut Response {
    let time_skip = if skip { Some(crate::command::TimeSkip::Mushroom) } else { None };
    execute_command(session, Command::FinishQuest { skip: time_skip })
}

#[no_mangle]
pub extern "C" fn exec_CheckNameAvailable(session: *mut Session, name: *const i8) -> *mut Response {
    if name.is_null() {
        return ptr::null_mut();
    }
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_str = name_cstr.to_str().unwrap_or("").to_string();
    execute_command(session, Command::CheckNameAvailable { name: name_str })
}

#[no_mangle]
pub extern "C" fn exec_ViewGuild(session: *mut Session, guild_ident: *const i8) -> *mut Response {
    if guild_ident.is_null() {
        return ptr::null_mut();
    }
    let guild_cstr = unsafe { CStr::from_ptr(guild_ident) };
    let guild_str = guild_cstr.to_str().unwrap_or("").to_string();
    execute_command(session, Command::ViewGuild { guild_ident: guild_str })
}

#[no_mangle]
pub extern "C" fn exec_GuildFound(session: *mut Session, name: *const i8) -> *mut Response {
    if name.is_null() {
        return ptr::null_mut();
    }
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_str = name_cstr.to_str().unwrap_or("").to_string();
    execute_command(session, Command::GuildFound { name: name_str })
}

#[no_mangle]
pub extern "C" fn exec_GuildInvitePlayer(session: *mut Session, name: *const i8) -> *mut Response {
    if name.is_null() {
        return ptr::null_mut();
    }
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_str = name_cstr.to_str().unwrap_or("").to_string();
    execute_command(session, Command::GuildInvitePlayer { name: name_str })
}

#[no_mangle]
pub extern "C" fn exec_GuildKickPlayer(session: *mut Session, name: *const i8) -> *mut Response {
    if name.is_null() {
        return ptr::null_mut();
    }
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_str = name_cstr.to_str().unwrap_or("").to_string();
    execute_command(session, Command::GuildKickPlayer { name: name_str })
}

#[no_mangle]
pub extern "C" fn exec_GuildSetLeader(session: *mut Session, name: *const i8) -> *mut Response {
    if name.is_null() {
        return ptr::null_mut();
    }
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_str = name_cstr.to_str().unwrap_or("").to_string();
    execute_command(session, Command::GuildSetLeader { name: name_str })
}

#[no_mangle]
pub extern "C" fn exec_GuildToggleOfficer(session: *mut Session, name: *const i8) -> *mut Response {
    if name.is_null() {
        return ptr::null_mut();
    }
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_str = name_cstr.to_str().unwrap_or("").to_string();
    execute_command(session, Command::GuildToggleOfficer { name: name_str })
}

// #[no_mangle]
// pub extern "C" fn exec_ToiletDrop(session: *mut Session, inventory: u8, pos: usize) -> *mut Response {
//     let inventory_enum = match inventory {
//         0 => crate::command::PlayerItemPlace::Inventory,
//         1 => crate::command::PlayerItemPlace::Shop,
//         _ => return ptr::null_mut(),
//     };
//     execute_command(session, Command::ToiletDrop { inventory: inventory_enum, pos })
// }

#[no_mangle]
pub extern "C" fn exec_MessageOpen(session: *mut Session, pos: i32) -> *mut Response {
    execute_command(session, Command::MessageOpen { pos })
}

#[no_mangle]
pub extern "C" fn exec_MessageDelete(session: *mut Session, pos: i32) -> *mut Response {
    execute_command(session, Command::MessageDelete { pos })
}

#[no_mangle]
pub extern "C" fn exec_ViewPet(session: *mut Session, pet_id: u16) -> *mut Response {
    execute_command(session, Command::ViewPet { pet_id })
}

// #[no_mangle]
// pub extern "C" fn exec_UnlockFeature(session: *mut Session, unlockable: u8) -> *mut Response {
//     let unlockable_enum = match unlockable {
//         0 => crate::command::Unlockable::Pets,
//         1 => crate::command::Unlockable::Fortress,
//         _ => return ptr::null_mut(),
//     };
//     execute_command(session, Command::UnlockFeature { unlockable: unlockable_enum })
// }

#[no_mangle]
pub extern "C" fn exec_GambleSilver(session: *mut Session, amount: u64) -> *mut Response {
    execute_command(session, Command::GambleSilver { amount })
}

#[no_mangle]
pub extern "C" fn exec_GambleMushrooms(session: *mut Session, amount: u64) -> *mut Response {
    execute_command(session, Command::GambleMushrooms { amount })
}

#[no_mangle]
pub extern "C" fn exec_SendMessage(session: *mut Session, to: *const i8, msg: *const i8) -> *mut Response {
    if to.is_null() || msg.is_null() {
        return ptr::null_mut();
    }
    let to_cstr = unsafe { CStr::from_ptr(to) };
    let to_str = to_cstr.to_str().unwrap_or("").to_string();
    
    let msg_cstr = unsafe { CStr::from_ptr(msg) };
    let msg_str = msg_cstr.to_str().unwrap_or("").to_string();

    execute_command(session, Command::SendMessage { to: to_str, msg: msg_str })
}

#[no_mangle]
pub extern "C" fn exec_Whisper(session: *mut Session, player_name: *const i8, message: *const i8) -> *mut Response {
    if player_name.is_null() || message.is_null() {
        return ptr::null_mut();
    }
    let player_name_cstr = unsafe { CStr::from_ptr(player_name) };
    let player_name_str = player_name_cstr.to_str().unwrap_or("").to_string();
    
    let message_cstr = unsafe { CStr::from_ptr(message) };
    let message_str = message_cstr.to_str().unwrap_or("").to_string();

    execute_command(session, Command::Whisper { player_name: player_name_str, message: message_str })
}

#[no_mangle]
pub extern "C" fn exec_SetLanguage(session: *mut Session, language: *const i8) -> *mut Response {
    if language.is_null() {
        return ptr::null_mut();
    }
    let language_cstr = unsafe { CStr::from_ptr(language) };
    let language_str = language_cstr.to_str().unwrap_or("").to_string();
    execute_command(session, Command::SetLanguage { language: language_str })
}




// #[no_mangle]
// pub extern "C" fn exec_HallOfFamePage(session: *mut Session, page: usize) -> *mut Response {
//     if session.is_null() {
//         return ptr::null_mut();
//     }

//     let command = Command::HallOfFamePage{ page };

//     let session = unsafe { &mut *session };
//     let runtime = Runtime::new().expect("Failed to create Tokio runtime");

//     match runtime.block_on(session.send_command(command)) {
//         Ok(response) => Box::into_raw(Box::new(response)), // Return Response pointer
//         Err(e) => {
//             eprintln!("sf_command_execute: Failed to execute command: {:?}", e);
//             ptr::null_mut()
//         }
//     }
// }