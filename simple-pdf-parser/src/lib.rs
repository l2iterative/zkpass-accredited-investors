use crate::structs::{EncryptObj, EncryptXrefEntry, Trailer};

mod parser;
pub mod structs;

#[derive(Default, Clone)]
pub struct Pdf {
    pub data: Vec<u8>,
    pub trailer: Option<Trailer>,
    pub encrypt_xref_entry: Option<EncryptXrefEntry>,
    pub encrypt_obj: Option<EncryptObj>,
}

impl Pdf {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    pub static PDF_FILE: &[u8] = include_bytes!("../../samples/encrypted-pdf-sample.pdf");
}
