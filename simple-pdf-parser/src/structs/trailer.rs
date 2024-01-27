#[derive(Clone)]
pub struct Trailer {
    pub encrypt_obj_id: u32,
    pub document_id: Vec<u8>,
    pub xref_offset: usize,
}

impl Default for Trailer {
    fn default() -> Self {
        Self {
            encrypt_obj_id: 0,
            document_id: vec![],
            xref_offset: 0,
        }
    }
}
