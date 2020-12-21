// https://yandex.ru/dev/dialogs/alice/doc/protocol.html

use serde::Serialize;

use crate::request;

#[derive(Serialize)]
pub struct Message<SessionState = serde_json::Value> {
    pub response: Response<SessionState>,

    // for marusya https://vk.com/dev/marusia_skill_docs
    pub session: Option<request::Session>,

    version: &'static str,
}

impl<SessionState> Message<SessionState>{
    pub fn new()->Self{
        return Message{
            response: Response::default(),
            version: "1.0",
            session: Some(request::Session::default()),
        }
    }

    pub fn from_request(req: &request::Request<SessionState>)->Self{
        let mut mess = Self::new();
        mess.session = Some(req.session.clone());
        return mess;
    }

    pub fn with_response(mut self, resp: Response<SessionState>)->Self
    {
        self.response = resp;
        return self;
    }

    pub fn with_session(mut self, session: request::Session)->Self{
        self.session = Some(session);
        return self;
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
