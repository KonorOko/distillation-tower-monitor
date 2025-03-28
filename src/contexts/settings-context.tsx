import { SettingsContextType, SettingsType } from "@/types";
import { createContext, useState } from "react";

export const SettingsContext = createContext<SettingsContextType>(
  {} as SettingsContextType,
);

export function SettingsProvider({ children }: { children: React.ReactNode }) {
  const [settings, setSettings] = useState<SettingsType>({
    modbus: {
      initialAddress: 0,
      usbPort: "",
      count: 2,
      timeout: 0,
      baudrate: 0,
      unitId: 0,
    },
  });
  return (
    <SettingsContext.Provider
      value={{
        settings,
        setSettings,
      }}
    >
      {children}
    </SettingsContext.Provider>
  );
}
