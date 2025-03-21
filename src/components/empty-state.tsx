import { ChartSpline } from "lucide-react";

export function EmptyState() {
  return (
    <div className="flex h-full w-full flex-col items-center justify-center space-y-1 rounded">
      <ChartSpline className="font aspect-video size-20 stroke-[0.5] text-slate-400" />
      <div className="flex flex-col items-center justify-center">
        <p className="text-sm font-medium text-slate-700">No data found</p>
        <p className="text-xs text-slate-500">
          Try to connect usb port or import data
        </p>
      </div>
    </div>
  );
}
