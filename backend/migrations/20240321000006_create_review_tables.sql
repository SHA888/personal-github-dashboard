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

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_pull_request_reviews_pr_id ON pull_request_reviews(pull_request_id);
CREATE INDEX IF NOT EXISTS idx_pull_request_reviews_reviewer_id ON pull_request_reviews(reviewer_id);
CREATE INDEX IF NOT EXISTS idx_review_comments_review_id ON review_comments(review_id);

-- Create triggers for updating updated_at
CREATE TRIGGER update_pull_request_reviews_updated_at
    BEFORE UPDATE ON pull_request_reviews
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_review_comments_updated_at
    BEFORE UPDATE ON review_comments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column(); 