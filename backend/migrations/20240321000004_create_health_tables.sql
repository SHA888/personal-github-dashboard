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

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_security_vulnerabilities_repository_id ON security_vulnerabilities(repository_id);
CREATE INDEX IF NOT EXISTS idx_security_vulnerabilities_severity ON security_vulnerabilities(severity);
CREATE INDEX IF NOT EXISTS idx_health_snapshots_repository_id ON health_snapshots(repository_id);
CREATE INDEX IF NOT EXISTS idx_health_snapshots_created_at ON health_snapshots(created_at);

-- Create triggers for updating updated_at
CREATE TRIGGER update_security_vulnerabilities_updated_at
    BEFORE UPDATE ON security_vulnerabilities
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column(); 