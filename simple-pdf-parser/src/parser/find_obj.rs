use crate::Pdf;

impl Pdf {
    pub fn find_specific_obj(&self, obj_id: u32) -> usize {
        let xref_offset = self.trailer.as_ref().unwrap().xref_offset;

        let read_a_number = |cur: &mut usize, data: &[u8], split: u8| {
            let start = *cur;
            while data[*cur] != split {
                *cur += 1;
            }
            let end = *cur;
            str::parse::<u32>(&String::from_utf8(data[start..end].to_vec()).unwrap()).unwrap()
        };

        let mut cur = (xref_offset + 5) as usize;
        loop {
            let subsection_start_obj_id = read_a_number(&mut cur, &self.data, 0x20);
            cur += 1;
            let subsection_num_objects = read_a_number(&mut cur, &self.data, 0x0A);
            cur += 1;

            if subsection_start_obj_id + subsection_num_objects < obj_id {
                // skip the entire subsections
                cur += (subsection_num_objects * 20) as usize;
            } else {
                // obj_id will be here
                let skip = obj_id - subsection_start_obj_id;
                cur += (skip * 20) as usize;

                let offset = read_a_number(&mut cur, &self.data, 0x20) as usize;
                assert_eq!(self.data[cur + 1..cur + 8], *"00000\x20n".as_bytes());

                return offset;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::structs::Trailer;
    use crate::test::PDF_FILE;
    use crate::Pdf;

    #[test]
    fn test_find_obj() {
        let mut pdf = Pdf::new(PDF_FILE);
        pdf.trailer = Some(Trailer {
            encrypt_obj_id: 16,
            document_id: (*b"3235663230333732663332343562303364373435323136306466313963313738")
                .to_vec(),
            xref_offset: 4719,
        });
        assert_eq!(pdf.find_specific_obj(14), 1579);
    }
}
