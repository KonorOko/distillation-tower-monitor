import { ChartConfig, ChartContainer } from "@/components/ui/chart";
import { useData } from "@/hooks/useData";
import { formatDistillationChart } from "@/lib/formatDataChart";
import { useEffect, useMemo } from "react";
import { CartesianGrid, Label, Line, LineChart, XAxis, YAxis } from "recharts";
import { EmptyState } from "./empty-state";

const chartConfig = {
  time: {
    label: "Time",
    color: "hsl(220, 100%, 80%)",
  },
  distilledMass: {
    label: "Distilled Mass",
    color: "hsl(var(--chart-2))",
  },
} satisfies ChartConfig;

export function DistillationMassChart() {
  let connected = useData((state) => state.connected);
  let columnData = useData((state) => state.columnData);
  let chartData = useMemo(
    () => formatDistillationChart(columnData),
    [columnData],
  );

  useEffect(
    () => console.log("chartData", chartData, "\n original", columnData),
    [chartData],
  );
  if (connected === "none") {
    return <EmptyState />;
  }

  return (
    <ChartContainer config={chartConfig} className="aspect-auto h-full w-full">
      <LineChart
        data={chartData}
        accessibilityLayer
        margin={{
          left: 27,
          right: 5,
          bottom: 17,
          top: 45,
        }}
      >
        <CartesianGrid vertical={false} />
        <YAxis
          tickLine={false}
          tick={true}
          tickMargin={5}
          axisLine={false}
          width={10}
          domain={[
            (dataMin: number) => (dataMin * (1 - 0.08)).toFixed(0),
            (dataMax: number) => (dataMax * (1 + 0.08)).toFixed(0),
          ]}
          className="text-xs font-thin tracking-tight"
          label={<Label value={"m(g)"} position={{ x: 2, y: -20 }} />}
        />
        <XAxis
          dataKey="time"
          tickLine={false}
          axisLine={false}
          tickMargin={10}
          interval={"equidistantPreserveStart"}
          minTickGap={15}
          className="overflow-hidden text-xs font-thin tracking-tight"
          label={
            <Label
              value={"Time (min)"}
              position={"insideBottom"}
              offset={-15}
            />
          }
        />
        <Line
          dataKey="distilledMass"
          stroke="var(--color-distilledMass)"
          isAnimationActive={false}
          dot={false}
        />
      </LineChart>
    </ChartContainer>
  );
}
