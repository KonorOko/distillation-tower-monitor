import { logger } from "@/adapters/tauri";
import { commands } from "@/bindings";
import { Button } from "@/components/ui/button";
import { useData } from "@/hooks/useData";
import { usePlates } from "@/hooks/usePlates";
import { cn } from "@/lib/utils";
import { Power, Save, Settings } from "lucide-react";
import { toast } from "sonner";
import { ExportDialog } from "./export-dialog";
import { FilePlayer } from "./file-player";
import { SettingsDialog } from "./settings-dialog";
import { StatusLed } from "./ui/status-led";

export function Header({ className }: { className?: string }) {
  const connected = useData((state) => state.connected);
  const setConnected = useData((state) => state.setConnected);
  const clearData = useData((state) => state.clearData);
  const numberPlates = usePlates((state) => state.numberPlates);

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
      toast.promise(
        commands.connectModbus().then(() => {
          setConnected("modbus");
          commands.sendColumnData(numberPlates);
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
