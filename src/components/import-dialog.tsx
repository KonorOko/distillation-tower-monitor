import { commands } from "@/bindings";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { useData } from "@/hooks/useData";
import { cn } from "@/lib/utils";
import { FileSpreadsheet, Upload, X } from "lucide-react";
import { useState } from "react";
import { toast } from "sonner";

export function ImportDialog({ children }: { children: React.ReactNode }) {
  const [isOpen, setIsOpen] = useState(false);
  const [filePath, setFilePath] = useState<string>("");
  const isImporting = useData((state) => state.isImporting);
  const setIsImporting = useData((state) => state.setIsImporting);

  const handleImport = () => {
    if (!filePath) {
      toast.error("Please select a file");
      return;
    }

    setIsImporting(true);

    const handleFile = async () => {
      const response = await commands.importToHistory(filePath);
      if (response.status !== "ok") {
        throw new Error("Failed to import data");
      }
    };

    toast.promise(handleFile(), {
      loading: "Importing data...",
      error: "Failed to import data",
      success: "Data imported successfully",
      finally: () => {
        setIsImporting(false);
        setIsOpen(false);
        setFilePath("");
      },
    });
  };

  const handleFileSelect = async () => {
    const path = await commands.filePath();
    setFilePath(path);
  };

  const clearSelection = async () => {
    setFilePath("");
    setIsImporting(false);
  };

  const getFileName = (path: string | null): string => {
    if (!path) return "";
    const pathStr = String(path);
    const parts = pathStr.split(/[/\\]/);
    return parts[parts.length - 1];
  };

  return (
    <Dialog
      open={isOpen}
      onOpenChange={(open) => !isImporting && setIsOpen(open)}
    >
      <DialogTrigger asChild>{children}</DialogTrigger>
      <DialogContent className="sm:max-w-[500px]">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <FileSpreadsheet className="h-5 w-5 text-primary" />
            Excel file
          </DialogTitle>
          <DialogDescription>
            Select an Excel file to import data into the application.
          </DialogDescription>
        </DialogHeader>
        {!filePath ? (
          <div
            className="cursor-pointer rounded-lg border-2 border-dashed border-muted-foreground/20 p-8 text-center transition-colors hover:bg-muted/50"
            onClick={handleFileSelect}
          >
            <Upload className="mx-auto mb-4 h-10 w-10 text-muted-foreground/50" />
            <p className="text-sm font-medium text-muted-foreground">
              Click to select an Excel file
            </p>
            <p className="mt-2 text-xs text-muted-foreground">
              Supported formats: .xlsx, .xls
            </p>
          </div>
        ) : (
          <div className="rounded-lg bg-muted/30 p-4">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <FileSpreadsheet className="h-8 w-8 text-primary/80" />
                <div>
                  <p className="max-w-[300px] truncate text-sm font-medium">
                    {getFileName(filePath)}
                  </p>
                  <p className="max-w-[300px] truncate text-xs text-muted-foreground">
                    {filePath}
                  </p>
                </div>
              </div>
              <Button
                variant="ghost"
                size="icon"
                onClick={clearSelection}
                disabled={isImporting}
              >
                <X className="h-4 w-4" />
              </Button>
            </div>
          </div>
        )}
        <DialogFooter className="gap-2">
          <Button
            variant="outline"
            onClick={() => setIsOpen(false)}
            disabled={isImporting}
          >
            Cancel
          </Button>
          <Button
            onClick={handleImport}
            disabled={!filePath || isImporting}
            className={cn(
              "gap-2",
              isImporting && "cursor-not-allowed opacity-80",
            )}
          >
            {isImporting ? (
              "Importing..."
            ) : (
              <>
                <Upload className="h-4 w-4" />
                Import Data
              </>
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
