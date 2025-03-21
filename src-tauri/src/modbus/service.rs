use crate::errors::Result;
use crate::settings::types::ModbusSettings;
use rodbus::client::{Channel, RequestParam};
use rodbus::{AddressRange, Indexed};

use super::types::ModbusConnection;

#[derive(Debug, Clone)]
pub struct ModbusService<T: ModbusConnection> {
    client: T,
}

impl<T: ModbusConnection> ModbusService<T> {
    pub fn new(client: T) -> Self {
        ModbusService { client }
    }

    pub async fn connect(&self, settings: &ModbusSettings) -> Result<Channel> {
        let channel = self.client.connect(settings).await?;
        Ok(channel)
    }

    pub async fn disconnect(&self, channel: Channel) -> Result<()> {
        self.client.disconnect(channel).await?;
        Ok(())
    }

    pub async fn write_single_register(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        request: Indexed<u16>,
    ) -> Result<Indexed<u16>> {
        // Implement write_single_register method
        let response = self
            .client
            .write_single_register(channel, param, request)
            .await?;

        Ok(response)
    }

    pub async fn write_single_coil(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        request: Indexed<bool>,
    ) -> Result<Indexed<bool>> {
        // Implement write_single_coil method
        let response = self
            .client
            .write_single_coil(channel, param, request)
            .await?;

        Ok(response)
    }

    pub async fn read_holding_registers(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        address: AddressRange,
    ) -> Result<Vec<Indexed<u16>>> {
        // Implement read_holding_registers method
        let response = self
            .client
            .read_holding_registers(channel, param, address)
            .await?;

        Ok(response)
    }

    pub async fn read_coils(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        address: AddressRange,
    ) -> Result<Vec<Indexed<bool>>> {
        // Implement read_coils method
        let response = self.client.read_coils(channel, param, address).await?;

        Ok(response)
    }
}
