import { useContext } from "react";
import { SettingsContext } from "@/contexts/settings-context";
import { invokeTauri, logger } from "@/adapters/tauri";
import { SettingsType } from "@/types";

export function useSettings() {
  const { settings, setSettings } = useContext(SettingsContext);
  const MAX_PLATES = 8;

  const addPlate = async () => {
    let newNumberPlates = Math.min(settings.numberPlates + 1, MAX_PLATES);
    try {
      await saveSettings({ ...settings, numberPlates: newNumberPlates });
    } catch (error) {
      logger.error("Error adding plate");
    }
  };

  const removePlate = async () => {
    let newNumberPlates = Math.max(settings.numberPlates - 1, 1);
    try {
      await saveSettings({ ...settings, numberPlates: newNumberPlates });
    } catch (error) {
      logger.error("Error removing plate");
    }
  };

  const loadSettings = async () => {
    try {
      let settings = await invokeTauri<SettingsType>("get_settings");
      setSettings(settings);
    } catch (error) {
      logger.error("Error fetching settings");
    }
  };

  const saveSettings = async (newSettings: Partial<SettingsType>) => {
    newSettings.numberPlates =
      newSettings.numberPlates ?? settings.numberPlates;
    try {
      await invokeTauri("save_settings", {
        settings: { ...newSettings },
      });
      await loadSettings();
    } catch (error) {
      logger.error("Error updating settings");
      throw error;
    }
  };

  return {
    settings,
    loadSettings,
    saveSettings,
    addPlate,
    removePlate,
    MAX_PLATES,
  };
}
