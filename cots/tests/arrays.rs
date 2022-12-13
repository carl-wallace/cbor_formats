use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use cots::arrays::*;
use cots::maps::*;
use hex_literal::hex;
mod utils;
use crate::utils::*;
extern crate alloc;
use alloc::vec::Vec;

#[test]
fn concise_ta_stores_test() {
    let valid = vec![
        "./tests/examples/tas1.cbor",
        "./tests/examples/tas2.cbor",
        "./tests/examples/tas3.cbor",
    ];
    let mut fab = ConciseTaStoresCbor(Vec::new());
    for f in valid {
        let expected = read_cbor(&Some(f.to_string()));
        let csc_d: ConciseTaStoreMapCbor = from_reader(expected.clone().as_slice()).unwrap();
        fab.0.push(csc_d);

        let mut encoded_token = vec![];
        let _ = into_writer(&fab, &mut encoded_token);
    }
    let fab_j: ConciseTaStores = fab.clone().try_into().unwrap();
    let _ = serde_json::to_string(&fab_j).unwrap();
    let fab_c: ConciseTaStoresCbor = fab_j.try_into().unwrap();
    assert_eq!(fab, fab_c);
}

#[test]
fn env_group_list_test() {
    fn parse_and_reencode_single(expected: &Vec<u8>) {
        println!("Encoded single: {:?}", buffer_to_hex(expected.as_slice()));

        let egl_d: EnvironmentGroupListMapCbor = from_reader(expected.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&egl_d, &mut encoded_token);
        assert_eq!(*expected, encoded_token);
    }
    fn parse_and_reencode_group(expected: &Vec<u8>) {
        println!("Encoded group: {:?}", buffer_to_hex(expected.as_slice()));
        let egl_d: EnvironmentGroupListCbor = from_reader(expected.clone().as_slice()).unwrap();
        let mut encoded_token = vec![];
        let _ = into_writer(&egl_d, &mut encoded_token);
        assert_eq!(*expected, encoded_token);
    }
    let tests_single = vec![
        hex!("a1036d536f6d652054412053746f7265").to_vec(),
        hex!("a101a100a100d86f442a030405").to_vec(),
        hex!("a102a102a2181f715a657374792048616e64732c20496e632e182102").to_vec(),
        hex!("a101a100a101715a657374792048616e64732c20496e632e").to_vec(),
        hex!("a101a100a10176536e6f6262697368204170706172656c2c20496e632e").to_vec(),
    ];
    let tests_group = vec![
        hex!("81a1036d536f6d652054412053746f7265").to_vec(),
        hex!("81a101a100a100d86f442a030405").to_vec(),
        hex!("81a102a102a2181f715a657374792048616e64732c20496e632e182102").to_vec(),
        hex!("81a101a100a101715a657374792048616e64732c20496e632e").to_vec(),
        hex!("82a101a100a101715a657374792048616e64732c20496e632ea101a100a10176536e6f6262697368204170706172656c2c20496e632e").to_vec(),
    ];

    for h in tests_single {
        parse_and_reencode_single(&h);
    }
    for h in tests_group {
        parse_and_reencode_group(&h);
    }
}
