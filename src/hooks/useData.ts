import { create } from "zustand";
import { MAX_DATA_LENGTH } from "@/constants";
import { ColumnEntry } from "@/bindings";
import { commands } from "@/bindings";

type DataMode = "none" | "modbus" | "file" | "paused";

interface DataState {
  columnData: ColumnEntry[];
  connected: DataMode;
  isLoading: boolean;
  filePath: string;
  fileProgress: number;
  setColumnData: (columnData: ColumnEntry) => void;
  setConnected: (connected: DataMode) => void;
  setLoading: (isLoading: boolean) => void;
  setFilePath: (filePath: string) => void;
  setFileProgress: (progress: number) => void;
  clearData: () => Promise<void>;
  refreshColumnData: (newColumnData: ColumnEntry[]) => Promise<void>;
}

export const useData = create<DataState>((set) => ({
  columnData: [],
  connected: "none",
  isLoading: false,
  filePath: "",
  fileProgress: 0,
  setColumnData: (columnData: ColumnEntry) => {
    set((state) => {
      let newColumnData = [...state.columnData, columnData];
      if (newColumnData.length > MAX_DATA_LENGTH + 1) {
        newColumnData = [
          newColumnData[0],
          ...newColumnData.slice(-MAX_DATA_LENGTH),
        ];
      }
      return {
        ...state,
        columnData: newColumnData,
        fileProgress: columnData.percentageComplete,
      };
    }, true);
  },
  setConnected: (connected: DataMode) => set(() => ({ connected })),
  setLoading: (isLoading: boolean) => set((state) => ({ ...state, isLoading })),
  setFilePath: (filePath: string) => set(() => ({ filePath })),
  setFileProgress: (fileProgress: number) => set(() => ({ fileProgress })),
  clearData: async () => {
    await commands.cancelColumnData();
    set(() => ({
      columnData: [],
      connected: "none",
      isLoading: false,
      filePath: "",
      fileProgress: 0,
    }));
  },
  refreshColumnData: async (newColumnData: ColumnEntry[]) =>
    set((state) => ({
      ...state,
      fileProgress: newColumnData.at(-1)?.percentageComplete ?? 0,
      columnData: [
        state.columnData[0],
        ...newColumnData.slice(-MAX_DATA_LENGTH),
      ],
    })),
}));
