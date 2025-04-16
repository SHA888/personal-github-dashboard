export interface User {
  id: string;
  github_id: number;
  login: string;
  name?: string;
  email?: string;
  avatar_url?: string;
  html_url?: string;
  created_at: string;
  updated_at: string;
  last_synced_at?: string;
}
