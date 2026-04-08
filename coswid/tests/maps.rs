use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use coswid::choices::*;
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
    let cst = ConciseSwidTagCbor {
        tag_id: common::TextOrBinary::Text("test-tag-id".to_string()),
        tag_version: 1,
        corpus: Some(false),
        patch: None,
        supplemental: None,
        software_name: "Test Software".to_string(),
        software_version: Some("2.0.0".to_string()),
        version_scheme: None,
        media: None,
        software_meta: None,
        entity: None,
        link: None,
        evidence: None,
        payload: None,
        lang: None,
        other: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&cst, &mut buf);
    let decoded: ConciseSwidTagCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(cst, decoded);
    assert_eq!(decoded.software_name, "Test Software");
    assert_eq!(decoded.tag_version, 1);
    assert_eq!(decoded.corpus, Some(false));

    let json: ConciseSwidTag = decoded.try_into().unwrap();
    let cbor_roundtrip: ConciseSwidTagCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
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
    let expected = hex!(
        "a60050f432dc992e06434db9ad2b22e35b6fa40c0001781a526f616472756e6e657220736f6674776172652062756e646c650d65312e302e3002a3181f6841434d45204c746418206c61636d652e6578616d706c65182182010204a21826782464383466623565322d643139382d343962342d396436352d336138323432316266313830182806"
    );
    let coswid: ConciseSwidTagCbor = from_reader(expected.as_slice()).unwrap();

    let mut actual = vec![];
    let _ = into_writer(&coswid, &mut actual);
    assert_eq!(expected.to_vec(), actual);
}

#[test]
fn directory_entry_test() {
    let de = DirectoryEntryCbor {
        key: Some(true),
        location: Some("/usr/local".to_string()),
        fs_name: "bin".to_string(),
        root: Some("/".to_string()),
        lang: None,
        other: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&de, &mut buf);
    let decoded: DirectoryEntryCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(de, decoded);
    assert_eq!(decoded.fs_name, "bin");
    assert_eq!(decoded.key, Some(true));

    let json: DirectoryEntry = decoded.try_into().unwrap();
    let cbor_roundtrip: DirectoryEntryCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
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
    let ee = EvidenceEntryCbor {
        directory: None,
        file: Some(OneOrMoreFileEntryCbor::One(FileEntryCbor {
            key: None,
            location: None,
            fs_name: "evidence.bin".to_string(),
            root: None,
            size: Some(1024),
            file_version: None,
            hash: None,
            lang: None,
            other: None,
        })),
        process: None,
        resource: None,
        date: None,
        device_id: Some("device-001".to_string()),
        location: None,
        lang: None,
        other: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&ee, &mut buf);
    let decoded: EvidenceEntryCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(ee, decoded);
    assert_eq!(decoded.device_id, Some("device-001".to_string()));
}

