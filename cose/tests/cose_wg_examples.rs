use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use cose::arrays::{
    TaggedCoseEncrypt, TaggedCoseEncrypt0, TaggedCoseMac, TaggedCoseMac0, TaggedCoseSign,
};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use subtle_encoding::hex;
use walkdir::WalkDir;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//todo figure out why this can't be referenced from utils
#[allow(dead_code)]
fn get_file_as_byte_vec(filename: &Path) -> Vec<u8> {
    match File::open(filename) {
        Ok(mut f) => match std::fs::metadata(filename) {
            Ok(metadata) => {
                let mut buffer = vec![0; metadata.len() as usize];
                match f.read_exact(&mut buffer) {
                    Ok(_) => buffer,
                    Err(_e) => panic!(),
                }
            }
            Err(_e) => panic!(),
        },
        Err(_e) => panic!(),
    }
}

// Example = {
//    title: tstr,                                 # summary of test
//    ? description: tstr,                         # longer description of test
//    ? fail : bool,                               # Is this a success or failure test
//    input: Inputs,                               # Inputs to create the test
//    ? intermediates : Intermediates,             # Intermediate values for debugging
//    output: Outputs                              # Outputs of the test
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Example {
    pub title: String,
    pub description: Option<String>,
    pub fail: Option<bool>,
    pub input: Inputs,
    pub intermediates: Intermediates,
    pub output: Outputs,
}

// Inputs = {
//    plaintext: bstr / tstr,
//    ? detached: bool,
//    (enveloped: Enveloped) /                     # Create an enveloped Message
//    (encrypt: Encrypt) /                         # Create an encrypt Message
//    (mac: Mac) /                                 # Create a MAC message
//    (mac0: Mac0) /                               # Craete a MAC0 message
//    (sign: Sign),
//    ? failures : FailureSet,                     # Description of failure changes applied
//    ? rng_description : tstr,                    # What is what in the RNG stream
//    ? rng_stream: [+ tstr]                       # Random number generator stream - encoded as hex
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Inputs {
    pub plaintext: Option<String>,
    pub plaintext_hex: Option<String>,
    pub detached: Option<bool>,
    pub enveloped: Option<Enveloped>,
    pub encrypt: Option<Encrypt>,
    pub mac: Option<Mac>,
    pub mac0: Option<Mac0>,
    pub sign: Option<Sign>,
    pub failures: Option<Failures>,
    pub rng_description: Option<String>,
    pub rng_stream: Option<Vec<String>>,
}

// Sign = {
//   signers: [+ Signers]
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Sign {
    signers: Vec<Signers>,
}

// headers = (
//    ? protected: { +header_items },          # Protected headers to be sent
//    ? unprotected: { +header_items },        # Unprotected headers to be sent
//    ? unsent: { +header_items }              # Headers not to be sent
// }

// header_items = (
//    "alg" : tstr,                                # Algorithm parameter
//    "kid" : tstr,                                # key identifier - cast to bstr
//    "kid_hex" : tstr,                            # key identifier - encoded as hex
//    "epk" : key,                                 # ephemeral key
//    "spk" : key,                                 # static key
//    "spk_kid" : tstr,                            # static key identifier - cast to bstr
//    "spk_kid_hex" : tstr,                        # static key identifier - encoded as hex
//    "apu_id" : tstr,                             # PartyU identifier - cast to bstr
//    "apu_nonce_hex" : tstr,                      # PartyU nonce - encoded as hex
//    "apv_id" : tstr,                             # PartyV identifier - cast to bstr
//    "pub_other" : tstr,                          # Public other Info - cast to bstr
//    "salt" : tstr,                               # Salt value - cast to bstr
// )
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HeaderItems {
    pub alg: Option<String>,
    pub kid: Option<String>,
    pub kid_hex: Option<String>,
    pub epk: Option<Key>,
    pub spk: Option<Key>,
    pub spk_kid: Option<String>,
    pub spk_kid_hex: Option<String>,
    pub apu_id: Option<String>,
    pub apu_nonce_hex: Option<String>,
    pub apv_id: Option<String>,
    pub pub_other: Option<String>,
    pub salt: Option<String>,
}

