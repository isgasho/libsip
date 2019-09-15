use libsip::headers::parse::parse_max_forwards_header;
use libsip::headers::Header;

#[test]
fn write() {
    let header = Header::MaxForwards(70);
    assert_eq!("Max-Forwards: 70".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![' ' as u8];
    let header = Header::MaxForwards(60);
    assert_eq!(Ok((remains.as_ref(), header)), parse_max_forwards_header(b"Max-Forwards: 60 "));
}