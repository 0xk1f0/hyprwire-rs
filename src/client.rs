use std::collections::HashMap;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

use crate::wire;

/// The hyprwire client
pub struct HyprWireClient {
    stream: UnixStream,
    sequence: u32,
    objects: HashMap<u32, String>,
}

impl HyprWireClient {
    fn parse_argument(&mut self, magic: u8) -> Result<wire::Value, String> {
        match magic {
            wire::Type::UINT => {
                let mut buf = [0u8; 4];
                self.stream
                    .read_exact(&mut buf)
                    .map_err(|e| e.to_string())?;
                let val = u32::from_le_bytes(buf);
                Ok(wire::Value::Uint(val))
            }
            wire::Type::INT => {
                let mut buf = [0u8; 4];
                self.stream
                    .read_exact(&mut buf)
                    .map_err(|e| e.to_string())?;
                let val = i32::from_le_bytes(buf);
                Ok(wire::Value::Int(val))
            }
            wire::Type::F32 => {
                let mut buf = [0u8; 4];
                self.stream
                    .read_exact(&mut buf)
                    .map_err(|e| e.to_string())?;
                let val = f32::from_le_bytes(buf);
                Ok(wire::Value::Float(val))
            }
            wire::Type::OBJECT_ID => {
                let mut buf = [0u8; 4];
                self.stream
                    .read_exact(&mut buf)
                    .map_err(|e| e.to_string())?;
                let val = u32::from_le_bytes(buf);
                Ok(wire::Value::ObjId(val))
            }
            wire::Type::SEQ => {
                let mut buf = [0u8; 4];
                self.stream
                    .read_exact(&mut buf)
                    .map_err(|e| e.to_string())?;
                let val = u32::from_le_bytes(buf);
                Ok(wire::Value::Seq(val))
            }
            wire::Type::VARCHAR => {
                let mut len_buf = [0u8; 1];
                self.stream
                    .read_exact(&mut len_buf)
                    .map_err(|e| e.to_string())?;
                let len = u8::from_le_bytes(len_buf) as usize;

                let mut str_buf = vec![0u8; len];
                self.stream
                    .read_exact(&mut str_buf)
                    .map_err(|e| e.to_string())?;

                let s = String::from_utf8(str_buf).map_err(|e| e.to_string())?;
                Ok(wire::Value::Varchar(s))
            }
            wire::Type::ARRAY => {
                let mut type_buf = [0u8; 1];
                self.stream
                    .read_exact(&mut type_buf)
                    .map_err(|e| e.to_string())?;
                let array_type = u8::from_le_bytes(type_buf);

                let mut count_buf = [0u8; 1];
                self.stream
                    .read_exact(&mut count_buf)
                    .map_err(|e| e.to_string())?;
                let count = u8::from_le_bytes(count_buf);

                match array_type {
                    wire::Type::VARCHAR => {
                        let mut vec = Vec::with_capacity(count as usize);

                        for _ in 0..count {
                            let mut len_buf = [0u8; 1];
                            self.stream
                                .read_exact(&mut len_buf)
                                .map_err(|e| e.to_string())?;
                            let len = u8::from_le_bytes(len_buf) as usize;

                            let mut str_buf = vec![0u8; len];
                            self.stream
                                .read_exact(&mut str_buf)
                                .map_err(|e| e.to_string())?;

                            let s = String::from_utf8(str_buf).map_err(|e| e.to_string())?;
                            vec.push(s);
                        }

                        return Ok(wire::Value::ArrayVarchar(vec));
                    }
                    wire::Type::UINT => {
                        let mut vec = Vec::with_capacity(count as usize);

                        for _ in 0..count {
                            let mut val_buf = [0u8; 4];
                            self.stream
                                .read_exact(&mut val_buf)
                                .map_err(|e| e.to_string())?;
                            vec.push(u32::from_le_bytes(val_buf));
                        }

                        return Ok(wire::Value::ArrayUint(vec));
                    }
                    _ => {
                        return Err(format!("Unknown type: {:#02x}", array_type));
                    }
                }
            }
            wire::Type::OBJECT => {
                let mut buf = [0u8; 4];
                self.stream
                    .read_exact(&mut buf)
                    .map_err(|e| e.to_string())?;
                let val = u32::from_le_bytes(buf);
                Ok(wire::Value::ObjId(val))
            }
            _ => Err(format!("Unknown magic byte: {:#02x}", magic)),
        }
    }

    /// Connect to a hyprwire-enabled Unix Socket
    pub fn connect(path: &str) -> Result<Self, String> {
        let stream = UnixStream::connect(path).map_err(|e| e.to_string())?;
        Ok(Self {
            stream,
            sequence: 0,
            objects: HashMap::new(),
        })
    }