// Signers = {
//   alg: tstr,
//   key: Key,
//   headers
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Signers {
    pub alg: Option<String>,
    pub key: Key,
    pub protected: Option<HeaderItems>,
    pub unprotected: Option<HeaderItems>,
    pub unsent: Option<HeaderItems>,
}

// Mac = {
//   alg: tstr,
//   headers,
//   recipients: [+ Recipients]
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Mac {
    pub alg: Option<String>,
    pub protected: Option<HeaderItems>,
    pub unprotected: Option<HeaderItems>,
    pub unsent: Option<HeaderItems>,
    pub recipients: Vec<Recipient>,
}

// Mac0 = {
//   alg: tstr,
//   headers
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Mac0 {
    pub alg: Option<String>,
    pub protected: Option<HeaderItems>,
    pub unprotected: Option<HeaderItems>,
    pub unsent: Option<HeaderItems>,
}

// Encrypt = {
//     alg: tstr,
//     headers
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Encrypt {
    pub alg: Option<String>,
    pub protected: Option<HeaderItems>,
    pub unprotected: Option<HeaderItems>,
    pub unsent: Option<HeaderItems>,
}

// Enveloped = {
//     alg: tstr,
//     headers,
//     recipients: [+ Recipients]
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Enveloped {
    pub alg: Option<String>,
    pub protected: Option<HeaderItems>,
    pub unprotected: Option<HeaderItems>,
    pub unsent: Option<HeaderItems>,
    pub recipients: Vec<Recipient>,
}

// Recipient = {
//    alg: tstr,
//    ? fail: bool,                                # does this recipient fail
//    headers,                                     # Headers for the recipient
//    key: Key,                                    # Recipient Key
//    ? sender_key: Key,                           # Sender key
//    failures: Failures                           # Set of failures to apply to recipient
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Recipient {
    pub alg: Option<String>,
    pub fail: Option<bool>,
    pub protected: Option<HeaderItems>,
    pub unprotected: Option<HeaderItems>,
    pub unsent: Option<HeaderItems>,
    pub key: Key,
    pub sender_key: Option<Key>,
    pub failures: Option<Failures>,
}

