extern crate ib_async;
extern crate bytes;
extern crate chrono;
extern crate failure;
extern crate futures;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;

use ib_async::domain;
use ib_async::message::request::*;
use ib_async::TwsClientBuilder;
use futures::{Future, Stream};
use std::net::SocketAddr;
use std::string::ToString;

fn main() {
    let port = std::env::args().nth(1).unwrap_or("".to_string());
    let port = port.parse::<u32>().unwrap_or(7497);
    let addr = format!("{}:{}", "127.0.0.1", port);
    let addr = addr.parse::<SocketAddr>().unwrap();
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let builder = TwsClientBuilder::new(0);
    let apple = domain::contract::Contract::new_stock("AAPL", "SMART", "USD").unwrap();
    let eur_gbp = domain::contract::Contract::new_forex("EUR.GBP").unwrap();
    let stock_request = Request::ReqMktData(ReqMktData {
        req_id: 1000,
        contract: apple,
        generic_tick_list: "".to_string(),
        snapshot: false,
        regulatory_snapshot: false,
        mkt_data_options: Vec::new(),
    });

    let forex_request = Request::ReqMktData(ReqMktData {
        req_id: 1001,
        contract: eur_gbp,
        generic_tick_list: "".to_string(),
        snapshot: false,
        regulatory_snapshot: false,
        mkt_data_options: Vec::new(),
    });

    let client = builder
        .connect(addr, 0)
        .map_err(|e| eprintln!("Read Error: {:?}", e))
        .map(move |c| c)
        .and_then(move |c| {
            println!("version:{}", c.server_version);
            c.send_request(stock_request);
            c.send_request(forex_request);
            c.for_each(move |buf| {
                println!("buf: {:?}", buf);
                Ok(())
            })
        });

    rt.spawn(client);

    rt.shutdown_on_idle().wait().unwrap();
}