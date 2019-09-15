use libsip::headers::Header;
use libsip::headers::parse::parse_date_header;

#[test]
fn write() {
    let header = Header::Date("wed 1 2001".into());
    assert_eq!("Date: wed 1 2001".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let header = Header::Date("wed 1 2001".into());
    assert_eq!(Ok((remains.as_ref(), header)), parse_date_header(b"Date: wed 1 2001\n"));
}