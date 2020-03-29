use serde::{Deserialize, Serialize};
mod messages;

use messages::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct WebHookMsg {
    #[serde(default)]
    pub message: Message,
    update_id: i32,
}
