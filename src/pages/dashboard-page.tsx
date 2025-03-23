import { Header } from "@/components/header";
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
      <Header className="col-span-6 row-span-1 rounded border" />
      <TowerSection className="col-span-2 row-span-8 row-start-2 rounded border" />
      <section className="col-span-2 row-span-2 row-start-10 flex items-center justify-center rounded border bg-slate-100">
        <h2 className="text-2xl font-semibold text-slate-500">WIP Control</h2>
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
      <div className="col-span-2 col-start-5 row-span-5 row-start-7 flex items-center justify-center rounded border bg-slate-100 shadow-none">
        <h2 className="text-2xl font-semibold text-slate-500">
          WIP Distillation
        </h2>
      </div>
    </div>
  );
}
