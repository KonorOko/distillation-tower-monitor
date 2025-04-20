import { DistillationMassChart } from "@/components/distillation-mass-chart";
import { Header } from "@/components/header";
import { XYvsTChart } from "@/components/t-vs-xy-chart";
import { TemperaturesChart } from "@/components/temperatures-chart";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { YVsXChart } from "@/components/y-vs-x-chart";
import { useData } from "@/hooks/useData";
import { ColumnDataEntry } from "@/types";
import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";
import { toast } from "sonner";
import { CardLayout } from "./card-layout";
import { TowerSection } from "./tower-section";

export function DashboardPage() {
  const setColumnData = useData((state) => state.setColumnData);
  const setConnected = useData((state) => state.setConnected);

  useEffect(() => {
    const unlisten = listen<ColumnDataEntry>("column_data", (event) => {
      const handleListen = async () => {
        if (event.payload.percentageComplete === 100) {
          setConnected("paused");
          toast.success("Playback complete!");
        }
        setColumnData(event.payload);
      };

      handleListen();
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <div className="grid h-screen w-full grid-cols-6 grid-rows-11 gap-1 p-1">
      <Header className="col-span-6 row-span-1 rounded border" />
      <TowerSection className="col-span-2 row-span-8 row-start-2 rounded border" />
      <section className="col-span-2 row-span-2 row-start-10 flex flex-col items-center justify-center gap-2 rounded border px-10">
        <div className="flex w-full items-center justify-center gap-2">
          <div className="grid w-1/2 space-y-1">
            <Label htmlFor="initialMass">Initial Mass</Label>
            <Input id="initialMass" placeholder="Enter initial mass" />
          </div>
          <div className="grid w-1/2 space-y-1">
            <Label htmlFor="initialComposition">Initial Composition</Label>
            <Input
              id="initialComposition"
              placeholder="Enter initial composition"
            />
          </div>
        </div>
        <Button className="w-full text-center" variant={"outline"}>
          Set Variables
        </Button>
      </section>
      <CardLayout
        title="Temperature"
        description="Interpolation of temperatures per plate"
        className="col-span-2 col-start-3 row-span-5 row-start-2 rounded shadow-none"
      >
        <TemperaturesChart />
      </CardLayout>
      <CardLayout
        title="y vs x"
        description="Vapor composition vs liquid composition"
        className="col-span-2 col-start-3 row-span-5 row-start-7 rounded shadow-none"
      >
        <YVsXChart />
      </CardLayout>
      <CardLayout
        title="x, y vs T"
        description="Compositions over temperature"
        className={
          "col-span-2 col-start-5 row-span-5 row-start-2 rounded shadow-none"
        }
      >
        <XYvsTChart />
      </CardLayout>
      <CardLayout
        title="Distillation Mass"
        description="Mass of distillation over time"
        className={
          "col-span-2 col-start-5 row-span-5 row-start-7 rounded shadow-none"
        }
      >
        <DistillationMassChart />
      </CardLayout>
    </div>
  );
}
