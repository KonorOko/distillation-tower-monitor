use std::time::Duration;

use super::types::ModbusConnection;
use crate::errors::{ModbusError, Result};
use crate::settings::types::ModbusSettings;
use rodbus::client::*;
use rodbus::*;

#[derive(Debug, Clone)]
pub struct ModbusClient;

impl ModbusClient {
    pub fn new() -> Self {
        ModbusClient
    }
}

impl ModbusConnection for ModbusClient {
    fn new() -> Self {
        ModbusClient
    }

    async fn connect(&self, settings: &ModbusSettings) -> Result<Channel> {
        let modbus_settings = SerialSettings {
            baud_rate: settings.baudrate,
            data_bits: rodbus::DataBits::Eight,
            stop_bits: rodbus::StopBits::One,
            parity: rodbus::Parity::None,
            flow_control: rodbus::FlowControl::None,
        };

        let max_queue_size = 1;
        let retry_strategy = default_retry_strategy();
        let decode = DecodeLevel::default();
        let listener = None;

        let channel = client::spawn_rtu_client_task(
            &settings.usb_port,
            modbus_settings,
            max_queue_size,
            retry_strategy,
            decode,
            listener,
        );

        channel
            .enable()
            .await
            .map_err(|e| ModbusError::ConnectionError(e.to_string()))?;

        Ok(channel)
    }

    async fn disconnect(&self, channel: Channel) -> Result<()> {
        // Implementation of disconnect method
        channel
            .disable()
            .await
            .map_err(|e| ModbusError::ConnectionError(e.to_string()))?;
        Ok(())
    }

    async fn read_coils(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        range: AddressRange,
    ) -> Result<Vec<Indexed<bool>>> {
        // Implementation of read_coils method
        let result = channel
            .read_coils(param, range)
            .await
            .map_err(|e| ModbusError::ReadCoilsError(e.to_string()))?;

        Ok(result)
    }

    async fn read_holding_registers(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        range: AddressRange,
    ) -> Result<Vec<Indexed<u16>>> {
        // Implementation of read_holding_registers method
        let result = channel
            .read_holding_registers(param, range)
            .await
            .map_err(|e| ModbusError::ReadHoldingRegistersError(e.to_string()))?;

        Ok(result)
    }

    async fn write_single_coil(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        request: Indexed<bool>,
    ) -> Result<Indexed<bool>> {
        // Implementation of write_single_coil method
        let result = channel
            .write_single_coil(param, request)
            .await
            .map_err(|e| ModbusError::WriteSingleCoilError(e.to_string()))?;
        Ok(result)
    }

    async fn write_single_register(
        &self,
        channel: &mut Channel,
        param: RequestParam,
        request: Indexed<u16>,
    ) -> Result<Indexed<u16>> {
        // Implementation of write_single_register method
        let result = channel
            .write_single_register(param, request)
            .await
            .map_err(|e| ModbusError::WriteSingleRegisterError(e.to_string()))?;
        Ok(result)
    }
}
