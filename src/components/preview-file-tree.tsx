import { CalculationStep, HistorySummary } from "@/bindings";
import { BarChart3, Calendar } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";

export function PreviewFileTree({
  selectedFile,
  steps,
}: {
  selectedFile: HistorySummary;
  steps: CalculationStep[];
}) {
  const startTime = selectedFile.start_time;
  const endTime = selectedFile.end_time;
  const elapsedSeconds = endTime - startTime;

  const hours = Math.floor(elapsedSeconds / 3600);
  const minutes = Math.floor(elapsedSeconds / 60);
  const seconds = elapsedSeconds % 60;

  const temperatures = steps.flatMap((step) => step.temperatures);
  const highestTemperature = Math.max(...temperatures);
  const lowestTemperature = Math.min(...temperatures);

  return (
    <Card className="h-full shadow-none">
      <CardHeader>
        <CardTitle className="text-lg">{selectedFile.id}</CardTitle>
        <div className="flex gap-4 text-sm text-muted-foreground">
          <span>
            Date:{" "}
            {new Date(selectedFile.start_time * 1000).toLocaleDateString()}
          </span>
          <span>
            Duration: {hours} h {minutes} min {seconds} sec
          </span>
          <span className="text-sm text-muted-foreground">
            Rows: {selectedFile.steps_count}
          </span>
        </div>
      </CardHeader>
      <CardContent className="space-y-6">
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-base">Initial Values</CardTitle>
            <p className="text-sm text-muted-foreground">
              Initial Mass of Mixture: {selectedFile.initial_mass} g
            </p>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-2 gap-4 md:grid-cols-2">
              <div className="rounded-lg bg-purple-50 p-3 text-center">
                <div className="text-2xl font-bold text-purple-600">
                  {selectedFile.initial_concentration * 100}%
                </div>
                <p className="text-xs font-medium text-purple-700">Etanol</p>
              </div>
              <div className="rounded-lg bg-blue-50 p-3 text-center">
                <div className="text-2xl font-bold text-blue-600">
                  {(1 - selectedFile.initial_concentration) * 100}%
                </div>
                <p className="text-xs font-medium text-blue-700">Agua</p>
              </div>
            </div>
          </CardContent>
        </Card>
        <div className="grid grid-cols-2 gap-6">
          <Card>
            <CardHeader className="pb-3">
              <CardTitle className="flex items-center gap-2 text-base">
                <Calendar className="h-4 w-4" />
                General Statistics
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <div className="flex justify-between">
                <span className="text-sm text-muted-foreground">
                  Eficiency:
                </span>
                <span className="text-sm font-medium text-green-600">-</span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-muted-foreground">
                  Energy Released:
                </span>
                <span className="text-sm font-medium">-</span>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="pb-3">
              <CardTitle className="flex items-center gap-2 text-base">
                <BarChart3 className="h-4 w-4" />
                Operation Summary
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <div className="flex justify-between">
                <span className="text-sm text-muted-foreground">
                  Highest Temperature:
                </span>
                <span className="text-sm font-medium text-red-600">
                  {highestTemperature.toFixed(1)}°C
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-muted-foreground">
                  Lowest Temperature:
                </span>
                <span className="text-sm font-medium text-blue-600">
                  {lowestTemperature.toFixed(1)} °C
                </span>
              </div>
            </CardContent>
          </Card>
        </div>
      </CardContent>
    </Card>
  );
}
