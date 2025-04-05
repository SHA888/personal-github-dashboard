-- Drop tables in reverse order to respect foreign key constraints
DROP TABLE IF EXISTS workflows;
DROP TABLE IF EXISTS milestones;
DROP TABLE IF EXISTS releases;
DROP TABLE IF EXISTS branches;
DROP TABLE IF EXISTS commits;
DROP TABLE IF EXISTS pull_requests;
DROP TABLE IF EXISTS issues;
DROP TABLE IF EXISTS repositories;
DROP TABLE IF EXISTS organizations;
DROP TABLE IF EXISTS users; 