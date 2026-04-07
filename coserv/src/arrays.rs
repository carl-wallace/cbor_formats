//! Array-based structs from the CoSERV specification (draft-ietf-rats-coserv-05)

use ciborium::{cbor, value::Value};
use core::{fmt, marker::PhantomData};
use serde::ser::Error as OtherError;
use serde::{Deserialize, Serialize};
use serde::{de::Error, de::Visitor};

use alloc::{vec, vec::Vec};

use alloc::format;
use alloc::string::{String, ToString};
use cbor_derive::StructToArray;

use corim::choices::{GroupIdTypeChoice, InstanceIdTypeChoice};
use corim::maps::{ClassMap, ClassMapCbor, MeasurementMap, MeasurementMapCbor};

// stateful-class = [
//   class: comid.class-map
//   ? measurements: [+ comid.measurement-map]
// ]

/// The `stateful-class` type from [CoSERV Section 4.1].
///
/// ```text
/// stateful-class = [
///   class: comid.class-map
///   ? measurements: [+ comid.measurement-map]
/// ]
/// ```
///
/// [CoSERV Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.1
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct StatefulClass {
    #[cbor(value = "Map", cbor = "true")]
    pub class: ClassMap,
    #[cbor(value = "Array", cbor = "true")]
    pub measurements: Option<Vec<MeasurementMap>>,
}

// stateful-instance = [
//   instance: comid.$instance-id-type-choice
//   ? measurements: [+ comid.measurement-map]
// ]

/// The `stateful-instance` type from [CoSERV Section 4.1].
///
/// ```text
/// stateful-instance = [
///   instance: comid.$instance-id-type-choice
///   ? measurements: [+ comid.measurement-map]
/// ]
/// ```
///
/// [CoSERV Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.1
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct StatefulInstance {
    pub instance: InstanceIdTypeChoice,
    #[cbor(value = "Array", cbor = "true")]
    pub measurements: Option<Vec<MeasurementMap>>,
}

// stateful-group = [
//   group: comid.$group-id-type-choice
//   ? measurements: [+ comid.measurement-map]
// ]

/// The `stateful-group` type from [CoSERV Section 4.1].
///
/// ```text
/// stateful-group = [
///   group: comid.$group-id-type-choice
///   ? measurements: [+ comid.measurement-map]
/// ]
/// ```
///
/// [CoSERV Section 4.1]: https://datatracker.ietf.org/doc/html/draft-ietf-rats-coserv-05#section-4.1
#[derive(Clone, Debug, PartialEq, StructToArray, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct StatefulGroup {
    pub group: GroupIdTypeChoice,
    #[cbor(value = "Array", cbor = "true")]
    pub measurements: Option<Vec<MeasurementMap>>,
}
