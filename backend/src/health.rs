use sqlx::PgPool;
use serde_json::Value;
use chrono::{DateTime, Utc, Duration};
use crate::cache::Cache;

pub struct Health {
    pool: PgPool,
    cache: Cache,
}

impl Health {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            cache: Cache::new().expect("Failed to initialize cache"),
        }
    }

    /// Gets technical debt indicators
    pub async fn get_technical_debt(
        &self,
        repository_id: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("tech_debt:{}", repository_id);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH debt_metrics AS (
                    SELECT 
                        COUNT(*) as total_issues,
                        COUNT(CASE WHEN labels ? 'bug' THEN 1 END) as bug_count,
                        COUNT(CASE WHEN labels ? 'enhancement' THEN 1 END) as enhancement_count,
                        COUNT(CASE WHEN labels ? 'documentation' THEN 1 END) as doc_count,
                        AVG(EXTRACT(EPOCH FROM (NOW() - created_at))/86400) as avg_age_days,
                        COUNT(CASE WHEN state = 'open' THEN 1 END) as open_count
                    FROM issues
                    WHERE repository_id = $1
                )
                SELECT 
                    total_issues,
                    bug_count,
                    enhancement_count,
                    doc_count,
                    avg_age_days,
                    open_count,
                    (bug_count::float / NULLIF(total_issues, 0) * 100) as bug_percentage,
                    (open_count::float / NULLIF(total_issues, 0) * 100) as open_percentage
                FROM debt_metrics
                "#,
                repository_id
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "repository_id": repository_id,
                "total_issues": result.total_issues,
                "bug_count": result.bug_count,
                "enhancement_count": result.enhancement_count,
                "documentation_count": result.doc_count,
                "average_issue_age_days": result.avg_age_days,
                "open_issues_count": result.open_count,
                "bug_percentage": result.bug_percentage,
                "open_issues_percentage": result.open_percentage,
                "technical_debt_score": calculate_debt_score(
                    result.bug_percentage,
                    result.open_percentage,
                    result.avg_age_days
                )
            }))
        }).await
    }

    /// Gets test coverage trends
    pub async fn get_test_coverage(
        &self,
        repository_id: i32,
        period: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("test_coverage:{}:{}", repository_id, period);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH coverage_data AS (
                    SELECT 
                        date_trunc('day', created_at) as date,
                        AVG(coverage_percentage) as daily_coverage,
                        COUNT(*) as test_count
                    FROM test_runs
                    WHERE repository_id = $1
                    AND created_at >= NOW() - ($2 || ' days')::interval
                    GROUP BY date
                    ORDER BY date
                )
                SELECT 
                    json_agg(
                        json_build_object(
                            'date', date,
                            'coverage_percentage', daily_coverage,
                            'test_count', test_count
                        )
                    ) as coverage_trend,
                    AVG(daily_coverage) as average_coverage,
                    MIN(daily_coverage) as min_coverage,
                    MAX(daily_coverage) as max_coverage
                FROM coverage_data
                "#,
                repository_id,
                period
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "repository_id": repository_id,
                "period_days": period,
                "coverage_trend": result.coverage_trend,
                "average_coverage": result.average_coverage,
                "minimum_coverage": result.min_coverage,
                "maximum_coverage": result.max_coverage
            }))
        }).await
    }

    /// Gets dependency update status
    pub async fn get_dependency_status(
        &self,
        repository_id: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("deps:{}", repository_id);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH dep_metrics AS (
                    SELECT 
                        COUNT(*) as total_dependencies,
                        COUNT(CASE WHEN is_outdated THEN 1 END) as outdated_count,
                        COUNT(CASE WHEN has_security_issues THEN 1 END) as security_issues_count,
                        AVG(days_behind) as avg_days_behind,
                        MAX(days_behind) as max_days_behind
                    FROM dependencies
                    WHERE repository_id = $1
                )
                SELECT 
                    total_dependencies,
                    outdated_count,
                    security_issues_count,
                    avg_days_behind,
                    max_days_behind,
                    (outdated_count::float / NULLIF(total_dependencies, 0) * 100) as outdated_percentage
                FROM dep_metrics
                "#,
                repository_id
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "repository_id": repository_id,
                "total_dependencies": result.total_dependencies,
                "outdated_dependencies": result.outdated_count,
                "security_issues": result.security_issues_count,
                "average_days_behind": result.avg_days_behind,
                "maximum_days_behind": result.max_days_behind,
                "outdated_percentage": result.outdated_percentage,
                "dependency_health_score": calculate_dependency_score(
                    result.outdated_percentage,
                    result.security_issues_count,
                    result.avg_days_behind
                )
            }))
        }).await
    }

    /// Gets security vulnerability tracking
    pub async fn get_security_vulnerabilities(
        &self,
        repository_id: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("security:{}", repository_id);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH security_metrics AS (
                    SELECT 
                        COUNT(*) as total_vulnerabilities,
                        COUNT(CASE WHEN severity = 'critical' THEN 1 END) as critical_count,
                        COUNT(CASE WHEN severity = 'high' THEN 1 END) as high_count,
                        COUNT(CASE WHEN severity = 'medium' THEN 1 END) as medium_count,
                        COUNT(CASE WHEN severity = 'low' THEN 1 END) as low_count,
                        AVG(EXTRACT(EPOCH FROM (NOW() - reported_at))/86400) as avg_age_days,
                        COUNT(CASE WHEN is_fixed THEN 1 END) as fixed_count
                    FROM security_vulnerabilities
                    WHERE repository_id = $1
                )
                SELECT 
                    total_vulnerabilities,
                    critical_count,
                    high_count,
                    medium_count,
                    low_count,
                    avg_age_days,
                    fixed_count,
                    (fixed_count::float / NULLIF(total_vulnerabilities, 0) * 100) as fix_rate
                FROM security_metrics
                "#,
                repository_id
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "repository_id": repository_id,
                "total_vulnerabilities": result.total_vulnerabilities,
                "critical_vulnerabilities": result.critical_count,
                "high_vulnerabilities": result.high_count,
                "medium_vulnerabilities": result.medium_count,
                "low_vulnerabilities": result.low_count,
                "average_vulnerability_age_days": result.avg_age_days,
                "fixed_vulnerabilities": result.fixed_count,
                "fix_rate_percentage": result.fix_rate,
                "security_risk_score": calculate_security_score(
                    result.critical_count,
                    result.high_count,
                    result.medium_count,
                    result.low_count,
                    result.avg_age_days
                )
            }))
        }).await
    }
}

