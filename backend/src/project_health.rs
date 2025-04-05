use sqlx::PgPool;
use serde_json::Value;
use chrono::{DateTime, Utc, Duration};
use crate::cache::Cache;
use crate::analytics::Analytics;
use crate::collaboration::Collaboration;
use crate::health::Health;

pub struct ProjectHealth {
    pool: PgPool,
    cache: Cache,
    analytics: Analytics,
    collaboration: Collaboration,
    health: Health,
}

impl ProjectHealth {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            cache: Cache::new().expect("Failed to initialize cache"),
            analytics: Analytics::new(pool.clone()),
            collaboration: Collaboration::new(pool.clone()),
            health: Health::new(pool),
        }
    }

    /// Gets overall project health score
    pub async fn get_project_health(
        &self,
        repository_id: i32,
        period: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("project_health:{}:{}", repository_id, period);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            // Get all component scores
            let velocity = self.analytics.get_repository_velocity(repository_id, period).await?;
            let review_times = self.collaboration.get_review_response_times(repository_id, period).await?;
            let tech_debt = self.health.get_technical_debt(repository_id).await?;
            let test_coverage = self.health.get_test_coverage(repository_id, period).await?;
            let dependency_status = self.health.get_dependency_status(repository_id).await?;
            let security = self.health.get_security_vulnerabilities(repository_id).await?;

            // Calculate weighted scores
            let velocity_score = calculate_velocity_score(&velocity);
            let review_score = calculate_review_score(&review_times);
            let debt_score = tech_debt["technical_debt_score"].as_f64().unwrap_or(0.0);
            let coverage_score = calculate_coverage_score(&test_coverage);
            let dependency_score = dependency_status["dependency_health_score"].as_f64().unwrap_or(0.0);
            let security_score = security["security_risk_score"].as_f64().unwrap_or(0.0);

            // Calculate overall health score
            let overall_score = calculate_overall_score(
                velocity_score,
                review_score,
                debt_score,
                coverage_score,
                dependency_score,
                security_score,
            );

            // Get trend data
            let growth = self.analytics.get_repository_growth(repository_id, period).await?;
            let team_performance = self.analytics.get_team_performance(repository_id).await?;

            Ok(serde_json::json!({
                "repository_id": repository_id,
                "period_days": period,
                "overall_health_score": overall_score,
                "component_scores": {
                    "velocity": velocity_score,
                    "code_review": review_score,
                    "technical_debt": debt_score,
                    "test_coverage": coverage_score,
                    "dependencies": dependency_score,
                    "security": security_score
                },
                "trends": {
                    "growth": growth,
                    "team_performance": team_performance
                },
                "health_status": get_health_status(overall_score),
                "recommendations": generate_recommendations(
                    velocity_score,
                    review_score,
                    debt_score,
                    coverage_score,
                    dependency_score,
                    security_score
                )
            }))
        }).await
    }

    /// Gets health trend over time
    pub async fn get_health_trend(
        &self,
        repository_id: i32,
        period: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("health_trend:{}:{}", repository_id, period);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH daily_health AS (
                    SELECT 
                        date_trunc('day', created_at) as date,
                        AVG(score) as daily_score
                    FROM health_snapshots
                    WHERE repository_id = $1
                    AND created_at >= NOW() - ($2 || ' days')::interval
                    GROUP BY date
                    ORDER BY date
                )
                SELECT 
                    json_agg(
                        json_build_object(
                            'date', date,
                            'score', daily_score
                        )
                    ) as trend_data,
                    AVG(daily_score) as average_score,
                    MIN(daily_score) as min_score,
                    MAX(daily_score) as max_score
                FROM daily_health
                "#,
                repository_id,
                period
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "repository_id": repository_id,
                "period_days": period,
                "trend_data": result.trend_data,
                "average_score": result.average_score,
                "minimum_score": result.min_score,
                "maximum_score": result.max_score
            }))
        }).await
    }

    /// Gets risk indicators
    pub async fn get_risk_indicators(
        &self,
        repository_id: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("risk_indicators:{}", repository_id);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH risk_metrics AS (
                    SELECT 
                        COUNT(CASE WHEN type = 'security' THEN 1 END) as security_risks,
                        COUNT(CASE WHEN type = 'performance' THEN 1 END) as performance_risks,
                        COUNT(CASE WHEN type = 'maintainability' THEN 1 END) as maintainability_risks,
                        COUNT(CASE WHEN type = 'reliability' THEN 1 END) as reliability_risks,
                        AVG(severity) as avg_severity,
                        MAX(severity) as max_severity
                    FROM risk_indicators
                    WHERE repository_id = $1
                    AND is_active = true
                )
                SELECT 
                    security_risks,
                    performance_risks,
                    maintainability_risks,
                    reliability_risks,
                    avg_severity,
                    max_severity
                FROM risk_metrics
                "#,
                repository_id
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "repository_id": repository_id,
                "security_risks": result.security_risks,
                "performance_risks": result.performance_risks,
                "maintainability_risks": result.maintainability_risks,
                "reliability_risks": result.reliability_risks,
                "average_severity": result.avg_severity,
                "maximum_severity": result.max_severity,
                "risk_score": calculate_risk_score(
                    result.security_risks,
                    result.performance_risks,
                    result.maintainability_risks,
                    result.reliability_risks,
                    result.avg_severity,
                    result.max_severity
                )
            }))
        }).await
    }

    /// Gets bottleneck analysis
    pub async fn get_bottleneck_analysis(
        &self,
        repository_id: i32,
        period: i32,
    ) -> Result<Value, sqlx::Error> {
        let key = format!("bottlenecks:{}:{}", repository_id, period);
        self.cache.get_cached_or_compute(&key, Duration::from_secs(3600), async {
            let result = sqlx::query!(
                r#"
                WITH bottleneck_metrics AS (
                    SELECT 
                        COUNT(CASE WHEN type = 'review' THEN 1 END) as review_bottlenecks,
                        COUNT(CASE WHEN type = 'testing' THEN 1 END) as testing_bottlenecks,
                        COUNT(CASE WHEN type = 'deployment' THEN 1 END) as deployment_bottlenecks,
                        COUNT(CASE WHEN type = 'documentation' THEN 1 END) as documentation_bottlenecks,
                        AVG(duration_hours) as avg_bottleneck_duration,
                        MAX(duration_hours) as max_bottleneck_duration
                    FROM bottlenecks
                    WHERE repository_id = $1
                    AND created_at >= NOW() - ($2 || ' days')::interval
                )
                SELECT 
                    review_bottlenecks,
                    testing_bottlenecks,
                    deployment_bottlenecks,
                    documentation_bottlenecks,
                    avg_bottleneck_duration,
                    max_bottleneck_duration
                FROM bottleneck_metrics
                "#,
                repository_id,
                period
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(serde_json::json!({
                "repository_id": repository_id,
                "period_days": period,
                "review_bottlenecks": result.review_bottlenecks,
                "testing_bottlenecks": result.testing_bottlenecks,
                "deployment_bottlenecks": result.deployment_bottlenecks,
                "documentation_bottlenecks": result.documentation_bottlenecks,
                "average_bottleneck_duration_hours": result.avg_bottleneck_duration,
                "maximum_bottleneck_duration_hours": result.max_bottleneck_duration,
                "bottleneck_score": calculate_bottleneck_score(
                    result.review_bottlenecks,
                    result.testing_bottlenecks,
                    result.deployment_bottlenecks,
                    result.documentation_bottlenecks,
                    result.avg_bottleneck_duration
                )
            }))
        }).await
    }
}

