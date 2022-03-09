use rasn::prelude::*;
use rasn_kerberos::*;

use pretty_assertions::assert_eq;

const _: () = assert!(AsReq::TAG.const_eq(&Tag::new(Class::Application, 10)));

#[test]
fn as_req() {
    let as_req = AsReq(KdcReq {
        pvno: Integer::parse_bytes(b"5", 10).unwrap(),
        msg_type: Integer::parse_bytes(b"10", 10).unwrap(),
        padata: Some(vec![PaData {
            r#type: 128,
            value: OctetString::from_static(&[0x30, 0x05, 0xa0, 0x03, 0x01, 0x01, 0xff]),
        }]),
        req_body: KdcReqBody {
            kdc_options: KdcOptions(KerberosFlags::from_slice(&[0x40, 0x81, 0x00, 0x10])),
            cname: Some(PrincipalName {
                r#type: 1,
                string: vec![KerberosString::new(String::from("user"))],
            }),
            realm: KerberosString::new("COMPANY.INT".to_string()),
            sname: Some(PrincipalName {
                r#type: 2,
                string: vec![
                    KerberosString::new(String::from("krbtgt")),
                    KerberosString::new(String::from("COMPANY.INT")),
                ],
            }),
            from: None,
            till: KerberosTime(
                GeneralizedTime::parse_from_rfc3339("2022-03-04T11:11:11Z").unwrap(),
            ),
            rtime: Some(KerberosTime(
                GeneralizedTime::parse_from_rfc3339("2052-03-04T11:11:11Z").unwrap(),
            )),
            nonce: 12345678,
            etype: vec![18, 23, -133, -128, 24, -135],
            addresses: Some(vec![HostAddress {
                addr_type: 20,
                address: OctetString::from("CLIENT01        "),
            }]),
            enc_authorization_data: None,
            additional_tickets: None,
        },
    });

    let data: &[u8] = &[
        0x6A, 0x81, 0xDC, 0x30, 0x81, 0xD9, 0xA1, 0x03, 0x02, 0x01, 0x05, 0xA2, 0x03, 0x02, 0x01,
        0x0A, 0xA3, 0x15, 0x30, 0x13, 0x30, 0x11, 0xA1, 0x04, 0x02, 0x02, 0x00, 0x80, 0xA2, 0x09,
        0x04, 0x07, 0x30, 0x05, 0xA0, 0x03, 0x01, 0x01, 0xFF, 0xA4, 0x81, 0xB5, 0x30, 0x81, 0xB2,
        0xA0, 0x07, 0x03, 0x05, 0x00, 0x40, 0x81, 0x00, 0x10, 0xA1, 0x11, 0x30, 0x0F, 0xA0, 0x03,
        0x02, 0x01, 0x01, 0xA1, 0x08, 0x30, 0x06, 0x1B, 0x04, 0x75, 0x73, 0x65, 0x72, 0xA2, 0x0D,
        0x1B, 0x0B, 0x43, 0x4F, 0x4D, 0x50, 0x41, 0x4E, 0x59, 0x2E, 0x49, 0x4E, 0x54, 0xA3, 0x20,
        0x30, 0x1E, 0xA0, 0x03, 0x02, 0x01, 0x02, 0xA1, 0x17, 0x30, 0x15, 0x1B, 0x06, 0x6B, 0x72,
        0x62, 0x74, 0x67, 0x74, 0x1B, 0x0B, 0x43, 0x4F, 0x4D, 0x50, 0x41, 0x4E, 0x59, 0x2E, 0x49,
        0x4E, 0x54, 0xA5, 0x11, 0x18, 0x0F, 0x32, 0x30, 0x32, 0x32, 0x30, 0x33, 0x30, 0x34, 0x31,
        0x31, 0x31, 0x31, 0x31, 0x31, 0x5A, 0xA6, 0x11, 0x18, 0x0F, 0x32, 0x30, 0x35, 0x32, 0x30,
        0x33, 0x30, 0x34, 0x31, 0x31, 0x31, 0x31, 0x31, 0x31, 0x5A, 0xA7, 0x06, 0x02, 0x04, 0x00,
        0xBC, 0x61, 0x4E, 0xA8, 0x16, 0x30, 0x14, 0x02, 0x01, 0x12, 0x02, 0x01, 0x17, 0x02, 0x02,
        0xFF, 0x7B, 0x02, 0x01, 0x80, 0x02, 0x01, 0x18, 0x02, 0x02, 0xFF, 0x79, 0xA9, 0x1D, 0x30,
        0x1B, 0x30, 0x19, 0xA0, 0x03, 0x02, 0x01, 0x14, 0xA1, 0x12, 0x04, 0x10, 0x43, 0x4C, 0x49,
        0x45, 0x4E, 0x54, 0x30, 0x31, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    ];

    let enc = rasn::der::encode(&as_req).unwrap();

    assert_eq!(data, enc);
    assert_eq!(as_req, rasn::der::decode(&enc).unwrap());
}