#[derive(Debug)]
pub struct Code {}
impl Code {
    pub const END: u8 = 0x00;
    pub const SUP: u8 = 0x01;
    pub const HANDSHAKE_BEGIN: u8 = 0x02;
    pub const HANDSHAKE_ACK: u8 = 0x03;
    pub const HANDSHAKE_PROTOCOLS: u8 = 0x04;
    pub const BIND_PROTOCOL: u8 = 0x0A;
    pub const NEW_OBJECT: u8 = 0x0B;
    pub const FATAL_PROTOCOL_ERROR: u8 = 0x0C;
    pub const ROUNDTRIP_REQUEST: u8 = 0x0D;
    pub const ROUNDTRIP_DONE: u8 = 0x0E;
    pub const GENERIC_PROTOCOL_MESSAGE: u8 = 0x64;
}

#[derive(Debug)]
pub struct Type {}
impl Type {
    pub const UINT: u8 = 0x10;
    pub const INT: u8 = 0x11;
    pub const F32: u8 = 0x12;
    pub const SEQ: u8 = 0x13;
    pub const OBJECT_ID: u8 = 0x14;
    pub const VARCHAR: u8 = 0x20;
    pub const ARRAY: u8 = 0x21;
    pub const OBJECT: u8 = 0x22;
}

#[derive(Debug)]
pub enum Value {
    Uint(u32),
    Int(i32),
    Float(f32),
    Seq(u32),
    ObjId(u32),
    Varchar(String),
    ArrayUint(Vec<u32>),
    ArrayVarchar(Vec<String>),
    Object((u32, String)),
}

#[derive(Debug)]
pub struct Message {
    pub code: u8,
    pub args: Vec<Value>,
}
