use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureAddress {
    pub top: u16,
    pub bottom: u16,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModbusSettings {
    pub usb_port: String,
    pub baudrate: u32,
    pub temperature_address: TemperatureAddress,
    pub count: u16,
    pub timeout: u64,
    pub unit_id: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub modbus: ModbusSettings,
    pub number_plates: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            modbus: ModbusSettings::default(),
            number_plates: 1,
        }
    }
}
