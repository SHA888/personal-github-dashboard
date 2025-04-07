-- Add more repository metadata
ALTER TABLE repositories
ADD COLUMN IF NOT EXISTS description TEXT,
ADD COLUMN IF NOT EXISTS language VARCHAR(100),
ADD COLUMN IF NOT EXISTS stars INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS forks INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS open_issues INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS is_private BOOLEAN DEFAULT false;

-- Create activity_types table
CREATE TABLE IF NOT EXISTS activity_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT
);

-- Insert default activity types
INSERT INTO activity_types (name, description) VALUES
    ('commit', 'Code commit'),
    ('pull_request', 'Pull request activity'),
    ('issue', 'Issue activity'),
    ('review', 'Code review activity')
ON CONFLICT (name) DO NOTHING;

-- Modify analytics_data table to use activity_types
ALTER TABLE analytics_data
ADD COLUMN IF NOT EXISTS activity_type_id INTEGER REFERENCES activity_types(id);

-- Create tasks table
CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER REFERENCES repositories(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'open',
    priority INTEGER NOT NULL DEFAULT 0,
    due_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create task_assignees table for many-to-many relationship
CREATE TABLE IF NOT EXISTS task_assignees (
    task_id INTEGER REFERENCES tasks(id) ON DELETE CASCADE,
    github_username VARCHAR(255) NOT NULL,
    PRIMARY KEY (task_id, github_username)
);

-- Create task_labels table for many-to-many relationship
CREATE TABLE IF NOT EXISTS task_labels (
    task_id INTEGER REFERENCES tasks(id) ON DELETE CASCADE,
    label_name VARCHAR(255) NOT NULL,
    PRIMARY KEY (task_id, label_name)
);

-- Create notifications table for real-time updates
CREATE TABLE IF NOT EXISTS notifications (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER REFERENCES repositories(id),
    type VARCHAR(50) NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_read BOOLEAN DEFAULT false
);

-- Add indexes for task management
CREATE INDEX IF NOT EXISTS idx_tasks_repository_id ON tasks(repository_id);
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority);
CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date);

-- Add indexes for activity tracking
CREATE INDEX IF NOT EXISTS idx_analytics_data_activity_type ON analytics_data(activity_type_id);

-- Add indexes for notifications
CREATE INDEX IF NOT EXISTS idx_notifications_repository_id ON notifications(repository_id);
CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at);
