import { cn } from "@/lib/utils";

export function StatusLed({
  connected,
  className,
}: {
  connected: boolean;
  className?: string;
}) {
  return (
    <div
      className={cn(
        "size-3 rounded-full border border-white shadow-lg",
        connected ? "bg-green-300" : "bg-slate-300",
        className,
      )}
    ></div>
  );
}