#[test]
fn file_entry_test() {
    let fe = FileEntryCbor {
        key: None,
        location: Some("/usr/bin".to_string()),
        fs_name: "test.exe".to_string(),
        root: None,
        size: Some(4096),
        file_version: Some("1.0".to_string()),
        hash: None,
        lang: None,
        other: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&fe, &mut buf);
    let decoded: FileEntryCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(fe, decoded);
    assert_eq!(decoded.fs_name, "test.exe");
    assert_eq!(decoded.size, Some(4096));

    let json: FileEntry = decoded.try_into().unwrap();
    let cbor_roundtrip: FileEntryCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn link_entry_test() {
    let le = LinkEntryCbor {
        artifact: Some("test-artifact".to_string()),
        href: "https://example.com/link".to_string(),
        media: None,
        ownership: None,
        rel: Rel::Known(RelKnown::Parent),
        media_type: None,
        use_choice: None,
        lang: None,
        other: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&le, &mut buf);
    let decoded: LinkEntryCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(le, decoded);
    assert_eq!(decoded.href, "https://example.com/link");

    let json: LinkEntry = decoded.try_into().unwrap();
    let cbor_roundtrip: LinkEntryCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
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
    let pe = PayloadEntryCbor {
        directory: None,
        file: Some(OneOrMoreFileEntryCbor::One(FileEntryCbor {
            key: None,
            location: None,
            fs_name: "payload.bin".to_string(),
            root: None,
            size: None,
            file_version: None,
            hash: None,
            lang: None,
            other: None,
        })),
        process: None,
        resource: None,
        lang: None,
        other: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&pe, &mut buf);
    let decoded: PayloadEntryCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(pe, decoded);
}

#[test]
fn process_entry_test() {
    let pe = ProcessEntryCbor {
        process_name: "svchost.exe".to_string(),
        pid: Some(1234),
        lang: None,
        other: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&pe, &mut buf);
    let decoded: ProcessEntryCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(pe, decoded);
    assert_eq!(decoded.process_name, "svchost.exe");
    assert_eq!(decoded.pid, Some(1234));

    let json: ProcessEntry = decoded.try_into().unwrap();
    let cbor_roundtrip: ProcessEntryCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn resource_entry_test() {
    let re = ResourceEntryCbor {
        resource_entry_type: "firmware".to_string(),
        lang: Some("en".to_string()),
        other: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&re, &mut buf);
    let decoded: ResourceEntryCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(re, decoded);
    assert_eq!(decoded.resource_entry_type, "firmware");

    let json: ResourceEntry = decoded.try_into().unwrap();
    let cbor_roundtrip: ResourceEntryCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn software_meta_entry_test() {
    let sme = SoftwareMetaEntryCbor {
        activation_status: Some("activated".to_string()),
        channel_type: None,
        colloquial_version: None,
        description: Some("Test software description".to_string()),
        edition: None,
        entitlement_data_required: Some(false),
        entitlement_key: None,
        generator: Some("test-generator".to_string()),
        persistent_id: None,
        product: Some("TestProduct".to_string()),
        product_family: None,
        revision: None,
        summary: Some("A test summary".to_string()),
        unspsc_code: None,
        unspsc_version: None,
        lang: None,
        other: None,
    };
    let mut buf = vec![];
    let _ = into_writer(&sme, &mut buf);
    let decoded: SoftwareMetaEntryCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(sme, decoded);
    assert_eq!(decoded.product, Some("TestProduct".to_string()));

    let json: SoftwareMetaEntry = decoded.try_into().unwrap();
    let cbor_roundtrip: SoftwareMetaEntryCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn resource_collection_test() {
    let rc = ResourceCollectionCbor {
        directory: Some(OneOrMoreDirectoryEntryCbor::One(DirectoryEntryCbor {
            key: None,
            location: Some("/opt".to_string()),
            fs_name: "myapp".to_string(),
            root: None,
            lang: None,
            other: None,
        })),
        file: Some(OneOrMoreFileEntryCbor::One(FileEntryCbor {
            key: None,
            location: Some("/opt/myapp".to_string()),
            fs_name: "config.yaml".to_string(),
            root: None,
            size: Some(512),
            file_version: Some("1.0".to_string()),
            hash: None,
            lang: None,
            other: None,
        })),
        process: Some(OneOrMoreProcessEntryCbor::One(ProcessEntryCbor {
            process_name: "myapp-daemon".to_string(),
            pid: Some(5678),
            lang: None,
            other: None,
        })),
        resource: Some(OneOrMoreResourceEntryCbor::One(ResourceEntryCbor {
            resource_entry_type: "hardware".to_string(),
            lang: None,
            other: None,
        })),
    };
    let mut buf = vec![];
    let _ = into_writer(&rc, &mut buf);
    let decoded: ResourceCollectionCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(rc, decoded);
    assert_eq!(
        decoded.directory.as_ref().unwrap(),
        rc.directory.as_ref().unwrap()
    );
    assert_eq!(decoded.file.as_ref().unwrap(), rc.file.as_ref().unwrap());
    assert_eq!(
        decoded.process.as_ref().unwrap(),
        rc.process.as_ref().unwrap()
    );
    assert_eq!(
        decoded.resource.as_ref().unwrap(),
        rc.resource.as_ref().unwrap()
    );

    let json: ResourceCollection = decoded.try_into().unwrap();
    let cbor_roundtrip: ResourceCollectionCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn path_elements_group_file_test() {
    let peg = PathElementsGroupCbor {
        directory: None,
        file: Some(OneOrMoreFileEntryCbor::One(FileEntryCbor {
            key: Some(false),
            location: Some("/var/log".to_string()),
            fs_name: "app.log".to_string(),
            root: None,
            size: Some(2048),
            file_version: None,
            hash: None,
            lang: None,
            other: None,
        })),
    };
    let mut buf = vec![];
    let _ = into_writer(&peg, &mut buf);
    let decoded: PathElementsGroupCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(peg, decoded);

    let json: PathElementsGroup = decoded.try_into().unwrap();
    let cbor_roundtrip: PathElementsGroupCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn path_elements_group_both_test() {
    let peg = PathElementsGroupCbor {
        directory: Some(OneOrMoreDirectoryEntryCbor::One(DirectoryEntryCbor {
            key: None,
            location: Some("/etc".to_string()),
            fs_name: "myapp.d".to_string(),
            root: None,
            lang: None,
            other: None,
        })),
        file: Some(OneOrMoreFileEntryCbor::One(FileEntryCbor {
            key: None,
            location: Some("/etc/myapp.d".to_string()),
            fs_name: "settings.conf".to_string(),
            root: None,
            size: Some(256),
            file_version: None,
            hash: None,
            lang: None,
            other: None,
        })),
    };
    let mut buf = vec![];
    let _ = into_writer(&peg, &mut buf);
    let decoded: PathElementsGroupCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(peg, decoded);
    assert!(decoded.directory.is_some());
    assert!(decoded.file.is_some());

    let json: PathElementsGroup = decoded.try_into().unwrap();
    let cbor_roundtrip: PathElementsGroupCbor = json.try_into().unwrap();
    let mut buf2 = vec![];
    let _ = into_writer(&cbor_roundtrip, &mut buf2);
    assert_eq!(buf, buf2);
}

#[test]
fn veraison_coswid_decode() {
    let data =
        std::fs::read("./tests/data/coswid-veraison.cbor").expect("test vector file missing");
    let decoded: ConciseSwidTagCbor = from_reader(data.as_slice()).unwrap();

    // roundtrip
    let mut buf = vec![];
    let _ = into_writer(&decoded, &mut buf);
    let decoded2: ConciseSwidTagCbor = from_reader(buf.as_slice()).unwrap();
    assert_eq!(decoded, decoded2);
}
