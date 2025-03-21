import { ChartConfig, ChartContainer } from "@/components/ui/chart";
import { equilibriumData } from "@/data";
import { useData } from "@/hooks/useData";
import { formatYvsX } from "@/lib/formatDataChart";
import { useMemo } from "react";
import {
  CartesianGrid,
  ComposedChart,
  Label,
  Line,
  ReferenceLine,
  Scatter,
  XAxis,
  YAxis,
} from "recharts";
import { EmptyState } from "./empty-state";

const chartConfig = {
  x: {
    label: "Líquido (x₁)",
    color: "hsl(220, 100%, 100%)",
  },
  y: {
    label: "Vapor (y₁)",
    color: "hsl(220, 100%, 70%)",
  },
} satisfies ChartConfig;

export function YVsXChart() {
  let columnData = useData((state) => state.columnData);
  let connected = useData((state) => state.connected);
  let chartData = useMemo(() => formatYvsX(columnData), [columnData]);

  if (connected === "none") {
    return <EmptyState />;
  }

  return (
    <ChartContainer config={chartConfig} className="aspect-auto h-full w-full">
      <ComposedChart
        accessibilityLayer
        data={chartData}
        margin={{
          left: 27,
          right: 5,
          bottom: 15,
          top: 45,
        }}
      >
        <CartesianGrid vertical={true} />
        <YAxis
          tickLine={false}
          tick={true}
          tickMargin={8}
          axisLine={false}
          width={10}
          tickCount={5}
          domain={[0, 1]}
          interval={"preserveStartEnd"}
          className="text-xs font-thin tracking-tight"
        />
        <XAxis
          dataKey="x"
          type="number"
          tickLine={false}
          axisLine={false}
          tickMargin={10}
          domain={[0, 1]}
          allowDataOverflow={true}
          className="overflow-hidden text-xs font-thin tracking-tight"
          label={<Label value={"x"} position={"insideBottom"} offset={-15} />}
        />
        <ReferenceLine
          key="reference"
          segment={[
            { x: 0, y: 0 },
            { x: 1, y: 1 },
          ]}
          stroke="#9ca3af"
        />
        <Line
          key="equilibrium"
          data={equilibriumData}
          dataKey={"y"}
          dot={false}
          stroke="#d1d5db"
          isAnimationActive={false}
        />
        <Scatter
          dataKey="y"
          fill="var(--color-y)"
          stroke="gray"
          isAnimationActive={false}
        />
      </ComposedChart>
    </ChartContainer>
  );
}
