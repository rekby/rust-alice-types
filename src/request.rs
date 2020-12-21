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
    pub request_type: Option<RequestInnerType>,
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

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct NluEntityTokens {
    pub start: u16,
    pub end: u16,
}

#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct Session{
    session_id: String,
    user_id: String,
    message_id: i64,
}

#[derive(Default,Debug,Deserialize)]
pub struct Intent {
    pub slots: HashMap<String,NluEntity>
}
