use rodbus::client::*;
use rodbus::*;

use crate::errors::Result;
use crate::settings::types::ModbusSettings;

pub trait ModbusConnection {
    async fn connect(&self, settings: &ModbusSettings) -> Result<Channel>;
    async fn disconnect(&self, channel: &mut Channel) -> Result<()>;
    async fn read_holding_registers(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        range: AddressRange,
    ) -> Result<Vec<Indexed<u16>>>;
}
