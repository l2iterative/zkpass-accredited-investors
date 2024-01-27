use crate::structs::EncryptObj;
use crate::Pdf;

impl Pdf {
    pub fn read_encrypt_object(&mut self) {
        let mut find_owner_start = self.encrypt_xref_entry.as_ref().unwrap().encrypt_offset + 10;
        while self.data[find_owner_start] != 0x2F
            || self.data[find_owner_start + 1] != 0x4F
            || self.data[find_owner_start + 2] != 0x20
            || self.data[find_owner_start + 3] != 0x3C
        {
            find_owner_start += 1;
        }

        let mut owner_password = [0u8; 64];
        owner_password.copy_from_slice(&self.data[find_owner_start + 4..find_owner_start + 68]);

        assert_eq!(self.data[find_owner_start + 68], 0x3E);

        let mut find_permission_start =
            self.encrypt_xref_entry.as_ref().unwrap().encrypt_offset + 10usize;
        while self.data[find_permission_start] != 0x2F
            || self.data[find_permission_start + 1] != 0x50
            || self.data[find_permission_start + 2] != 0x20
        {
            find_permission_start += 1;
        }

        let mut find_permission_end = find_permission_start + 3;
        while self.data[find_permission_end] != 0x0A {
            find_permission_end += 1;
        }

        let permission = str::parse::<i64>(
            &String::from_utf8(self.data[find_permission_start + 3..find_permission_end].to_vec())
                .unwrap(),
        )
        .unwrap() as i32;
        self.encrypt_obj = Some(EncryptObj {
            owner_password,
            permission,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::structs::{EncryptXrefEntry, Trailer};
    use crate::test::PDF_FILE;
    use crate::Pdf;

    #[test]
    fn test_encrypt_obj() {
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
        pdf.read_encrypt_object();
        assert_eq!(
            pdf.encrypt_obj.as_ref().unwrap().owner_password.as_slice(),
            b"03f54f9bcd3f4d417d56aef191bce332c2aaabec0e1ed90a5d92182935e4d6e8",
        );
        assert_eq!(
            pdf.encrypt_obj.as_ref().unwrap().permission as u32,
            4294967292u32
        );
    }
}
