use super::Message;
use crate::interpreter::messages::EntityType;
use crate::interpreter::commands::BotCommands::JoinChallenge;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BotCommands {
    JoinChallenge(String),
    LeaveChallege(String),
    Flash(String),
    Complete(String),
    TerminateChallenge(String),
}

impl BotCommands {
    const JOIN: &'static str = "/join";
    const LEAVE: &'static str = "/leave";
    const FLASH: &'static str = "/flash";
    const COMPLETE: &'static str = "/complete";
    const TERMINATE: &'static str = "/terminate";

    pub(super) fn from_msg(msg: &Message) {
        let mut cmd_set = HashSet::<BotCommands>::new();
        for ent in msg.entities.iter() {
            if let Some(ref ent_type) = ent.entity_type {
                match ent_type {
                    EntityType::Mention => unimplemented!(),
                    EntityType::Hashtag => unimplemented!(),
                    EntityType::Cashtag => unimplemented!(),
                    EntityType::BotCmd => {
                        let utf16 = msg.text.encode_utf16()
                            .skip(ent.offset)
                            .collect::<Vec<u16>>();
                        let cmd = String::from_utf16(&utf16[..ent.length])
                            .unwrap_or_default();
                        let args = String::from_utf16(&utf16[ent.length..])
                            .unwrap_or_default()
                            .trim().to_owned();
                            match cmd.as_str() {
                            Self::JOIN => {
                                dbg!(args);
                            }
                            Self::LEAVE => {dbg!(utf16);}
                            Self::JOIN => {dbg!(utf16);}
                            Self::JOIN => {dbg!(utf16);}
                            _ => {dbg!("none");}
                        }
                    },
                    EntityType::Url => unimplemented!(),
                    EntityType::Email => unimplemented!(),
                    EntityType::PhoneNumber => unimplemented!(),
                    EntityType::Bold => unimplemented!(),
                    EntityType::Italic => unimplemented!(),
                    EntityType::Underline => unimplemented!(),
                    EntityType::Strikethrough => unimplemented!(),
                    EntityType::Code => unimplemented!(),
                    EntityType::Pre => unimplemented!(),
                    EntityType::TextLink => unimplemented!(),
                    EntityType::TextMention => unimplemented!(),

                }
            }
        }
    }
}