fn calculate_velocity_score(velocity: &Value) -> f64 {
    let completion_rate = velocity["completion_rate"].as_f64().unwrap_or(0.0);
    let avg_time_to_close = velocity["average_time_to_close_hours"].as_f64().unwrap_or(0.0);
    
    // Normalize values
    let normalized_completion = completion_rate / 100.0;
    let normalized_time = (1.0 - (avg_time_to_close / 168.0).min(1.0)).max(0.0);
    
    (normalized_completion * 0.6 + normalized_time * 0.4) * 100.0
}

fn calculate_review_score(review_times: &Value) -> f64 {
    let avg_response = review_times["average_response_time_hours"].as_f64().unwrap_or(0.0);
    let review_coverage = review_times["review_coverage"].as_f64().unwrap_or(0.0);
    
    // Normalize values
    let normalized_response = (1.0 - (avg_response / 48.0).min(1.0)).max(0.0);
    let normalized_coverage = review_coverage / 100.0;
    
    (normalized_response * 0.5 + normalized_coverage * 0.5) * 100.0
}

fn calculate_coverage_score(test_coverage: &Value) -> f64 {
    let avg_coverage = test_coverage["average_coverage"].as_f64().unwrap_or(0.0);
    let min_coverage = test_coverage["minimum_coverage"].as_f64().unwrap_or(0.0);
    
    // Weighted average favoring minimum coverage
    (avg_coverage * 0.3 + min_coverage * 0.7)
}

