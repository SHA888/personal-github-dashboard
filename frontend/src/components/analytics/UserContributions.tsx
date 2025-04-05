import React, { useState, useEffect } from 'react';
import { Box, Typography, LinearProgress, Grid, Card, CardContent, Avatar, List, ListItem, ListItemAvatar, ListItemText, styled } from '@mui/material';
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

const UserContributions: React.FC = () => {
    const [contributionData, setContributionData] = useState<any>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchContributionData = async () => {
            try {
                const response = await fetch('/api/analytics/user-contributions');
                const data = await response.json();
                setContributionData(data);
            } catch (error) {
                console.error('Error fetching user contributions:', error);
            } finally {
                setLoading(false);
            }
        };

        fetchContributionData();
    }, []);

    if (loading) {
        return <LinearProgress />;
    }

    if (!contributionData) {
        return <Typography>No contribution data available</Typography>;
    }

    const { activity, codeChanges, topContributors } = contributionData;

    const activityChartData = {
        labels: activity.dates.map((date: string) => new Date(date).toLocaleDateString()),
        datasets: [
            {
                label: 'User Activity',
                data: activity.counts,
                backgroundColor: 'rgba(75, 192, 192, 0.6)',
            },
        ],
    };

    const codeChangesChartData = {
        labels: ['Additions', 'Deletions', 'Comments', 'Reviews'],
        datasets: [
            {
                label: 'Code Changes',
                data: [
                    codeChanges.additions,
                    codeChanges.deletions,
                    codeChanges.comments,
                    codeChanges.reviews,
                ],
                backgroundColor: [
                    'rgba(75, 192, 192, 0.6)',
                    'rgba(255, 99, 132, 0.6)',
                    'rgba(54, 162, 235, 0.6)',
                    'rgba(255, 206, 86, 0.6)',
                ],
            },
        ],
    };

    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                User Contributions
            </Typography>

            <Grid container spacing={3}>
                <Grid item xs={12}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Activity Trend
                            </Typography>
                            <Box height={300}>
                                <Bar
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

                <Grid item xs={12}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Code Changes
                            </Typography>
                            <Box height={300}>
                                <Bar
                                    data={codeChangesChartData}
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
                                Top Contributors
                            </Typography>
                            <List>
                                {topContributors.map((contributor: any) => (
                                    <ListItem key={contributor.id}>
                                        <ListItemAvatar>
                                            <Avatar src={contributor.avatar} alt={contributor.name} />
                                        </ListItemAvatar>
                                        <ListItemText
                                            primary={contributor.name}
                                            secondary={
                                                <>
                                                    <Typography component="span" variant="body2" color="text.primary">
                                                        {contributor.commits} commits
                                                    </Typography>
                                                    {` — ${contributor.additions.toLocaleString()} additions, ${contributor.deletions.toLocaleString()} deletions`}
                                                </>
                                            }
                                        />
                                    </ListItem>
                                ))}
                            </List>
                        </CardContent>
                    </StatCard>
                </Grid>
            </Grid>
        </Box>
    );
};

export default UserContributions; 