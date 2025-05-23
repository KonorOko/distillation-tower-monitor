export type SettingsType = {
  modbus: {
    usbPort: string;
    baudrate: number;
    initialAddress: number;
    count: number;
    timeout: number;
    unitId: number;
  };
};

export type SettingsContextType = {
  settings: SettingsType;
  setSettings: React.Dispatch<React.SetStateAction<SettingsType>>;
};

export type RegisterResponseType = {
  index: number;
  value: number;
};

type CompositionResult = {
  x_1: number;
  y_1: number;
};

type ColumnDataEntry = {
  timestamp: number;
  temperatures: number[];
  compositions: CompositionResult[];
  percentageComplete: number;
  distilledMass: number;
};
