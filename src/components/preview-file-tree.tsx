import { HistorySummary } from "@/bindings";
import { BarChart3 } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";

export function PreviewFileTree({
  selectedFile,
}: {
  selectedFile: HistorySummary;
}) {
  return (
    <Card className="h-full shadow-none">
      <CardHeader>
        <CardTitle className="text-lg">{selectedFile.id}</CardTitle>
        <div className="flex gap-4 text-sm text-muted-foreground">
          <span>
            Fecha: {new Date(selectedFile.start_time).toLocaleDateString()}
          </span>
          <span>
            Duración: {selectedFile.start_time - selectedFile.end_time}
          </span>
          <span className="text-sm text-muted-foreground">
            Rows: {selectedFile.steps_count}
          </span>
        </div>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="flex flex-col gap-1">
          <span className="font-medium">Initial Values</span>
          <div className="text-sm">
            <div className="text-muted-foreground">
              Mass: {selectedFile.initial_mass} g
            </div>
            <div className="text-muted-foreground">
              Composition: {selectedFile.initial_concentration * 100} %m/m
            </div>
          </div>
        </div>
        <Card>
          <CardHeader>
            <CardTitle className="text-base">
              Evolución de temperatura
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex h-48 items-center justify-center rounded-lg border-2 border-dashed border-gray-300 bg-gradient-to-r from-blue-50 to-green-50">
              <div className="text-center text-muted-foreground">
                <BarChart3 className="mx-auto mb-2 h-12 w-12" />
                <p>Gráfico de evolución temporal</p>
                <p className="text-sm">Temperatura vs Tiempo</p>
              </div>
            </div>
          </CardContent>
        </Card>
      </CardContent>
    </Card>
  );
}