    /// Send a hyprwire message
    pub fn send_message(
        &mut self,
        code: u8,
        args: &[wire::Value],
        sequenced: bool,
    ) -> Result<(), String> {
        let mut buffer = Vec::new();

        buffer.push(code);

        for arg in args {
            match arg {
                wire::Value::Uint(val) => {
                    buffer.push(wire::Type::UINT);
                    buffer.extend_from_slice(&val.to_le_bytes());
                }
                wire::Value::Int(val) => {
                    buffer.push(wire::Type::INT);
                    buffer.extend_from_slice(&val.to_le_bytes());
                }
                wire::Value::Float(val) => {
                    buffer.push(wire::Type::F32);
                    buffer.extend_from_slice(&val.to_le_bytes());
                }
                wire::Value::Seq(val) => {
                    buffer.push(wire::Type::SEQ);
                    buffer.extend_from_slice(&val.to_le_bytes());
                }
                wire::Value::ObjId(val) => {
                    buffer.push(wire::Type::OBJECT_ID);
                    buffer.extend_from_slice(&val.to_le_bytes());
                }
                wire::Value::Varchar(val) => {
                    buffer.push(wire::Type::VARCHAR);
                    let len = val.len() as u8;
                    buffer.extend_from_slice(&len.to_le_bytes());
                    buffer.extend_from_slice(val.as_bytes());
                }
                wire::Value::ArrayUint(vals) => {
                    buffer.push(wire::Type::ARRAY);
                    let count = vals.len() as u8;
                    buffer.extend_from_slice(&count.to_le_bytes());
                    for v in vals {
                        buffer.extend_from_slice(&v.to_le_bytes());
                    }
                }
                wire::Value::ArrayVarchar(vals) => {
                    buffer.push(wire::Type::ARRAY);
                    let count = vals.len() as u8;
                    buffer.extend_from_slice(&count.to_le_bytes());
                    for v in vals {
                        let len = v.len() as u8;
                        buffer.extend_from_slice(&len.to_le_bytes());
                        buffer.extend_from_slice(v.as_bytes());
                    }
                }
                wire::Value::Object(val) => {
                    buffer.push(wire::Type::OBJECT);
                    buffer.extend_from_slice(&val.0.to_le_bytes());
                }
            }
        }

        buffer.push(wire::Code::END);

        self.stream.write_all(&buffer).map_err(|e| e.to_string())?;
        self.stream.flush().map_err(|e| e.to_string())?;

        if sequenced {
            self.sequence += 1;
        }

        Ok(())
    }

    /// Read a hyprwire message
    pub fn read_message(&mut self) -> Result<wire::Message, String> {
        let mut code_buf = [0u8; 1];
        self.stream
            .read_exact(&mut code_buf)
            .map_err(|e| e.to_string())?;
        let code = code_buf[0];
        let mut args = Vec::new();

        loop {
            let mut magic_buf = [0u8; 1];
            self.stream
                .read_exact(&mut magic_buf)
                .map_err(|e| e.to_string())?;
            let magic = magic_buf[0];

            if magic == wire::Code::END {
                break;
            }

            let val = self.parse_argument(magic).map_err(|e| e.to_string())?;
            args.push(val);
        }

        Ok(wire::Message { code, args })
    }

    /// Perform the hyprwire handshake procedure
    pub fn perform_handshake(&mut self, version: u32) -> Result<Vec<String>, String> {
        self.send_message(
            wire::Code::SUP,
            &[wire::Value::Varchar("VAX".to_string())],
            false,
        )?;

        let msg = self.read_message()?;
        if msg.code != wire::Code::HANDSHAKE_BEGIN {
            return Err(format!("Expected HANDSHAKE_BEGIN, got {:#02x}", msg.code));
        }

        let supported = match msg.args.get(0) {
            Some(wire::Value::ArrayUint(arr)) => arr,
            _ => {
                return Err("Invalid HANDSHAKE_BEGIN args".to_string());
            }
        };

        if !supported.contains(&version) {
            return Err(format!(
                "Server does not support protocol version {}",
                version
            ));
        }

        self.send_message(wire::Code::HANDSHAKE_ACK, &[wire::Value::Uint(1)], false)?;

        let msg = self.read_message()?;
        if msg.code != wire::Code::HANDSHAKE_PROTOCOLS {
            return Err(format!(
                "Expected HANDSHAKE_PROTOCOLS, got {:#02x}",
                msg.code
            ));
        }

        let protocols = match msg.args.get(0) {
            Some(wire::Value::ArrayVarchar(arr)) => arr.to_owned(),
            _ => {
                return Err("Invalid HANDSHAKE_PROTOCOLS args".to_string());
            }
        };

        Ok(protocols)
    }

    /// Bind to a hyprwire protocol
    pub fn bind_protocol(&mut self, protocol: String) -> Result<(), String> {
        let spec = protocol
            .split_once("@")
            .ok_or(format!("Invalid Protocol String: {}", protocol))?
            .0;
        self.send_message(
            wire::Code::BIND_PROTOCOL,
            &[
                wire::Value::Uint(self.sequence),
                wire::Value::Varchar(spec.to_string()),
                wire::Value::Uint(1),
            ],
            true,
        )?;

        return Ok(());
    }
}
