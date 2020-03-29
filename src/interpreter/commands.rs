use super::Message;
use crate::interpreter::messages::EntityType;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BotCommands {
    Start(String),
    Join(String),
    Leave(String),
    Flash(String),
    Complete(String),
    Abandon(String),
    Results(String),
    Terminate(String),
}

impl BotCommands {
    const JOIN: &'static str = "/join";
    const LEAVE: &'static str = "/leave";
    const FLASH: &'static str = "/flash";
    const COMPLETE: &'static str = "/complete";
    const ABANDON: &'static str = "/abandon";
    const RESULTS: &'static str = "/results";
    const START: &'static str = "/start";
    const TERMINATE: &'static str = "/terminate";

    pub(super) fn from_msg(msg: &Message) {
        let mut cmd_set = HashSet::<BotCommands>::new();
        for ent in msg.entities.iter() {
            if let Some(ref ent_type) = ent.entity_type {
                match ent_type {
                    EntityType::Mention => {
                        dbg!(&msg.text);
                    }
                    EntityType::Hashtag => unimplemented!(),
                    EntityType::Cashtag => unimplemented!(),
                    EntityType::BotCmd => {
                        let utf16 = msg
                            .text
                            .encode_utf16()
                            .skip(ent.offset)
                            .collect::<Vec<u16>>();
                        let cmd = String::from_utf16(&utf16[..ent.length]).unwrap_or_default();
                        let args = String::from_utf16(&utf16[ent.length..])
                            .unwrap_or_default()
                            .trim()
                            .to_owned();
                        match cmd.as_str() {
                            Self::JOIN => {
                                cmd_set.insert(Self::Join(args));
                            }
                            Self::LEAVE => {
                                cmd_set.insert(Self::Leave(args));
                            }
                            Self::START => {
                                cmd_set.insert(Self::Start(args));
                            }
                            Self::TERMINATE => {
                                cmd_set.insert(Self::Terminate(args));
                            }
                            Self::FLASH => {
                                cmd_set.insert(Self::Flash(args));
                            }
                            Self::COMPLETE => {
                                cmd_set.insert(Self::Complete(args));
                            }
                            Self::ABANDON => {
                                cmd_set.insert(Self::Abandon(args));
                            }
                            Self::RESULTS => {
                                cmd_set.insert(Self::Results(args));
                            }
                            _ => {}
                        }
                    }
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
        println!("{:?}", cmd_set);
    }
}
