use futures::TryStreamExt;
use ipfs_api_backend_actix::{IpfsApi, IpfsClient};
use libipld::prelude::*;
use libipld::{json, Cid, Ipld};
use std::io::Cursor;
use std::str::FromStr;

pub struct IpfsPortal;

impl IpfsPortal {
    pub async fn get(cid: String) -> Ipld {
        let client = IpfsClient::default();
        // default codec: dag-json
        match client
            .dag_get(&cid)
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await
        {
            Ok(bytes) => json::DagJsonCodec.decode(&bytes).unwrap(),
            Err(e) => panic!("{}", e),
        }
    }

    pub async fn upload(dag: Vec<u8>) -> Ipld {
        let dag_node = Cursor::new(dag);
        let client = IpfsClient::default();
        let cid = client
            .dag_put(dag_node)
            .await
            .expect("error adding dag node");

        Ipld::Link(Cid::from_str(&cid.cid.cid_string).unwrap())
    }
}
