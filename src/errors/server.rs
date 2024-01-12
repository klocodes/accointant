use thiserror::Error;
use serde_json::Value;

#[derive(Debug, Error)]
pub enum ServerErrors {
    #[error("Internal Server Error: A general error when an unexpected condition was encountered on the server. {context:?}")]
    InternalServerError { context: Option<Value> },

    #[error("Forbidden: Access to the requested resource is denied. {context:?}")]
    Forbidden { context: Option<Value> },

    #[error("Conflict: The request could not be completed due to a conflict with the current state of the resource. {context:?}")]
    Conflict { context: Option<Value> },

    #[error("Gone: The requested resource is no longer available and will not be available again. {context:?}")]
    Gone { context: Option<Value> },

    #[error("Length Required: The request did not specify the length of its content, which is required by the requested resource. {context:?}")]
    LengthRequired { context: Option<Value> },

    #[error("Precondition Failed: The server does not meet one of the preconditions that the requester put on the request. {context:?}")]
    PreconditionFailed { context: Option<Value> },

    #[error("Payload Too Large: The request is larger than the server is willing or able to process. {context:?}")]
    PayloadTooLarge { context: Option<Value> },

    #[error("Expectation Failed: The server cannot meet the requirements of the Expect request-header field. {context:?}")]
    ExpectationFailed { context: Option<Value> },

    #[error("I'm a teapot: The server refuses the attempt to brew coffee with a teapot. {context:?}")]
    ImATeapot { context: Option<Value> },

    #[error("Misdirected Request: The request was directed at a server that is not able to produce a response. {context:?}")]
    MisdirectedRequest { context: Option<Value> },

    #[error("Unprocessable Entity: The server understands the content type of the request entity, but was unable to process the contained instructions. {context:?}")]
    UnprocessableEntity { context: Option<Value> },

    #[error("Locked: The resource that is being accessed is locked. {context:?}")]
    Locked { context: Option<Value> },

    #[error("Failed Dependency: The request failed because it depended on another request and that request failed. {context:?}")]
    FailedDependency { context: Option<Value> },

    #[error("Upgrade Required: The client should switch to a different protocol such as TLS/1.0. {context:?}")]
    UpgradeRequired { context: Option<Value> },

    #[error("Precondition Required: The server requires that the request be conditional. {context:?}")]
    PreconditionRequired { context: Option<Value> },

    #[error("Unavailable For Legal Reasons: The server is denying access to the resource as a consequence of a legal demand. {context:?}")]
    UnavailableForLegalReasons { context: Option<Value> },

    #[error("Not Implemented: The server does not support the functionality required to fulfill the request. {context:?}")]
    NotImplemented { context: Option<Value> },

    #[error("Bad Gateway: The server, while acting as a gateway or proxy, received an invalid response from the upstream server. {context:?}")]
    BadGateway { context: Option<Value> },

    #[error("Service Unavailable: The server is currently unable to handle the request due to a temporary overloading or maintenance. {context:?}")]
    ServiceUnavailable { context: Option<Value> },

    #[error("Gateway Timeout: The server, while acting as a gateway or proxy, did not receive a timely response from the upstream server. {context:?}")]
    GatewayTimeout { context: Option<Value> },

    #[error("HTTP Version Not Supported: The server does not support the HTTP protocol version used in the request. {context:?}")]
    HttpVersionNotSupported { context: Option<Value> },

    #[error("Variant Also Negotiates: Transparent content negotiation for the request results in a circular reference. {context:?}")]
    VariantAlsoNegotiates { context: Option<Value> },

    #[error("Insufficient Storage: The server is unable to store the representation needed to complete the request. {context:?}")]
    InsufficientStorage { context: Option<Value> },

    #[error("Loop Detected: The server detected an infinite loop while processing the request. {context:?}")]
    LoopDetected { context: Option<Value> },

    #[error("Not Extended: Further extensions to the request are required for the server to fulfill it. {context:?}")]
    NotExtended { context: Option<Value> },
}