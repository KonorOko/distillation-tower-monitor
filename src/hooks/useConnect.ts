import { create } from "zustand";

interface ConnectState {
  connected: boolean;
  loading: boolean;
  setLoading: (isLoading: boolean) => void;
  setConnected: (isConnected: boolean) => void;
}

export const useConnect = create<ConnectState>((set) => ({
  connected: false,
  loading: false,
  setLoading: (loading) => set({ loading }),
  setConnected: (connected) => set({ connected }),
}));
