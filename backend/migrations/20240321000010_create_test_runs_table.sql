-- Create test_runs table
CREATE TABLE IF NOT EXISTS test_runs (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    commit_sha VARCHAR(40) NOT NULL,
    total_tests INTEGER NOT NULL,
    passed_tests INTEGER NOT NULL,
    failed_tests INTEGER NOT NULL,
    skipped_tests INTEGER NOT NULL,
    coverage_percentage DECIMAL(5,2) NOT NULL,
    duration_seconds INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_test_runs_repository_id ON test_runs(repository_id);
CREATE INDEX IF NOT EXISTS idx_test_runs_commit_sha ON test_runs(commit_sha);

-- Create trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_test_runs_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_test_runs_updated_at
    BEFORE UPDATE ON test_runs
    FOR EACH ROW
    EXECUTE FUNCTION update_test_runs_updated_at(); 