#[cfg(test)]
mod tests {
    use super::{RESPONSE_LIMIT, Subscription, perform_request, request_url};
    use crate::protocol::Request;
    use futures_channel::mpsc;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use url::Url;

    #[test]
    fn route_is_appended_without_implicit_endpoint_fallback() {
        let request = Request::get("/provider").with_directory("/workspace/fixture");
        let url = request_url("http://127.0.0.1:4096/base", &request).expect("URL is valid");
        assert_eq!(
            url.as_str(),
            "http://127.0.0.1:4096/base/provider?directory=%2Fworkspace%2Ffixture"
        );
        assert!(request_url("http://user:secret@localhost:4096", &request).is_err());
    }

    #[test]
    fn response_limit_overflow_reports_the_bounded_input_failure() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener binds");
        let address = listener.local_addr().expect("fixture address is available");
        let server = thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut request = [0u8; 1024];
                let _ = stream.read(&mut request);
                let body = vec![b'x'; RESPONSE_LIMIT + 1];
                let header = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len());
                let _ = stream.write_all(header.as_bytes());
                let _ = stream.write_all(&body);
                let _ = stream.flush();
            }
        });
        let url = Url::parse(&format!("http://{address}/overflow")).expect("fixture URL is valid");
        let result = perform_request(
            url,
            Request::get("/overflow"),
            Arc::new(AtomicBool::new(false)),
        );
        let _ = server.join();
        let error = match result {
            Ok(_) => panic!("an oversized response must abort instead of truncating"),
            Err(error) => error,
        };
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.response_limit"
        );
    }

    #[test]
    fn dropping_a_subscription_cancels_its_blocking_worker() {
        let (sender, receiver) = mpsc::channel(1);
        drop(sender);
        let cancelled = Arc::new(AtomicBool::new(false));
        let subscription = Subscription {
            receiver,
            cancelled: Arc::clone(&cancelled),
            work: None,
        };
        assert!(!cancelled.load(Ordering::SeqCst));
        drop(subscription);
        assert!(
            cancelled.load(Ordering::SeqCst),
            "a dropped subscription must signal its blocking worker to stop"
        );
    }

    #[test]
    fn closing_a_subscription_cancels_its_blocking_worker() {
        let (sender, receiver) = mpsc::channel(1);
        drop(sender);
        let cancelled = Arc::new(AtomicBool::new(false));
        let subscription = Subscription {
            receiver,
            cancelled: Arc::clone(&cancelled),
            work: None,
        };
        futures_executor::block_on(subscription.close()).expect("a drained subscription closes");
        assert!(cancelled.load(Ordering::SeqCst));
    }
}
