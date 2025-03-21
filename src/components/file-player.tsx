import { ImportDialog } from "@/components/import-dialog";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { StatusLed } from "@/components/ui/status-led";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useData } from "@/hooks/useData";
import { cn } from "@/lib/utils";
import {
  Clock,
  FastForward,
  Pause,
  Play,
  Rewind,
  SkipBack,
  SkipForward,
} from "lucide-react";
import { useState } from "react";

import { invokeTauri, logger } from "@/adapters/tauri";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

export function FilePlayer({ className = "" }: { className?: string }) {
  const [playbackSpeed, setPlaybackSpeed] = useState(2);
  const connected = useData((state) => state.connected);
  const fileProgress = useData((state) => state.fileProgress);
  const setConnected = useData((state) => state.setConnected);

  // Handle speed change
  const handleSpeedChange = async (value: string) => {
    const speed = Number.parseFloat(value);
    setPlaybackSpeed(speed);
    await invokeTauri("set_speed", { speedFactor: speed }).catch((e) =>
      console.log("Error setting speed:", e),
    );
  };

  // Handle skip forward/backward
  const handleSkip = async (amount: number) => {
    console.log(`Skipping ${amount} seconds`);
    await invokeTauri("handle_skip", { skipCount: amount }).catch((e) =>
      console.log("Error skipping", e),
    );
  };

  const handleFile = async () => {
    if (fileProgress === 100) return;
    if (connected === "file") {
      await invokeTauri("pause_column_data")
        .then(() => setConnected("paused"))
        .catch((error) => {
          logger.error(`Error canceling column data: ${error}`);
          setConnected("none");
        });
      return;
    }
    setConnected("file");
    await invokeTauri("send_column_data").catch(() => setConnected("none"));
  };

  return (
    <header
      className={`flex flex-col items-center justify-center px-3 py-2 ${className}`}
    >
      <div className="flex flex-col gap-2">
        <div className="flex items-center">
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" size="icon" className="h-7 w-7">
                <Clock className="h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
              <DropdownMenuLabel>Playback Speed</DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuRadioGroup
                value={playbackSpeed.toString()}
                onValueChange={handleSpeedChange}
              >
                <DropdownMenuRadioItem value="1">1x</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="2">2x</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="5">5x</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="10">10x</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="15">15x</DropdownMenuRadioItem>
              </DropdownMenuRadioGroup>
            </DropdownMenuContent>
          </DropdownMenu>

          <div className="mx-1 flex items-center">
            <DefaultTooltip text="Rewind 1 hr">
              <Button
                variant="ghost"
                size="icon"
                className="h-7 w-7"
                onClick={() => handleSkip(-1 * 60 * 60)}
              >
                <Rewind className="h-4 w-4" />
              </Button>
            </DefaultTooltip>
            <DefaultTooltip text="Rewind 30 min">
              <Button
                variant="ghost"
                size="icon"
                className="h-7 w-7"
                onClick={() => handleSkip(-0.5 * 60 * 60)}
              >
                <SkipBack className="h-4 w-4" />
              </Button>
            </DefaultTooltip>

            <Button
              variant="outline"
              size="icon"
              onClick={handleFile}
              className="mx-1 h-7 w-7"
            >
              {connected === "file" ? (
                <Pause className="h-4 w-4" />
              ) : (
                <Play className="h-4 w-4" />
              )}
            </Button>

            <DefaultTooltip text="Skip 30 min">
              <Button
                variant="ghost"
                size="icon"
                className="h-7 w-7"
                onClick={() => handleSkip(0.5 * 60 * 60)}
              >
                <SkipForward className="h-4 w-4" />
              </Button>
            </DefaultTooltip>

            <DefaultTooltip text="Skip 1 hour">
              <Button
                variant="ghost"
                size="icon"
                className="h-7 w-7"
                onClick={() => handleSkip(1 * 60 * 60)}
              >
                <FastForward className="h-4 w-4" />
              </Button>
            </DefaultTooltip>
          </div>

          <ImportDialog>
            <Button variant="outline" className="relative h-7">
              <span className="text-xs">Excel File</span>
              {(connected === "file" || connected === "paused") && (
                <StatusLed
                  connected={connected === "file"}
                  className="absolute right-0 top-0 -mr-1 -mt-1"
                />
              )}
            </Button>
          </ImportDialog>
        </div>

        <div className="flex w-full items-center gap-2">
          <span className="w-10 text-right text-xs text-muted-foreground">
            {fileProgress.toFixed(1)}%
          </span>
          <div className="relative w-full">
            <Progress
              value={fileProgress}
              className="h-1"
              color={connected === "file" ? "bg-green-600" : undefined}
            />
          </div>
          <span
            className={cn("rounded bg-primary/10 px-1 text-xs text-primary")}
          >
            {playbackSpeed}x
          </span>
        </div>
      </div>
    </header>
  );
}

export function DefaultTooltip({
  children,
  text,
}: {
  children: React.ReactNode;
  text: string;
}) {
  return (
    <TooltipProvider delayDuration={100}>
      <Tooltip>
        <TooltipTrigger asChild>{children}</TooltipTrigger>
        <TooltipContent>{text}</TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
}
