use crate::KeyManager;

impl KeyManager {
    pub fn compute_workspace_key(&mut self) {
        let mut buf = Vec::<u8>::new();
        buf.extend_from_slice(&[
            0x28, 0xBF, 0x4E, 0x5E, 0x4E, 0x75, 0x8A, 0x41, 0x64, 0x00, 0x4E, 0x56, 0xFF, 0xFA,
            0x01, 0x08, 0x2E, 0x2E, 0x00, 0xB6, 0xD0, 0x68, 0x3E, 0x80, 0x2F, 0x0C, 0xA9, 0xFE,
            0x64, 0x53, 0x69, 0x7A,
        ]);
        buf.extend_from_slice(
            &hex::decode(
                self.pdf_rc
                    .borrow()
                    .encrypt_obj
                    .as_ref()
                    .unwrap()
                    .owner_password
                    .as_slice(),
            )
            .unwrap(),
        );

        let permissions = self
            .pdf_rc
            .borrow()
            .encrypt_obj
            .as_ref()
            .unwrap()
            .permission;
        buf.push((permissions & 0xff) as u8);
        buf.push(((permissions >> 8) & 0xff) as u8);
        buf.push(((permissions >> 16) & 0xff) as u8);
        buf.push(((permissions >> 24) & 0xff) as u8);

        buf.extend_from_slice(
            &hex::decode(
                self.pdf_rc
                    .borrow()
                    .trailer
                    .as_ref()
                    .unwrap()
                    .document_id
                    .as_slice(),
            )
            .unwrap(),
        );

        let mut workspace_key = md5::compute(buf);

        // assume that we ae dealing with the version that md5 50 times
        for _ in 0..50 {
            workspace_key = md5::compute(*workspace_key);
        }

        self.workspace_key = Some(*workspace_key);
    }
}

#[cfg(test)]
mod test {
    use simple_pdf_parser::structs::{EncryptObj, EncryptXrefEntry, Trailer};

    pub static PDF_FILE: &[u8] = include_bytes!("../../../samples/encrypted-pdf-sample.pdf");

    use crate::{KeyManager, Pdf};

    #[test]
    fn test_workspace_key() {
        let mut pdf = Pdf::new(PDF_FILE);
        pdf.trailer = Some(Trailer {
            encrypt_obj_id: 16,
            document_id: (*b"3235663230333732663332343562303364373435323136306466313963313738")
                .to_vec(),
            xref_offset: 4719,
        });
        pdf.encrypt_xref_entry = Some(EncryptXrefEntry {
            encrypt_offset: 4503,
        });
        pdf.encrypt_obj = Some(EncryptObj {
            owner_password: *b"03f54f9bcd3f4d417d56aef191bce332c2aaabec0e1ed90a5d92182935e4d6e8",
            permission: 4294967292u32 as i32,
        });

        let mut key_manager = KeyManager::new(&pdf);
        key_manager.compute_workspace_key();

        assert_eq!(
            key_manager.workspace_key.unwrap().as_slice(),
            hex::decode("a11d74ea25fa38ef25d854725490128c").unwrap()
        );
    }
}
