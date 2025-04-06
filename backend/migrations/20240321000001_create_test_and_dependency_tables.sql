-- Create test_runs table
CREATE TABLE IF NOT EXISTS test_runs (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id),
    commit_sha VARCHAR(40) NOT NULL,
    total_tests INTEGER NOT NULL,
    passed_tests INTEGER NOT NULL,
    failed_tests INTEGER NOT NULL,
    skipped_tests INTEGER NOT NULL,
    coverage_percentage DECIMAL(5,2),
    duration_seconds INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(repository_id, commit_sha)
);

-- Create dependencies table
CREATE TABLE IF NOT EXISTS dependencies (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id),
    name VARCHAR(255) NOT NULL,
    version VARCHAR(50) NOT NULL,
    type VARCHAR(50) NOT NULL,
    is_outdated BOOLEAN DEFAULT FALSE,
    latest_version VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(repository_id, name, type)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_test_runs_repository_id ON test_runs(repository_id);
CREATE INDEX IF NOT EXISTS idx_test_runs_commit_sha ON test_runs(commit_sha);
CREATE INDEX IF NOT EXISTS idx_dependencies_repository_id ON dependencies(repository_id);
CREATE INDEX IF NOT EXISTS idx_dependencies_name ON dependencies(name);
CREATE INDEX IF NOT EXISTS idx_dependencies_type ON dependencies(type);

-- Create trigger for updating updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_dependencies_updated_at
    BEFORE UPDATE ON dependencies
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column(); 