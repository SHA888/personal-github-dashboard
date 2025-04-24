-- Add is_private column to repositories table
ALTER TABLE repositories ADD COLUMN is_private BOOLEAN NOT NULL DEFAULT false;
