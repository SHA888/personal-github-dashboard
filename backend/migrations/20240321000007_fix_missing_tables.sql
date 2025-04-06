-- Add organization_id to repositories table
ALTER TABLE repositories ADD COLUMN IF NOT EXISTS organization_id INTEGER REFERENCES organizations(id);

-- Create pull_request_reviews table
CREATE TABLE IF NOT EXISTS pull_request_reviews (
    id SERIAL PRIMARY KEY,
    pull_request_id INTEGER NOT NULL REFERENCES pull_requests(id),
    reviewer_id INTEGER NOT NULL REFERENCES users(id),
    state VARCHAR(20) NOT NULL CHECK (state IN ('approved', 'changes_requested', 'commented')),
    body TEXT,
    submitted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create review_comments table
CREATE TABLE IF NOT EXISTS review_comments (
    id SERIAL PRIMARY KEY,
    review_id INTEGER NOT NULL REFERENCES pull_request_reviews(id),
    body TEXT NOT NULL,
    path TEXT,
    position INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create test_runs table
CREATE TABLE IF NOT EXISTS test_runs (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id),
    commit_sha VARCHAR(40) NOT NULL,
    total_tests INTEGER NOT NULL,
    passed_tests INTEGER NOT NULL,
    failed_tests INTEGER NOT NULL,
    skipped_tests INTEGER NOT NULL,
    coverage_percentage DECIMAL(5,2) NOT NULL,
    duration_seconds INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create dependencies table
CREATE TABLE IF NOT EXISTS dependencies (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id),
    name VARCHAR(255) NOT NULL,
    version VARCHAR(50) NOT NULL,
    type VARCHAR(50) NOT NULL,
    is_outdated BOOLEAN NOT NULL DEFAULT false,
    latest_version VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create security_vulnerabilities table
CREATE TABLE IF NOT EXISTS security_vulnerabilities (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id),
    severity VARCHAR(20) NOT NULL CHECK (severity IN ('critical', 'high', 'medium', 'low')),
    description TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create health_snapshots table
CREATE TABLE IF NOT EXISTS health_snapshots (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id),
    score DECIMAL(5,2) NOT NULL CHECK (score >= 0 AND score <= 100),
    metrics JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create risk_indicators table
CREATE TABLE IF NOT EXISTS risk_indicators (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id),
    indicator_type VARCHAR(50) NOT NULL,
    value DECIMAL(10,2) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create bottlenecks table
CREATE TABLE IF NOT EXISTS bottlenecks (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id),
    type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL CHECK (severity IN ('critical', 'high', 'medium', 'low')),
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_pull_request_reviews_pr_id ON pull_request_reviews(pull_request_id);
CREATE INDEX IF NOT EXISTS idx_pull_request_reviews_reviewer_id ON pull_request_reviews(reviewer_id);
CREATE INDEX IF NOT EXISTS idx_review_comments_review_id ON review_comments(review_id);
CREATE INDEX IF NOT EXISTS idx_test_runs_repository_id ON test_runs(repository_id);
CREATE INDEX IF NOT EXISTS idx_test_runs_commit_sha ON test_runs(commit_sha);
CREATE INDEX IF NOT EXISTS idx_dependencies_repository_id ON dependencies(repository_id);
CREATE INDEX IF NOT EXISTS idx_dependencies_name ON dependencies(name);
CREATE INDEX IF NOT EXISTS idx_security_vulnerabilities_repository_id ON security_vulnerabilities(repository_id);
CREATE INDEX IF NOT EXISTS idx_security_vulnerabilities_severity ON security_vulnerabilities(severity);
CREATE INDEX IF NOT EXISTS idx_health_snapshots_repository_id ON health_snapshots(repository_id);
CREATE INDEX IF NOT EXISTS idx_health_snapshots_created_at ON health_snapshots(created_at);
CREATE INDEX IF NOT EXISTS idx_risk_indicators_repository_id ON risk_indicators(repository_id);
CREATE INDEX IF NOT EXISTS idx_risk_indicators_type ON risk_indicators(indicator_type);
CREATE INDEX IF NOT EXISTS idx_bottlenecks_repository_id ON bottlenecks(repository_id);
CREATE INDEX IF NOT EXISTS idx_bottlenecks_type ON bottlenecks(type);

-- Create triggers for updating updated_at
CREATE TRIGGER update_pull_request_reviews_updated_at
    BEFORE UPDATE ON pull_request_reviews
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_review_comments_updated_at
    BEFORE UPDATE ON review_comments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_dependencies_updated_at
    BEFORE UPDATE ON dependencies
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_security_vulnerabilities_updated_at
    BEFORE UPDATE ON security_vulnerabilities
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_risk_indicators_updated_at
    BEFORE UPDATE ON risk_indicators
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bottlenecks_updated_at
    BEFORE UPDATE ON bottlenecks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column(); 