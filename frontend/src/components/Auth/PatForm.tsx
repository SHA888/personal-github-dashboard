import React, { useState } from 'react';
import axios from 'axios';
import { TextField, Button, Box, Typography } from '@mui/material';
import { useNavigate } from 'react-router-dom';

const PatForm: React.FC = () => {
  const [pat, setPat] = useState('');
  const [error, setError] = useState('');
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const resp = await axios.post(
        `${import.meta.env.VITE_API_BASE_URL}/auth/pat`,
        { pat },
        { withCredentials: true },
      );
      const { jwt } = resp.data;
      localStorage.setItem('auth_token', jwt);
      navigate('/');
    } catch (err) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const e = err as any;
      setError(e.response?.data || 'Invalid token');
    }
  };

  return (
    <Box component="form" onSubmit={handleSubmit} sx={{ mt: 2 }}>
      <Typography variant="h6">Login with Personal Access Token</Typography>
      <TextField
        label="Personal Access Token"
        variant="outlined"
        fullWidth
        value={pat}
        onChange={(e) => setPat(e.target.value)}
        margin="normal"
      />
      {error && <Typography color="error">{error}</Typography>}
      <Button type="submit" variant="contained" color="primary">
        Login
      </Button>
    </Box>
  );
};

export default PatForm;
