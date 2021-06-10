use serde_json;
use crate::request::*;

#[test]
fn deserialize_interfaces(){
    let v: Interfaces = serde_json::from_str("{}").expect("invalid json");
    assert!(v.account_linking.is_none());
    assert!(v.screen.is_none());

    let v: Interfaces = serde_json::from_str(r#"{"screen":{}}"#).expect("invalid json");
    assert!(v.account_linking.is_none());
    assert!(v.screen.is_some());

    let v: Interfaces = serde_json::from_str(r#"{"account_linking":{}}"#).expect("invalid json");
    assert!(v.account_linking.is_some());
    assert!(v.screen.is_none());

    let v: Interfaces = serde_json::from_str(r#"{"account_linking":{}, "screen":{"a": true}}"#).expect("invalid json");
    assert!(v.account_linking.is_some());
    assert!(v.screen.is_some());
}

#[test]
fn deserialize_markup(){
    let v: Markup = serde_json::from_str("{}").expect("invalid json in test");
    assert!(!v.dangerous_context, "{:?}", v);

    let v: Markup = serde_json::from_str(r#"{"dangerous_context": true}"#).expect("invalid json in test");
    assert!(v.dangerous_context, "{:?}", v);
}

#[test]
fn deserialize_request_inner_type(){
    let a: RequestInnerType = serde_json::from_str("\"SimpleUtterance\"").expect("invalid json in test");
    assert!(a == RequestInnerType::SimpleUtterance, "{:?}", a);

    let a: RequestInnerType = serde_json::from_str("\"ButtonPressed\"").expect("invalid json in test");
    assert!(a == RequestInnerType::ButtonPressed, "{:?}", a);

    let a: RequestInnerType = serde_json::from_str("\"bad-value\"").expect("invalid json in test");

    assert!(match a {
        RequestInnerType::Unknown(_)=>true,
        _ => false,
    }, "{:?}", a);
}

#[test]
fn deserialize_full_request() {
    // test deserialize full json.
    // request DOES NOT valid logical alice request
    let req_s = r#"{
  "meta": {
    "locale": "ru-RU",
    "timezone": "Europe/Moscow",
    "client_id": "ru.yandex.searchplugin/5.80 (Samsung Galaxy; Android 4.4)",
    "interfaces": {
      "screen": { },
      "account_linking": { }
    }
  },
  "request": {
    "command": "тестовая команда 1",
    "original_utterance": "тестовая команда 2",
    "type": "SimpleUtterance",
    "markup": {
      "dangerous_context": true
    },
    "payload": {},
    "nlu": {
      "tokens": [
        "закажи",
        "пиццу"
      ],
      "entities": [
        {
          "tokens": {
            "start": 2,
            "end": 6
          },
          "type": "YANDEX.GEO",
          "value": {
            "house_number": "16",
            "street": "льва толстого"
          }
        }
      ]
    }
  },
  "session": {
    "message_id": 2,
    "session_id": "sess-id",
    "skill_id": "skill-id",
    "user_id": "user-id",
    "new": true
  },
  "version": "1.0"
}"#;

    let req: Request = serde_json::from_str(req_s).expect("deserialize request correct");
    assert_eq!(req.meta.locale, "ru-RU");
    assert_eq!(req.meta.timezone, "Europe/Moscow");
    assert_eq!(req.meta.client_id, "ru.yandex.searchplugin/5.80 (Samsung Galaxy; Android 4.4)");
    assert!(req.meta.interfaces.account_linking.is_some());
    assert!(req.meta.interfaces.screen.is_some());
    assert_eq!(req.request.command, "тестовая команда 1");
    assert_eq!(req.request.original_utterance, "тестовая команда 2");
    assert_eq!(req.request.request_type.unwrap(), RequestInnerType::SimpleUtterance);
    assert!(req.request.markup.dangerous_context);
    assert!(req.request.payload.is_object());
    assert_eq!(req.request.nlu.tokens, vec!["закажи", "пиццу"]);
    assert_eq!(req.request.nlu.entities[0].tokens.start, 2);
    assert_eq!(req.request.nlu.entities[0].tokens.end, 6);
    assert_eq!(req.request.nlu.entities[0].entity_type, "YANDEX.GEO");
    assert_eq!(req.request.nlu.entities[0].value.as_object().unwrap()["house_number"].as_str().unwrap(), "16");
    assert_eq!(req.request.nlu.entities[0].value.as_object().unwrap()["street"].as_str().unwrap(), "льва толстого");
    assert_eq!(req.session.message_id, 2);
    assert_eq!(req.session.session_id, "sess-id");
    assert_eq!(req.session.skill_id, "skill-id");
    assert_eq!(req.session.user_id, "user-id");
    assert_eq!(req.session.new, true);
    assert_eq!(req.version, "1.0");
}