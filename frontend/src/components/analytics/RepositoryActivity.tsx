import React, { useState, useEffect } from 'react';
import { Box, Typography, LinearProgress, Grid, Card, CardContent, styled } from '@mui/material';
import { Bar } from 'react-chartjs-2';
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    BarElement,
    Title,
    Tooltip,
    Legend,
} from 'chart.js';

ChartJS.register(
    CategoryScale,
    LinearScale,
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

interface Filters {
    timeRange: string;
    repository: string;
}

interface RepositoryActivityProps {
    filters: Filters;
}

const RepositoryActivity: React.FC<RepositoryActivityProps> = ({ filters }) => {
    const [activityData, setActivityData] = useState<any>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchActivityData = async () => {
            try {
                const response = await fetch(`/api/analytics/repository-activity?days=${filters.timeRange}`);
                const data = await response.json();
                setActivityData(data);
            } catch (error) {
                console.error('Error fetching repository activity:', error);
            } finally {
                setLoading(false);
            }
        };

        fetchActivityData();
    }, [filters.timeRange]);

    if (loading) {
        return <LinearProgress />;
    }

    if (!activityData) {
        return <Typography>No activity data available</Typography>;
    }

    const { dailyActivity, commits } = activityData;

    const dailyActivityChartData = {
        labels: dailyActivity.dates.map((date: string) => new Date(date).toLocaleDateString()),
        datasets: [
            {
                label: 'Daily Activity',
                data: dailyActivity.counts,
                backgroundColor: 'rgba(75, 192, 192, 0.6)',
            },
        ],
    };

    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                Repository Activity
            </Typography>

            <Grid container spacing={3}>
                <Grid item xs={12}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Daily Activity
                            </Typography>
                            <Box height={300}>
                                <Bar
                                    data={dailyActivityChartData}
                                    options={{
                                        responsive: true,
                                        maintainAspectRatio: false,
                                    }}
                                />
                            </Box>
                        </CardContent>
                    </StatCard>
                </Grid>

                <Grid item xs={12} md={6}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Commits
                            </Typography>
                            <Box>
                                <Typography variant="h4">{commits.total}</Typography>
                                <Typography color="text.secondary">
                                    {commits.authors} unique authors
                                </Typography>
                            </Box>
                        </CardContent>
                    </StatCard>
                </Grid>
            </Grid>
        </Box>
    );
};

export default RepositoryActivity; 