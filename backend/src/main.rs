use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::{path::PathBuf, sync::Arc, time::Duration};
use tower_http::cors::{AllowHeaders, AllowMethods, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod application;
mod domain;
mod infrastructure;
mod presentation;
mod shared;

use application::services::{ActivityAppService, AttachmentAppService, AuthAppService, ProjectAppService, TagAppService, TaskAppService, TeamAppService, TimeLogAppService};
use infrastructure::{
    config::AppConfig,
    database,
    persistence::{PgActivityLogRepository, PgAttachmentRepository, PgProjectRepository, PgTagRepository, PgTaskRepository, PgTeamRepository, PgTimeLogRepository, PgUserRepository},
};
use presentation::handlers::{activity_handler, attachment_handler, auth_handler, project_handler, tag_handler, task_handler, team_handler, time_log_handler};
use presentation::middleware::auth_middleware;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let config = AppConfig::from_env();

    // Database connection
    let pool = database::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // Create repositories
    let user_repository = Arc::new(PgUserRepository::new(pool.clone()));
    let project_repository = Arc::new(PgProjectRepository::new(pool.clone()));
    let task_repository = Arc::new(PgTaskRepository::new(pool.clone()));
    let team_repository = Arc::new(PgTeamRepository::new(pool.clone()));
    let activity_repository = Arc::new(PgActivityLogRepository::new(pool.clone()));
    let time_log_repository = Arc::new(PgTimeLogRepository::new(pool.clone()));
    let tag_repository = Arc::new(PgTagRepository::new(pool.clone()));
    let attachment_repository = Arc::new(PgAttachmentRepository::new(pool.clone()));

    // Setup upload directory
    let upload_dir = PathBuf::from(
        std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string()),
    );
    tokio::fs::create_dir_all(&upload_dir)
        .await
        .expect("Failed to create upload directory");

    // Create application services
    let auth_service = Arc::new(AuthAppService::new(
        user_repository,
        config.jwt_secret.clone(),
        config.jwt_expiration,
    ));
    let project_service = Arc::new(ProjectAppService::new(project_repository));
    let task_service = Arc::new(TaskAppService::new(task_repository));
    let team_service = Arc::new(TeamAppService::new(team_repository));
    let activity_service = Arc::new(ActivityAppService::new(activity_repository));
    let time_log_service = Arc::new(TimeLogAppService::new(time_log_repository));
    let tag_service = Arc::new(TagAppService::new(tag_repository));
    let attachment_service = Arc::new(AttachmentAppService::new(attachment_repository, upload_dir));

    // CORS configuration - restrict to allowed origins
    let cors = CorsLayer::new()
        .allow_origin(config.allowed_origins())
        .allow_methods(AllowMethods::list([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ]))
        .allow_headers(AllowHeaders::list([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ]))
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    // Build router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .nest(
            "/api/v1",
            api_routes(auth_service, project_service, task_service, team_service, activity_service, time_log_service, tag_service, attachment_service),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Start server
    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .unwrap();

    tracing::info!("Server running on http://{}", config.server_addr);
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "PMO Backend is running!"
}

fn api_routes(
    auth_service: Arc<AuthAppService>,
    project_service: Arc<ProjectAppService>,
    task_service: Arc<TaskAppService>,
    team_service: Arc<TeamAppService>,
    activity_service: Arc<ActivityAppService>,
    time_log_service: Arc<TimeLogAppService>,
    tag_service: Arc<TagAppService>,
    attachment_service: Arc<AttachmentAppService>,
) -> Router {
    // Public auth routes (no authentication required)
    let public_auth_routes = Router::new()
        .route("/auth/register", post(auth_handler::register))
        .route("/auth/login", post(auth_handler::login))
        .with_state(auth_service);

    // Protected project routes
    let project_routes = Router::new()
        .route("/projects", get(project_handler::list_projects))
        .route("/projects", post(project_handler::create_project))
        .route("/projects/{id}", get(project_handler::get_project))
        .route("/projects/{id}", put(project_handler::update_project))
        .route("/projects/{id}", delete(project_handler::delete_project))
        .route(
            "/projects/{id}/tasks",
            get(project_handler::get_project_tasks),
        )
        .route(
            "/projects/{id}/milestones",
            get(project_handler::get_project_milestones),
        )
        .layer(middleware::from_fn(auth_middleware))
        .with_state(project_service);

    // Protected task routes
    let task_routes = Router::new()
        .route("/tasks", get(task_handler::list_tasks))
        .route("/tasks", post(task_handler::create_task))
        .route("/tasks/{id}", get(task_handler::get_task))
        .route("/tasks/{id}", put(task_handler::update_task))
        .route("/tasks/{id}", delete(task_handler::delete_task))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(task_service);

    // Protected team routes
    let team_routes = Router::new()
        .route("/teams", get(team_handler::list_teams))
        .route("/teams", post(team_handler::create_team))
        .route("/teams/{id}", get(team_handler::get_team))
        .route("/teams/{id}", put(team_handler::update_team))
        .route("/teams/{id}", delete(team_handler::delete_team))
        .route("/teams/{id}/members", get(team_handler::get_team_members))
        .route("/teams/{id}/members", post(team_handler::add_team_member))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(team_service);

    // Protected activity routes
    let activity_routes = Router::new()
        .route("/activities", get(activity_handler::list_activities))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(activity_service);

    // Protected time log routes
    let time_log_routes = Router::new()
        .route("/time-logs", get(time_log_handler::list_my_time_logs))
        .route("/time-logs", post(time_log_handler::create_time_log))
        .route("/time-logs/{id}", get(time_log_handler::get_time_log))
        .route("/time-logs/{id}", put(time_log_handler::update_time_log))
        .route("/time-logs/{id}", delete(time_log_handler::delete_time_log))
        .route("/tasks/{task_id}/time-logs", get(time_log_handler::list_task_time_logs))
        .route("/users/{user_id}/time-logs", get(time_log_handler::list_user_time_logs))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(time_log_service);

    // Protected tag routes
    let tag_routes = Router::new()
        .route("/tags", get(tag_handler::list_tags))
        .route("/tags", post(tag_handler::create_tag))
        .route("/tags/{id}", get(tag_handler::get_tag))
        .route("/tags/{id}", put(tag_handler::update_tag))
        .route("/tags/{id}", delete(tag_handler::delete_tag))
        .route("/tasks/{task_id}/tags", get(tag_handler::get_task_tags))
        .route("/tasks/{task_id}/tags", put(tag_handler::set_task_tags))
        .route("/tasks/{task_id}/tags/{tag_id}", post(tag_handler::add_tag_to_task))
        .route("/tasks/{task_id}/tags/{tag_id}", delete(tag_handler::remove_tag_from_task))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(tag_service);

    // Protected attachment routes
    let attachment_routes = Router::new()
        .route("/tasks/{task_id}/attachments", get(attachment_handler::get_task_attachments))
        .route("/tasks/{task_id}/attachments", post(attachment_handler::upload_attachment))
        .route("/attachments/{id}", get(attachment_handler::download_attachment))
        .route("/attachments/{id}", delete(attachment_handler::delete_attachment))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(attachment_service);

    Router::new()
        .merge(public_auth_routes)
        .merge(project_routes)
        .merge(task_routes)
        .merge(team_routes)
        .merge(activity_routes)
        .merge(time_log_routes)
        .merge(tag_routes)
        .merge(attachment_routes)
}
