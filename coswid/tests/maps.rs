use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use coswid::maps::*;
use hex_literal::hex;

#[allow(dead_code)]
pub fn buffer_to_hex(buffer: &[u8]) -> String {
    let hex = subtle_encoding::hex::encode_upper(buffer);
    let r = std::str::from_utf8(hex.as_slice());
    if let Ok(s) = r {
        s.to_string()
    } else {
        "".to_string()
    }
}

#[test]
fn concise_swid_tag_test() {
    //todo
}

#[test]
fn coswid_test() {
    // {
    // 		"tag-id": "f432dc99-2e06-434d-b9ad-2b22e35b6fa4",
    // 		"tag-version": 0,
    // 		"software-name": "Roadrunner software bundle",
    // 		"software-version": "1.0.0",
    // 		"entity": [
    // 		  {
    // 			"entity-name": "ACME Ltd",
    // 			"reg-id": "acme.example",
    // 			"role": [
    // 			  "tagCreator",
    // 			  "softwareCreator"
    // 			]
    // 		  }
    // 		],
    // 		"link": [
    // 		  {
    // 			"href": "d84fb5e2-d198-49b4-9d65-3a82421bf180",
    // 			"rel": "parent"
    // 		  }
    // 		]
    // 	}
    let expected = hex!("a60050f432dc992e06434db9ad2b22e35b6fa40c0001781a526f616472756e6e657220736f6674776172652062756e646c650d65312e302e3002a3181f6841434d45204c746418206c61636d652e6578616d706c65182182010204a21826782464383466623565322d643139382d343962342d396436352d336138323432316266313830182806");
    let coswid: ConciseSwidTagCbor = from_reader(expected.as_slice()).unwrap();

    let mut actual = vec![];
    let _ = into_writer(&coswid, &mut actual);
    assert_eq!(expected.to_vec(), actual);
}

#[test]
fn directory_entry_test() {
    //todo
}

#[test]
fn entity_entry_test() {
    let expected = hex!("a2181f715a657374792048616e64732c20496e632e182102");
    let egl_d: EntityEntryCbor = from_reader(expected.clone().as_slice()).unwrap();
    let mut encoded_token = vec![];
    let _ = into_writer(&egl_d, &mut encoded_token);
    assert_eq!(expected.to_vec(), encoded_token);
}

#[test]
fn evidence_entry_test() {
    //todo
}

#[test]
fn file_entry_test() {
    //todo
}

#[test]
fn link_entry_test() {
    //todo
}

#[test]
fn path_elements_group_test() {
    let mut encoded_token = vec![];
    let fm = PathElementsGroupCbor {
        directory: Some(OneOrMoreDirectoryEntryCbor::One(DirectoryEntryCbor {
            key: None,
            location: None,
            fs_name: "fs_name".to_string(),
            root: None,
            lang: None,
            other: None,
        })),
        file: None,
    };
    let _ = into_writer(&fm, &mut encoded_token);
    println!(
        "PathElementsGroupCbor: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );
    let fm_d: PathElementsGroupCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    assert_eq!(fm_d, fm);
}

#[test]
fn path_elements_group_test2() {
    let d1 = DirectoryEntryCbor {
        key: None,
        location: None,
        fs_name: "fs_name1".to_string(),
        root: None,
        lang: None,
        other: None,
    };
    let d2 = DirectoryEntryCbor {
        key: None,
        location: None,
        fs_name: "fs_name2".to_string(),
        root: None,
        lang: None,
        other: None,
    };
    let v = vec![d1, d2];

    let mut encoded_token = vec![];
    let fm = PathElementsGroupCbor {
        directory: Some(OneOrMoreDirectoryEntryCbor::More(v)),
        file: None,
    };
    let _ = into_writer(&fm, &mut encoded_token);
    println!(
        "PathElementsGroupCbor: {:?}",
        buffer_to_hex(encoded_token.as_slice())
    );
    let fm_d: PathElementsGroupCbor = from_reader(encoded_token.clone().as_slice()).unwrap();
    assert_eq!(fm_d, fm);

    //let d : OneOrMoreDirectoryEntry = fm_d.directory.unwrap().try_into().unwrap();

    let d3 = DirectoryEntryCbor {
        key: None,
        location: None,
        fs_name: "fs_name2".to_string(),
        root: None,
        lang: None,
        other: None,
    };
    let oam = OneOrMoreDirectoryEntryCbor::One(d3);
    let oam_ref = &oam;
    let dref_tf: OneOrMoreDirectoryEntry = oam_ref.try_into().unwrap();
    println!("dref_tf: {:?}", dref_tf);

    let d4 = DirectoryEntry {
        key: None,
        location: None,
        fs_name: "fs_name2".to_string(),
        root: None,
        lang: None,
        other: None,
    };
    let oam2 = OneOrMoreDirectoryEntry::One(d4);
    let back: OneOrMoreDirectoryEntryCbor = oam2.try_into().unwrap();
    assert_eq!(back, oam);
}

#[test]
fn payload_entry_test() {
    //todo
}

#[test]
fn process_entry_test() {
    //todo
}

#[test]
fn resource_entry_test() {
    //todo
}

#[test]
fn software_meta_entry_test() {
    //todo
}
