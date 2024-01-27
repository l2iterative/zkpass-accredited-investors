use crate::structs::EncryptXrefEntry;
use crate::Pdf;

impl Pdf {
    pub fn find_encrypt_object(&mut self) {
        let encrypt_offset = self.find_specific_obj(self.trailer.as_ref().unwrap().encrypt_obj_id);
        self.encrypt_xref_entry = Some(EncryptXrefEntry { encrypt_offset });
    }
}

#[cfg(test)]
mod test {
    use crate::structs::Trailer;
    use crate::test::PDF_FILE;
    use crate::Pdf;

    #[test]
    fn test_xref_tables() {
        let mut pdf = Pdf::new(PDF_FILE);
        pdf.trailer = Some(Trailer {
            encrypt_obj_id: 16,
            document_id: (*b"3235663230333732663332343562303364373435323136306466313963313738")
                .to_vec(),
            xref_offset: 4719,
        });
        pdf.find_encrypt_object();
        assert_eq!(
            pdf.encrypt_xref_entry.as_ref().unwrap().encrypt_offset,
            4503
        );
    }
}
