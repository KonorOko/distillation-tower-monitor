use rodbus::client::*;
use rodbus::*;

use crate::errors::Result;
use crate::settings::types::ModbusSettings;

pub trait ModbusConnection {
    fn new() -> Self;
    async fn connect(&self, settings: &ModbusSettings) -> Result<Channel>;
    async fn disconnect(&self, channel: Channel) -> Result<()>;
    async fn read_coils(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        range: AddressRange,
    ) -> Result<Vec<Indexed<bool>>>;
    async fn read_holding_registers(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        range: AddressRange,
    ) -> Result<Vec<Indexed<u16>>>;
    async fn write_single_coil(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        request: Indexed<bool>,
    ) -> Result<Indexed<bool>>;
    async fn write_single_register(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        request: Indexed<u16>,
    ) -> Result<Indexed<u16>>;
}
