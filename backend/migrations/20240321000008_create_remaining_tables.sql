-- Create pull_request_reviews table
CREATE TABLE IF NOT EXISTS pull_request_reviews (
    id SERIAL PRIMARY KEY,
    pull_request_id INTEGER NOT NULL REFERENCES pull_requests(id),
    reviewer_id INTEGER NOT NULL REFERENCES users(id),
    state VARCHAR(50) NOT NULL,
    submitted_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create review_comments table
CREATE TABLE IF NOT EXISTS review_comments (
    id SERIAL PRIMARY KEY,
    review_id INTEGER NOT NULL REFERENCES pull_request_reviews(id),
    author_id INTEGER NOT NULL REFERENCES users(id),
    body TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for pull_request_reviews
CREATE INDEX IF NOT EXISTS idx_pull_request_reviews_pull_request_id ON pull_request_reviews(pull_request_id);
CREATE INDEX IF NOT EXISTS idx_pull_request_reviews_reviewer_id ON pull_request_reviews(reviewer_id);
CREATE INDEX IF NOT EXISTS idx_pull_request_reviews_submitted_at ON pull_request_reviews(submitted_at);

-- Create indexes for review_comments
CREATE INDEX IF NOT EXISTS idx_review_comments_review_id ON review_comments(review_id);
CREATE INDEX IF NOT EXISTS idx_review_comments_author_id ON review_comments(author_id);

-- Create triggers for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_pull_request_reviews_updated_at
    BEFORE UPDATE ON pull_request_reviews
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_review_comments_updated_at
    BEFORE UPDATE ON review_comments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column(); 