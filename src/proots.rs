// pRoots Data Structure
//
// A DNA Sequence
// {"Type": "sequence",
//  "Seq": "AATCGATCGATGCTAGTAGATTACGTA",
//  "Addr": "asdasdasdasdasdasdasdasd",
//  "Annots" [Cid, Cid, Cid, Cid]}
//
// An Annotation
// {"Type": "annotation",
//  "Addr": "qweqwewqeqweqweqweqwe",
//  "From": 20,
//  "End": 25,
//  "Comment": "The segment looks very important!"}

use futures::TryStreamExt;
use ipfs_api_backend_actix::{IpfsApi, IpfsClient};
use libipld::multibase::Base;
use libipld::prelude::*;
use libipld::{ipld, json, Cid, Ipld};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Sequence {
    address: String,
    sequence: String,
    annotations: Vec<Annotation>,
}

impl Sequence {
    pub fn new(dag: &Ipld) -> Self {
        match dag {
            Ipld::Map(dag) => Sequence {
                address: match dag.get("Addr").unwrap() {
                    Ipld::String(s) => s,
                    _ => panic!("Not a String"),
                }
                .clone(),
                sequence: match dag.get("Seq").unwrap() {
                    Ipld::String(s) => s,
                    _ => panic!("Not a String"),
                }
                .clone(),
                annotations: match dag.get("Annots").unwrap() {
                    Ipld::List(l) => {
                        let mut vec = Vec::new();
                        for cid in l {
                            match cid {
                                Ipld::Link(cid) => {
                                    // TODO: wrap it as a fn and block_on the future
                                    async {
                                        let client = IpfsClient::default();
                                        let cid_str = cid.to_string_of_base(Base::Base64).unwrap();
                                        match client
                                            .dag_get(&cid_str)
                                            .map_ok(|chunk| chunk.to_vec())
                                            .try_concat()
                                            .await
                                        {
                                            Ok(bytes) => {
                                                let ipld_annot: Ipld =
                                                    json::DagJsonCodec.decode(&bytes).unwrap();
                                                let annot = Annotation::new(&ipld_annot);
                                                vec.push(annot);
                                            }
                                            Err(e) => {
                                                eprintln!("error reading dag node: {}", e);
                                            }
                                        }
                                    };
                                }
                                _ => panic!("Not a Link"),
                            }
                        }
                        vec
                    }
                    // vec![Annotation {
                    // address: "addr".to_string(),
                    // from: 0,
                    // end: 10,
                    // comment: "This is a note".to_string(),}],
                    _ => panic!("Not a List"),
                },
            },
            _ => Sequence {
                address: "addr".to_string(),
                sequence: "seq".to_string(),
                annotations: vec![Annotation {
                    address: "addr".to_string(),
                    from: 12,
                    end: 15,
                    comment: "not a map".to_string(),
                }],
            },
        }
        //TODO: fn add_notation(&mut self) -> bool;
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Annotation {
    address: String,
    from: usize,
    end: usize,
    comment: String,
}

impl Annotation {
    pub fn new(annot: &Ipld) -> Self {
        match annot {
            Ipld::Map(annot) => Annotation {
                address: match annot.get("Addr").unwrap() {
                    Ipld::String(s) => s,
                    _ => panic!("Not a String"),
                }
                .clone(),
                from: match annot.get("from").unwrap() {
                    Ipld::Integer(i) => *i as usize,
                    _ => panic!("Not a Integer"),
                },
                end: match annot.get("end").unwrap() {
                    Ipld::Integer(i) => *i as usize,
                    _ => panic!("Not a Integer"),
                },
                comment: match annot.get("comment").unwrap() {
                    Ipld::String(s) => s,
                    _ => panic!("Not a String"),
                }
                .clone(),
            },
            _ => Annotation {
                address: "addr".to_string(),
                from: 0,
                end: 10,
                comment: "This is a note".to_string(),
            },
        }
    }
}
