use serde::{
    Deserialize, Serialize,
};
use std::{ops::Deref, time::{Duration, Instant}};

#[derive(Serialize, Deserialize)]
pub struct Timestamped<T> {
    timestamp: Duration,
    value: T,
}

// TODO serialize implementation for Timestamped<T : Serialize>
// impl<T> Serialize for Timestamped<T>
// where
//     T: Serialize,
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut stru = serializer.serialize_struct("Timestamped", 2)?;
//         stru.serialize_field("timestamp", &self.timestamp)?;
//         stru.serialize_field("value", &self.value)?;
//         stru.end()
//     }
// }
//TODO deserialize implementation for Timestamped<T : Deserialize>

impl<T> Deref for Timestamped<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> Timestamped<T> {
    pub fn new(timestamp: Duration, value: T) -> Self {
        Self { timestamp, value }
    }

    pub fn now(base_time : Instant, value : T) -> Self {
        Self {
            timestamp : Instant::now().duration_since(base_time),
            value,
        }
    }

    pub fn timestamp(&self) -> &Duration {
        &self.timestamp
    }

    pub fn destruct(self) -> (Duration, T) {
        (self.timestamp, self.value)
    }

    pub fn new_value<R>(&self, value: R) -> Timestamped<R> {
        Timestamped::new(*self.timestamp(), value)
    }
}

pub type TCanFrame = Timestamped<CanFrame>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanFrame {
    id: u32,
    dlc: u8,
    data: u64,
}

enum CanFrameIdFlags {
    IdeMask = 0x80000000,
    RtrMask = 0x40000000,
    ExtMask = 0x1FFFFFFF,
}

impl CanFrame {
    /// Least significant byte of data attribute corresponds to first byte of data field in CAN message.
    /// Just think about it as a char-array.
    /// Bit order within each byte should not be a concern
    /// (bits not really addressable in a byte-addressable system).
    /// Just think about it as least significant bit in each byte is also
    /// least significant bit in CAN message and at receiver.
    pub fn new(id: u32, ide: bool, rtr: bool, dlc: u8, data: u64) -> Self {
        Self {
            id: id
                | (if ide {
                    CanFrameIdFlags::IdeMask as u32
                } else {
                    0x0u32
                })
                | (if rtr {
                    CanFrameIdFlags::RtrMask as u32
                } else {
                    0x0u32
                }),
            dlc,
            data,
        }
    }

    pub fn key(&self) -> u32 {
        self.id
    }

    #[allow(unused)]
    pub fn get_id(&self) -> u32 {
        self.id & CanFrameIdFlags::ExtMask as u32
    }
    #[allow(unused)]
    pub fn get_ide_flag(&self) -> bool {
        (self.id & CanFrameIdFlags::IdeMask as u32) != 0
    }
    #[allow(unused)]
    pub fn get_rtr_flag(&self) -> bool {
        (self.id & CanFrameIdFlags::RtrMask as u32) != 0
    }
    #[allow(unused)]
    pub fn get_dlc(&self) -> u8 {
        self.dlc
    }
    pub fn get_data_u64(&self) -> u64 {
        self.data
    }
    #[allow(dead_code)]
    pub fn get_data_8u8(&self) -> [u8; 8] {
        unsafe { std::mem::transmute::<u64, [u8; 8]>(self.data) }
    }
}

// impl Serialize for CanFrame {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut map = serializer.serialize_map(Some(5))?;
//         let id = self.id & CanFrameIdFlags::ExtMask as u32;
//         map.serialize_entry("id", &id)?;
//         let ide = (self.id & CanFrameIdFlags::IdeMask as u32) != 0;
//         map.serialize_entry("ide", &ide)?;
//         let rtr = (self.id & CanFrameIdFlags::RtrMask as u32) != 0;
//         map.serialize_entry("rtr", &rtr)?;
//         map.serialize_entry("dlc", &self.dlc)?;
//         map.serialize_entry("data", &self.data)?;
//         map.end()
//     }
// }

pub type TCanError = Timestamped<CanError>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanError(pub u64);

impl CanError {
    pub fn erno(&self) -> u64 {
        self.0
    }
}

// impl Serialize for CanError {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut map = serializer.serialize_map(Some(3))?;
//
//         let bits = self.0;
//         map.serialize_entry("data", &bits)?;
//         if bits & 1 != 0 {
//             map.serialize_entry("name", "CAN Bit Error")?;
//             map.serialize_entry("description", "Wtf i didn't send that shit")?;
//         } else if bits & 2 != 0 {
//             map.serialize_entry("name", "CAN Bit Stuffing Error")?;
//             map.serialize_entry("description", "Whhyy is everybody sending bullshit!")?;
//         } else if bits & 4 != 0 {
//             map.serialize_entry("name", "CAN Form Error")?;
//             map.serialize_entry(
//                 "description",
//                 "Somebody in this network is to stupid to follow CAN standards!",
//             )?;
//         } else if bits & 8 != 0 {
//             map.serialize_entry("name", "CAN ACK Error")?;
//             map.serialize_entry("description", "Wait what CAN has ACK!")?;
//         } else if bits & 16 != 0 {
//             map.serialize_entry("name", "CAN CRC Error")?;
//             map.serialize_entry("description", "Some CRC was computed incorrectly!")?;
//         } else {
//             map.serialize_entry("name", "Internal Error")?;
//             map.serialize_entry("description", "The CNL fucked up some how!")?;
//         }
//         map.end()
//     }
// }
