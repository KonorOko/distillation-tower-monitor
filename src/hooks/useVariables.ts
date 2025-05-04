import { create } from "zustand";

const MAX_PLATES = 15;
const MIN_PLATES = 1;

interface DataState {
  numberPlates: number;
  initialMass: number | undefined;
  initialComposition: number | undefined;
  addPlate: () => void;
  removePlate: () => void;
  setPlates: (numberPlates: number) => void;
  setInitialMass: (initialMass: number) => void;
  setInitialComposition: (initialComposition: number) => void;
}

export const useVariables = create<DataState>((set) => ({
  numberPlates: 1,
  initialMass: undefined,
  initialComposition: undefined,
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
  setInitialMass: (initialMass: number) => {
    set({ initialMass });
  },
  setInitialComposition: (initialComposition: number) => {
    set({ initialComposition });
  },
}));
