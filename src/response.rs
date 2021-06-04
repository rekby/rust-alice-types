// https://yandex.ru/dev/dialogs/alice/doc/protocol.html

use serde::Serialize;

use crate::request;

#[derive(Serialize)]
pub struct Message<SessionState: Clone = serde_json::Value> {
    pub response: Response,

    // for marusya https://vk.com/dev/marusia_skill_docs
    pub session: Option<request::Session>,

    // https://yandex.ru/dev/dialogs/alice/doc/session-persistence.html/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_state:Option<SessionState>,

    version: &'static str,
}

impl<SessionState: Clone> Message<SessionState>{
    pub fn new()->Self{
        return Message{
            response: Response::default(),
            version: "1.0",
            session_state: None,
            session: Some(request::Session::default()),
        }
    }

    pub fn from_request(req: &request::Request<SessionState>)->Self{
        let mut mess = Self::new();
        mess.session = Some(req.session.clone());
        mess.session_state = req.session_state.clone();
        return mess;
    }

    pub fn with_response(mut self, resp: Response)->Self
    {
        self.response = resp;
        return self;
    }

    // technical session info, need for marusia
    pub fn with_session(mut self, session: request::Session)->Self{
        self.session = Some(session);
        return self;
    }

    pub fn with_session_state(mut self, session_state: SessionState)->Self{
        self.session_state = Some(session_state);
        return self;
    }
}


#[derive(Serialize)]
pub struct Response {
    pub text: String,
    pub end_session: bool,
}

impl Response{
    pub fn with_text(mut self, text: String)->Self{
        self.text = text;
        return self;
    }
    pub fn with_session_end(mut self)->Self{
        self.end_session = true;
        return self;
    }
}

impl Default for Response{
    fn default()->Self{
        return Response{
            text: String::default(),
            end_session: bool::default(),
        }
    }
}
