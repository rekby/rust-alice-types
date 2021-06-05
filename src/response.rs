// https://yandex.ru/dev/dialogs/alice/doc/protocol.html

use serde::Serialize;

use crate::request;

#[derive(Serialize)]
pub struct Message<SessionState=serde_json::Value, UserState=serde_json::Value> {
    pub response: Response,

    // for marusya https://vk.com/dev/marusia_skill_docs
    pub session: Option<request::Session>,

    // https://yandex.ru/dev/dialogs/alice/doc/session-persistence.html#store-session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_state: Option<SessionState>,

    // https://yandex.ru/dev/dialogs/alice/doc/session-persistence.html#store-between-sessions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_state_update: Option<UserState>,

    version: &'static str,
}

impl<SessionState, UserState> Message<SessionState, UserState> {
    pub fn new()->Self{
        return Message{
            response: Response::default(),
            version: "1.0",
            session_state: None,
            user_state_update: None,
            session: Some(request::Session::default()),
        }
    }
    pub fn with_response(mut self, resp: Response)->Self
    {
        self.response = resp;
        return self;
    }

    // technical session info, need for marusia
    pub fn with_session(mut self, session: Option<request::Session>)->Self{
        self.session = session;
        return self;
    }

    pub fn with_session_state(mut self, session_state: Option<SessionState>)->Self{
        self.session_state = session_state;
        return self;
    }

    pub fn with_user_state(mut self, user_state: Option<UserState>)->Self {
        self.user_state_update = user_state;
        return self;
    }
}

impl<SessionState: Clone, UserState> Message<SessionState, UserState>{
    pub fn from_request(req: &request::Request<SessionState, UserState>)->Self{
        let mut mess = Self::new();
        mess.session = Some(req.session.clone());
        mess.session_state = req.state.session.clone();
        return mess;
    }
}


#[derive(Serialize)]
pub struct Response {
    pub text: String,
    pub end_session: bool,
}

impl Response{
    pub fn with_text<T: Into<String>>(mut self, text: T)->Self{
        self.text = text.into();
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
