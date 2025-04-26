import { DistillationTower } from "@/components/distillation-tower";
import { Button } from "@/components/ui/button";
import { useData } from "@/hooks/useData";
import { useVariables } from "@/hooks/useVariables";
import { cn } from "@/lib/utils";
import { listen } from "@tauri-apps/api/event";
import { Minus, Plus } from "lucide-react";
import { useEffect } from "react";

export function TowerSection({ className }: { className?: string }) {
  const connected = useData((state) => state.connected);
  const numberPlates = useVariables((state) => state.numberPlates);
  const addPlate = useVariables((state) => state.addPlate);
  const removePlate = useVariables((state) => state.removePlate);
  const setPlates = useVariables((state) => state.setPlates);
  const setInitialMass = useVariables((state) => state.setInitialMass);
  const setInitialComposition = useVariables(
    (state) => state.setInitialComposition,
  );

  useEffect(() => {
    const unlisten = listen<Array<number>>("initial_data", (event) => {
      let payload = event.payload;
      setPlates(payload[0]);
      setInitialMass(payload[1]);
      setInitialComposition(payload[2]);
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
      <footer className="absolute bottom-0 right-0">
        <div className="z-10 flex w-10 items-center justify-center rounded border border-b-0 border-r-0 bg-background p-2 shadow-inner">
          <p className="text-sm font-semibold text-muted-foreground">
            {numberPlates}
          </p>
        </div>
      </footer>
    </section>
  );
}
