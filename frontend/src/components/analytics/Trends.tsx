import React, { useState, useEffect } from 'react';
import { Box, Typography, LinearProgress, Grid, Card, CardContent, styled } from '@mui/material';
import { Line } from 'react-chartjs-2';
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
} from 'chart.js';

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
);

const StatCard = styled(Card)(({ theme }) => ({
    height: '100%',
    display: 'flex',
    flexDirection: 'column',
}));

interface Filters {
    timeRange: string;
    repository: string;
}

interface TrendsProps {
    filters: Filters;
}

const Trends: React.FC<TrendsProps> = ({ filters }) => {
    const [trendData, setTrendData] = useState<any>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchTrendData = async () => {
            try {
                const response = await fetch(`/api/analytics/trends?days=${filters.timeRange}`);
                const data = await response.json();
                setTrendData(data);
            } catch (error) {
                console.error('Error fetching trend data:', error);
            } finally {
                setLoading(false);
            }
        };

        fetchTrendData();
    }, [filters.timeRange]);

    if (loading) {
        return <LinearProgress />;
    }

    if (!trendData) {
        return <Typography>No trend data available</Typography>;
    }

    const { commits } = trendData;

    const commitChartData = {
        labels: commits.dates.map((date: string) => new Date(date).toLocaleDateString()),
        datasets: [
            {
                label: 'Daily Commits',
                data: commits.counts,
                borderColor: 'rgb(75, 192, 192)',
                tension: 0.1,
            },
        ],
    };

    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                Commit Trends
            </Typography>

            <Grid container spacing={3}>
                <Grid item xs={12}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Commit Activity
                            </Typography>
                            <Box height={300}>
                                <Line
                                    data={commitChartData}
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
        </Box>
    );
};

export default Trends; 