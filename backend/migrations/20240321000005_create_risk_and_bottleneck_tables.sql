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
CREATE INDEX IF NOT EXISTS idx_risk_indicators_repository_id ON risk_indicators(repository_id);
CREATE INDEX IF NOT EXISTS idx_risk_indicators_type ON risk_indicators(indicator_type);
CREATE INDEX IF NOT EXISTS idx_bottlenecks_repository_id ON bottlenecks(repository_id);
CREATE INDEX IF NOT EXISTS idx_bottlenecks_type ON bottlenecks(type);

-- Create triggers for updating updated_at
CREATE TRIGGER update_risk_indicators_updated_at
    BEFORE UPDATE ON risk_indicators
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bottlenecks_updated_at
    BEFORE UPDATE ON bottlenecks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column(); 