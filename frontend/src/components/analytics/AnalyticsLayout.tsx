import React, { useState } from "react";
import { Box, TextField, MenuItem } from "@mui/material";
import RepositoryActivity from "./RepositoryActivity";
import ActivityTrends from "./ActivityTrends";
import { Filters } from "../../services/api";

interface AnalyticsLayoutProps {
  onFilterChange: (filters: Filters) => void;
}

const AnalyticsLayout: React.FC<AnalyticsLayoutProps> = ({
  onFilterChange,
}) => {
  const [filters, setFilters] = useState<Filters>({
    timeRange: "30",
    owner: "SHA888", // Default owner
    repo: "github-dashboard", // Default repo
  });

  const handleFilterChange = (field: keyof Filters, value: string) => {
    const newFilters = { ...filters, [field]: value };
    setFilters(newFilters);
    onFilterChange(newFilters);
  };

  return (
    <Box sx={{ display: "flex", flexDirection: "column", minHeight: "100vh" }}>
      {/* Filter Controls */}
      <Box sx={{ mb: 3, display: "flex", gap: 2, flexWrap: "wrap" }}>
        <TextField
          select
          label="Time Range"
          value={filters.timeRange}
          onChange={(e) => handleFilterChange("timeRange", e.target.value)}
          sx={{ minWidth: 200 }}
        >
          <MenuItem value="7">Last 7 days</MenuItem>
          <MenuItem value="30">Last 30 days</MenuItem>
          <MenuItem value="90">Last 90 days</MenuItem>
          <MenuItem value="180">Last 180 days</MenuItem>
          <MenuItem value="365">Last year</MenuItem>
        </TextField>
        <TextField
          label="Owner"
          value={filters.owner}
          onChange={(e) => handleFilterChange("owner", e.target.value)}
          sx={{ minWidth: 200 }}
        />
        <TextField
          label="Repository"
          value={filters.repo}
          onChange={(e) => handleFilterChange("repo", e.target.value)}
          sx={{ minWidth: 200 }}
        />
      </Box>

      {/* Main Content */}
      <Box sx={{ display: "flex", flexWrap: "wrap", gap: 3 }}>
        <Box sx={{ flex: "1 1 400px", minWidth: 0 }}>
          <RepositoryActivity filters={filters} />
        </Box>
        <Box sx={{ flex: "1 1 400px", minWidth: 0 }}>
          <ActivityTrends filters={filters} />
        </Box>
      </Box>
    </Box>
  );
};

export default AnalyticsLayout;
