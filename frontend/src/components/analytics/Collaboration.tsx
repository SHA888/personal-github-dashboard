import React, { useState, useEffect } from 'react';
import { Box, Typography, LinearProgress, Grid, Card, CardContent, List, ListItem, ListItemText, Avatar, Chip, styled } from '@mui/material';
import { Bar, Line } from 'react-chartjs-2';
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    BarElement,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
} from 'chart.js';

ChartJS.register(
    CategoryScale,
    LinearScale,
    BarElement,
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

const Collaboration: React.FC = () => {
    const [collabData, setCollabData] = useState<any>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchCollabData = async () => {
            try {
                const response = await fetch('/api/analytics/collaboration');
                const data = await response.json();
                setCollabData(data);
            } catch (error) {
                console.error('Error fetching collaboration data:', error);
            } finally {
                setLoading(false);
            }
        };

        fetchCollabData();
    }, []);

    if (loading) {
        return <LinearProgress />;
    }

    if (!collabData) {
        return <Typography>No collaboration data available</Typography>;
    }

    const { team_interactions, review_times, review_distribution, cross_team } = collabData;

    const reviewTimeChartData = {
        labels: review_times.dates.map((date: string) => new Date(date).toLocaleDateString()),
        datasets: [
            {
                label: 'Average Review Time (hours)',
                data: review_times.average_times,
                borderColor: 'rgb(75, 192, 192)',
                tension: 0.1,
            },
        ],
    };

    const reviewDistributionChartData = {
        labels: review_distribution.map((item: any) => item.reviewer),
        datasets: [
            {
                label: 'Reviews per Reviewer',
                data: review_distribution.map((item: any) => item.review_count),
                backgroundColor: 'rgba(54, 162, 235, 0.5)',
            },
        ],
    };

    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                Team Collaboration
            </Typography>

            <Grid container spacing={3}>
                <Grid item xs={12} md={6}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Review Response Times
                            </Typography>
                            <Box height={300}>
                                <Line
                                    data={reviewTimeChartData}
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
                                Review Distribution
                            </Typography>
                            <Box height={300}>
                                <Bar
                                    data={reviewDistributionChartData}
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
                                Team Interactions
                            </Typography>
                            <List>
                                {team_interactions.map((interaction: any, index: number) => (
                                    <ListItem key={index}>
                                        <Avatar sx={{ mr: 2 }}>{interaction.user1[0]}</Avatar>
                                        <ListItemText
                                            primary={`${interaction.user1} ↔ ${interaction.user2}`}
                                            secondary={`${interaction.count} interactions`}
                                        />
                                    </ListItem>
                                ))}
                            </List>
                        </CardContent>
                    </StatCard>
                </Grid>

                <Grid item xs={12} md={6}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Cross-Team Collaboration
                            </Typography>
                            <Box display="flex" flexWrap="wrap" gap={1}>
                                {cross_team.map((team: any, index: number) => (
                                    <Chip
                                        key={index}
                                        label={`${team.team1} ↔ ${team.team2}: ${team.shared_contributors} shared`}
                                        color="primary"
                                        variant="outlined"
                                    />
                                ))}
                            </Box>
                            <Typography variant="body2" mt={2}>
                                Total Cross-Team Contributors: {cross_team.reduce((acc: number, team: any) => acc + team.shared_contributors, 0)}
                            </Typography>
                        </CardContent>
                    </StatCard>
                </Grid>
            </Grid>
        </Box>
    );
};

export default Collaboration; 