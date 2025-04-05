import React, { useState, useEffect } from 'react';
import { Box, Grid, Paper, Typography, CircularProgress } from '@mui/material';
import { styled } from '@mui/material/styles';
import RepositoryActivity from './RepositoryActivity';
import UserContributions from './UserContributions';
import OrganizationStats from './OrganizationStats';
import CodeQuality from './CodeQuality';
import TeamPerformance from './TeamPerformance';
import Trends from './Trends';
import Collaboration from './Collaboration';
import Health from './Health';
import ProjectHealth from './ProjectHealth';
import AnalyticsLayout from './AnalyticsLayout';
import { useWebSocket } from '../../services/websocket';

const DashboardPaper = styled(Paper)(({ theme }) => ({
    padding: theme.spacing(3),
    marginBottom: theme.spacing(3),
    height: '100%',
    display: 'flex',
    flexDirection: 'column',
}));

interface Filters {
    timeRange: string;
    repository: string;
    team: string;
}

const AnalyticsDashboard: React.FC = () => {
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [filters, setFilters] = useState<Filters>({
        timeRange: '30',
        repository: 'all',
        team: 'all',
    });

    // Handle real-time updates for project health
    useWebSocket('project_health', (data) => {
        // Update project health data
        console.log('Received project health update:', data);
    });

    // Handle real-time updates for repository activity
    useWebSocket('repository_activity', (data) => {
        // Update repository activity data
        console.log('Received repository activity update:', data);
    });

    // Handle real-time updates for user contributions
    useWebSocket('user_contributions', (data) => {
        // Update user contributions data
        console.log('Received user contributions update:', data);
    });

    const handleFilterChange = (newFilters: Filters) => {
        setFilters(newFilters);
        // Here you would typically trigger a refetch of data with the new filters
        console.log('Filters changed:', newFilters);
    };

    useEffect(() => {
        // Initial data fetch
        const fetchData = async () => {
            try {
                // Fetch initial data here
                setLoading(false);
            } catch (err) {
                setError('Failed to load analytics data');
                setLoading(false);
            }
        };

        fetchData();
    }, []);

    if (loading) {
        return (
            <Box display="flex" justifyContent="center" alignItems="center" minHeight="100vh">
                <CircularProgress />
            </Box>
        );
    }

    if (error) {
        return (
            <Box display="flex" justifyContent="center" alignItems="center" minHeight="100vh">
                <Typography color="error">{error}</Typography>
            </Box>
        );
    }

    return (
        <Box sx={{ flexGrow: 1 }}>
            <AnalyticsLayout onFilterChange={handleFilterChange} />
        </Box>
    );
};

export default AnalyticsDashboard; 