// Key = {
//   (tstr/int) => *
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Key {
    pub kty: String,
    pub kid: Option<String>,
    #[serde(rename = "use")]
    pub key_use: Option<String>,
    pub k: Option<String>,
    pub crv: Option<String>,
    pub x_hex: Option<String>,
    pub d_hex: Option<String>,
    pub y_hex: Option<String>,
    pub k_hex: Option<String>,
    pub x: Option<String>,
    pub d: Option<String>,
    pub y: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum StringOrInt {
    String(String),
    Int(i64),
}

// Failures = {
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Failures {
    #[serde(rename = "ChangeTag")]
    pub change_tag: Option<u64>,
    #[serde(rename = "ChangeCBORTag")]
    pub change_cbortag: Option<u64>,
    #[serde(rename = "RemoveCBORTag")]
    pub remove_cbortag: Option<u64>,
    #[serde(rename = "ChangeProtected")]
    pub change_protected: Option<String>,
    #[serde(rename = "ChangeAttr")]
    pub change_attr: Option<BTreeMap<String, StringOrInt>>,
    #[serde(rename = "AddProtected")]
    pub add_protected: Option<BTreeMap<String, u64>>,
    #[serde(rename = "RemoveProtected")]
    pub remove_protected: Option<BTreeMap<String, u64>>,
}

// Outputs = {
//    ? cbor: bstr,                                # CBOR encoding in HEX
//    ? cbor_diag: tstr,                           # CBOR Diagnositc encoding
//    ? content : bstr                             # deatched content encoded in HEX
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Outputs {
    pub cbor: Option<String>,
    pub cbor_diag: Option<String>,
    pub content: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct IntermediateRecipients {
    #[serde(rename = "Content_hex")]
    pub content_hex: Option<String>,
    #[serde(rename = "Secret_hex")]
    pub secret_hex: Option<String>,
}

// Intermediates = {
//    ? "ToMax_hex": tstr,                         # Value to be MAC-ed encoded in HEX
//    ? "CEK_hex": tstr,                           # CEK used in MAC or encryption encoded in hex
//    ? "AAD_hex": tstr,                           # AEAD Additional Data encoded in hex
//    ? "recipients" : [ +{
//        ? "Context_hex": tstr,                   # Context structure encoded in hex
//        ? "Secret_hex" : tstr,                   # ECDH shared secret encoded in hex
//    }]
//    ]
// }
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Intermediates {
    #[serde(rename = "ToMax_hex")]
    pub to_max_hex: Option<String>,
    #[serde(rename = "CEK_hex")]
    pub cek_hex: Option<String>,
    #[serde(rename = "AAD_hex")]
    pub aad_hex: Option<String>,
    pub recipients: Option<Vec<IntermediateRecipients>>,
}

#[allow(dead_code)]
pub fn check_failures(failures: &Failures) {
    if failures.change_tag.is_none()
        && failures.change_attr.is_none()
        && failures.add_protected.is_none()
        && failures.remove_protected.is_none()
    {
        println!("FAILURES: {:?}", failures);
        panic!()
    }
}

#[allow(dead_code)]
pub fn walk_cose_wg_dir(cose_wg_dir: &str) -> Result<(), String> {
    for entry in WalkDir::new(cose_wg_dir) {
        match entry {
            Ok(e) => {
                let path = e.path();
                if e.file_type().is_dir() {
                    match path.to_str() {
                        Some(s) => {
                            if s != cose_wg_dir {
                                let r = walk_cose_wg_dir(s);
                                if r.is_err() {
                                    continue;
                                }
                            }
                        }
                        None => {
                            continue;
                        }
                    }
                } else {
                    let file_exts = vec!["json"];
                    if let Some(ext) = e.path().extension().and_then(OsStr::to_str) {
                        if !file_exts.contains(&ext) {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    let json_buf = get_file_as_byte_vec(e.path());
                    let ex: Result<Example, _> = serde_json::from_slice(json_buf.as_slice());
                    match ex {
                        Ok(ex) => {
                            println!("{:?} - {}", e.path(), ex.title);
                            let buf = match &ex.output.cbor {
                                Some(b) => hex::decode(b.to_ascii_lowercase()).unwrap(),
                                None => panic!(),
                            };
                            if ex.input.enveloped.is_some() {
                                let result: Result<TaggedCoseEncrypt, _> =
                                    from_reader(buf.as_slice());
                                match result {
                                    Ok(parsed) => {
                                        if let Some(some) = ex.fail {
                                            if some {
                                                match &ex.input.failures {
                                                    Some(failures) => {
                                                        check_failures(failures);
                                                    }
                                                    None => {}
                                                }
                                            }
                                        }
                                        let mut encoded = vec![];
                                        let _ = into_writer(&parsed, &mut encoded);
                                        if buf != encoded {
                                            // buffers may not match but parsed structures should
                                            let parsed2: TaggedCoseEncrypt =
                                                from_reader(encoded.as_slice()).unwrap();
                                            assert_eq!(parsed, parsed2);
                                        }
                                    }
                                    Err(_) => {
                                        if let Some(some) = ex.fail {
                                            if !some {
                                                panic!()
                                            }
                                        }
                                    }
                                }
                            } else if ex.input.encrypt.is_some() {
                                let result: Result<TaggedCoseEncrypt0, _> =
                                    from_reader(buf.as_slice());
                                match result {
                                    Ok(parsed) => {
                                        if let Some(some) = ex.fail {
                                            if some {
                                                match &ex.input.failures {
                                                    Some(failures) => {
                                                        check_failures(failures);
                                                    }
                                                    None => {}
                                                }
                                            }
                                        }
                                        let mut encoded = vec![];
                                        let _ = into_writer(&parsed, &mut encoded);
                                        if buf != encoded {
                                            // buffers may not match but parsed structures should
                                            let parsed2: TaggedCoseEncrypt0 =
                                                from_reader(encoded.as_slice()).unwrap();
                                            assert_eq!(parsed, parsed2);
                                        }
                                    }
                                    Err(_) => {
                                        if let Some(some) = ex.fail {
                                            if !some {
                                                panic!()
                                            }
                                        }
                                    }
                                }
                            } else if ex.input.mac.is_some() {
                                let result: Result<TaggedCoseMac, _> = from_reader(buf.as_slice());
                                match result {
                                    Ok(parsed) => {
                                        if let Some(some) = ex.fail {
                                            if some {
                                                match &ex.input.failures {
                                                    Some(failures) => {
                                                        check_failures(failures);
                                                    }
                                                    None => {}
                                                }
                                            }
                                        }
                                        let mut encoded = vec![];
                                        let _ = into_writer(&parsed, &mut encoded);
                                        if buf != encoded {
                                            // buffers may not match but parsed structures should
                                            let parsed2: TaggedCoseMac =
                                                from_reader(encoded.as_slice()).unwrap();
                                            assert_eq!(parsed, parsed2);
                                        }
                                    }
                                    Err(_) => {
                                        if let Some(some) = ex.fail {
                                            if !some {
                                                panic!()
                                            }
                                        }
                                    }
                                }
                            } else if ex.input.mac0.is_some() {
                                let result: Result<TaggedCoseMac0, _> = from_reader(buf.as_slice());
                                match result {
                                    Ok(parsed) => {
                                        if let Some(some) = ex.fail {
                                            if some {
                                                match &ex.input.failures {
                                                    Some(failures) => {
                                                        check_failures(failures);
                                                    }
                                                    None => {}
                                                }
                                            }
                                        }
                                        let mut encoded = vec![];
                                        let _ = into_writer(&parsed, &mut encoded);
                                        if buf != encoded {
                                            // buffers may not match but parsed structures should
                                            let parsed2: TaggedCoseMac0 =
                                                from_reader(encoded.as_slice()).unwrap();
                                            assert_eq!(parsed, parsed2);
                                        }
                                    }
                                    Err(_) => {
                                        if let Some(some) = ex.fail {
                                            if !some {
                                                panic!()
                                            }
                                        }
                                    }
                                }
                            } else if ex.input.sign.is_some() {
                                let result: Result<TaggedCoseSign, _> = from_reader(buf.as_slice());
                                match result {
                                    Ok(parsed) => {
                                        if let Some(some) = ex.fail {
                                            if some {
                                                match &ex.input.failures {
                                                    Some(failures) => {
                                                        check_failures(failures);
                                                    }
                                                    None => {}
                                                }
                                            }
                                        }
                                        let mut encoded = vec![];
                                        let _ = into_writer(&parsed, &mut encoded);
                                        if buf != encoded {
                                            // buffers may not match but parsed structures should
                                            let parsed2: TaggedCoseSign =
                                                from_reader(encoded.as_slice()).unwrap();
                                            assert_eq!(parsed, parsed2);
                                        }
                                    }
                                    Err(_) => {
                                        if let Some(some) = ex.fail {
                                            if !some {
                                                panic!()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            println!("FAILED: {:?} - {}", e.path(), err)
                        }
                    }
                }
            }
            _ => {
                panic!("Failed to unwrap directory entry while searching for JSON file");
            }
        }
    }
    Ok(())
}
