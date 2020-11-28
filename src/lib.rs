use tide::{Middleware, Next, Request, Result};

pub struct LogMiddleware {}

impl LogMiddleware {
    fn log_request(&self) {}

    fn log_response(&self) {}
}

#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for LogMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> crate::Result {
        self.log_request();

        let result = next.run(req).await;

        self.log_response();
        Ok(result)
    }
}
