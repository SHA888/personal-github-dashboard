import React, { useState, useEffect } from 'react';
import { Box, Typography, LinearProgress, Grid, Card, CardContent, styled, Tabs, Tab } from '@mui/material';
import { Line, Bar } from 'react-chartjs-2';
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    BarElement,
    Title,
    Tooltip,
    Legend,
} from 'chart.js';

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    BarElement,
    Title,
    Tooltip,
    Legend
);

const StatCard = styled(Card)(({ theme }) => ({
    height: '100%',
    display: 'flex',
    flexDirection: 'column',
}));

interface TabPanelProps {
    children?: React.ReactNode;
    index: number;
    value: number;
}

function TabPanel(props: TabPanelProps) {
    const { children, value, index, ...other } = props;

    return (
        <div
            role="tabpanel"
            hidden={value !== index}
            id={`trend-tabpanel-${index}`}
            aria-labelledby={`trend-tab-${index}`}
            {...other}
        >
            {value === index && <Box sx={{ p: 3 }}>{children}</Box>}
        </div>
    );
}

const Trends: React.FC = () => {
    const [trendData, setTrendData] = useState<any>(null);
    const [loading, setLoading] = useState(true);
    const [tabValue, setTabValue] = useState(0);

    useEffect(() => {
        const fetchTrendData = async () => {
            try {
                const response = await fetch('/api/analytics/trends');
                const data = await response.json();
                setTrendData(data);
            } catch (error) {
                console.error('Error fetching trend data:', error);
            } finally {
                setLoading(false);
            }
        };

        fetchTrendData();
    }, []);

    const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
        setTabValue(newValue);
    };

    if (loading) {
        return <LinearProgress />;
    }

    if (!trendData) {
        return <Typography>No trend data available</Typography>;
    }

    const { growth, velocity, burndown, release_cycle } = trendData;

    const growthChartData = {
        labels: growth.dates.map((date: string) => new Date(date).toLocaleDateString()),
        datasets: [
            {
                label: 'Growth Rate',
                data: growth.rates,
                borderColor: 'rgb(75, 192, 192)',
                tension: 0.1,
            },
        ],
    };

    const velocityChartData = {
        labels: velocity.dates.map((date: string) => new Date(date).toLocaleDateString()),
        datasets: [
            {
                label: 'Velocity',
                data: velocity.values,
                backgroundColor: 'rgba(54, 162, 235, 0.5)',
            },
        ],
    };

    const burndownChartData = {
        labels: burndown.dates.map((date: string) => new Date(date).toLocaleDateString()),
        datasets: [
            {
                label: 'Remaining Work',
                data: burndown.remaining,
                borderColor: 'rgb(255, 99, 132)',
                tension: 0.1,
            },
            {
                label: 'Ideal Burndown',
                data: burndown.ideal,
                borderColor: 'rgb(75, 192, 192)',
                borderDash: [5, 5],
                tension: 0.1,
            },
        ],
    };

    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                Trend Analysis
            </Typography>

            <Tabs value={tabValue} onChange={handleTabChange} aria-label="trend tabs">
                <Tab label="Growth" />
                <Tab label="Velocity" />
                <Tab label="Burndown" />
                <Tab label="Release Cycle" />
            </Tabs>

            <TabPanel value={tabValue} index={0}>
                <Grid container spacing={3}>
                    <Grid item xs={12}>
                        <StatCard>
                            <CardContent>
                                <Typography variant="h6" gutterBottom>
                                    Growth Rate Trend
                                </Typography>
                                <Box height={300}>
                                    <Line
                                        data={growthChartData}
                                        options={{
                                            responsive: true,
                                            maintainAspectRatio: false,
                                        }}
                                    />
                                </Box>
                            </CardContent>
                        </StatCard>
                    </Grid>
                </Grid>
            </TabPanel>

            <TabPanel value={tabValue} index={1}>
                <Grid container spacing={3}>
                    <Grid item xs={12}>
                        <StatCard>
                            <CardContent>
                                <Typography variant="h6" gutterBottom>
                                    Development Velocity
                                </Typography>
                                <Box height={300}>
                                    <Bar
                                        data={velocityChartData}
                                        options={{
                                            responsive: true,
                                            maintainAspectRatio: false,
                                        }}
                                    />
                                </Box>
                            </CardContent>
                        </StatCard>
                    </Grid>
                </Grid>
            </TabPanel>

            <TabPanel value={tabValue} index={2}>
                <Grid container spacing={3}>
                    <Grid item xs={12}>
                        <StatCard>
                            <CardContent>
                                <Typography variant="h6" gutterBottom>
                                    Burndown Chart
                                </Typography>
                                <Box height={300}>
                                    <Line
                                        data={burndownChartData}
                                        options={{
                                            responsive: true,
                                            maintainAspectRatio: false,
                                        }}
                                    />
                                </Box>
                            </CardContent>
                        </StatCard>
                    </Grid>
                </Grid>
            </TabPanel>

            <TabPanel value={tabValue} index={3}>
                <Grid container spacing={3}>
                    <Grid item xs={12} md={6}>
                        <StatCard>
                            <CardContent>
                                <Typography variant="h6" gutterBottom>
                                    Release Cycle Metrics
                                </Typography>
                                <Typography variant="h3">
                                    {release_cycle.average_days} days
                                </Typography>
                                <Typography color="textSecondary">
                                    Average Time to Release
                                </Typography>
                                <Typography variant="body2" mt={2}>
                                    Total Releases: {release_cycle.total_releases}
                                </Typography>
                                <Typography variant="body2">
                                    Releases per Year: {release_cycle.releases_per_year}
                                </Typography>
                            </CardContent>
                        </StatCard>
                    </Grid>
                </Grid>
            </TabPanel>
        </Box>
    );
};

export default Trends; 