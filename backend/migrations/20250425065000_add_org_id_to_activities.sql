-- Add org_id column to activities table
ALTER TABLE activities ADD COLUMN org_id UUID REFERENCES organizations(id) ON DELETE SET NULL;
