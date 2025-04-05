import React, { useState, useEffect } from 'react';
import { Box, Grid, Paper, Typography, Tabs, Tab, TextField, MenuItem, Button } from '@mui/material';
import { styled } from '@mui/material/styles';
import ProjectHealth from './ProjectHealth';
import RepositoryActivity from './RepositoryActivity';
import UserContributions from './UserContributions';
import OrganizationStats from './OrganizationStats';
import Collaboration from './Collaboration';
import Health from './Health';
import Trends from './Trends';

const DashboardPaper = styled(Paper)(({ theme }) => ({
    padding: theme.spacing(3),
    marginBottom: theme.spacing(3),
    height: '100%',
    display: 'flex',
    flexDirection: 'column',
}));

interface AnalyticsLayoutProps {
    onFilterChange: (filters: any) => void;
}

const AnalyticsLayout: React.FC<AnalyticsLayoutProps> = ({ onFilterChange }) => {
    const [activeTab, setActiveTab] = useState(0);
    const [filters, setFilters] = useState({
        timeRange: '30',
        repository: 'all',
        team: 'all',
    });

    const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
        setActiveTab(newValue);
    };

    const handleFilterChange = (field: string, value: string) => {
        const newFilters = { ...filters, [field]: value };
        setFilters(newFilters);
        onFilterChange(newFilters);
    };

    return (
        <Box sx={{ flexGrow: 1, p: 3 }}>
            {/* Filter Controls */}
            <Box sx={{ mb: 3, display: 'flex', gap: 2, flexWrap: 'wrap' }}>
                <TextField
                    select
                    label="Time Range"
                    value={filters.timeRange}
                    onChange={(e) => handleFilterChange('timeRange', e.target.value)}
                    sx={{ minWidth: 200 }}
                >
                    <MenuItem value="7">Last 7 days</MenuItem>
                    <MenuItem value="30">Last 30 days</MenuItem>
                    <MenuItem value="90">Last 90 days</MenuItem>
                    <MenuItem value="180">Last 180 days</MenuItem>
                    <MenuItem value="365">Last year</MenuItem>
                </TextField>
                <TextField
                    select
                    label="Repository"
                    value={filters.repository}
                    onChange={(e) => handleFilterChange('repository', e.target.value)}
                    sx={{ minWidth: 200 }}
                >
                    <MenuItem value="all">All Repositories</MenuItem>
                    {/* Repository options will be populated dynamically */}
                </TextField>
                <TextField
                    select
                    label="Team"
                    value={filters.team}
                    onChange={(e) => handleFilterChange('team', e.target.value)}
                    sx={{ minWidth: 200 }}
                >
                    <MenuItem value="all">All Teams</MenuItem>
                    {/* Team options will be populated dynamically */}
                </TextField>
            </Box>

            {/* Main Content */}
            <Grid container spacing={3}>
                {/* Project Health Overview */}
                <Grid item xs={12}>
                    <DashboardPaper>
                        <ProjectHealth />
                    </DashboardPaper>
                </Grid>

                {/* Repository Activity and Trends */}
                <Grid item xs={12} md={6}>
                    <DashboardPaper>
                        <RepositoryActivity />
                    </DashboardPaper>
                </Grid>

                <Grid item xs={12} md={6}>
                    <DashboardPaper>
                        <Trends />
                    </DashboardPaper>
                </Grid>

                {/* User Contributions and Team Performance */}
                <Grid item xs={12} md={6}>
                    <DashboardPaper>
                        <UserContributions />
                    </DashboardPaper>
                </Grid>

                <Grid item xs={12} md={6}>
                    <DashboardPaper>
                        <Collaboration />
                    </DashboardPaper>
                </Grid>

                {/* Organization Stats and Health */}
                <Grid item xs={12} md={6}>
                    <DashboardPaper>
                        <OrganizationStats />
                    </DashboardPaper>
                </Grid>

                <Grid item xs={12} md={6}>
                    <DashboardPaper>
                        <Health />
                    </DashboardPaper>
                </Grid>
            </Grid>
        </Box>
    );
};

export default AnalyticsLayout; 