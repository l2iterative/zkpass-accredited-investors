use crate::Pdf;

impl Pdf {
    pub fn find_stream(&self, offset: usize) -> Vec<u8> {
        let mut find_length = offset + 1;
        while self.data[find_length] != 0x2F
            || self.data[find_length + 1] != 0x4C
            || self.data[find_length + 2] != 0x65
            || self.data[find_length + 3] != 0x6E
            || self.data[find_length + 4] != 0x67
            || self.data[find_length + 5] != 0x74
            || self.data[find_length + 6] != 0x68
            || self.data[find_length + 7] != 0x20
        {
            find_length += 1;
        }

        let mut cur = find_length + 8;

        let read_a_number = |cur: &mut usize, data: &[u8], split: u8| {
            let start = *cur;
            while data[*cur] != split {
                *cur += 1;
            }
            let end = *cur;
            str::parse::<u32>(&String::from_utf8(data[start..end].to_vec()).unwrap()).unwrap()
        };

        let length = read_a_number(&mut cur, &self.data, 0x0A) as usize;

        let mut find_stream_start = cur;
        while self.data[find_stream_start] != 0x3E
            || self.data[find_stream_start + 1] != 0x3E
            || self.data[find_stream_start + 2] != 0x0A
            || self.data[find_stream_start + 3] != 0x73
            || self.data[find_stream_start + 4] != 0x74
            || self.data[find_stream_start + 5] != 0x72
            || self.data[find_stream_start + 6] != 0x65
            || self.data[find_stream_start + 7] != 0x61
            || self.data[find_stream_start + 8] != 0x6D
            || (self.data[find_stream_start + 9] != 0x0A
                && (self.data[find_stream_start + 9] != 0x0D
                    || self.data[find_stream_start + 10] != 0x0A))
        {
            find_stream_start += 1;
        }

        if self.data[find_stream_start + 9] == 0x0D {
            find_stream_start += 11;
        } else {
            find_stream_start += 10;
        }

        return self.data[find_stream_start..find_stream_start + length].to_vec();
    }
}
