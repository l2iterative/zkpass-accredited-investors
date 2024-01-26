use libflate::zlib::Decoder;
use std::io;

fn main() {
    let mut input = io::stdin();
    let mut decoder = Decoder::new(&mut input).unwrap();
    io::copy(&mut decoder, &mut io::stdout()).unwrap();
}
