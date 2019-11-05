use std::io::Result as IoResult;

use crate::core::Method;
use crate::headers::ContentType;
use crate::headers::Headers;
use crate::headers::Header;
use crate::headers::NamedHeader;
use crate::headers::via::ViaHeader;
use crate::uri::Uri;
use crate::uri::Schema;
use crate::SipMessage;
use crate::ResponseGenerator;
use crate::RequestGenerator;

macro_rules! impl_simple_header_method {
    ($name:ident, $variant:ident, $ty: ident) => {
        /// Retrieve value of the $variant header.
        pub fn $name(&self) -> IoResult<$ty> {
            if let Some(Header::$variant(header)) = self.headers.$name() {
                Ok(header)
            } else {
                Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, format!("message doesnt contain a {} header", stringify!($variant))))
            }
        }
    }
}

/// Structure to ease getting data from a Sip Message request.
pub struct MessageHelper {
    pub uri: Uri,
    pub headers: Headers,
    pub body: Vec<u8>
}

impl MessageHelper {

    /// Create a new MessageHelper from variables of a sip message.
    /// `uri` is the uri from the request line of the received sip message.
    pub fn new(uri: Uri, headers: Headers, body: Vec<u8>) -> IoResult<MessageHelper> {
        Ok(MessageHelper { uri, headers, body })
    }

    impl_simple_header_method!(from, From, NamedHeader);
    impl_simple_header_method!(to, To, NamedHeader);
    impl_simple_header_method!(contact, Contact, NamedHeader);
    impl_simple_header_method!(call_id, CallId, String);
    impl_simple_header_method!(xfs_sending_message, XFsSendingMessage, String);
    impl_simple_header_method!(via, Via, ViaHeader);

    /// Retrieve the data of this message, currently this
    /// just clone's the message body.
    ///
    /// TODO: Return no more than the Content-Length header amount.
    pub fn data(&self) -> Vec<u8> {
        self.body.clone()
    }

    /// Generate an OK response. Send this mesesage to the server
    /// immediatly after receiving the message to tell it to stop
    /// transmiting.
    pub fn received(&self) -> IoResult<SipMessage> {
        ResponseGenerator::new()
            .code(200)
            .header(self.headers.from().unwrap())
            .header(self.headers.to().unwrap())
            .header(self.headers.call_id().unwrap())
            .header(self.headers.cseq().unwrap())
            .header(self.headers.via().unwrap())
            .header(Header::ContentLength(0))
            .build()

    }
}

/// Structure to help when sending Sip messages. Handles the message CSeq,
/// Call-Id and User-Agent headers.
pub struct MessageWriter {
    cseq: u32,
    uri: Uri,
    call_id: String,
    user_agent: Option<String>,
}

impl MessageWriter {

    /// Create a new MessageWriter. `uri` is the account uri that will
    /// be placed in the `From` header.
    pub fn new(uri: Uri) -> MessageWriter {
        let _call_id = md5::compute(rand::random::<[u8 ; 16]>());
        let call_id = format!("{:x}@{}", _call_id, uri.host);
        MessageWriter {
            cseq: 0,
            user_agent: None,
            uri, call_id
        }
    }

    pub fn write_message(&mut self, body: Vec<u8>, to: Uri, via_header: Header) -> IoResult<SipMessage> {
        self.cseq += 1;
        RequestGenerator::new()
            .method(Method::Message)
            .uri(to.clone().schema(Schema::Sip))
            .header(via_header)
            .header(Header::To(NamedHeader::new(to)))
            .header(self.from())
            .header(self.cseq())
            .header(self.call_id())
            .header(self.user_agent())
            .header(self.max_forwards())
            .header(self.content_type())
            .header(Header::ContentLength(body.len() as u32))
            .body(body)
            .build()

    }

    /// Get a new CSeq header.
    pub fn cseq(&self) -> Header {
        Header::CSeq(self.cseq, Method::Message)
    }

    /// Get a new Content-Type header.
    pub fn content_type(&self) -> Header {
        Header::ContentType(ContentType::PlainText)
    }

    /// Get a new Max-Forwards header.
    pub fn max_forwards(&self) -> Header {
        Header::MaxForwards(70)
    }

    /// Get a new User-Agent header.
    pub fn user_agent(&self) -> Header {
        if let Some(agent) = &self.user_agent {
            Header::UserAgent(agent.clone())
        } else {
            Header::UserAgent(format!("libsip {}", env!("CARGO_PKG_VERSION")))
        }
    }

    /// Get a new Call-Id header.
    pub fn call_id(&self) -> Header {
        Header::CallId(self.call_id.clone())
    }

    /// Get a new From header.
    pub fn from(&self) -> Header {
        Header::From(NamedHeader::new(self.uri.clone()))
    }
}