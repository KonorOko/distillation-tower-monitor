import { logger } from "@/adapters/tauri";
import { commands } from "@/bindings";
import { Button } from "@/components/ui/button";
import { useData } from "@/hooks/useData";
import { useSettings } from "@/hooks/useSettings";
import { useVariables } from "@/hooks/useVariables";
import { cn } from "@/lib/utils";
import { settingsSchema } from "@/schemas/settings";
import { Power, Save, Settings } from "lucide-react";
import { toast } from "sonner";
import { ExportDialog } from "./export-dialog";
import { SettingsDialog } from "./settings-dialog";
import { StatusLed } from "./ui/status-led";

export function ModbusPlayer() {
  const connected = useData((state) => state.connected);
  const setConnected = useData((state) => state.setConnected);
  const clearData = useData((state) => state.clearData);
  const numberPlates = useVariables((state) => state.numberPlates);
  const initialComposition = useVariables((state) => state.initialComposition);
  const initialMass = useVariables((state) => state.initialMass);
  const { settings } = useSettings();

  const handleConnection = async () => {
    if (connected === "modbus") {
      const response = await commands.disconnectModbus();
      if (response.status !== "ok") {
        logger.error(`Error disconnecting from MODBUS: ${response.error}`);
        setConnected("modbus");
        return;
      }
      setConnected("none");
      clearData();
    } else {
      const connectModbus = async () => {
        const response = await commands.connectModbus();
        if (response.status !== "ok") {
          throw new Error(response.error.type);
        }
      };

      const validData = settingsSchema.safeParse(settings);
      if (!validData.success) {
        toast.error("Missing settings");
        return;
      }
      if (!initialMass || !initialComposition) {
        toast.error("Missing initial values");
        return;
      }

      toast.promise(
        connectModbus().then(() => {
          setConnected("modbus");
          commands.sendColumnData(
            numberPlates,
            initialMass,
            initialComposition,
          );
        }),
        {
          loading: "Connecting to MODBUS...",
          success: "Connected to MODBUS",
          error: "Error connecting to MODBUS",
        },
      );
    }
  };

  return (
    <div className="flex items-center">
      <SettingsDialog>
        <Button size={"icon"} className="rounded-r-none">
          <Settings className="h-4 w-4" />
        </Button>
      </SettingsDialog>
      <Button
        className={cn(
          "rounded-none border-x-[0.1px] border-x-gray-700",
          connected === "modbus" && "bg-red-500",
        )}
        onClick={handleConnection}
      >
        <Power className="h-4 w-4" />
        Connect MODBUS
      </Button>
      <ExportDialog>
        <Button className="rounded-l-none">
          <Save className="h-4 w-4" />
          Save
        </Button>
      </ExportDialog>
      <StatusLed connected={connected === "modbus"} className="ml-4" />
    </div>
  );
}
