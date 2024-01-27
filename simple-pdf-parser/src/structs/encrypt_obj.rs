#[derive(Clone)]
pub struct EncryptObj {
    pub owner_password: [u8; 64],
    pub permission: i32,
}
