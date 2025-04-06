-- Create risk_indicators table if it doesn't exist
CREATE TABLE IF NOT EXISTS risk_indicators (
    id SERIAL PRIMARY KEY,
    repository_id INTEGER NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    indicator_type VARCHAR(50) NOT NULL,
    value DECIMAL(10,2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create index if it doesn't exist
CREATE INDEX IF NOT EXISTS idx_risk_indicators_repository_id ON risk_indicators(repository_id);

-- Drop and recreate the trigger function
DROP FUNCTION IF EXISTS update_risk_indicators_updated_at() CASCADE;
CREATE FUNCTION update_risk_indicators_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create the trigger
CREATE TRIGGER update_risk_indicators_updated_at
    BEFORE UPDATE ON risk_indicators
    FOR EACH ROW
    EXECUTE FUNCTION update_risk_indicators_updated_at(); 