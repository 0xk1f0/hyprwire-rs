#[derive(Debug)]
pub struct Code {}
impl Code {
    pub const HW_INVALID: u8 = 0x00;
    /*
        Sent by the client to initiate the handshake.
        Param 1: varchar -> with a value of "VAX"
    */
    pub const HW_SUP: u8 = 0x01;
    /*
        Sent by the server after a HELLO.
        Param 1: array<uint> -> Supported Versions
    */
    pub const HW_HANDSHAKE_BEGIN: u8 = 0x02;
    /*
        Sent by the client to confirm a choice of a protocol version
        Param 1: uint -> Version Number
    */
    pub const HW_HANDSHAKE_ACK: u8 = 0x03;
    /*
        Sent by the server to advertise supported protocols
        Param 1: array<varchar> -> Protocols
    */
    pub const HW_HANDSHAKE_PROTOCOLS: u8 = 0x04;
    /*
        Sent by the client to bind to a specific protocol spec
        Param 1: uint -> Sequence
        Param 2: str -> Protocol Specification
    */
    pub const HW_BIND_PROTOCOL: u8 = 0x0A;
    /*
        Sent by the server to acknowledge the bind and return a handle
        Param 1: uint -> Object Handle
        Param 2: uint -> Sequence
    */
    pub const HW_NEW_OBJECT: u8 = 0x0B;
    /*
        Sent by the server to indicate a fatal protocol error
        Param 1: uint -> Object Handle ID
        Param 2: uint -> Error IDX
        Param 3: varchar -> Error Message
    */
    pub const HW_FATAL_PROTOCOL_ERROR: u8 = 0x0C;
    /*
        Sent from the client to initiate a roundtrip.
        Params: uint -> Sequence
    */
    pub const HW_ROUNDTRIP_REQUEST: u8 = 0x0D;
    /*
        Sent from the server to finalize the roundtrip.
        Params: uint -> Sequence
    */
    pub const HW_ROUNDTRIP_DONE: u8 = 0x0E;
    /*
        Generic protocol message. Can be either direction.
        Param 1: uint -> Object Handle ID
        Param 2: uint -> Method ID
        Param ...: argument data
    */
    pub const HW_GENERIC_PROTOCOL_MESSAGE: u8 = 0x64;
}

#[derive(Debug)]
pub struct MagicType {}
impl MagicType {
    /*
        Signifies an end of a message
        Length: 1 Byte U8
    */
    pub const HW_END: u8 = 0x00;
    /*
        Primitive type unsigned Integer.
        Length: 4 Bytes U32
    */
    pub const HW_UINT: u8 = 0x10;
    /*
        Primitive type Integer.
        Length: 4 Bytes I32
    */
    pub const HW_INT: u8 = 0x11;
    /*
        Primitive type Float.
        Length: 4 Bytes F32
    */
    pub const HW_F32: u8 = 0x12;
    /*
        Primitive type Sequence Integer.
        Length: 4 Bytes U32
    */
    pub const HW_SEQ: u8 = 0x13;
    /*
        Primitive type Sequence Integer.
        Length: 4 Bytes U32
    */
    pub const HW_OBJECT_ID: u8 = 0x14;
    /*
        Variable length type String.
        Structure: [magic : 1B][len : VLQ][data : len B]
    */
    pub const HW_VARCHAR: u8 = 0x20;
    /*
        Variable length type Array.
        Structure: [magic : 1B][type : 1B][elements : VLQ][data...]

        NOTE: Array strips the magic byte from each element, as it's already contained.
    */
    pub const HW_ARRAY: u8 = 0x21;
    /*
        Variable length type Array.
        Structure: [magic : 1B][id : 4B][name_len : VLQ][object name ...]
    */
    pub const HW_OBJECT: u8 = 0x22;
    /*
        Special Type FD
        Structure: [magic : 1B]
    */
    pub const HW_FD: u8 = 0x40;
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
    ObjectId(u32),
}

#[derive(Debug)]
pub struct Message {
    pub code: u8,
    pub args: Vec<Value>,
}

#[derive(Debug)]
pub struct Protocol {
    pub spec: String,
    pub version: u32,
}
