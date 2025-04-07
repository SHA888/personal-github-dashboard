import React from "react";
import { Box, Typography } from "@mui/material";

const Security: React.FC = () => {
  return (
    <Box p={3}>
      <Typography variant="h4" gutterBottom>
        Security
      </Typography>
      <Typography variant="body1">
        Security settings and configurations will be available here.
      </Typography>
    </Box>
  );
};

export default Security;
