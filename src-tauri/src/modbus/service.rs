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

    pub async fn disconnect(&self, channel: &mut Channel) -> Result<()> {
        self.client.disconnect(channel).await?;
        Ok(())
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
}
