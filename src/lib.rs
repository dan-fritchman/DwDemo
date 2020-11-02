
#[allow(unused_imports)] // These are used by the macro-expanded code
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use prost::Message;

// Include the prost-expanded proto-file content
include!(concat!(env!("OUT_DIR"), "/dwdemo.rs"));

