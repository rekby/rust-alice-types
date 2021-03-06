// https://yandex.ru/dev/dialogs/alice/doc/protocol.html

use chrono::prelude as ch;
use serde::{Deserialize,Serialize};
use std::{
    cell::Cell,
    collections::HashMap
};

use crate::{
    errors::{Error, Result},
    yandex_types,
};
use crate::yandex_types::YandexDateTime;

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct IncomingMessage<SessionState=serde_json::Value, UserState=serde_json::Value> {
    pub meta: Meta,
    pub request: Request,
    pub session: Session,
    pub version: String,

    pub state: State<SessionState, UserState>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State<SessionState, UserState> {
    // https://yandex.ru/dev/dialogs/alice/doc/session-persistence.html#store-session
    pub session: Option<SessionState>,

    // https://yandex.ru/dev/dialogs/alice/doc/session-persistence.html#store-between-sessions
    pub user: Option<UserState>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Meta {
    pub locale: String,
    pub timezone: String,
    pub client_id: String,
    pub interfaces: Interfaces,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Interfaces {
    pub screen: Option<InterfaceScreen>,
    pub account_linking: Option<InterfaceAccountLinking>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceScreen {}

#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceAccountLinking{}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Request {
    pub command: String,
    pub original_utterance: String,

    #[serde(rename="type")]
    pub request_type: Option<RequestInnerType>,
    pub markup: Markup,
    pub payload: serde_json::Value,
    pub nlu: Nlu,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged, from="String")]
pub enum RequestInnerType {
    SimpleUtterance,
    ButtonPressed,
    Unknown(String),
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

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Markup{
    pub dangerous_context: bool,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Nlu {
    pub tokens: Vec<String>,
    pub entities: Vec<NluEntity>,
    pub intents: HashMap<String,Intent>
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct NluEntity{
    pub tokens: NluEntityTokens,

    #[serde(rename="type")]
    pub entity_type: String,
    pub value: serde_json::Value,

    ydt: Cell<Option<YandexDateTime>>,
}

impl NluEntity {
    // https://yandex.ru/dev/dialogs/alice/doc/naming-entities.html#naming-entities__datetime
    pub fn get_date_time<Tz: ch::TimeZone>(&self, now: ch::DateTime<Tz>)->Result<ch::DateTime<Tz>> {
        if !self.is_date_time() {
            return Err(Error::NoDateTime(self.entity_type.clone()))
        }
        return self.get_ydt()?.date_time(now);
    }

    pub fn is_date_time(&self)-> bool {
        return self.entity_type.as_str() == "YANDEX.DATETIME"
    }

    pub fn has_date(&self)->bool {
        if !self.is_date_time(){
            return false;
        }
        if let Ok(ydt) = self.get_ydt() {
            return ydt.has_date();
        }
        return false;
    }

    pub fn has_time(&self)->bool {
        if !self.is_date_time(){
            return false;
        }
        if let Ok(ydt) = self.get_ydt() {
            return ydt.has_time();
        }
        return false;
    }

    fn get_ydt(&self)->Result<YandexDateTime>{
        let res = match self.ydt.get() {
            Some(ydt)=>ydt,
            None=>{
                let ydt: yandex_types::YandexDateTime = serde_json::from_value(self.value.clone())?;
                self.ydt.replace(Some(ydt));
                ydt
            }
        };
        return Ok(res);
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct NluEntityTokens {
    pub start: u16,
    pub end: u16,
}

#[derive(Clone,Default,Debug,Deserialize,Serialize)]
#[serde(default)]
pub struct Session{
    pub session_id: String,
    pub skill_id: String,
    pub user_id: String,
    pub message_id: i64,
    pub new: bool,
}

#[derive(Default,Debug,Deserialize, Serialize)]
pub struct Intent {
    pub slots: HashMap<String,NluEntity>
}
