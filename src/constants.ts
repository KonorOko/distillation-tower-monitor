import { SettingsType } from "./types";

export const DEFAULT_SETTINGS: SettingsType = {
  modbus: {
    initialAddress: 100,
    usbPort: "",
    unitId: 10,
    baudrate: 9600,
    count: 1,
    timeout: 1000,
  },
};

export const MAX_DATA_LENGTH = 120;
