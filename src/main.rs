#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate tokio;

use std::str;
use std::path::Path;
use tokio::fs::{self, DirEntry};
use tokio_stream::{self as stream};
use futures::{self, Stream, StreamExt};
use curl::easy::Easy;
use streaming_iterator::StreamingIterator;
use databento_defs::record::{TickMsg,TradeMsg};
use dbz_lib::Dbz;
use scylla::{Session,SessionConfig,SessionBuilder};
use scylla::prepared_statement::PreparedStatement;
use uuid::Uuid;

fn get_path_from_directory() {}

static CREATE_KEYSPACE_QUERY: &str = r#"
    CREATE KEYSPACE IF NOT EXISTS soyfutures
        WITH REPLICATION = {
            'class': 'SimpleStrategy',
            'replication_factor': 1
        };
"#;

static CREATE_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS soyfutures.contracts (
    commodity Text,
    id_uuid uuid,
    hd BigInt,
    price BigInt,
    size BigInt,
    action BigInt,
    side BigInt,
    flags BigInt,
    depth BigInt,
    ts_recv BigInt,
    ts_in_delta BigInt,
    sequence BigInt,
    PRIMARY KEY(commodity, id_uuid)
    );
"#;


pub async fn visit(
    paths: &str,
) -> Result<(Vec<DirEntry>), std::io::Error>{
    let mut paths = Path::new(paths);
    print!("{:?}", paths);
    let mut entries = fs::read_dir(paths).await.unwrap();
    let count = 0;
    let mut pths: Vec<DirEntry> = Vec::new();
    while let Some(entry) = entries.next_entry().await.unwrap() {
        pths.push(entry);
    }
    Ok(pths)
}

#[tokio::main]
async fn main() {
    let url: &str = "209.127.152.40:21";
    let ext: &str = "/home/yakaman/GLBX-20221130-VPX9AXX459/";
    let path_vec = visit(ext).await.unwrap();
    let session: Session = SessionBuilder::new()
        .known_node("172.104.21.214:9042")
        .build()
        .await.unwrap();
    session.query(CREATE_KEYSPACE_QUERY, &[]).await.unwrap();
    session.query(CREATE_TABLE_QUERY, &[]).await.unwrap();
    let trade_prepare: PreparedStatement = session.prepare("INSERT INTO soyfutures.contracts (commodity, id_uuid, hd, price, size, action, side, flags, depth, ts_recv, ts_in_delta, sequence) VALUES (? , ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)").await.unwrap();
    let mut counter = 0;
    println!("done create keyspace and table");
    for entry in path_vec.iter() {
        counter += 1;
        println!("{:?}", entry.path());
        println!("{:?}", counter);
        let mut dbz = Dbz::from_file(entry.path()).unwrap();
        let mut commodity = dbz.metadata().dataset.to_string().clone();
        let mut dbz_trades = dbz.try_into_iter::<TradeMsg>().unwrap();
        while let Some(trade) = dbz_trades.next() {
            let hd: i64 = i64::try_from(trade.hd.ts_event).unwrap();
            let price: i64 = i64::from(trade.price);
            let size: i64 = i64::from(trade.size);
            let action: i64 = i64::from(trade.action);
            let side: i64 = i64::from(trade.side);
            let flags: i64 = i64::from(trade.flags);
            let depth: i64 = i64::from(trade.depth);
            let ts_recv: i64 = i64::try_from(trade.ts_recv).unwrap();
            let ts_in_delta: i64 = i64::from(trade.ts_in_delta);
            let sequence: i64 = i64::from(trade.sequence);
            let id_uuid = Uuid::new_v4();
            println!("{},{},{},{},{},{},{},{},{},{},{},{},entry path: {}, counter: {}, commodity, id_uuid, price, size, action, hd, side, flags, depth, ts_recv, ts_in_delta, sequence);
            session.execute(&trade_prepare,(&commodity, id_uuid, hd, price, size, action, side, flags, depth, ts_recv, ts_in_delta, sequence)).await.unwrap();
        }
    }
}