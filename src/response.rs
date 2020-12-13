// https://yandex.ru/dev/dialogs/alice/doc/protocol.html

use serde::Serialize;

use crate::request;

#[derive(Serialize)]
pub struct Message<SessionState = serde_json::Value> {
    pub response: Response<SessionState>,

    version: &'static str,
}

impl<SessionState> Message<SessionState>{
    pub fn new()->Self{
        return Self::with_response(Response::<SessionState>::default())
    }

    pub fn with_response(resp: Response<SessionState>)->Self
    {
        return Message{
            response: resp,
            version: "1.0",
        }
    }
}

#[derive(Serialize)]
pub struct Response<SessionState=serde_json::Value> {
    pub text: String,
    pub end_session: bool,

    // https://yandex.ru/dev/dialogs/alice/doc/session-persistence.html/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_state:Option<SessionState>,
}

impl<SessionState> Default for Response<SessionState>{
    fn default()->Self{
        return Response::<SessionState>{
            text: String::default(),
            end_session: bool::default(),
            session_state: None,
        }
    }
}

pub fn new_response()->Response<serde_json::Value>{
    return Response::default()
}

impl<SessionState> Response<SessionState>{
    pub fn from_request(req: request::Request<SessionState>)->Self{
        let mut resp = Self::default();
        resp.session_state = req.session_state;
        return resp;
    }
}
