import React from "react";
import ReactDOM from "react-dom/client";
import { scan } from "react-scan";
import App from "./App";

scan({ enabled: false });

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
