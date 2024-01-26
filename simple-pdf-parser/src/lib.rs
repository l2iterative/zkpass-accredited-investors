use crate::structs::Trailer;

mod parser;
mod structs;

#[derive(Default, Clone)]
pub struct Pdf {
    pub data: Vec<u8>,
    pub trailer: Option<Trailer>,
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
    pub(crate) static PDF_FILE: &[u8] = include_bytes!("../../post-enc.pdf");
}
