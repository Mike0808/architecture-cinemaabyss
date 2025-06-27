use hyper::{
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server, Uri, StatusCode
};
use log::{info, error};
use rand::Rng;
use std::{env, net::SocketAddr};

// Feature flag configuration
#[derive(Clone)]
struct FeatureFlags {
    gradual_migration: bool,
    rollout_percent: u8,
    monolith: Uri,
    movies: Uri,
}

impl FeatureFlags {
    fn new() -> Self {
        let gradual_migration = env::var("GRADUAL_MIGRATION")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);

        let rollout_percent = env::var("MOVIES_MIGRATION_PERCENT")
            .map(|v| v.parse().unwrap_or(0))
            .unwrap_or(0)
            .clamp(0, 100);

        let monolith = env::var("MONOLITH_URL")
            .unwrap_or("http://localhost:8080".into())
            .parse()
            .expect("Invalid MONOLITH URI");

        let movies = env::var("MOVIES_SERVICE_URL")
            .unwrap_or("http://localhost:8083".into())
            .parse()
            .expect("Invalid MOVIES URI");

        FeatureFlags {
            gradual_migration,
            rollout_percent,
            monolith,
            movies,
        }
    }

    fn use_movies(&self) -> bool {
        if !self.gradual_migration {
            return false;
        }

        if self.rollout_percent >= 100 {
            return true;
        }

        if self.rollout_percent <= 0 {
            return false;
        }

        rand::thread_rng().gen_range(0..100) < self.rollout_percent
    }
}

async fn handle_request(
    req: Request<Body>,
    client: Client<hyper::client::HttpConnector>,
    flags: FeatureFlags,
) -> Result<Response<Body>, hyper::Error> {
    if req.uri().path() == "/health" {
        return Ok(Response::new(Body::from("OK")));
    }

    let backend = if flags.use_movies() {
        &flags.movies
    } else {
        &flags.monolith
    };

    info!("Choosed backend: {}", backend);

    let (parts, body) = req.into_parts();

    
    let path = parts.uri.path().trim_start_matches('/');
    info!("Trimmed path to: {}", path);
    let backend_uri = match format!("{}{}", backend, path).parse::<Uri>() {
        Ok(uri) => uri,
        Err(e) => {
            error!("Failed to construct backend URI: {}", e);
            return Ok(Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Body::from("Invalid backend URL"))
                .unwrap());
        }
    };
    info!("Proxying to: {}", backend_uri);
    // Build new request to backend
    let mut new_request = Request::builder()
        .method(parts.method)
        .uri(backend_uri)
        .body(body)
        .unwrap();

    *new_request.headers_mut() = parts.headers.clone();

    client.request(new_request).await
}

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .format_level(true)
        .format_module_path(false)
        .init();

    let flags = FeatureFlags::new();
    let client = Client::builder()
    .pool_idle_timeout(std::time::Duration::from_secs(30))
    .build_http();

    let service = make_service_fn(move |_| {
        let client = client.clone();
        let flags = flags.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handle_request(req, client.clone(), flags.clone())
            }))
        }
    });
    let port = env::var("PORT")
        .unwrap_or("8000".into())
        .parse()
        .expect("Invalid PORT");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server = Server::bind(&addr).serve(service);

    println!("Proxy running on http://{}", addr);
    println!("Health check available at http://{}/health", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
