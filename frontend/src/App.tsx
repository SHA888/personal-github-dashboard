import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Layout from "./components/Layout";
import { WebSocketProvider } from "./components/WebSocketProvider";
import "./App.css";
import Dashboard from "./components/analytics/Dashboard";
import Repositories from "./pages/Repositories";
import Organizations from "./pages/Organizations";
import Security from "./pages/Security";
import Settings from "./pages/Settings";

const App: React.FC = () => {
  return (
    <WebSocketProvider>
      <Router>
        <Layout>
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/repositories" element={<Repositories />} />
            <Route path="/organizations" element={<Organizations />} />
            <Route path="/security" element={<Security />} />
            <Route path="/settings" element={<Settings />} />
          </Routes>
        </Layout>
      </Router>
    </WebSocketProvider>
  );
};

export default App;
