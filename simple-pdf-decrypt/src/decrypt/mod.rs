use crate::KeyManager;
use rc4::{consts::*, KeyInit, StreamCipher};
use rc4::{Key, Rc4};

impl KeyManager {
    pub fn decrypt(workspace_key: &[u8; 16], encrypted_data: &[u8]) -> Vec<u8> {
        let key = Key::<U16>::from_slice(workspace_key);
        let mut cipher = Rc4::<_>::new(key);
        let mut data = encrypted_data.to_vec();
        cipher.apply_keystream(data.as_mut_slice());

        return data;
    }
}

#[cfg(test)]
mod test {
    use simple_pdf_parser::structs::{EncryptObj, EncryptXrefEntry, Trailer};

    pub static PDF_FILE: &[u8] = include_bytes!("../../../samples/encrypted-pdf-sample.pdf");

    use crate::{KeyManager, Pdf};

    #[test]
    fn test_decrypt() {
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
            permission: (4294967292u32) as i32,
        });

        let mut key_manager = KeyManager::new(&pdf);
        key_manager.compute_workspace_key();

        let ciphertext_offset = pdf.find_specific_obj(5);
        let ciphertext = pdf.find_stream(ciphertext_offset);

        let plaintext = KeyManager::decrypt(&key_manager.compute_object_key(5, 0), &ciphertext);
        assert_eq!(
            plaintext[0..8],
            *b"\x78\x9c\x55\x8d\x3f\x0b\x02\x31".as_slice()
        )
    }
}
