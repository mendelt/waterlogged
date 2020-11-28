use tide::{Middleware, Next, Request, Result};

pub struct LogMiddleware { }

impl LogMiddleware {

    fn log_request() {

    }

    fn log_response() {

    }
}

#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for LogMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> crate::Result {
        Ok(next.run(req).await)
    }
}
