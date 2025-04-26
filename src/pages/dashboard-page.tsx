import { DistillationMassChart } from "@/components/distillation-mass-chart";
import { Header } from "@/components/header";
import { InitialValuesForm } from "@/components/initial-values-form";
import { XYvsTChart } from "@/components/t-vs-xy-chart";
import { TemperaturesChart } from "@/components/temperatures-chart";
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
      <Header className="col-span-6 row-span-1 rounded border bg-background" />
      <TowerSection className="col-span-2 row-span-8 row-start-2 rounded border bg-background" />
      <InitialValuesForm className="col-span-2 row-span-2 row-start-10 flex flex-col items-center justify-center gap-2 rounded border bg-background px-10" />
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
        title="T vs x, y"
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
