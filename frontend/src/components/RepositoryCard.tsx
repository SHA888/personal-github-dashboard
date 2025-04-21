import React from 'react';
import { Box, Heading, Text, Link } from '@primer/react';
import {
  StarIcon,
  RepoForkedIcon,
  IssueOpenedIcon,
} from '@primer/octicons-react';
import { RepositoryInfo } from '../hooks/useRepositories';

interface RepositoryCardProps {
  repo: RepositoryInfo;
}

const RepositoryCard: React.FC<RepositoryCardProps> = ({ repo }) => {
  return (
    <Box
      sx={{
        border: '1px solid',
        borderColor: 'border.default',
        borderRadius: 2,
        p: 3,
        mb: 3,
        bg: 'canvas.default',
      }}
    >
      <Heading sx={{ fontSize: 18, mb: 1 }}>
        <Link href={repo.html_url} target="_blank" rel="noopener noreferrer">
          {repo.name}
        </Link>
      </Heading>
      {repo.description && (
        <Text sx={{ color: 'fg.muted', mb: 2 }}>{repo.description}</Text>
      )}
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 3 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <StarIcon size={16} />
          <Text as="span">{repo.stargazers_count}</Text>
        </Box>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <RepoForkedIcon size={16} />
          <Text as="span">{repo.forks_count}</Text>
        </Box>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <IssueOpenedIcon size={16} />
          <Text as="span">{repo.open_issues_count}</Text>
        </Box>
      </Box>
    </Box>
  );
};

export default RepositoryCard;
