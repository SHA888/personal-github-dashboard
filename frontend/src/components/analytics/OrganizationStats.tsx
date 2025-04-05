import React, { useState, useEffect } from 'react';
import { Box, Typography, LinearProgress, Grid, Card, CardContent, Chip, styled } from '@mui/material';
import { Line, Bar, Doughnut } from 'react-chartjs-2';
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    BarElement,
    ArcElement,
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
    ArcElement,
    Title,
    Tooltip,
    Legend
);

const StatCard = styled(Card)(({ theme }) => ({
    height: '100%',
    display: 'flex',
    flexDirection: 'column',
}));

const OrganizationStats: React.FC = () => {
    const [orgData, setOrgData] = useState<any>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchOrgData = async () => {
            try {
                const response = await fetch('/api/analytics/organization-stats');
                const data = await response.json();
                setOrgData(data);
            } catch (error) {
                console.error('Error fetching organization data:', error);
            } finally {
                setLoading(false);
            }
        };

        fetchOrgData();
    }, []);

    if (loading) {
        return <LinearProgress />;
    }

    if (!orgData) {
        return <Typography>No organization data available</Typography>;
    }

    const { repositories, members, activity, languages } = orgData;

    const activityChartData = {
        labels: activity.dates.map((date: string) => new Date(date).toLocaleDateString()),
        datasets: [
            {
                label: 'Total Activity',
                data: activity.counts,
                borderColor: 'rgb(75, 192, 192)',
                tension: 0.1,
            },
        ],
    };

    const languageChartData = {
        labels: languages.map((lang: any) => lang.name),
        datasets: [
            {
                data: languages.map((lang: any) => lang.percentage),
                backgroundColor: languages.map((lang: any) => lang.color),
            },
        ],
    };

    const memberActivityChartData = {
        labels: ['Active', 'Inactive', 'New'],
        datasets: [
            {
                data: [
                    members.active,
                    members.total - members.active - members.new,
                    members.new,
                ],
                backgroundColor: [
                    'rgba(75, 192, 192, 0.6)',
                    'rgba(255, 99, 132, 0.6)',
                    'rgba(54, 162, 235, 0.6)',
                ],
            },
        ],
    };

    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                Organization Overview
            </Typography>

            <Grid container spacing={3}>
                <Grid item xs={12} md={6}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Activity Trend
                            </Typography>
                            <Box height={300}>
                                <Line
                                    data={activityChartData}
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
                                Language Distribution
                            </Typography>
                            <Box height={300}>
                                <Doughnut
                                    data={languageChartData}
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
                                Member Activity
                            </Typography>
                            <Box height={300}>
                                <Bar
                                    data={memberActivityChartData}
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
                                Repository Statistics
                            </Typography>
                            <Box display="flex" flexDirection="column" gap={2}>
                                <Box>
                                    <Typography variant="subtitle1">Total Repositories</Typography>
                                    <Typography variant="h4">{repositories.total}</Typography>
                                </Box>
                                <Box>
                                    <Typography variant="subtitle1">Active Repositories</Typography>
                                    <Typography variant="h4">{repositories.active}</Typography>
                                </Box>
                                <Box>
                                    <Typography variant="subtitle1">Top Repositories</Typography>
                                    <Box display="flex" flexWrap="wrap" gap={1} mt={1}>
                                        {repositories.top.map((repo: any) => (
                                            <Chip
                                                key={repo.name}
                                                label={`${repo.name} (${repo.stars}⭐)`}
                                                color="primary"
                                                variant="outlined"
                                            />
                                        ))}
                                    </Box>
                                </Box>
                            </Box>
                        </CardContent>
                    </StatCard>
                </Grid>
            </Grid>
        </Box>
    );
};

export default OrganizationStats; 