use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub struct Request {
    pub request_type: RequestType,
    pub url: String,
    pub body: Body,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Body {
    pub format: BodyFormat,
    pub payload: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum RequestType {
    #[default]
    Get,
    Post,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BodyFormat {
    Json,
    XWwwFormUrlEncoded,
}

impl Default for Body {
    fn default() -> Self {
        Body::new(BodyFormat::Json, String::new())
    }
}

impl Body {
    pub fn new(format: BodyFormat, payload: String) -> Body {
        Body { format, payload }
    }
}