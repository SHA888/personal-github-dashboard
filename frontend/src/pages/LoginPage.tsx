import React from 'react';
import AuthButtons from '../components/Auth/AuthButtons';
import PatForm from '../components/Auth/PatForm';
import { Box, Typography } from '@mui/material';

const LoginPage: React.FC = () => (
  <Box display="flex" flexDirection="column" alignItems="center" mt={5} gap={3}>
    <Typography variant="h4" gutterBottom>
      Login
    </Typography>
    <AuthButtons />
    <PatForm />
  </Box>
);

export default LoginPage;
