import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import { apiService, Repository } from '../../services/api';

interface RepositoryState {
    repositories: Repository[];
    selectedRepository: Repository | null;
    loading: boolean;
    error: string | null;
}

const initialState: RepositoryState = {
    repositories: [],
    selectedRepository: null,
    loading: false,
    error: null,
};

export const fetchRepositories = createAsyncThunk(
    'repository/fetchRepositories',
    async () => {
        const response = await apiService.listRepositories();
        return response.data.repositories;
    }
);

export const fetchRepositoryDetails = createAsyncThunk(
    'repository/fetchRepositoryDetails',
    async (id: number) => {
        return await apiService.getRepositoryDetails(id);
    }
);

export const addRepository = createAsyncThunk(
    'repository/addRepository',
    async ({ owner, repo }: { owner: string; repo: string }) => {
        return await apiService.addRepository(owner, repo);
    }
);

export const removeRepository = createAsyncThunk(
    'repository/removeRepository',
    async (id: number) => {
        await apiService.removeRepository(id);
        return id;
    }
);

const repositorySlice = createSlice({
    name: 'repository',
    initialState,
    reducers: {
        clearError: (state) => {
            state.error = null;
        },
    },
    extraReducers: (builder) => {
        builder
            // Fetch Repositories
            .addCase(fetchRepositories.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(fetchRepositories.fulfilled, (state, action) => {
                state.loading = false;
                state.repositories = action.payload;
            })
            .addCase(fetchRepositories.rejected, (state, action) => {
                state.loading = false;
                state.error = action.error.message || 'Failed to fetch repositories';
            })
            // Fetch Repository Details
            .addCase(fetchRepositoryDetails.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(fetchRepositoryDetails.fulfilled, (state, action) => {
                state.loading = false;
                state.selectedRepository = action.payload;
            })
            .addCase(fetchRepositoryDetails.rejected, (state, action) => {
                state.loading = false;
                state.error = action.error.message || 'Failed to fetch repository details';
            })
            // Add Repository
            .addCase(addRepository.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(addRepository.fulfilled, (state, action) => {
                state.loading = false;
                state.repositories.push(action.payload);
            })
            .addCase(addRepository.rejected, (state, action) => {
                state.loading = false;
                state.error = action.error.message || 'Failed to add repository';
            })
            // Remove Repository
            .addCase(removeRepository.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(removeRepository.fulfilled, (state, action) => {
                state.loading = false;
                state.repositories = state.repositories.filter(repo => repo.id !== action.payload);
                if (state.selectedRepository?.id === action.payload) {
                    state.selectedRepository = null;
                }
            })
            .addCase(removeRepository.rejected, (state, action) => {
                state.loading = false;
                state.error = action.error.message || 'Failed to remove repository';
            });
    },
});

export const { clearError } = repositorySlice.actions;
export default repositorySlice.reducer; 