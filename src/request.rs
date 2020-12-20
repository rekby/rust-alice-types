// https://yandex.ru/dev/dialogs/alice/doc/protocol.html

use serde::{Deserialize,Serialize};
use crate::request::RequestInnerType::Unknown;
use std::collections::HashMap;

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct Request<SessionState=serde_json::Value> {
    pub meta: Meta,
    pub request: RequestInner,
    pub session: Session,
    pub version: String,

    // https://yandex.ru/dev/dialogs/alice/doc/session-persistence.html/
    #[serde(rename="state")]
    pub session_state: Option<SessionState>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct Meta {
    pub locale: String,
    pub timezone: String,
    pub client_id: String,
    pub interfaces: Interfaces,
}

#[derive(Default, Debug, Deserialize)]
pub struct Interfaces {
    pub screen: Option<InterfaceScreen>,
    pub account_linking: Option<InterfaceAccountLinking>
}

#[derive(Debug, Deserialize)]
pub struct InterfaceScreen {}

#[derive(Debug, Deserialize)]
pub struct InterfaceAccountLinking{}

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct RequestInner {
    pub command: String,
    pub original_utterance: String,
    pub request_type: RequestInnerType,
    pub markup: Markup,
    pub payload: serde_json::Value,
    pub nlu: Nlu,
}

#[serde(untagged, from="String")]
#[derive(PartialEq, Debug, Deserialize)]
pub enum RequestInnerType {
    SimpleUtterance,
    ButtonPressed,
    Unknown(String),
}

impl Default for RequestInnerType{
    fn default() ->Self{return Unknown("default".to_string())}
}

impl From<String> for RequestInnerType{
    fn from(s: String)->Self{
        use RequestInnerType::*;

        return match s.as_str() {
            "SimpleUtterance" => SimpleUtterance,
            "ButtonPressed" => ButtonPressed,
            _ => Unknown(s),
        }
    }
}

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct Markup{
    pub dangerous_context: bool,
}

#[derive(Default, Debug, Deserialize)]
pub struct Nlu {
    #[serde(default)]
    pub tokens: Vec<String>,
    pub entities: Vec<NluEntity>,
    pub intents: HashMap<String,Intent>
}

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct NluEntity{
    pub tokens: NluEntityTokens,

    #[serde(rename="type")]
    pub entity_type: String,
    pub value: serde_json::Value,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct NluEntityTokens {
    pub start: u16,
    pub end: u16,
}

#[derive(Default,Debug,Deserialize,Serialize)]
pub struct Session{
    session_id: String,
    user_id: String,
    message_id: i64,
}

#[derive(Default,Debug,Deserialize)]
pub struct Intent {
    pub slots: HashMap<String,NluEntity>
}
