import { ChartConfig, ChartContainer } from "@/components/ui/chart";
import { equilibriumData2 } from "@/data";
import { useData } from "@/hooks/useData";
import { formatXYvsTemp } from "@/lib/formatDataChart";
import { useMemo } from "react";
import {
  CartesianGrid,
  ComposedChart,
  Label,
  Line,
  XAxis,
  YAxis,
} from "recharts";
import { EmptyState } from "./empty-state";

const chartConfig = {
  x: {
    label: "x₁",
    color: "hsl(220, 100%, 80%)",
  },
  reference: {
    label: "",
    color: "#9ca3af",
  },
} satisfies ChartConfig;

export function XYvsTChart() {
  let columnData = useData((state) => state.columnData);
  let connected = useData((state) => state.connected);
  let chartData = useMemo(() => formatXYvsTemp(columnData), [columnData]);

  if (connected === "none") {
    return <EmptyState />;
  }

  return (
    <ChartContainer config={chartConfig} className="aspect-auto h-full w-full">
      <ComposedChart
        accessibilityLayer
        margin={{
          left: 27,
          right: 5,
          bottom: 17,
          top: 45,
        }}
      >
        <CartesianGrid />
        <YAxis
          tickLine={false}
          tick={true}
          tickMargin={8}
          axisLine={false}
          width={10}
          tickCount={11}
          domain={[70, 95]}
          type="number"
          allowDataOverflow
          interval={"equidistantPreserveStart"}
          className="text-xs font-thin tracking-tight"
        />
        <XAxis
          xAxisId={0}
          tickLine={false}
          axisLine={false}
          tickMargin={10}
          dataKey={"x"}
          type="number"
          className="overflow-hidden text-xs font-thin tracking-tight"
          label={
            <Label value={"x, y"} position={"insideBottom"} offset={-15} />
          }
        />
        <XAxis
          xAxisId={1}
          tickLine={false}
          axisLine={false}
          tickMargin={10}
          dataKey={"y"}
          type="number"
          hide
          className="overflow-hidden text-xs font-thin tracking-tight"
        />
        <Line
          xAxisId={0}
          dataKey="temp"
          stroke="var(--color-reference)"
          isAnimationActive={false}
          data={equilibriumData2}
          dot={false}
        />
        <Line
          xAxisId={1}
          dataKey="temp"
          stroke="var(--color-reference)"
          isAnimationActive={false}
          data={equilibriumData2}
          dot={false}
        />
        {chartData.map((entry) => (
          <Line
            xAxisId={0}
            dataKey="temp"
            stroke="var(--color-x)"
            isAnimationActive={false}
            data={entry}
            dot={false}
          />
        ))}
      </ComposedChart>
    </ChartContainer>
  );
}