fn calculate_debt_score(bug_percentage: f64, open_percentage: f64, avg_age_days: f64) -> f64 {
    // Normalize values and calculate weighted score
    let bug_weight = 0.4;
    let open_weight = 0.3;
    let age_weight = 0.3;
    
    let normalized_bug = bug_percentage / 100.0;
    let normalized_open = open_percentage / 100.0;
    let normalized_age = (avg_age_days / 365.0).min(1.0);
    
    (1.0 - (normalized_bug * bug_weight + normalized_open * open_weight + normalized_age * age_weight)) * 100.0
}

fn calculate_dependency_score(outdated_percentage: f64, security_issues: i64, avg_days_behind: f64) -> f64 {
    // Normalize values and calculate weighted score
    let outdated_weight = 0.4;
    let security_weight = 0.4;
    let age_weight = 0.2;
    
    let normalized_outdated = outdated_percentage / 100.0;
    let normalized_security = (security_issues as f64 / 10.0).min(1.0);
    let normalized_age = (avg_days_behind / 365.0).min(1.0);
    
    (1.0 - (normalized_outdated * outdated_weight + normalized_security * security_weight + normalized_age * age_weight)) * 100.0
}

fn calculate_security_score(critical: i64, high: i64, medium: i64, low: i64, avg_age_days: f64) -> f64 {
    // Weighted scoring based on severity and age
    let critical_weight = 0.4;
    let high_weight = 0.3;
    let medium_weight = 0.2;
    let low_weight = 0.1;
    let age_weight = 0.2;
    
    let total = critical + high + medium + low;
    if total == 0 {
        return 100.0;
    }
    
    let normalized_critical = (critical as f64 / total as f64);
    let normalized_high = (high as f64 / total as f64);
    let normalized_medium = (medium as f64 / total as f64);
    let normalized_low = (low as f64 / total as f64);
    let normalized_age = (avg_age_days / 365.0).min(1.0);
    
    (1.0 - (
        normalized_critical * critical_weight +
        normalized_high * high_weight +
        normalized_medium * medium_weight +
        normalized_low * low_weight +
        normalized_age * age_weight
    )) * 100.0
} 