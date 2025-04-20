import { invokeTauri, logger } from "@/adapters/tauri";
import { Button } from "@/components/ui/button";
import { useData } from "@/hooks/useData";
import { Power, Save, Settings } from "lucide-react";
import { toast } from "sonner";
import { ExportDialog } from "./export-dialog";
import { FilePlayer } from "./file-player";
import { SettingsDialog } from "./settings-dialog";
import { StatusLed } from "./ui/status-led";
import { cn } from "@/lib/utils";

export function Header({ className }: { className?: string }) {
  const connected = useData((state) => state.connected);
  const setConnected = useData((state) => state.setConnected);
  const clearData = useData((state) => state.clearData);

  const handleConnection = async () => {
    if (connected === "modbus") {
      await invokeTauri("disconnect_modbus")
        .then(() => {
          setConnected("none");
          clearData();
        })
        .catch((error) => {
          logger.error(`Error disconnecting from MODBUS: ${error}`);
          setConnected("modbus");
        });
      return;
    }
    try {
      toast.promise(
        invokeTauri("connect_modbus").then(() => {
          setConnected("modbus");
          invokeTauri("send_column_data", { numberPlates: 8 });
        }),
        {
          loading: "Connecting to MODBUS...",
          success: "Connected to MODBUS",
          error: "Error connecting to MODBUS",
        },
      );
    } catch (error) {
      logger.error(`Error connecting to MODBUS: ${error}`);
      setConnected("none");
    }
  };

  return (
    <header
      className={`flex flex-col items-center justify-center px-3 py-2 ${className}`}
    >
      <div className="flex w-full items-center justify-between rounded">
        <div className="flex items-center">
          <SettingsDialog>
            <Button
              variant={"outline"}
              size={"icon"}
              className="rounded-r-none"
            >
              <Settings className="h-4 w-4" />
            </Button>
          </SettingsDialog>
          <Button
            variant={"outline"}
            className={cn(
              "rounded-none border-x-0",
              connected === "modbus" && "bg-red-500",
            )}
            disabled={connected === "file"}
            onClick={handleConnection}
          >
            <Power className="h-4 w-4" />
            Connect MODBUS
          </Button>
          <ExportDialog>
            <Button
              variant={"outline"}
              disabled={connected === "file"}
              className="rounded-l-none"
            >
              <Save className="h-4 w-4" />
              Save
            </Button>
          </ExportDialog>
          <StatusLed connected={connected === "modbus"} className="ml-4" />
        </div>
        <FilePlayer />
      </div>
    </header>
  );
}
