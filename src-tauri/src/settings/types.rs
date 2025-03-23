use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModbusSettings {
    pub usb_port: String,
    pub baudrate: u32,
    pub initial_address: u16,
    pub count: u16,
    pub timeout: u64,
    pub unit_id: u8,
}

impl Default for ModbusSettings {
    fn default() -> Self {
        Self {
            usb_port: String::default(),
            baudrate: 9600,
            initial_address: 100,
            count: 2,
            timeout: 1000,
            unit_id: 10,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub modbus: ModbusSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            modbus: ModbusSettings::default(),
        }
    }
}
