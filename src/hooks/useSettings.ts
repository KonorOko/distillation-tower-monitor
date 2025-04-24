import { useContext } from "react";
import { SettingsContext } from "@/contexts/settings-context";
import { logger } from "@/adapters/tauri";
import { SettingsType } from "@/types";
import { commands } from "@/bindings";

enum ErrorMessages {
  FETCH_ERROR = "Error fetching settings: ",
  SAVE_ERROR = "Error saving settings: ",
}

export function useSettings() {
  const { settings, setSettings } = useContext(SettingsContext);

  const loadSettings = async () => {
    try {
      const response = await commands.getSettings();
      if (response.status !== "ok") {
        throw response.error;
      }
      setSettings(response.data);
    } catch (error) {
      logger.error(ErrorMessages.FETCH_ERROR + (error as Error).message);
      throw new Error(ErrorMessages.FETCH_ERROR);
    }
  };

  const saveSettings = async (newSettings: Partial<SettingsType>) => {
    try {
      const updatedSettings = { ...settings, ...newSettings };
      const response = await commands.saveSettings(updatedSettings);
      if (response.status !== "ok") {
        throw response.error;
      }

      await loadSettings();
    } catch (error) {
      logger.error(ErrorMessages.SAVE_ERROR + (error as Error).message);
      throw new Error(ErrorMessages.SAVE_ERROR);
    }
  };

  return {
    settings,
    loadSettings,
    saveSettings,
  };
}
