extern crate rustc_serialize;
use rustc_serialize::json;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}



#[derive(RustcDecodable, RustcEncodable)]
struct Message {
    channel: str,
    user: str,
    text: str,
    ts: str,
}