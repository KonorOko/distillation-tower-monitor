import { Toaster } from "@/components/ui/sonner";
import { SettingsProvider } from "@/contexts/settings-context";
import { AppLayout } from "@/pages/app-layout";
import { DashboardPage } from "@/pages/dashboard-page";
import { BrowserRouter, Route, Routes } from "react-router";
import "./index.css";

function App() {
  return (
    <SettingsProvider>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<AppLayout />}>
            <Route index element={<DashboardPage />} />
          </Route>
        </Routes>
      </BrowserRouter>
      <Toaster />
    </SettingsProvider>
  );
}

export default App;
