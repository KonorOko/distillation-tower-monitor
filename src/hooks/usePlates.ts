import { create } from "zustand";

const MAX_PLATES = 8;
const MIN_PLATES = 1;

interface DataState {
  numberPlates: number;
  addPlate: () => void;
  removePlate: () => void;
  setPlates: (numberPlates: number) => void;
}

export const usePlates = create<DataState>((set) => ({
  numberPlates: 1,
  addPlate: () => {
    set((state) => {
      if (state.numberPlates < MAX_PLATES) {
        return { numberPlates: state.numberPlates + 1 };
      }
      return { numberPlates: state.numberPlates };
    });
  },
  removePlate: () => {
    set((state) => {
      if (state.numberPlates > MIN_PLATES) {
        return { numberPlates: state.numberPlates - 1 };
      }
      return { numberPlates: state.numberPlates };
    });
  },
  setPlates: (numberPlates: number) => {
    set({ numberPlates });
  },
}));
