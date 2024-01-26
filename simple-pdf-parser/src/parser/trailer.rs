use crate::structs::Trailer;
use crate::Pdf;

impl Pdf {
    pub fn parse_trailer(&mut self) {
        let len = self.data.len();
        assert_eq!(self.data[len - 7..], *"\x0A%%EOF\x0A".as_bytes());

        let mut find_startxref_head = len - 8;
        while self.data[find_startxref_head] != 0x0A {
            find_startxref_head -= 1;
        }

        let xref_offset = str::parse::<u32>(
            &String::from_utf8(self.data[find_startxref_head + 1..len - 7].to_vec()).unwrap(),
        )
        .unwrap();
        assert_eq!(
            self.data[find_startxref_head - 12..find_startxref_head],
            *">>\x0Astartxref".as_bytes()
        );

        let mut find_trailer_begin = find_startxref_head - 13;
        while self.data[find_trailer_begin] != 0x3C || self.data[find_trailer_begin - 1] != 0x3C {
            assert!(
                self.data[find_trailer_begin] != 0x3E || self.data[find_trailer_begin - 1] != 0x3E
            );
            find_trailer_begin -= 1;
        }
        assert_eq!(
            self.data[find_trailer_begin - 9..find_trailer_begin - 1],
            *"trailer\x0A".as_bytes()
        );

        let mut find_encrypt_head = find_trailer_begin + 2;
        while self.data[find_encrypt_head] != 0x2F
            || self.data[find_encrypt_head + 1] != 0x45
            || self.data[find_encrypt_head + 2] != 0x6E
            || self.data[find_encrypt_head + 3] != 0x63
            || self.data[find_encrypt_head + 4] != 0x72
            || self.data[find_encrypt_head + 5] != 0x79
            || self.data[find_encrypt_head + 6] != 0x70
            || self.data[find_encrypt_head + 7] != 0x74
            || self.data[find_encrypt_head + 8] != 0x20
        {
            find_encrypt_head += 1;
        }

        let mut find_encrypt_obj_id_end = find_encrypt_head + 9;
        while self.data[find_encrypt_obj_id_end] != 0x20 {
            find_encrypt_obj_id_end += 1;
        }

        assert_eq!(
            self.data[find_encrypt_obj_id_end + 1..find_encrypt_obj_id_end + 5],
            *"0\x20R\x0A".as_bytes()
        );

        let encrypt_obj_id = str::parse::<u32>(
            &String::from_utf8(self.data[find_encrypt_head + 9..find_encrypt_obj_id_end].to_vec())
                .unwrap(),
        )
        .unwrap();
        self.trailer = Some(Trailer {
            encrypt_obj_id,
            xref_offset,
        });
    }
}

#[cfg(test)]
mod test {
    use crate::test::PDF_FILE;
    use crate::Pdf;

    #[test]
    fn test_trailer() {
        let mut pdf = Pdf::new(PDF_FILE);
        pdf.parse_trailer();
        assert_eq!(pdf.trailer.as_ref().unwrap().encrypt_obj_id, 16);
        assert_eq!(pdf.trailer.as_ref().unwrap().xref_offset, 4719);
    }
}
