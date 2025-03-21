import { useSettings } from "@/hooks/useSettings";
import { useEffect } from "react";
import { Outlet } from "react-router";

export function AppLayout() {
  const { settings, loadSettings } = useSettings();

  useEffect(() => {
    const getSettings = async () => {
      loadSettings();
    };
    getSettings();
  }, []);

  if (!settings) {
    return null;
  }

  return (
    <main className="flex min-h-screen">
      <Outlet />
    </main>
  );
}
