-- Drop all existing triggers and functions
DROP TRIGGER IF EXISTS update_risk_indicators_updated_at ON risk_indicators;
DROP FUNCTION IF EXISTS update_risk_indicators_updated_at() CASCADE;

-- Recreate the function
CREATE OR REPLACE FUNCTION update_risk_indicators_updated_at()
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