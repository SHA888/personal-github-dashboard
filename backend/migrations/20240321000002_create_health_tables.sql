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
    score DECIMAL(5,2) NOT NULL,
    metrics JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Drop existing trigger and function if they exist
DROP TRIGGER IF EXISTS update_risk_indicators_updated_at ON risk_indicators;
DROP FUNCTION IF EXISTS update_risk_indicators_updated_at() CASCADE;

-- Create risk_indicators table if it doesn't exist
CREATE TABLE IF NOT EXISTS risk_indicators (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    indicator_type VARCHAR(50) NOT NULL,
    value DECIMAL(10,2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create index for faster lookups if it doesn't exist
CREATE INDEX IF NOT EXISTS idx_risk_indicators_repository_id ON risk_indicators(repository_id);

-- Create trigger function
CREATE OR REPLACE FUNCTION update_risk_indicators_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger
CREATE TRIGGER update_risk_indicators_updated_at
    BEFORE UPDATE ON risk_indicators
    FOR EACH ROW
    EXECUTE FUNCTION update_risk_indicators_updated_at();

-- Create bottlenecks table
CREATE TABLE IF NOT EXISTS bottlenecks (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id),
    type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL CHECK (severity IN ('high', 'medium', 'low')),
    description TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_security_vulns_repository_id ON security_vulnerabilities(repository_id);
CREATE INDEX IF NOT EXISTS idx_security_vulns_severity ON security_vulnerabilities(severity);
CREATE INDEX IF NOT EXISTS idx_health_snapshots_repository_id ON health_snapshots(repository_id);
CREATE INDEX IF NOT EXISTS idx_bottlenecks_repository_id ON bottlenecks(repository_id);
CREATE INDEX IF NOT EXISTS idx_bottlenecks_type ON bottlenecks(type);

-- Create triggers for updating updated_at
CREATE TRIGGER update_security_vulns_updated_at
    BEFORE UPDATE ON security_vulnerabilities
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bottlenecks_updated_at
    BEFORE UPDATE ON bottlenecks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column(); 