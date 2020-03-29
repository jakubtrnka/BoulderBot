use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Serialize, Deserialize, Debug)]
pub(super) enum ChatType {
    #[serde(rename = "private")]
    Private,
}

impl Default for ChatType {
    fn default() -> Self {
        ChatType::Private
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub(super) struct Chat {
    #[serde(default)]
    pub(super) first_name: String,
    pub(super) id: i32,
    #[serde(default)]
    pub(super) last_name: String,
    #[serde(rename = "type")]
    pub(super) chat_type: ChatType,
    #[serde(default)]
    pub(super) username: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub(super) struct MessageOriginator {
    pub(super) first_name: String,
    id: i32,
    pub(super) is_bot: bool,
    pub(super) language_code: String,
    pub(super) last_name: String,
    pub(super) username: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Message {
    pub(super) chat: Chat,
    pub(super) date: u64,
    #[serde(default)]
    pub(super) from: MessageOriginator,
    #[serde(default)]
    pub(super) entities: Vec<MessageEntity>,
    pub(super) message_id: i32,
    #[serde(default)]
    pub(super) text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) enum EntityType {
    #[serde(rename = "mention")]
    Mention,
    #[serde(rename = "hashtag")]
    Hashtag,
    #[serde(rename = "cashtag")]
    Cashtag,
    #[serde(rename = "bot_command")]
    BotCmd,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone_number")]
    PhoneNumber,
    #[serde(rename = "bold")]
    Bold,
    #[serde(rename = "italic")]
    Italic,
    #[serde(rename = "underline")]
    Underline,
    #[serde(rename = "strikethrough")]
    Strikethrough,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "pre")]
    Pre,
    #[serde(rename = "text_link")]
    TextLink,
    #[serde(rename = "text_mention")]
    TextMention,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub(super) struct MessageEntity {
    #[serde(rename = "type")]
    pub(super) entity_type: Option<EntityType>,
    pub(super) offset: usize,
    pub(super) length: usize,
    #[serde(default)]
    pub(super) url: String,
    //pub(super) user: User,
    #[serde(default)]
    pub(super) language: String,
}

impl Message {
    pub fn reply(&self, text: String) -> SendMessage {
        SendMessage::new(self.chat.id, text)
    }
    pub fn raw_text(&self) -> std::str::Chars {
        self.text.chars()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessage {
    pub(super) method: String,
    pub(super) chat_id: i32,
    pub(super) text: String,
}

impl SendMessage {
    pub(super) fn new(chat_id: i32, text: String) -> Self {
        Self {
            method: "sendMessage".to_owned(),
            chat_id,
            text,
        }
    }
}
