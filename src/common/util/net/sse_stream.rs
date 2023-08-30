use std::fmt::Display;

use tokio::sync::mpsc;

pub struct SseStream<T> where T: Display,
{
    pub receiver: Option<mpsc::UnboundedReceiver<T>>,
}

impl<T: std::fmt::Display> SseStream<T> {
    
}

impl<T: std::fmt::Display> futures::Stream for SseStream<T> {
    type Item = Result<actix_web::web::Bytes, actix_web::Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        // get reciver data and send
        if let Some(receiver) = &mut self.receiver {
            match receiver.poll_recv(ctx) {
                std::task::Poll::Ready(Some(data)) => {
                    // Create the SSE event
                    let message = format!("data: {}\n\n", data);
    
                    // Return the event as a stream item
                    return std::task::Poll::Ready(Some(Ok(actix_web::web::Bytes::from(message))));
                }
                std::task::Poll::Ready(None) => {
                    return std::task::Poll::Ready(None);
                }
                std::task::Poll::Pending => {
                    return std::task::Poll::Pending;
                }
            }
        } else {
            return std::task::Poll::Ready(None);
        }
    }
}