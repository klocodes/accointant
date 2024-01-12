use thiserror::Error;
use serde_json::Value;

#[derive(Debug, Error)]
pub enum NetworkErrors {
    #[error("Method Not Allowed: The request method is known by the server but is not supported by the target resource. {context:?}")]
    MethodNotAllowed { context: Option<Value> },

    #[error("Not Acceptable: The server cannot produce a response matching the list of acceptable values defined in the request's proactive content negotiation headers. {context:?}")]
    NotAcceptable { context: Option<Value> },

    #[error("Request Timeout: The server timed out waiting for the request. {context:?}")]
    RequestTimeout { context: Option<Value> },

    #[error("URI Too Long: The URI requested by the client is longer than the server is willing to interpret. {context:?}")]
    UriTooLong { context: Option<Value> },

    #[error("Unsupported Media Type: The media format of the requested data is not supported by the server, so the server is rejecting the request. {context:?}")]
    UnsupportedMediaType { context: Option<Value> },

    #[error("Range Not Satisfiable: The range specified by the Range header field in the request can't be fulfilled. {context:?}")]
    RangeNotSatisfiable { context: Option<Value> },

    #[error("Too Many Requests: The user has sent too many requests in a given amount of time ('rate limiting'). {context:?}")]
    TooManyRequests { context: Option<Value> },

    #[error("Request Header Fields Too Large: The server is unwilling to process the request because its header fields are too large. {context:?}")]
    RequestHeaderFieldsTooLarge { context: Option<Value> },

    #[error("Network Authentication Required: The client needs to authenticate to gain network access. {context:?}")]
    NetworkAuthenticationRequired { context: Option<Value> },
}
