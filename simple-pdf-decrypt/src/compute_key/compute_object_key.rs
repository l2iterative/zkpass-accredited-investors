use crate::KeyManager;

impl KeyManager {
    pub fn compute_object_key(&self, object_id: u32, generation_id: u32) -> [u8; 16] {
        let mut buf = self.workspace_key.as_ref().unwrap().to_vec();

        buf.push((object_id & 0xff) as u8);
        buf.push(((object_id >> 8) & 0xff) as u8);
        buf.push(((object_id >> 16) & 0xff) as u8);

        buf.push((generation_id & 0xff) as u8);
        buf.push(((generation_id >> 8) & 0xff) as u8);

        *md5::compute(buf)
    }
}
