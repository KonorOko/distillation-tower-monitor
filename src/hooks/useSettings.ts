import { useContext } from "react";
import { SettingsContext } from "@/contexts/settings-context";
import { invokeTauri, logger } from "@/adapters/tauri";
import { SettingsType } from "@/types";

const MAX_PLATES = 8;
const MIN_PLATES = 1;

enum ErrorMessages {
  FETCH_ERROR = "Error fetching settings: ",
  SAVE_ERROR = "Error saving settings: ",
  ADD_PLATE_ERROR = "Error adding plate: ",
  REMOVE_PLATE_ERROR = "Error removing plate: ",
}

export function useSettings() {
  const { settings, setSettings } = useContext(SettingsContext);

  const loadSettings = async () => {
    try {
      let settings = await invokeTauri<SettingsType>("get_settings");
      console.log("Hook loaded settings:", settings);
      setSettings(settings);
    } catch (error) {
      logger.error(ErrorMessages.FETCH_ERROR + (error as Error).message);
      throw new Error(ErrorMessages.FETCH_ERROR);
    }
  };

  const saveSettings = async (newSettings: Partial<SettingsType>) => {
    try {
      const updatedSettings = { ...settings, ...newSettings };
      console.log("Hook saving settings:", updatedSettings);
      await invokeTauri("save_settings", {
        settings: updatedSettings,
      });

      await loadSettings();
    } catch (error) {
      logger.error(ErrorMessages.SAVE_ERROR + (error as Error).message);
      throw new Error(ErrorMessages.SAVE_ERROR);
    }
  };

  const addPlate = async () => {
    try {
      if (settings.numberPlates < MAX_PLATES) {
        await saveSettings({
          numberPlates: settings.numberPlates + 1,
        });
      }
    } catch (error) {
      logger.error(ErrorMessages.ADD_PLATE_ERROR + (error as Error).message);
      throw new Error(ErrorMessages.ADD_PLATE_ERROR);
    }
  };

  const removePlate = async () => {
    try {
      if (settings.numberPlates > MIN_PLATES) {
        await saveSettings({
          numberPlates: settings.numberPlates - 1,
        });
      }
    } catch (error) {
      logger.error(ErrorMessages.REMOVE_PLATE_ERROR + (error as Error).message);
      throw new Error(ErrorMessages.REMOVE_PLATE_ERROR);
    }
  };

  return {
    settings,
    loadSettings,
    saveSettings,
    addPlate,
    removePlate,
    MAX_PLATES,
    MIN_PLATES,
  };
}
