use ciborium::{de::from_reader, ser::into_writer, tag::Required};

use corim::{
    choices::{ClassIdTypeChoice, GroupIdTypeChoice, InstanceIdTypeChoice},
    maps::{ClassMap, MeasurementMap, MeasurementValuesMap},
};
use coserv::arrays::{
    StatefulClass, StatefulClassCbor, StatefulGroup, StatefulGroupCbor, StatefulInstance,
    StatefulInstanceCbor,
};

// ── StatefulClass ──

#[test]
fn stateful_class_roundtrip() {
    let sc = StatefulClass {
        class: ClassMap {
            id: Some(ClassIdTypeChoice::uuid(common::UuidType(vec![
                0x31, 0xfb, 0x5a, 0xbf, 0x02, 0x3e, 0x49, 0x92, 0xaa, 0x4e, 0x95, 0xf9, 0xc1, 0x50,
                0x3b, 0xfa,
            ]))),
            vendor: Some("ACME".to_string()),
            model: Some("Widget".to_string()),
            layer: None,
            index: None,
        },
        measurements: None,
    };

    let cbor: StatefulClassCbor = sc.clone().try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();

    let decoded: StatefulClassCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(cbor, decoded);

    let roundtrip: StatefulClass = decoded.try_into().unwrap();
    assert_eq!(sc, roundtrip);
}

#[test]
fn stateful_class_with_measurements_roundtrip() {
    let sc = StatefulClass {
        class: ClassMap {
            id: Some(ClassIdTypeChoice::uuid(common::UuidType(vec![
                0x31, 0xfb, 0x5a, 0xbf, 0x02, 0x3e, 0x49, 0x92, 0xaa, 0x4e, 0x95, 0xf9, 0xc1, 0x50,
                0x3b, 0xfa,
            ]))),
            vendor: Some("ACME".to_string()),
            model: None,
            layer: None,
            index: None,
        },
        measurements: Some(vec![MeasurementMap {
            mkey: None,
            value: MeasurementValuesMap {
                version: None,
                svn: None,
                digests: None,
                flags: None,
                raw_value: None,
                raw_value_mask: None,
                mac_addr: None,
                ip_addr: None,
                serial_number: None,
                ueid: None,
                uuid: None,
                name: Some("test-measurement".to_string()),
                cryptokeys: None,
                int_range: None,
                other: None,
            },
            authorized_by: None,
        }]),
    };

    let cbor: StatefulClassCbor = sc.clone().try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();

    let decoded: StatefulClassCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(cbor, decoded);

    let roundtrip: StatefulClass = decoded.try_into().unwrap();
    assert_eq!(sc, roundtrip);
}

// ── StatefulInstance ──

#[test]
fn stateful_instance_roundtrip() {
    let si = StatefulInstance {
        instance: InstanceIdTypeChoice::Ueid(Required(common::UeidType(vec![
            0x02, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad,
        ]))),
        measurements: None,
    };

    let cbor: StatefulInstanceCbor = si.clone().try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();

    let decoded: StatefulInstanceCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(cbor, decoded);

    let roundtrip: StatefulInstance = decoded.try_into().unwrap();
    assert_eq!(si, roundtrip);
}

// ── StatefulGroup ──

#[test]
fn stateful_group_roundtrip() {
    let sg = StatefulGroup {
        group: GroupIdTypeChoice::Uuid(Required(common::UuidType(vec![
            0x31, 0xfb, 0x5a, 0xbf, 0x02, 0x3e, 0x49, 0x92, 0xaa, 0x4e, 0x95, 0xf9, 0xc1, 0x50,
            0x3b, 0xfa,
        ]))),
        measurements: None,
    };

    let cbor: StatefulGroupCbor = sg.clone().try_into().unwrap();
    let mut buf = vec![];
    into_writer(&cbor, &mut buf).unwrap();

    let decoded: StatefulGroupCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(cbor, decoded);

    let roundtrip: StatefulGroup = decoded.try_into().unwrap();
    assert_eq!(sg, roundtrip);
}
