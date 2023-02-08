mod ipfs_portal;
mod proots;

use libipld::multibase::Base;
use libipld::multihash::{Code, MultihashDigest};
use libipld::prelude::*;
use libipld::{ipld, json, Cid, Ipld};
use structopt::StructOpt;

// TODO: make it for real case API, what parameters we need? how to integrete it with blockchain
// such as Filecoin?
#[derive(Debug, StructOpt)]
struct Opt {
    /// Test flag
    #[structopt(short, long)]
    flag: bool,

    /// Test str input
    #[structopt(short, long)]
    str_input: String,

    /// Test optional parameters
    #[structopt(name = "PARA")]
    parameters: Vec<String>,
}

// TODO: need to clear up the main section and implement the pRoots command-line API
fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    let ipld_map = ipld!({"bool": opt.flag, "string": opt.str_input});
    let ipld_map_encoded = json::DagJsonCodec.encode(&ipld_map).unwrap();
    let ipld_decoded: Ipld = json::DagJsonCodec.decode(&ipld_map_encoded).unwrap();
    println!("{:?} {:?}", ipld_map, ipld_decoded);
    println!("{:02x?}", ipld_map_encoded);
    let digest = Code::Blake3_256.digest(&ipld_map_encoded);
    let cid = Cid::new_v1(u64::from(json::DagJsonCodec), digest);
    let byte_arr = cid.to_bytes();
    println!("{} {:?}", cid, byte_arr);
    println!(
        "encoded string {}",
        std::str::from_utf8(&ipld_map_encoded).unwrap()
    );

    // let sq = proots::Sequence::from(&ipld_map);
    // println!("{:?}", sq);

    // test for IpfsApi/IpfsClient
    let fut = ipfs_portal::IpfsPortal::upload(ipld_map_encoded);
    let sys = actix_rt::System::new();
    let res: Ipld = sys.block_on(fut);
    println!("{:?}", res);
    match res {
        Ipld::Link(s) => {
            println!("{}", s);
            let s = s.to_string_of_base(Base::Base64).unwrap();
            let fut = ipfs_portal::IpfsPortal::get(s);
            let res2 = sys.block_on(fut);
            println!("{:?}", res2);
        }
        _ => (),
    }
}
