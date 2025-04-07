import { createSlice, createAsyncThunk } from "@reduxjs/toolkit";
import { apiService, ActivityData } from "../../services/api";

interface AnalyticsState {
  data: ActivityData | null;
  loading: boolean;
  error: string | null;
  timeRange: string;
}

const initialState: AnalyticsState = {
  data: null,
  loading: false,
  error: null,
  timeRange: "weekly",
};

export const fetchAnalytics = createAsyncThunk(
  "analytics/fetchAnalytics",
  async ({ owner, repo }: { owner: string; repo: string }) => {
    const response = await apiService.getRepositoryAnalytics(owner, repo);
    return response.data;
  },
);

export const fetchActivityTrends = createAsyncThunk(
  "analytics/fetchActivityTrends",
  async ({
    owner,
    repo,
    timeRange,
  }: {
    owner: string;
    repo: string;
    timeRange: string;
  }) => {
    const response = await apiService.getActivityTrends(owner, repo, timeRange);
    return response.data;
  },
);

const analyticsSlice = createSlice({
  name: "analytics",
  initialState,
  reducers: {
    setTimeRange: (state, action) => {
      state.timeRange = action.payload;
    },
    clearError: (state) => {
      state.error = null;
    },
  },
  extraReducers: (builder) => {
    builder
      // Fetch Analytics
      .addCase(fetchAnalytics.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(fetchAnalytics.fulfilled, (state, action) => {
        state.loading = false;
        state.data = action.payload;
      })
      .addCase(fetchAnalytics.rejected, (state, action) => {
        state.loading = false;
        state.error = action.error.message || "Failed to fetch analytics";
      })
      // Fetch Activity Trends
      .addCase(fetchActivityTrends.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(fetchActivityTrends.fulfilled, (state, action) => {
        state.loading = false;
        state.data = action.payload;
      })
      .addCase(fetchActivityTrends.rejected, (state, action) => {
        state.loading = false;
        state.error = action.error.message || "Failed to fetch activity trends";
      });
  },
});

export const { setTimeRange, clearError } = analyticsSlice.actions;
export default analyticsSlice.reducer;