fn calculate_overall_score(
    velocity: f64,
    review: f64,
    debt: f64,
    coverage: f64,
    dependencies: f64,
    security: f64,
) -> f64 {
    // Weighted average of all component scores
    let weights = vec![0.2, 0.15, 0.2, 0.15, 0.15, 0.15];
    let scores = vec![velocity, review, debt, coverage, dependencies, security];
    
    scores.iter()
        .zip(weights.iter())
        .map(|(score, weight)| score * weight)
        .sum()
}

fn get_health_status(score: f64) -> &'static str {
    match score {
        s if s >= 90.0 => "Excellent",
        s if s >= 75.0 => "Good",
        s if s >= 60.0 => "Fair",
        s if s >= 40.0 => "Poor",
        _ => "Critical"
    }
}

fn generate_recommendations(
    velocity: f64,
    review: f64,
    debt: f64,
    coverage: f64,
    dependencies: f64,
    security: f64,
) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    if velocity < 70.0 {
        recommendations.push("Consider improving development velocity by addressing bottlenecks in the development process".to_string());
    }
    
    if review < 70.0 {
        recommendations.push("Improve code review practices by reducing response times and increasing review coverage".to_string());
    }
    
    if debt < 60.0 {
        recommendations.push("Address technical debt by prioritizing bug fixes and reducing open issues".to_string());
    }
    
    if coverage < 80.0 {
        recommendations.push("Increase test coverage to improve code quality and reliability".to_string());
    }
    
    if dependencies < 70.0 {
        recommendations.push("Update outdated dependencies and address security vulnerabilities".to_string());
    }
    
    if security < 80.0 {
        recommendations.push("Prioritize security improvements and vulnerability fixes".to_string());
    }
    
    recommendations
}

fn calculate_risk_score(
    security: i64,
    performance: i64,
    maintainability: i64,
    reliability: i64,
    avg_severity: f64,
    max_severity: f64,
) -> f64 {
    let total_risks = security + performance + maintainability + reliability;
    if total_risks == 0 {
        return 100.0;
    }
    
    let normalized_security = (security as f64 / total_risks as f64) * 0.4;
    let normalized_performance = (performance as f64 / total_risks as f64) * 0.2;
    let normalized_maintainability = (maintainability as f64 / total_risks as f64) * 0.2;
    let normalized_reliability = (reliability as f64 / total_risks as f64) * 0.2;
    
    let severity_factor = (avg_severity + max_severity) / 2.0;
    
    (1.0 - (
        normalized_security +
        normalized_performance +
        normalized_maintainability +
        normalized_reliability +
        (severity_factor / 10.0)
    )) * 100.0
}

fn calculate_bottleneck_score(
    review: i64,
    testing: i64,
    deployment: i64,
    documentation: i64,
    avg_duration: f64,
) -> f64 {
    let total_bottlenecks = review + testing + deployment + documentation;
    if total_bottlenecks == 0 {
        return 100.0;
    }
    
    let normalized_review = (review as f64 / total_bottlenecks as f64) * 0.3;
    let normalized_testing = (testing as f64 / total_bottlenecks as f64) * 0.3;
    let normalized_deployment = (deployment as f64 / total_bottlenecks as f64) * 0.2;
    let normalized_documentation = (documentation as f64 / total_bottlenecks as f64) * 0.2;
    
    let duration_factor = (avg_duration / 24.0).min(1.0);
    
    (1.0 - (
        normalized_review +
        normalized_testing +
        normalized_deployment +
        normalized_documentation +
        duration_factor
    )) * 100.0
} 