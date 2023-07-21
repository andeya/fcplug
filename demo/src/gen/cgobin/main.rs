use demo::gen::{Client, GoFfi, SearchRequest};
use demo::Test;
use fcplug::protobuf::PbMessage;
use fcplug::TryIntoTBytes;

fn main() {
    let req = SearchRequest {
        query: "query abc".to_string(),
        page_number: 10,
        result_per_page: 30,
    }.try_into_tbytes::<PbMessage<_>>().unwrap();
    let cli = unsafe { Test::search_client::<Client>(req).unwrap() };
    println!("{:?}", cli);
}
