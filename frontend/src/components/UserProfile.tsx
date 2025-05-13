import React from 'react';
import { useSelector } from 'react-redux';
import { RootState } from '../store';
import { Box, Avatar, Text, Heading } from '@primer/react';

const UserProfile: React.FC = () => {
  const user = useSelector((state: RootState) => state.auth.user);

  if (!user) {
    return <Text>No user info available.</Text>;
  }

  return (
    <Box
      sx={{
        p: 3,
        border: '1px solid',
        borderColor: 'border.default',
        borderRadius: 2,
        maxWidth: 400,
      }}
    >
      <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
        {user.avatar_url && (
          <Avatar src={user.avatar_url} size={48} sx={{ mr: 2 }} />
        )}
        <Heading as="h3" sx={{ mb: 0 }}>
          {user.login}
        </Heading>
      </Box>
      {user.email && <Text>Email: {user.email}</Text>}
    </Box>
  );
};

export default UserProfile;
