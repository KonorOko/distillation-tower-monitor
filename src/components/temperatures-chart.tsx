import {
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart";
import { useData } from "@/hooks/useData";
import { formatTempPerTime } from "@/lib/formatDataChart";
import { useMemo } from "react";
import { CartesianGrid, Label, Line, LineChart, XAxis, YAxis } from "recharts";
import { EmptyState } from "./empty-state";

export function TemperaturesChart() {
  let columnData = useData((state) => state.columnData);
  let connected = useData((state) => state.connected);
  let chartData = useMemo(() => formatTempPerTime(columnData), [columnData]);

  let plateKeys = useMemo(() => {
    if (chartData.length === 0) return [];
    return Object.keys(chartData[0]).filter((key) => key !== "time");
  }, [chartData]);

  let chartConfig = Object.fromEntries(
    plateKeys.map((key, index) => [
      key,
      {
        label: `Plate ${key.slice(-1)}`,
        color: `hsl(220, 100%, ${40 + index * 10}%)`,
      },
    ]),
  );

  if (connected === "none") {
    return <EmptyState />;
  }

  return (
    <ChartContainer config={chartConfig} className="aspect-auto h-full w-full">
      <LineChart
        accessibilityLayer
        data={chartData}
        margin={{
          left: 24,
          right: 5,
          bottom: 15,
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
          label={<Label value={"T(°C)"} position={{ x: 2, y: -20 }} />}
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
        <ChartTooltip
          cursor={false}
          isAnimationActive={false}
          labelFormatter={(label) => `Time: ${label} min`}
          content={
            <ChartTooltipContent
              className="bg-background/80"
              formatter={(value, name) => (
                <>
                  <div
                    className="h-2.5 w-1 shrink-0 rounded-[2px] bg-[--color-bg]"
                    style={
                      {
                        "--color-bg": `var(--color-${name})`,
                      } as React.CSSProperties
                    }
                  />
                  <div className="flex min-w-[110px] items-center text-xs text-muted-foreground">
                    {chartConfig[name as keyof typeof chartConfig]?.label ||
                      name}
                    <div className="ml-auto flex items-baseline gap-0.5 font-mono font-medium tabular-nums text-foreground">
                      {(value as number).toFixed(2)}
                      <span className="font-normal text-muted-foreground">
                        °C
                      </span>
                    </div>
                  </div>
                </>
              )}
            />
          }
        />
        {plateKeys.map((key) => (
          <Line
            dataKey={key}
            stroke={`var(--color-${key})`}
            strokeWidth={2}
            isAnimationActive={false}
            dot={false}
          />
        ))}
      </LineChart>
    </ChartContainer>
  );
}
