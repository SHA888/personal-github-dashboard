import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { Provider } from "react-redux";
import { store } from "./store";
import AnalyticsDashboard from "./components/analytics/AnalyticsDashboard";
import Layout from "./components/Layout";
import ErrorBoundary from "./components/ErrorBoundary";
import { WebSocketProvider } from "./components/WebSocketProvider";
import "./App.css";

// Test comment for pre-commit hooks
const App: React.FC = () => {
  return (
    <Provider store={store}>
      <ErrorBoundary>
        <WebSocketProvider>
          <Router>
            <Layout>
              <Routes>
                <Route path="/" element={<AnalyticsDashboard />} />
                <Route path="/analytics" element={<AnalyticsDashboard />} />
              </Routes>
            </Layout>
          </Router>
        </WebSocketProvider>
      </ErrorBoundary>
    </Provider>
  );
};

export default App;
