use libsip::headers::parse::parse_allow_header;
use libsip::headers::Header;
use libsip::core::Method;

#[test]
fn write() {
    let header = Header::Allow(vec![Method::Invite, Method::Options]);
    assert_eq!("Allow: INVITE,OPTIONS".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![' ' as u8];
    let header = Header::Allow(vec![Method::Register, Method::Invite]);
    assert_eq!(Ok((remains.as_ref(), header)), parse_allow_header(b"Allow: REGISTER,INVITE "));
}