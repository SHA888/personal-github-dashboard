-- Drop all existing triggers and functions
DO $$
BEGIN
    -- Drop triggers
    DROP TRIGGER IF EXISTS update_risk_indicators_updated_at ON risk_indicators;
    DROP TRIGGER IF EXISTS update_bottlenecks_updated_at ON bottlenecks;
    DROP TRIGGER IF EXISTS update_security_vulns_updated_at ON security_vulnerabilities;
    
    -- Drop functions
    DROP FUNCTION IF EXISTS update_risk_indicators_updated_at() CASCADE;
    DROP FUNCTION IF EXISTS update_bottlenecks_updated_at() CASCADE;
    DROP FUNCTION IF EXISTS update_security_vulns_updated_at() CASCADE;
END $$;

-- Create a generic update_updated_at function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create triggers for each table
CREATE TRIGGER update_risk_indicators_updated_at
    BEFORE UPDATE ON risk_indicators
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_bottlenecks_updated_at
    BEFORE UPDATE ON bottlenecks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_security_vulns_updated_at
    BEFORE UPDATE ON security_vulnerabilities
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column(); 