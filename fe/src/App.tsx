// @ts-ignore
import SideNav from "./components/sidenav/sidenav.tsx";
import {ApiClient} from "./lib/client.ts";
import { Routes, Route } from "react-router-dom";
import HealthCheck from "./components/healthcheck/healthcheck.tsx";
import React from "react";

const client = (param: string): any => {
    const apiClient = new ApiClient();
    return apiClient.get(param).then((res) => res )
}

export default function App() {

    return (
        <div className="App">
            <Routes>

            </Routes>
        </div>
    )
}