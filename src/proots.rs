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
//  "Cmt": "The segment looks very important!"}

use crate::ipfs_portal::IpfsPortal;
use actix_rt::System;
use libipld::multibase::Base;
use libipld::prelude::*;
use libipld::{ipld, json, Ipld};
use std::convert::From;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Sequence {
    // The address of the sequence in IPFS
    address: String,
    // The actual DNA sequence
    sequence: String,
    // A vector of annotations for the sequence
    annotations: Vec<Annotation>,
}

impl From<&Ipld> for Sequence {
    fn from(dag: &Ipld) -> Self {
        match dag {
            Ipld::Map(dag) => Sequence {
                // Extract the address from the IPLD object
                address: match dag.get("Addr").unwrap() {
                    Ipld::String(s) => s,
                    _ => panic!("Not a String"),
                }
                .clone(),
                // Extract the DNA sequence from the IPLD object
                sequence: match dag.get("Seq").unwrap() {
                    Ipld::String(s) => s,
                    _ => panic!("Not a String"),
                }
                .clone(),
                // Extract the annotations from the IPLD object
                annotations: match dag.get("Annots").unwrap() {
                    Ipld::List(l) => {
                        let system = System::new();
                        let mut vec = Vec::new();
                        for cid in l {
                            match cid {
                                Ipld::Link(cid) => {
                                    let cid_str = cid.to_string_of_base(Base::Base64).unwrap();
                                    vec.push(IpfsPortal::get(cid_str));
                                }
                                _ => panic!("Not a Link"),
                            }
                        }
                        let fut = async move { futures::future::join_all(vec).await };
                        let vec = system.block_on(fut);

                        vec.iter()
                            .map(|ipld_annot| Annotation::from(ipld_annot))
                            .collect()
                    }
                    _ => panic!("Not a List"),
                },
            },
            _ => panic!("Not a Map"),
        }
    }
}

impl Sequence {
    // Create a new instance of `Sequence`, a dummy currently
    pub fn new() -> Self {
        Sequence {
            address: "addr".to_string(),
            sequence: "seq".to_string(),
            annotations: vec![Annotation {
                address: "addr".to_string(),
                from: 0,
                end: 0,
                comment: "not a map".to_string(),
            }],
        }
    }

    //TODO: add notation implementation
    pub fn add_notation(
        &mut self,
        address: String,
        from: usize,
        end: usize,
        comment: String,
    ) -> bool {
        false
    }

    pub fn to_ipld(&self) -> Ipld {
        let mut vec = Vec::new();

        let system = System::new();
        for annot in &self.annotations {
            let ipld_annot = annot.to_ipld();
            let ipld_annot_encoded = json::DagJsonCodec.encode(&ipld_annot).unwrap();
            vec.push(IpfsPortal::upload(ipld_annot_encoded));
        }
        let fut = async move { futures::future::join_all(vec).await };
        let vec = system.block_on(fut);

        ipld!({
            "Type": "sequence".to_string(),
            "Addr": self.address.clone(),
            "Seq": self.sequence.clone(),
            "Annots": vec
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Annotation {
    address: String,
    from: usize,
    end: usize,
    comment: String,
}

impl From<&Ipld> for Annotation {
    fn from(annot: &Ipld) -> Self {
        match annot {
            Ipld::Map(annot) => Annotation {
                address: match annot.get("Addr").unwrap() {
                    Ipld::String(s) => s,
                    _ => panic!("Not a String"),
                }
                .clone(),
                from: match annot.get("From").unwrap() {
                    Ipld::Integer(i) => *i as usize,
                    _ => panic!("Not a Integer"),
                },
                end: match annot.get("End").unwrap() {
                    Ipld::Integer(i) => *i as usize,
                    _ => panic!("Not a Integer"),
                },
                comment: match annot.get("Cmt").unwrap() {
                    Ipld::String(s) => s,
                    _ => panic!("Not a String"),
                }
                .clone(),
            },
            _ => panic!("Not a Map"),
        }
    }
}

impl Annotation {
    // dummy instance
    pub fn new() -> Self {
        Annotation {
            address: "addr".to_string(),
            from: 0,
            end: 10,
            comment: "This is a note".to_string(),
        }
    }

    pub fn to_ipld(&self) -> Ipld {
        ipld!({
            "Type": "annotation".to_string(),
            "Addr": self.address.clone(),
            "From": self.from,
            "End": self.end,
            "Cmt": self.comment.clone()
        })
    }
}
