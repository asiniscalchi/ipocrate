use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
};

use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{any, get},
};
use clap::Parser;
use local_ip_address::list_afinet_netifas;
use serde::Serialize;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
pub struct Config {
    #[arg(long, env = "HOST", default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    pub host: IpAddr,

    #[arg(long, env = "PORT", default_value_t = 8080)]
    pub port: u16,

    #[arg(
        long = "frontend-dist",
        env = "FRONTEND_DIST",
        default_value = "frontend/dist"
    )]
    pub frontend_dist: PathBuf,
}

impl Config {
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

pub fn served_urls(address: SocketAddr) -> Vec<String> {
    let addresses = if address.ip().is_unspecified() {
        interface_addresses()
    } else {
        vec![address.ip()]
    };

    addresses
        .into_iter()
        .map(|ip| format!("http://{}:{}", host_for_url(ip), address.port()))
        .collect()
}

fn interface_addresses() -> Vec<IpAddr> {
    let mut addresses = vec![IpAddr::V4(Ipv4Addr::LOCALHOST)];

    if let Ok(interfaces) = list_afinet_netifas() {
        for (_, ip) in interfaces {
            if is_useful_served_url_address(ip) && !addresses.contains(&ip) {
                addresses.push(ip);
            }
        }
    }

    addresses
}

fn is_useful_served_url_address(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => !ip.is_unspecified(),
        IpAddr::V6(ip) => !ip.is_unspecified() && !ip.is_unicast_link_local(),
    }
}

fn host_for_url(ip: IpAddr) -> String {
    match ip {
        IpAddr::V4(ip) => ip.to_string(),
        IpAddr::V6(ip) => format!("[{ip}]"),
    }
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: &'static str,
}

pub fn router(frontend_dist: impl Into<PathBuf>) -> Router {
    let frontend_dist = frontend_dist.into();
    let index_file = frontend_dist.join("index.html");

    Router::new()
        .route("/health", get(health))
        .route("/api/{*path}", any(api_not_found))
        .fallback_service(ServeDir::new(frontend_dist).fallback(ServeFile::new(index_file)))
}

pub fn ensure_frontend_dist(frontend_dist: &Path) -> std::io::Result<()> {
    let index_file = frontend_dist.join("index.html");

    if index_file.is_file() {
        return Ok(());
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!(
            "frontend build output is missing: expected {}. Run the documented frontend build before starting the backend.",
            index_file.display()
        ),
    ))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn api_not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse { error: "not_found" }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tempfile::TempDir;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_returns_ok_json() {
        let dist = frontend_dist();
        let response = router(dist.path())
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_text(response).await, r#"{"status":"ok"}"#);
    }

    #[tokio::test]
    async fn root_serves_react_shell() {
        let dist = frontend_dist();
        let response = router(dist.path())
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_text(response).await, "<main>Ipocrate</main>");
    }

    #[tokio::test]
    async fn frontend_route_falls_back_to_react_shell() {
        let dist = frontend_dist();
        let response = router(dist.path())
            .oneshot(
                Request::builder()
                    .uri("/some-route")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_text(response).await, "<main>Ipocrate</main>");
    }

    #[tokio::test]
    async fn api_routes_are_not_handled_by_frontend_fallback() {
        let dist = frontend_dist();
        let response = router(dist.path())
            .oneshot(
                Request::builder()
                    .uri("/api/missing")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(body_text(response).await, r#"{"error":"not_found"}"#);
    }

    #[test]
    fn config_uses_host_for_socket_address() {
        let config = Config {
            host: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: 8080,
            frontend_dist: PathBuf::from("frontend/dist"),
        };

        assert_eq!(
            config.socket_addr(),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 8080)
        );
    }

    #[test]
    fn served_urls_include_specific_bound_address() {
        let urls = served_urls(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(192, 0, 2, 10)),
            8080,
        ));

        assert_eq!(urls, vec!["http://192.0.2.10:8080"]);
    }

    #[test]
    fn served_urls_bracket_ipv6_hosts() {
        let urls = served_urls(SocketAddr::new(
            IpAddr::V6(std::net::Ipv6Addr::LOCALHOST),
            8080,
        ));

        assert_eq!(urls, vec!["http://[::1]:8080"]);
    }

    #[test]
    fn link_local_ipv6_addresses_are_not_logged_as_served_urls() {
        let link_local = "fe80::1".parse().unwrap();

        assert!(!is_useful_served_url_address(IpAddr::V6(link_local)));
    }

    #[test]
    fn frontend_dist_requires_index_html() {
        let missing = TempDir::new().unwrap();

        let error = ensure_frontend_dist(missing.path()).unwrap_err();

        assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
        assert!(
            error
                .to_string()
                .contains("frontend build output is missing")
        );
    }

    fn frontend_dist() -> TempDir {
        let dist = TempDir::new().unwrap();
        std::fs::write(dist.path().join("index.html"), "<main>Ipocrate</main>").unwrap();
        dist
    }

    async fn body_text(response: axum::response::Response) -> String {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}
