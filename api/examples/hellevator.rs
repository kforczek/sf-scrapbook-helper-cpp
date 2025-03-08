#![allow(unused)]
use std::{borrow::Borrow, time::Duration};

use chrono::{DateTime, Local};
use sf_api::{
    command::{Command, ExpeditionSetting, TimeSkip},
    gamestate::{
        items::{Enchantment, EquipmentSlot},
        tavern::{AvailableTasks, CurrentAction},
        unlockables::HellevatorStatus,
    },
    misc::EnumMapGet,
    session::SimpleSession,
};
use sha1::digest::Update;
use tokio::time::sleep;

#[tokio::main]
pub async fn main() {
    let mut session = login_with_env().await;

    loop {
        sleep(Duration::from_secs(2)).await;
        let gs = session.game_state().unwrap();

        if gs.character.level < 10 {
            println!("Character level too low");
            break;
        }

        let hellevator = match gs.hellevator.status() {
            HellevatorStatus::RewardClaimable => {
                session
                    .send_command(Command::HellevatorClaimFinal)
                    .await
                    .unwrap();
                continue;
            }
            HellevatorStatus::NotEntered => {
                session
                    .send_command(Command::HellevatorEnter)
                    .await
                    .unwrap();
                continue;
            }
            HellevatorStatus::NotAvailable => {
                println!("Hellevator is not available currently");
                break;
            }
            HellevatorStatus::Active(h) => h,
        };
    }
}

pub fn time_remaining<T: Borrow<DateTime<Local>>>(time: T) -> Duration {
    (*time.borrow() - Local::now()).to_std().unwrap_or_default()
}

pub async fn login_with_env() -> SimpleSession {
    let username = std::env::var("USERNAME").unwrap();
    let password = std::env::var("PASSWORD").unwrap();
    let server = std::env::var("SERVER").unwrap();
    SimpleSession::login(&username, &password, &server)
        .await
        .unwrap()
}
