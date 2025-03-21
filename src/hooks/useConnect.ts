import { create } from "zustand";
import { invokeTauri, logger } from "@/adapters/tauri";
import { toast } from "sonner";
import { settingsSchema } from "@/schemas/settings";
import { SettingsType } from "@/types";
import { useData } from "./useData";

interface ConnectState {
  connected: boolean;
  loading: boolean;
  setLoading: (isLoading: boolean) => void;
  setConnected: (isConnected: boolean) => void;
  connect: (settings: SettingsType) => Promise<void>;
  disconnect: () => Promise<void>;
}

export const useConnect = create<ConnectState>((set) => ({
  connected: false,
  loading: false,
  setLoading: (loading) => set({ loading }),
  setConnected: (connected) => set({ connected }),
  connect: async (settings: SettingsType) => {
    const result = settingsSchema.safeParse(settings);

    if (!result.success) {
      let errors = result;
      toast.error("Missing settings");
      logger.error(errors.error.message);
      return;
    }
    set({ loading: true });

    toast.promise(invokeTauri("connect_modbus"), {
      loading: "Connecting...",
      success: () => {
        set({ connected: true });
        return "Connected";
      },
      error: (error) => {
        logger.error("Error connecting Modbus: ", error);
        if (error === "Already connected") {
          set({ connected: true });
          return "Already connected";
        }
        set({ connected: false });
        return "Connection error";
      },
      finally: () => set({ loading: false }),
    });
  },
  disconnect: async () => {
    set({ loading: true });
    try {
      await invokeTauri("disconnect_modbus");
      useData.getState().clearData();
      set({ connected: false });
    } catch (error: any) {
      toast.error("Error in disconnect");
      logger.error("Error in disconnect Modbus: ", error);
    } finally {
      set({ loading: false });
    }
  },
}));
