import React, { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./app.css";

// @ts-ignore
import App from "./App.tsx";
import {BrowserRouter} from "react-router-dom";

const root = createRoot(document.getElementById("root"));
root.render(
    <StrictMode>
        <BrowserRouter>
            <App />
        </BrowserRouter>
    </StrictMode>
);