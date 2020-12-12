// https://yandex.ru/dev/dialogs/alice/doc/protocol.html

use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub response: Response,

    version: &'static str,
}

#[derive(Serialize)]
pub struct Response {
    pub text: String,
    pub end_session: bool,
}

impl Message{
    pub fn new()->Message{
        return Message{
            response: Response{text: String::new(), end_session: false},
            version: "1.0",
        }
    }
}
