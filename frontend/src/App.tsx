import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Dashboard from "./components/analytics/Dashboard";
import Layout from "./components/analytics/Layout";

const App: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-50 font-sans">
      <Router>
        <Routes>
          <Route path="/" element={<Layout />}>
            <Route index element={<Dashboard />} />
          </Route>
        </Routes>
      </Router>
    </div>
  );
};

export default App;
