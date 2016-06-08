enum MessageAttribute {
    MappedAddress,
    ResponseAddress,
    ChangeRequest,
    SourceAddress,
    ChangedAddress,
    Username,
    Password,
    MessageIntegrity,
    ErrorCode,
    UnknownAttributes,
    ReflectedFrom
}

pub struct StunPacket {
    data: Vec<u8>
}

impl StunPacket {
    fn set_transaction_id(&mut self, value: [u16; 8usize]) {
        {
            {
                let value = value[0usize];
                {
                    self.data[8usize] = (self.data[8usize] & !255u8) |
                                        ((value >> 4usize) as u8) & 255u8;
                    self.data[9usize] = (self.data[9usize] & !240u8) |
                                        ((value << 4usize) as u8) & 240u8;
                };
            }
            {
                let value = value[1usize];
                {
                    self.data[9usize] = (self.data[9usize] & !15u8) |
                                        ((value >> 8usize) as u8) & 15u8;
                    self.data[10usize] = (self.data[10usize] & !255u8) |
                                         ((value << 0usize) as u8) & 255u8;
                };
            }
            {
                let value = value[2usize];
                {
                    self.data[11usize] = (self.data[11usize] & !255u8) |
                                         ((value >> 4usize) as u8) & 255u8;
                    self.data[12usize] = (self.data[12usize] & !240u8) |
                                         ((value << 4usize) as u8) & 240u8;
                };
            }
            {
                let value = value[3usize];
                {
                    self.data[12usize] = (self.data[12usize] & !15u8) |
                                         ((value >> 8usize) as u8) & 15u8;
                    self.data[13usize] = (self.data[13usize] & !255u8) |
                                         ((value << 0usize) as u8) & 255u8;
                };
            }
            {
                let value = value[4usize];
                {
                    self.data[14usize] = (self.data[14usize] & !255u8) |
                                         ((value >> 4usize) as u8) & 255u8;
                    self.data[15usize] = (self.data[15usize] & !240u8) |
                                         ((value << 4usize) as u8) & 240u8;
                };
            }
            {
                let value = value[5usize];
                {
                    self.data[15usize] = (self.data[15usize] & !15u8) |
                                         ((value >> 8usize) as u8) & 15u8;
                    self.data[16usize] = (self.data[16usize] & !255u8) |
                                         ((value << 0usize) as u8) & 255u8;
                };
            }
            {
                let value = value[6usize];
                {
                    self.data[17usize] = (self.data[17usize] & !255u8) |
                                         ((value >> 4usize) as u8) & 255u8;
                    self.data[18usize] = (self.data[18usize] & !240u8) |
                                         ((value << 4usize) as u8) & 240u8;
                };
            }
            {
                let value = value[7usize];
                {
                    self.data[18usize] = (self.data[18usize] & !15u8) |
                                         ((value >> 8usize) as u8) & 15u8;
                    self.data[19usize] = (self.data[19usize] & !255u8) |
                                         ((value << 0usize) as u8) & 255u8;
                };
            }
        };
    }
    #[doc = "Creates a new `StunPacket`"]
    pub fn new(data: Vec<u8>) -> StunPacket {
        StunPacket { data: data }
    }
    fn get_marker(&self) -> u8 {
        ((self.data[0usize] >> 6usize) & 3u8) as u8
    }
    fn set_marker(&mut self, value: u8) {
        {
            self.data[0usize] = (self.data[0usize] & !192u8) | ((value << 6usize) as u8) & 192u8;
        };
    }
    fn get_m0(&self) -> u8 {
        ((self.data[0usize] >> 1usize) & 31u8) as u8
    }
    fn set_m0(&mut self, value: u8) {
        {
            self.data[0usize] = (self.data[0usize] & !62u8) | ((value << 1usize) as u8) & 62u8;
        };
    }
    fn get_c0(&self) -> u8 {
        (self.data[(7u64 / 8) as usize] & (1 << 0usize))
    }
    fn set_c0(&mut self, value: u8) {
        if value == 1 {
            self.data[0usize] |= 1u8
        } else {
            self.data[0usize] &= !(1u8)
        }
    }
    fn get_m1(&self) -> u8 {
        ((self.data[1usize] >> 5usize) & 7u8) as u8
    }
    fn set_m1(&mut self, value: u8) {
        {
            self.data[1usize] = (self.data[1usize] & !224u8) | ((value << 5usize) as u8) & 224u8;
        };
    }
    fn get_c1(&self) -> bool {
        (self.data[(11u64 / 8) as usize] & (1 << 4usize)) != 0
    }
    fn set_c1(&mut self, value: bool) {
        if value {
            self.data[1usize] |= 16u8
        } else {
            self.data[1usize] &= !(16u8)
        }
    }
    fn get_m2(&self) -> u8 {
        ((self.data[1usize] >> 0usize) & 15u8) as u8
    }
    fn set_m2(&mut self, value: u8) {
        {
            self.data[1usize] = (self.data[1usize] & !15u8) | ((value << 0usize) as u8) & 15u8;
        };
    }
    fn get_length(&self) -> u16 {
        (((self.data[2usize] >> 0usize) & 255u8) as u16) << 8usize |
        (((self.data[3usize] >> 0usize) & 255u8) as u16)
    }
    fn set_length(&mut self, value: u16) {
        {
            self.data[2usize] = (self.data[2usize] & !255u8) | ((value >> 8usize) as u8) & 255u8;
            self.data[3usize] = (self.data[3usize] & !255u8) | ((value << 0usize) as u8) & 255u8;
        };
    }
    fn get_magic_cookie(&self) -> u32 {
        (((((self.data[4usize] >> 0usize) & 255u8) as u32) << 8usize |
          (((self.data[5usize] >> 0usize) & 255u8) as u32)) << 8usize |
         (((self.data[6usize] >> 0usize) & 255u8) as u32)) << 8usize |
        (((self.data[7usize] >> 0usize) & 255u8) as u32)
    }
    fn set_magic_cookie(&mut self, value: u32) {
        {
            self.data[4usize] = (self.data[4usize] & !255u8) | ((value >> 24usize) as u8) & 255u8;
            self.data[5usize] = (self.data[5usize] & !255u8) | ((value >> 16usize) as u8) & 255u8;
            self.data[6usize] = (self.data[6usize] & !255u8) | ((value >> 8usize) as u8) & 255u8;
            self.data[7usize] = (self.data[7usize] & !255u8) | ((value << 0usize) as u8) & 255u8;
        };
    }
    fn get_transaction_id(&self) -> [u16; 8usize] {
        [(((self.data[8usize] >> 0usize) & 255u8) as u16) << 4usize |
         (((self.data[9usize] >> 4usize) & 15u8) as u16),
         (((self.data[9usize] >> 0usize) & 15u8) as u16) << 8usize |
         (((self.data[10usize] >> 0usize) & 255u8) as u16),
         (((self.data[11usize] >> 0usize) & 255u8) as u16) << 4usize |
         (((self.data[12usize] >> 4usize) & 15u8) as u16),
         (((self.data[12usize] >> 0usize) & 15u8) as u16) << 8usize |
         (((self.data[13usize] >> 0usize) & 255u8) as u16),
         (((self.data[14usize] >> 0usize) & 255u8) as u16) << 4usize |
         (((self.data[15usize] >> 4usize) & 15u8) as u16),
         (((self.data[15usize] >> 0usize) & 15u8) as u16) << 8usize |
         (((self.data[16usize] >> 0usize) & 255u8) as u16),
         (((self.data[17usize] >> 0usize) & 255u8) as u16) << 4usize |
         (((self.data[18usize] >> 4usize) & 15u8) as u16),
         (((self.data[18usize] >> 0usize) & 15u8) as u16) << 8usize |
         (((self.data[19usize] >> 0usize) & 255u8) as u16)]
    }
}
pub struct StunMethod {
    data: Vec<u8>
}
impl StunMethod {
    fn set_method(&mut self, value: u16) {
        {
            self.data[0usize] = (self.data[0usize] & !255u8) | ((value >> 4usize) as u8) & 255u8;
            self.data[1usize] = (self.data[1usize] & !240u8) | ((value << 4usize) as u8) & 240u8;
        };
    }
    #[doc = "Creates a new `StunMethod`"]
    pub fn new(data: Vec<u8>) -> StunMethod {
        StunMethod { data: data }
    }
    fn get_method(&self) -> u16 {
        // self.data[1usize] as u16
        (((self.data[0usize] >> 0usize) & 255u8) as u16) << 4usize |
        (((self.data[1usize] << 4usize) & 15u8) as u16)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::fs::File;
    use std::io::Read;
    #[test]
    fn it_works() {
        let mut f = File::open("/home/xeno/daz.bin").unwrap();
        let mut stun_header_raw = vec![];
        f.read_to_end(&mut stun_header_raw).unwrap();
//        let stun_header_raw = vec![0,1,0,88,33,18,164,66,183,231,167,1,188,52,214,134,250,135,223,174];
        let stun_header = StunPacket::new(stun_header_raw);
        assert_eq!(0, stun_header.get_marker());
        assert_eq!(0, stun_header.get_m0());
        assert_eq!(0, stun_header.get_m1());
        assert_eq!(1, stun_header.get_m2());
        assert_eq!(56, stun_header.get_length());
        assert_eq!(0x2112A442, stun_header.get_magic_cookie());
        let stun_method = StunMethod::new(vec![stun_header.get_m0(), stun_header.get_m1() | stun_header.get_m2()]);
        assert_eq!(1, stun_method.get_method());
    }
}
