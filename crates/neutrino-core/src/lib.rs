wit_bindgen::generate!({
    path: "../../wit",
});

use exports::neutrino::core::ipc::{Guest, Request, Response};

struct NeutrinoCore;

impl Guest for NeutrinoCore {
    fn invoke(request: Request) -> Response {
        match request.action.as_str() {
            "ping" => Response {
                status: 200,
                body: "pong".to_owned(),
                error: None,
            },
            "echo" => Response {
                status: 200,
                body: request.payload,
                error: None,
            },
            _ => Response {
                status: 404,
                body: String::new(),
                error: Some(format!("unknown action: {}", request.action)),
            },
        }
    }
}

export!(NeutrinoCore with_types_in self);
