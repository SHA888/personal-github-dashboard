import React, { useState, useEffect } from 'react';
import { Box, Typography, LinearProgress, Grid, Card, CardContent, styled } from '@mui/material';
import { Bar, Line } from 'react-chartjs-2';
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

const RepositoryActivity: React.FC = () => {
    const [activityData, setActivityData] = useState<any>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchActivityData = async () => {
            try {
                const response = await fetch('/api/analytics/repository-activity');
                const data = await response.json();
                setActivityData(data);
            } catch (error) {
                console.error('Error fetching repository activity:', error);
            } finally {
                setLoading(false);
            }
        };

        fetchActivityData();
    }, []);

    if (loading) {
        return <LinearProgress />;
    }

    if (!activityData) {
        return <Typography>No activity data available</Typography>;
    }

    const { dailyActivity, issues, pullRequests, commits, codeReview } = activityData;

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

    const issuesChartData = {
        labels: issues.dates.map((date: string) => new Date(date).toLocaleDateString()),
        datasets: [
            {
                label: 'Open Issues',
                data: issues.open,
                borderColor: 'rgb(255, 99, 132)',
                tension: 0.1,
            },
            {
                label: 'Closed Issues',
                data: issues.closed,
                borderColor: 'rgb(75, 192, 192)',
                tension: 0.1,
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

                <Grid item xs={12}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Issues Trend
                            </Typography>
                            <Box height={300}>
                                <Line
                                    data={issuesChartData}
                                    options={{
                                        responsive: true,
                                        maintainAspectRatio: false,
                                    }}
                                />
                            </Box>
                        </CardContent>
                    </StatCard>
                </Grid>

                <Grid item xs={12} md={4}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Pull Requests
                            </Typography>
                            <Box>
                                <Typography variant="h4">{pullRequests.total}</Typography>
                                <Typography color="text.secondary">
                                    {pullRequests.open} open, {pullRequests.merged} merged
                                </Typography>
                            </Box>
                        </CardContent>
                    </StatCard>
                </Grid>

                <Grid item xs={12} md={4}>
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

                <Grid item xs={12} md={4}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Code Review
                            </Typography>
                            <Box>
                                <Typography variant="h4">{codeReview.averageTime}h</Typography>
                                <Typography color="text.secondary">
                                    Average review time
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