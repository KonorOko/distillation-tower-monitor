import { SettingsType } from "./types";

export const DEFAULT_SETTINGS: SettingsType = {
  modbus: {
    temperatureAddress: {
      top: 101,
      bottom: 100,
    },
    usbPort: "",
    unitId: 10,
    baudrate: 9600,
    count: 1,
    timeout: 1000,
  },
  numberPlates: 1,
};

export const MAX_DATA_LENGTH = 120;
