import { DistillationTower } from "@/components/distillation-tower";
import { Button } from "@/components/ui/button";
import { useData } from "@/hooks/useData";
import { usePlates } from "@/hooks/usePlates";
import { cn } from "@/lib/utils";
import { listen } from "@tauri-apps/api/event";
import { Minus, Plus } from "lucide-react";
import { useEffect } from "react";

export function TowerSection({ className }: { className?: string }) {
  const connected = useData((state) => state.connected);
  const numberPlates = usePlates((state) => state.numberPlates);
  const addPlate = usePlates((state) => state.addPlate);
  const removePlate = usePlates((state) => state.removePlate);
  const setPlates = usePlates((state) => state.setPlates);

  useEffect(() => {
    const unlisten = listen<number>("number_plates", (event) => {
      const handleListen = async () => {
        setPlates(event.payload);
      };
      handleListen();
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <section className={cn("relative", className)}>
      <header className="absolute right-0 z-10">
        <Button
          size={"icon"}
          variant={"outline"}
          onClick={removePlate}
          disabled={connected !== "none"}
          className="rounded-r-none rounded-t-none border-t-0"
        >
          <Minus className="h-4 w-4" />
        </Button>
        <Button
          size={"icon"}
          onClick={addPlate}
          variant={"outline"}
          disabled={connected !== "none"}
          className="rounded-b-none rounded-tl-none border-x-0 border-t-0"
        >
          <Plus className="h-4 w-4" />
        </Button>
      </header>
      <DistillationTower plates={numberPlates} />
      <footer className="absolute bottom-0 right-0 z-10">
        <div className="flex w-10 items-center justify-center rounded border border-b-0 border-r-0 bg-background p-2 shadow-inner">
          <p className="text-sm font-semibold text-muted-foreground">
            {numberPlates}
          </p>
        </div>
      </footer>
    </section>
  );
}
