import { CalculationStep, commands, HistorySummary } from "@/bindings";
import { useData } from "@/hooks/useData";
import { Tabs } from "@radix-ui/react-tabs";
import { ColumnDef } from "@tanstack/react-table";
import {
  Calendar,
  Clock,
  Download,
  FileText,
  HardDrive,
  Trash2,
  Upload,
} from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { ImportDialog } from "./import-dialog";
import { PreviewFileTree } from "./preview-file-tree";
import { DataTable } from "./steps-data-table";
import { Button } from "./ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "./ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "./ui/dropdown-menu";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "./ui/table";
import { TabsContent, TabsList, TabsTrigger } from "./ui/tabs";

export function FilesTree({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useState(false);
  const [files, setFiles] = useState<HistorySummary[]>([]);
  const [selectedFile, setSelectedFile] = useState<HistorySummary | null>(null);
  const [steps, setSteps] = useState<CalculationStep[]>([]);
  const setConnected = useData((state) => state.setConnected);
  const connected = useData((state) => state.connected);
  const filePath = useData((state) => state.filePath);
  const setFilePath = useData((state) => state.setFilePath);
  const clearData = useData((state) => state.clearData);
  const isImporting = useData((state) => state.isImporting);

  const initialTimestamp = useMemo(() => {
    if (!steps || steps.length === 0) return 0;
    return steps?.[0].timestamp;
  }, [steps]);

  const handleFileSelect = (file: HistorySummary) => {
    setSelectedFile(file);
    commands.getCalculationHistory(file.id).then((response) => {
      if (response.status === "ok") {
        setFilePath(file.id);
        setSteps(response.data.steps);
        console.log(response.data.steps);
      } else {
        console.error("Error fetching file data:", response.error);
      }
    });
  };

  const handleStartPlayback = async () => {
    if (!selectedFile) return;
    const setData = await commands.loadHistoryData(selectedFile.id);
    if (setData.status !== "ok") {
      console.error("Error loading history data:", setData.error);
      return;
    }

    commands
      .sendColumnData(
        selectedFile.number_plates,
        selectedFile.initial_mass,
        selectedFile.initial_concentration,
      )
      .catch(() => setConnected("none"));
    setConnected("file");
    setFilePath(selectedFile.id);
    console.log("Playback started for file:", selectedFile.id);
  };

  const handleCancelPlayback = () => {
    commands.cancelColumnData().then(() => {
      clearData();
      console.log("Playback cancelled");
    });
  };

  const handleDeleteHistory = (history_id: string | undefined) => {
    if (!history_id) return;
    commands.deleteCalculationHistory(history_id).then((response) => {
      if (response.status === "ok") {
        setFiles((prev) => prev.filter((file) => file.id !== history_id));
        setSelectedFile(null);
        setSteps([]);
        setFilePath("");
        console.log("History deleted successfully");
      } else {
        console.error("Error deleting history:", response.error);
      }
    });
  };

  useEffect(() => {
    const getFiles = async () => {
      const response = await commands.listCalculationHistories();
      if (response.status === "ok") {
        console.log("Calculations history: ", response.data);
        setFiles(response.data);
      } else {
        console.error("Error fetching files:", response.error);
      }
    };
    if (open && !isImporting) getFiles();
  }, [open, isImporting]);
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>{children}</DialogTrigger>
      <DialogContent className="max-w-7xl px-4">
        <DialogHeader className="hidden">
          <DialogTitle>Files Tree</DialogTitle>
          <DialogDescription>
            Files tree to manage and view calculation histories.
          </DialogDescription>
        </DialogHeader>
        <div className="flex h-[calc(90vh-120px)] gap-6 overflow-hidden p-4">
          <div className="flex w-1/2 flex-col">
            <div className="mb-4 flex items-center justify-between">
              <h3 className="text-lg font-semibold">List of Files</h3>
              <ImportDialog>
                <Button variant="outline" size="sm" className="gap-2">
                  <Upload className="h-4 w-4" />
                  Import File
                </Button>
              </ImportDialog>
            </div>

            <div className="w-full flex-1 overflow-auto rounded-xl border">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Archivo</TableHead>
                    <TableHead>Fecha</TableHead>
                    <TableHead>Duración</TableHead>
                    <TableHead>Tamaño</TableHead>
                    <TableHead></TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {files.map((file) => (
                    <TableRow
                      key={file.id}
                      className={`cursor-pointer hover:bg-muted/50 ${file.id ? "bg-muted" : ""}`}
                      onClick={() => handleFileSelect(file)}
                    >
                      <TableCell>
                        <div className="flex items-center gap-2">
                          <FileText className="h-4 w-4 text-slate-700" />
                          <span className="font-medium">{file.id}</span>
                        </div>
                      </TableCell>
                      <TableCell>
                        <div className="flex items-center gap-1 text-sm text-muted-foreground">
                          <Calendar className="h-3 w-3" />
                          {new Date(file.start_time).toLocaleDateString()}
                        </div>
                      </TableCell>
                      <TableCell>
                        <div className="flex items-center gap-1 text-sm">
                          <Clock className="h-3 w-3" />
                          {file.start_time - file.end_time}
                        </div>
                      </TableCell>
                      <TableCell>
                        <div className="flex items-center gap-1 text-sm">
                          <HardDrive className="h-3 w-3" />
                          <span className="text-nowrap">
                            {file.file_size
                              ? (file.file_size / 1e6).toFixed(2)
                              : 0}{" "}
                            MB
                          </span>
                        </div>
                      </TableCell>
                      <TableCell>
                        <DropdownMenu>
                          <DropdownMenuTrigger asChild>
                            <Button
                              variant="ghost"
                              size="sm"
                              onClick={(e) => e.stopPropagation()}
                            >
                              <Download className="h-4 w-4" />
                            </Button>
                          </DropdownMenuTrigger>
                          <DropdownMenuContent>
                            <DropdownMenuItem>
                              Exportar como JSON
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                              Exportar como CSV
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                              Exportar como Excel
                            </DropdownMenuItem>
                          </DropdownMenuContent>
                        </DropdownMenu>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>
          </div>
          <div className="flex w-1/2 flex-col">
            {selectedFile ? (
              <Tabs defaultValue="preview" className="flex flex-1 flex-col">
                <div className="mb-1 flex items-center justify-between">
                  <TabsList>
                    <TabsTrigger value="preview">Preview</TabsTrigger>
                    <TabsTrigger value="data">Data</TabsTrigger>
                  </TabsList>

                  <div className="flex items-center gap-1">
                    {connected === "file" && filePath === selectedFile.id ? (
                      <Button
                        size="sm"
                        variant="outline"
                        className="gap-2"
                        onClick={() => handleCancelPlayback()}
                      >
                        Stop Playback
                      </Button>
                    ) : (
                      <Button size="sm" onClick={() => handleStartPlayback()}>
                        Start Playback
                      </Button>
                    )}
                    <Button
                      size={"icon"}
                      variant={"outline"}
                      className="size-8"
                      onClick={() => handleDeleteHistory(selectedFile?.id)}
                    >
                      <Trash2 className="size-3" />
                    </Button>
                  </div>
                </div>

                <TabsContent
                  value="preview"
                  className="h-full flex-1 overflow-hidden"
                >
                  <PreviewFileTree selectedFile={selectedFile} />
                </TabsContent>

                <TabsContent value="data" className="h-full">
                  <Card className="h-full w-full shadow-none">
                    <CardHeader className="hidden">
                      <CardTitle className="text-lg">Tabla de Datos</CardTitle>
                    </CardHeader>
                    <CardContent className="h-full max-w-[593px] p-0 px-2">
                      <DataTable
                        columns={getColumns(initialTimestamp)}
                        data={steps}
                      />
                    </CardContent>
                  </Card>
                </TabsContent>
              </Tabs>
            ) : (
              <Card className="flex h-full items-center justify-center shadow-none">
                <CardContent className="text-center">
                  <FileText className="mx-auto mb-4 h-16 w-16 text-muted-foreground" />
                  <h3 className="mb-2 text-lg font-semibold">
                    Selecciona un archivo
                  </h3>
                  <p className="text-muted-foreground">
                    Haz clic en cualquier archivo de la lista para ver su
                    contenido
                  </p>
                </CardContent>
              </Card>
            )}
          </div>
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Close</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

const getColumns = (initialTimestamp: number): ColumnDef<CalculationStep>[] => [
  {
    accessorKey: "timestamp",
    header: "Time",
    cell: ({ row }) => {
      const currentTimestamp = row.getValue("timestamp") as number;
      const elapsedSeconds = Math.max(0, currentTimestamp - initialTimestamp);

      const minutes = Math.floor(elapsedSeconds / 60);
      const seconds = elapsedSeconds % 60;

      return (
        <span>
          {minutes}:{seconds.toString().padStart(2, "0")}
        </span>
      );
    },
  },
  {
    id: "temperature_reboiler",
    header: "Temperature Reboiler",
    accessorFn: (row) => row.temperatures[0],
  },
  {
    id: "temperature_condensor",
    header: "Temperature Condensor",
    accessorFn: (row) => row.temperatures.at(-1),
  },
  {
    id: "composition_reboiler",
    header: "Composition Reboiler",
    accessorFn: (row) => row.compositions[0].x_1 || "-",
  },
  {
    id: "composition_condensor",
    header: "Composition Condensor",
    accessorFn: (row) => row.compositions.at(-1)?.x_1 || "-",
  },
  {
    accessorKey: "delta_x",
    header: "Delta X",
  },
  {
    accessorKey: "f_0",
    header: "F0",
  },
  {
    accessorKey: "f_1",
    header: "F1",
  },
  {
    accessorKey: "partial_integral",
    header: "Partial Integral",
  },
  {
    accessorKey: "remaining_mass",
    header: "Remaining Mass",
  },
];
