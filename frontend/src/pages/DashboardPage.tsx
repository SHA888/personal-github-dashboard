import React, { useState } from 'react';
import { Box, Heading, Button, Text, TextInput } from '@primer/react';
import { useAuth } from '../hooks/useAuth';
import { useRepositories, RepositoryInfo } from '../hooks/useRepositories';
import RepositoryCard from '../components/RepositoryCard';

const DashboardPage: React.FC = () => {
  const { user, logout } = useAuth();
  const [page, setPage] = useState(1);
  const perPage = 10;
  const {
    data: repos,
    isLoading,
    isError,
    error,
  } = useRepositories(page, perPage);
  const reposList: RepositoryInfo[] = repos ?? [];
  const [search, setSearch] = useState('');
  const filteredRepos = reposList.filter(
    (repo) =>
      repo.name.toLowerCase().includes(search.toLowerCase()) ||
      (repo.description?.toLowerCase().includes(search.toLowerCase()) ?? false),
  );

  return (
    <Box sx={{ p: 4 }}>
      <Box
        sx={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          mb: 4,
        }}
      >
        <Heading>Dashboard</Heading>
        {user && (
          <Box sx={{ display: 'flex', alignItems: 'center' }}>
            <Text sx={{ mr: 2 }}>{user.login}</Text>
            <Button variant="default" onClick={logout}>
              Logout
            </Button>
          </Box>
        )}
      </Box>
      {isLoading && <Text>Loading repositories...</Text>}
      {isError && <Text>Error: {error?.message}</Text>}
      {/* Search filter */}
      <Box sx={{ mb: 3 }}>
        <TextInput
          placeholder="Search repositories"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </Box>
      {filteredRepos.map((repo) => (
        <RepositoryCard key={repo.name} repo={repo} />
      ))}
      <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 4 }}>
        <Button
          variant="default"
          onClick={() => setPage((p) => Math.max(1, p - 1))}
          disabled={page === 1}
        >
          Previous
        </Button>
        <Text>Page {page}</Text>
        <Button
          variant="default"
          onClick={() => setPage((p) => p + 1)}
          disabled={reposList.length < perPage}
        >
          Next
        </Button>
      </Box>
    </Box>
  );
};

export default DashboardPage;
