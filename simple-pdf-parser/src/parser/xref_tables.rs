use crate::Pdf;

impl Pdf {
    pub fn find_encrypt_object(&mut self) {
        let xref_offset = self.trailer.as_ref().unwrap().xref_offset;
        let encrypt_obj_id = self.trailer.as_ref().unwrap().encrypt_obj_id;

        let read_a_number = |cur: &mut usize, data: &[u8]| {
            let start = *cur;
            while data[*cur] != 0x0A {
                *cur += 1;
            }
            let end = *cur;
            str::parse::<u32>(&String::from_utf8(data[start..end].to_vec()).unwrap()).unwrap()
        };

        let mut cur = (xref_offset + 5) as usize;
        loop {
            let subsection_start_obj_id = read_a_number(&mut cur, &self.data);
            cur += 1;
            let subsection_num_objects = read_a_number(&mut cur, &self.data);
            cur += 1;

            if subsection_start_obj_id + subsection_num_objects < encrypt_obj_id {
                // skip the entire subsections
                cur += subsection_num_objects * 20;
            } else {
                // encrypt_obj_id will be here
                let skip = encrypt_obj_id - subsection_start_obj_id;
                cur += skip * 20;

                let encrypt_offset = read_a_number(&mut cur, &self.data);
                let encrypt_generation = read_a_number(&mut cur, &self.data);
            }
        }
    }
}
