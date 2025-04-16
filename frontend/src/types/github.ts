export interface Repository {
  id: number;
  name: string;
  description: string | null;
  stars: number;
  forks: number;
  openIssues: number;
  openPRs: number;
  updatedAt: string;
}

export interface Notification {
  id: string;
  type: 'issue' | 'pr' | 'mention' | 'review';
  title: string;
  repository: string;
  unread: boolean;
  updatedAt: string;
}

export interface Metrics {
  contributionCount: number;
  issueResponseTime: number;
  prResponseTime: number;
  commitFrequency: {
    date: string;
    count: number;
  }[];
}
