use std::time::Duration;
use std::time::Instant;
use tide::{Middleware, Next, Request, Response, Result};

pub struct LogMiddleware;

impl LogMiddleware {
    fn log_request<State>(&self, request: &Request<State>) {
        log::info!(
            "<-- Request received, method: {}, path {}",
            request.method(),
            request.url().path()
        );
    }

    fn log_response(&self, response: &Response, duration: Duration) {
        let status = response.status();
        if response.status().is_server_error() {
            if let Some(error) = response.error() {
                log::error!("Internal error --> Response sent, message: {:?}, error_type: {}, status: {} - {}, duration: {:?}", 
                    error,
                    error.type_name().unwrap_or("<unknown>"),
                    status as u16,
                    status.canonical_reason(),
                    duration,);
            } else {
                log::error!(
                    "Internal error --> Response sent, status: {} - {}, duration: {:?}",
                    status as u16,
                    status.canonical_reason(),
                    duration,
                );
            }
        } else if status.is_client_error() {
            if let Some(error) = response.error() {
                log::warn!("Client error --> Response sent, message: {:?}, error_type: {}, status: {} - {}, duration: {:?}",
                    error,
                    error.type_name().unwrap_or("<unknown>"),
                    status as u16,
                    status.canonical_reason(),
                    duration,);
            } else {
                log::warn!(
                    "Client error --> Response sent, status: {} - {}, duration: {:?}",
                    status as u16,
                    status.canonical_reason(),
                    duration,
                );
            }
        } else {
            log::info!(
                "--> Response sent, status: {} - {}, duration: {:?}",
                status as u16,
                status.canonical_reason(),
                duration,
            );
        }
    }
}

#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for LogMiddleware {
    async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> crate::Result {
        if request.ext::<LogMiddleware>().is_some() {
            Ok(next.run(request).await)
        } else {
            request.set_ext(LogMiddleware);

            self.log_request(&request);

            let start = Instant::now();
            let response = next.run(request).await;

            self.log_response(&response, start.elapsed());
            Ok(response)
        }
    }
}
