import React, { useState, useEffect } from 'react';
import { Box, Typography, LinearProgress, Grid, Card, CardContent, CircularProgress, styled } from '@mui/material';
import { Line, Bar, Doughnut } from 'react-chartjs-2';
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    BarElement,
    PointElement,
    LineElement,
    ArcElement,
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

const Health: React.FC = () => {
    const [healthData, setHealthData] = useState<any>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchHealthData = async () => {
            try {
                const response = await fetch('/api/analytics/health');
                const data = await response.json();
                setHealthData(data);
            } catch (error) {
                console.error('Error fetching health data:', error);
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

    const { technicalDebt, testCoverage, dependencies, security } = healthData;

    const testCoverageChartData = {
        labels: testCoverage.dates.map((date: string) => new Date(date).toLocaleDateString()),
        datasets: [
            {
                label: 'Test Coverage',
                data: testCoverage.percentages,
                borderColor: 'rgb(75, 192, 192)',
                tension: 0.1,
            },
        ],
    };

    const dependenciesChartData = {
        labels: ['Up to Date', 'Outdated', 'Critical'],
        datasets: [
            {
                data: [
                    dependencies.upToDate,
                    dependencies.outdated,
                    dependencies.critical,
                ],
                backgroundColor: [
                    'rgba(75, 192, 192, 0.6)',
                    'rgba(255, 206, 86, 0.6)',
                    'rgba(255, 99, 132, 0.6)',
                ],
            },
        ],
    };

    const securityChartData = {
        labels: security.vulnerabilities.map((v: any) => v.name),
        datasets: [
            {
                label: 'Severity',
                data: security.vulnerabilities.map((v: any) => v.severity),
                backgroundColor: 'rgba(255, 99, 132, 0.6)',
            },
        ],
    };

    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                Code Health
            </Typography>

            <Grid container spacing={3}>
                <Grid item xs={12} md={6}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Technical Debt
                            </Typography>
                            <Box display="flex" alignItems="center" justifyContent="center" height={200}>
                                <CircularProgress
                                    variant="determinate"
                                    value={technicalDebt.score}
                                    size={120}
                                    thickness={4}
                                    sx={{
                                        color: technicalDebt.score > 80 ? 'success.main' : technicalDebt.score > 60 ? 'warning.main' : 'error.main',
                                    }}
                                />
                                <Box
                                    sx={{
                                        position: 'absolute',
                                        display: 'flex',
                                        alignItems: 'center',
                                        justifyContent: 'center',
                                    }}
                                >
                                    <Typography variant="h4" component="div" color="text.secondary">
                                        {`${technicalDebt.score}%`}
                                    </Typography>
                                </Box>
                            </Box>
                        </CardContent>
                    </StatCard>
                </Grid>

                <Grid item xs={12} md={6}>
                    <StatCard>
                        <CardContent>
                            <Typography variant="h6" gutterBottom>
                                Test Coverage Trend
                            </Typography>
                            <Box height={300}>
                                <Line
                                    data={testCoverageChartData}
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
                                Dependencies Status
                            </Typography>
                            <Box height={300}>
                                <Doughnut
                                    data={dependenciesChartData}
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
                                Security Vulnerabilities
                            </Typography>
                            <Box height={300}>
                                <Bar
                                    data={securityChartData}
                                    options={{
                                        responsive: true,
                                        maintainAspectRatio: false,
                                        indexAxis: 'y',
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

export default Health; 