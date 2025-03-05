/// Example JSON:
/// {
///     "request": {
///         "type": "(GET, POST)",
///         "url": "example.com",
///         "body": {
///             "format": "(application/json or x-www-urlencoded)",
///             "payload": "Hello world!"
///         }
///     }
/// }

pub struct Request {
    pub request_type: RequestType,
    pub url: String,
    pub body: Body
}

pub struct Body {
    pub format: BodyFormat,
    pub payload: String
}

pub enum RequestType {
    Get,
    Post
}

pub enum BodyFormat {
    Json,
    XWwwFormUrlEncoded
}

impl Request {
    pub fn new(request_type: RequestType, url: String, body: Body) -> Request {
        Request {
            request_type,
            url,
            body
        }
    }
}

impl Body {
    pub fn new(format: BodyFormat, payload: String) -> Body {
        Body {
            format,
            payload
        }
    }
}

impl RequestType {
    pub fn from_str(request_type: &str) -> RequestType {
        match request_type {
            "GET" => RequestType::Get,
            "POST" => RequestType::Post,
            _ => RequestType::Get
        }
    }
}