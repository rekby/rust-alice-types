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
