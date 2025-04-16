import React from 'react';
import { Box, Typography } from '@mui/material';

const Settings: React.FC = () => {
  return (
    <Box p={3}>
      <Typography variant="h4" gutterBottom>
        Settings
      </Typography>
      <Typography variant="body1">
        Application settings and preferences will be available here.
      </Typography>
    </Box>
  );
};

export default Settings;
