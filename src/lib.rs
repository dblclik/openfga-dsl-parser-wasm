use std::{ffi::CString, os::raw::c_char};

use openfga_dsl_parser::*;

static mut DOC: String = String::new();

#[no_mangle]
pub fn dsl_to_json(offset: *mut u8, len: usize) -> *mut c_char {
    let input_buf = unsafe {
        let buf: &mut [u8] = core::slice::from_raw_parts_mut(offset, len);
        buf
    };
    let mut parser = Parser::new(std::str::from_utf8(input_buf).unwrap());
    let doc = parser.parse_document().unwrap();

    let json = json::JsonTransformer::new(&doc).serialize();
    unsafe {
        DOC = json.clone();
    }
    CString::new(json).unwrap().into_raw()
}

#[no_mangle]
pub fn doc_len() -> usize {
    unsafe { DOC.len() }
}

#[no_mangle]
pub fn set_at(ptr: *mut i32, byte: i32, overwrite: u32) -> u8 {
    unsafe {
        let buf: &mut [i32] = core::slice::from_raw_parts_mut(ptr, 1);
        if buf[0] > 0 {
            if overwrite == 0 {
                return 0;
            }
        }
        buf[0] = byte;
    }
    return 1;
}

// use wasm_bindgen::prelude::*;
// // use wasmedge_bindgen_macro::*;
// use openfga_dsl_parser::{Parser, json::JsonTransformer};

// #[wasm_bindgen]
// pub fn dsl_to_json(input: String) -> Result<String, String> {
//     let mut parser = Parser::new(&input);
//     let doc = parser
//         .parse_document()
//         .map_err(|e| e.to_string())?;

//     let json = JsonTransformer::new(&doc)
//         .serialize();
//     Ok(json)
// }

// #[cfg(test)]
// mod tests {
//     use serde_json::Value;
//     use super::*;
//     #[test]
//     fn parser_test() {
//         let i = "type team
//   relations
//     define member as self
// type repo
//   relations
//     define admin as self or repo_admin from owner
//     define maintainer as self or admin
//     define owner as self
//     define reader as self or triager or repo_reader from owner
//     define triager as self or writer
//     define writer as self or maintainer or repo_writer from owner
// type org
//   relations
//     define billing_manager as self or owner
//     define member as self or owner
//     define owner as self
//     define repo_admin as self
//     define repo_reader as self
//     define repo_writer as self
// type app
//   relations
//     define app_manager as self or owner from owner
//     define owner as self";
//     let exp_raw = r#"{
//   "type_definitions": [
//     {
//       "type": "team",
//       "relations": {
//         "member": {
//           "this": {}
//         }
//       }
//     },
//     {
//       "type": "repo",
//       "relations": {
//         "admin": {
//           "union": {
//             "child": [
//               {
//                 "this": {}
//               },
//               {
//                 "tupleToUserset": {
//                   "tupleset": {
//                     "object": "",
//                     "relation": "owner"
//                   },
//                   "computedUserset": {
//                     "object": "",
//                     "relation": "repo_admin"
//                   }
//                 }
//               }
//             ]
//           }
//         },
//         "maintainer": {
//           "union": {
//             "child": [
//               {
//                 "this": {}
//               },
//               {
//                 "computedUserset": {
//                   "object": "",
//                   "relation": "admin"
//                 }
//               }
//             ]
//           }
//         },
//         "owner": {
//           "this": {}
//         },
//         "reader": {
//           "union": {
//             "child": [
//               {
//                 "this": {}
//               },
//               {
//                 "computedUserset": {
//                   "object": "",
//                   "relation": "triager"
//                 }
//               },
//               {
//                 "tupleToUserset": {
//                   "tupleset": {
//                     "object": "",
//                     "relation": "owner"
//                   },
//                   "computedUserset": {
//                     "object": "",
//                     "relation": "repo_reader"
//                   }
//                 }
//               }
//             ]
//           }
//         },
//         "triager": {
//           "union": {
//             "child": [
//               {
//                 "this": {}
//               },
//               {
//                 "computedUserset": {
//                   "object": "",
//                   "relation": "writer"
//                 }
//               }
//             ]
//           }
//         },
//         "writer": {
//           "union": {
//             "child": [
//               {
//                 "this": {}
//               },
//               {
//                 "computedUserset": {
//                   "object": "",
//                   "relation": "maintainer"
//                 }
//               },
//               {
//                 "tupleToUserset": {
//                   "tupleset": {
//                     "object": "",
//                     "relation": "owner"
//                   },
//                   "computedUserset": {
//                     "object": "",
//                     "relation": "repo_writer"
//                   }
//                 }
//               }
//             ]
//           }
//         }
//       }
//     },
//     {
//       "type": "org",
//       "relations": {
//         "billing_manager": {
//           "union": {
//             "child": [
//               {
//                 "this": {}
//               },
//               {
//                 "computedUserset": {
//                   "object": "",
//                   "relation": "owner"
//                 }
//               }
//             ]
//           }
//         },
//         "member": {
//           "union": {
//             "child": [
//               {
//                 "this": {}
//               },
//               {
//                 "computedUserset": {
//                   "object": "",
//                   "relation": "owner"
//                 }
//               }
//             ]
//           }
//         },
//         "owner": {
//           "this": {}
//         },
//         "repo_admin": {
//           "this": {}
//         },
//         "repo_reader": {
//           "this": {}
//         },
//         "repo_writer": {
//           "this": {}
//         }
//       }
//     },
//     {
//       "type": "app",
//       "relations": {
//         "app_manager": {
//           "union": {
//             "child": [
//               {
//                 "this": {}
//               },
//               {
//                 "tupleToUserset": {
//                   "tupleset": {
//                     "object": "",
//                     "relation": "owner"
//                   },
//                   "computedUserset": {
//                     "object": "",
//                     "relation": "owner"
//                   }
//                 }
//               }
//             ]
//           }
//         },
//         "owner": {
//           "this": {}
//         }
//       }
//     }
//   ]
// }"#;
//     let exp: Value = serde_json::from_str(exp_raw).unwrap();

//     let doc = dsl_to_json(i.to_string()).unwrap();

//     assert_eq!(exp.to_string(), doc);
//     }
// }
