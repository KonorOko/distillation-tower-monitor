import { FilePlayer } from "./file-player";
import { ModbusPlayer } from "./modbus-player";

export function Header({ className }: { className?: string }) {
  return (
    <header
      className={`flex flex-col items-center justify-center px-3 py-2 ${className}`}
    >
      <div className="flex w-full items-center justify-between rounded">
        <ModbusPlayer />
        <FilePlayer />
      </div>
    </header>
  );
}
