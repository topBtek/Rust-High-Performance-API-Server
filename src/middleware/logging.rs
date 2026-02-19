use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
    time::Instant,
};
use tracing::{info, Span};
use uuid::Uuid;

/// Request logging middleware with trace IDs
#[derive(Clone, Default)]
pub struct RequestLogging;

impl<S, B> Transform<S, ServiceRequest> for RequestLogging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggingMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct RequestLoggingMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestLoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let trace_id = Uuid::new_v4();
        
        // Attach trace ID to request extensions
        req.extensions_mut().insert(trace_id);

        // Create a span for this request
        let span = Span::current();
        span.record("trace_id", trace_id.to_string());

        let method = req.method().clone();
        let path = req.path().to_string();
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            let res = service.call(req).await?;
            let elapsed = start.elapsed();
            let status = res.status();

            info!(
                method = %method,
                path = %path,
                status = %status.as_u16(),
                duration_ms = elapsed.as_millis(),
                trace_id = %trace_id,
                "Request completed"
            );

            Ok(res)
        })
    }
}
