import { commands } from "@/bindings";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useState } from "react";
import { toast } from "sonner";

export function ExportDialog({ children }: { children: React.ReactNode }) {
  const [isOpen, setIsOpen] = useState(false);
  const [fileName, setFileName] = useState("column-data");
  const [folderPath, setFolderPath] = useState("");

  const handleExport = () => {
    const newPath = folderPath + "/" + fileName + ".xlsx";
    toast.promise(commands.exportData(newPath), {
      loading: "Saving data...",
      error: "Error saving data",
      success: "Data saved",
    });
    setIsOpen(false);
  };

  const handleDialog = async () => {
    const path = await commands.folderPath();
    setFolderPath(path);
  };
  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>{children}</DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Export Data</DialogTitle>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="filename" className="text-right">
              File name
            </Label>
            <Input
              id="filename"
              value={fileName}
              onChange={(e) => setFileName(e.target.value)}
              className="col-span-3"
            />
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="folder" className="text-right">
              Folder path
            </Label>
            <Button
              id="folder"
              className="col-span-3"
              value={folderPath}
              onClick={handleDialog}
              variant={"outline"}
            >
              {" "}
              {folderPath ? folderPath : "Choose path"}
            </Button>
          </div>
        </div>
        <Button onClick={handleExport}>Descargar</Button>
      </DialogContent>
    </Dialog>
  );
}
