use serde::{Deserialize, Serialize};
use messages::Message;
use std::slice::Iter;

mod messages;
mod commands;

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct WebHookMsg {
    #[serde(default)]
    pub(super) message: Message,
    update_id: i32,
}

pub(super) fn interpret(raw_msg: Vec<u8>) -> String {

    match serde_json::from_slice::<WebHookMsg>(&raw_msg) {
        Ok(update) => {
            let mut ctr = 0;
            for char in update.message.raw_text() {
                if char == 'a' || char == 'A' {
                    ctr += 1;
                }
            }
            commands::BotCommands::from_msg(&update.message);
            // println!("{:#?}", update);
            let mut x = format!("počet áček je: {}", ctr);

            let c = String::from_utf8(vec![240, 159, 141, 135]).unwrap_or_default();
            let resp = update.message.reply(x);
            serde_json::to_string(&resp).unwrap_or_default()
        }
        Err(e) => {
            dbg!(e);
            String::new()
        }
    }
}

