import React, { useState, useEffect } from 'react';
import { Box, Typography, LinearProgress, Grid, Card, CardContent, Chip, styled } from '@mui/material';
import { Line, Bar } from 'react-chartjs-2';
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
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
);

const HealthScoreBox = styled(Box)(({ theme }) => ({
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    padding: theme.spacing(3),
    borderRadius: theme.shape.borderRadius,
    backgroundColor: theme.palette.background.paper,
}));

const getHealthColor = (score: number) => {
    if (score >= 80) return 'success';
    if (score >= 60) return 'warning';
    return 'error';
};

const ProjectHealth: React.FC = () => {
    const [healthData, setHealthData] = useState<any>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchHealthData = async () => {
            try {
                const response = await fetch('/api/analytics/project-health');
                const data = await response.json();
                setHealthData(data);
            } catch (error) {
                console.error('Error fetching project health:', error);
            } finally {
                setLoading(false);
            }
        };

        fetchHealthData();
    }, []);

    if (loading) {
        return <LinearProgress />;
    }

    if (!healthData) {
        return <Typography>No health data available</Typography>;
    }

    const { overallScore, componentScores, trend, recommendations } = healthData;

    const trendChartData = {
        labels: trend.dates.map((date: string) => new Date(date).toLocaleDateString()),
        datasets: [
            {
                label: 'Health Score',
                data: trend.scores,
                borderColor: 'rgb(75, 192, 192)',
                tension: 0.1,
            },
        ],
    };

    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                Project Health
            </Typography>

            <Grid container spacing={3}>
                <Grid item xs={12} md={4}>
                    <HealthScoreBox>
                        <Typography variant="h6" gutterBottom>
                            Overall Health Score
                        </Typography>
                        <Typography
                            variant="h2"
                            color={`${getHealthColor(overallScore)}.main`}
                        >
                            {overallScore}
                        </Typography>
                    </HealthScoreBox>
                </Grid>

                <Grid item xs={12} md={8}>
                    <Card>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Component Scores
                            </Typography>
                            <Box display="flex" flexWrap="wrap" gap={1}>
                                {Object.entries(componentScores).map(([component, score]) => (
                                    <Chip
                                        key={component}
                                        label={`${component}: ${score}`}
                                        color={getHealthColor(score as number)}
                                        variant="outlined"
                                    />
                                ))}
                            </Box>
                        </CardContent>
                    </Card>
                </Grid>

                <Grid item xs={12}>
                    <Card>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Health Trend
                            </Typography>
                            <Box height={300}>
                                <Line
                                    data={trendChartData}
                                    options={{
                                        responsive: true,
                                        maintainAspectRatio: false,
                                    }}
                                />
                            </Box>
                        </CardContent>
                    </Card>
                </Grid>

                <Grid item xs={12}>
                    <Card>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Recommendations
                            </Typography>
                            <Box component="ul" sx={{ pl: 2 }}>
                                {recommendations.map((rec: string, index: number) => (
                                    <Typography component="li" key={index}>
                                        {rec}
                                    </Typography>
                                ))}
                            </Box>
                        </CardContent>
                    </Card>
                </Grid>
            </Grid>
        </Box>
    );
};

export default ProjectHealth; 