import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import AnalyticsDashboard from "./components/analytics/AnalyticsDashboard";
import Layout from "./components/Layout";
import ErrorBoundary from "./components/ErrorBoundary";
import "./App.css";

const App: React.FC = () => {
  return (
    <ErrorBoundary>
      <Router>
        <Layout>
          <Routes>
            <Route path="/" element={<AnalyticsDashboard />} />
            <Route path="/analytics" element={<AnalyticsDashboard />} />
          </Routes>
        </Layout>
      </Router>
    </ErrorBoundary>
  );
};

export default App;
