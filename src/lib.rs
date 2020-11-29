use tide::{Middleware, Next, Request, Result};

pub struct LogMiddleware;

impl LogMiddleware {
    fn log_request<State>(&self, request: &Request<State>) {
        log::info!(
            "<-- Request received, method: {}, path {}",
            request.method(),
            request.url().path()
        );
    }

    fn log_response(&self) {}
}

#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for LogMiddleware {
    async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> crate::Result {
        if request.ext::<LogMiddleware>().is_some() {
            Ok(next.run(request).await)
        } else {
            request.set_ext(LogMiddleware);

            self.log_request(&request);

            let response = next.run(request).await;

            self.log_response();
            Ok(response)
        }
    }
}
