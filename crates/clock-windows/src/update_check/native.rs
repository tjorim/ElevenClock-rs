//! Asynchronous WinHTTP on a detached, bounded-lifetime worker. Handles never
//! leave that worker. HANDLE_CLOSING releases the callback's context reference;
//! cancelling does not free a buffer that WinHTTP may still be writing.
use super::{CheckResult, Failure, RESPONSE_LIMIT, backoff, decode};
use crate::{bindings::*, managed};
use std::{
    cell::UnsafeCell,
    sync::{
        Arc,
        mpsc::{self, Receiver, SyncSender},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use windows_core::{PCWSTR, w};

const LIMIT: Duration = Duration::from_secs(10);
const CHUNK: usize = 8192;

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[derive(Debug)]
enum Completion {
    Sent,
    Headers,
    Read(usize),
    Error,
    Cancel,
}

struct Context {
    events: SyncSender<Completion>,
    buffer: UnsafeCell<[u8; CHUNK]>,
    #[cfg(test)]
    closed: Option<SyncSender<()>>,
}
// SAFETY: only WinHTTP writes the buffer, one outstanding read at a time. The
// worker reads it only after READ_COMPLETE arrives through the synchronized
// channel, and finishes copying before starting another read. The callback never
// accesses the buffer. Arc keeps it alive through HANDLE_CLOSING.
unsafe impl Sync for Context {}

#[cfg(test)]
impl Drop for Context {
    fn drop(&mut self) {
        if let Some(closed) = &self.closed {
            let _ = closed.try_send(());
        }
    }
}

unsafe extern "system" fn callback(
    _: HINTERNET,
    context: usize,
    status: u32,
    _: *mut core::ffi::c_void,
    length: u32,
) {
    if context == 0 {
        return;
    }
    let pointer = context as *const Context;
    if status == WINHTTP_CALLBACK_STATUS_HANDLE_CLOSING as u32 {
        // SAFETY: exactly one Arc was lent to this request, and this is its last
        // callback. WinHTTP guarantees no concurrent callbacks at HANDLE_CLOSING.
        drop(unsafe { Arc::from_raw(pointer) });
        return;
    }
    // SAFETY: the request's Arc remains alive until HANDLE_CLOSING.
    let context = unsafe { &*pointer };
    let event = match status as i32 {
        WINHTTP_CALLBACK_STATUS_SENDREQUEST_COMPLETE => Completion::Sent,
        WINHTTP_CALLBACK_STATUS_HEADERS_AVAILABLE => Completion::Headers,
        WINHTTP_CALLBACK_STATUS_READ_COMPLETE => Completion::Read(length as usize),
        WINHTTP_CALLBACK_STATUS_REQUEST_ERROR => Completion::Error,
        _ => return,
    };
    // Never block or unwind in a WinHTTP callback. Only one operation is pending,
    // so capacity eight also leaves space for cancellation and an error on close.
    let _ = context.events.try_send(event);
}

struct Handle(HINTERNET);
impl Handle {
    fn new(handle: HINTERNET) -> Result<Self, Failure> {
        if handle.0.is_null() {
            Err(Failure::Network)
        } else {
            Ok(Self(handle))
        }
    }
}
impl Drop for Handle {
    fn drop(&mut self) {
        // SAFETY: unique worker-owned handle, closed once, children dropped first.
        // For async requests this cancels pending I/O without waiting for it.
        unsafe {
            let _ = WinHttpCloseHandle(self.0);
        }
    }
}

pub struct Worker {
    cancel: SyncSender<Completion>,
    pub results: Receiver<CheckResult>,
}
impl Worker {
    /// A second managed check protects the native dispatch boundary itself.
    pub fn start(thread: u32, wake: u32) -> Option<Self> {
        if managed::detect() {
            return None;
        }
        let (events, receiver) = mpsc::sync_channel(8);
        let (results, output) = mpsc::sync_channel(1);
        let context = Arc::new(Context {
            events: events.clone(),
            buffer: UnsafeCell::new([0; CHUNK]),
            #[cfg(test)]
            closed: None,
        });
        std::thread::Builder::new()
            .name("update-check".into())
            .spawn(move || {
                let result = request(context, receiver, LIMIT).unwrap_or_else(CheckResult::Failed);
                let _ = results.try_send(result);
                // SAFETY: a numeric thread id, no window handle or borrowed state.
                // The UI owns its message queue; a late wake contains no result pointer.
                unsafe {
                    let _ = PostThreadMessageW(thread, wake, WPARAM(0), LPARAM(0));
                }
            })
            .ok()?;
        Some(Self {
            cancel: events,
            results: output,
        })
    }
    pub fn cancel(&self) {
        let _ = self.cancel.try_send(Completion::Cancel);
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        self.cancel();
    } // deliberately no join on the UI thread
}

fn wait(events: &Receiver<Completion>, deadline: Instant) -> Result<Completion, Failure> {
    let left = deadline
        .checked_duration_since(Instant::now())
        .ok_or(Failure::Timeout)?;
    match events.recv_timeout(left) {
        Ok(Completion::Cancel) => Err(Failure::Cancelled),
        Ok(Completion::Error) | Err(mpsc::RecvTimeoutError::Disconnected) => Err(Failure::Network),
        Ok(value) => Ok(value),
        Err(mpsc::RecvTimeoutError::Timeout) => Err(Failure::Timeout),
    }
}

fn header(request: HINTERNET, name: PCWSTR) -> Option<String> {
    let mut buffer = [0u16; 128];
    let mut size = std::mem::size_of_val(&buffer) as u32;
    // SAFETY: fixed writable buffer; overlong or missing headers fail closed.
    if !unsafe {
        WinHttpQueryHeaders(
            request,
            WINHTTP_QUERY_CUSTOM as u32,
            name,
            Some(buffer.as_mut_ptr().cast()),
            &mut size,
            None,
        )
    }
    .as_bool()
    {
        return None;
    }
    let count = (size as usize / 2).min(buffer.len());
    String::from_utf16(&buffer[..count])
        .ok()
        .map(|s| s.trim_end_matches('\0').trim().to_owned())
}

fn request(
    context: Arc<Context>,
    events: Receiver<Completion>,
    limit: Duration,
) -> Result<CheckResult, Failure> {
    request_to(
        context,
        events,
        limit,
        Target {
            host: w!("api.github.com"),
            port: 443,
            path: w!("/repos/tjorim/ElevenClock-rs/releases/latest"),
            flags: WINHTTP_FLAG_SECURE as u32,
        },
    )
}

// Private transport target: production has only the fixed HTTPS endpoint above.
// Tests exercise the same async engine against a local scripted TCP server.
struct Target {
    host: PCWSTR,
    port: u16,
    path: PCWSTR,
    flags: u32,
}
fn request_to(
    context: Arc<Context>,
    events: Receiver<Completion>,
    limit: Duration,
    target: Target,
) -> Result<CheckResult, Failure> {
    let deadline = Instant::now() + limit;
    if matches!(events.try_recv(), Ok(Completion::Cancel)) {
        return Err(Failure::Cancelled);
    }
    let agent: Vec<u16> = format!("ElevenClock-rs/{}\0", env!("CARGO_PKG_VERSION"))
        .encode_utf16()
        .collect();
    // SAFETY: all strings/buffers live through their calls; handles stay on this
    // worker. No proxy auto-discovery, cookies, auth, redirects or TLS overrides.
    unsafe {
        let session = Handle::new(WinHttpOpen(
            PCWSTR(agent.as_ptr()),
            WINHTTP_ACCESS_TYPE_NO_PROXY as u32,
            None,
            None,
            WINHTTP_FLAG_ASYNC as u32,
        ))?;
        if !WinHttpSetTimeouts(session.0, 10000, 10000, 10000, 10000).as_bool() {
            return Err(Failure::Network);
        }
        let connection = Handle::new(WinHttpConnect(
            session.0,
            target.host,
            INTERNET_PORT(target.port),
            0,
        ))?;
        let request = Handle::new(WinHttpOpenRequest(
            connection.0,
            w!("GET"),
            target.path,
            None,
            None,
            None,
            target.flags,
        ))?;
        let mut disabled = (WINHTTP_DISABLE_AUTHENTICATION
            | WINHTTP_DISABLE_COOKIES
            | WINHTTP_DISABLE_REDIRECTS) as u32;
        if !WinHttpSetOption(
            Some(request.0),
            WINHTTP_OPTION_DISABLE_FEATURE as u32,
            (&mut disabled as *mut u32).cast(),
            4,
        )
        .as_bool()
        {
            return Err(Failure::Network);
        }
        let raw = Arc::into_raw(context.clone());
        let mut value = raw as usize;
        if !WinHttpSetOption(
            Some(request.0),
            WINHTTP_OPTION_CONTEXT_VALUE as u32,
            (&mut value as *mut usize).cast(),
            std::mem::size_of::<usize>() as u32,
        )
        .as_bool()
        {
            drop(Arc::from_raw(raw));
            return Err(Failure::Network);
        }
        let previous = WinHttpSetStatusCallback(
            request.0,
            Some(callback),
            (WINHTTP_CALLBACK_FLAG_ALL_COMPLETIONS | WINHTTP_CALLBACK_FLAG_HANDLES) as u32,
            0,
        );
        if previous.is_some_and(|f| f as usize == usize::MAX) {
            drop(Arc::from_raw(raw));
            return Err(Failure::Network);
        }
        // From here the callback owns raw until HANDLE_CLOSING, including errors
        // and cancellation. No stack buffers are used for asynchronous reads.
        if matches!(events.try_recv(), Ok(Completion::Cancel)) {
            return Err(Failure::Cancelled);
        }
        if Instant::now() >= deadline {
            return Err(Failure::Timeout);
        }
        if !WinHttpSendRequest(request.0, None, None, 0, 0, value).as_bool() {
            return Err(Failure::Network);
        }
        if !matches!(wait(&events, deadline)?, Completion::Sent) {
            return Err(Failure::Network);
        }
        if !WinHttpReceiveResponse(request.0, std::ptr::null()).as_bool() {
            return Err(Failure::Network);
        }
        if !matches!(wait(&events, deadline)?, Completion::Headers) {
            return Err(Failure::Network);
        }
        let mut status = 0u32;
        let mut size = 4;
        if !WinHttpQueryHeaders(
            request.0,
            (WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER) as u32,
            None,
            Some((&mut status as *mut u32).cast()),
            &mut size,
            None,
        )
        .as_bool()
        {
            return Err(Failure::BadResponse);
        }
        if status == 429 || status == 403 {
            return Err(Failure::RateLimited {
                until: backoff(
                    now(),
                    header(request.0, w!("Retry-After")).as_deref(),
                    header(request.0, w!("X-RateLimit-Reset")).as_deref(),
                ),
            });
        }
        if status != 200 {
            return Err(Failure::BadResponse);
        }
        let mut body = Vec::new();
        loop {
            // Request at most the remaining cap plus one byte to detect overflow.
            let count = CHUNK.min(RESPONSE_LIMIT + 1 - body.len());
            if !WinHttpReadData(
                request.0,
                context.buffer.get().cast(),
                count as u32,
                std::ptr::null_mut(),
            )
            .as_bool()
            {
                return Err(Failure::Network);
            }
            let Completion::Read(length) = wait(&events, deadline)? else {
                return Err(Failure::Network);
            };
            if length == 0 {
                break;
            }
            if length > count || body.len() + length > RESPONSE_LIMIT {
                return Err(Failure::Oversized);
            }
            body.extend_from_slice(&(&*context.buffer.get())[..length]);
        }
        Ok(decode(&body, env!("CARGO_PKG_VERSION")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cancellation_and_overall_deadline_are_local_and_bounded() {
        let (tx, rx) = mpsc::sync_channel(8);
        tx.send(Completion::Cancel).unwrap();
        assert!(matches!(
            wait(&rx, Instant::now() + LIMIT),
            Err(Failure::Cancelled)
        ));
        assert!(matches!(wait(&rx, Instant::now()), Err(Failure::Timeout)));
    }
    #[test]
    fn closing_callback_releases_its_reference_after_cancellation() {
        let (events, _) = mpsc::sync_channel(8);
        let context = Arc::new(Context {
            events,
            buffer: UnsafeCell::new([0; CHUNK]),
            #[cfg(test)]
            closed: None,
        });
        let raw = Arc::into_raw(context.clone());
        assert_eq!(Arc::strong_count(&context), 2);
        // SAFETY: simulate the final WinHTTP callback with its owned Arc.
        unsafe {
            callback(
                HINTERNET::default(),
                raw as usize,
                WINHTTP_CALLBACK_STATUS_HANDLE_CLOSING as u32,
                std::ptr::null_mut(),
                0,
            );
        }
        assert_eq!(Arc::strong_count(&context), 1);
    }

    struct Fixture {
        cancel: SyncSender<Completion>,
        result: Receiver<Result<CheckResult, Failure>>,
        closed: Receiver<()>,
        headers: Receiver<String>,
        release: SyncSender<()>,
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = self.release.try_send(());
        }
    }
    fn fixture(response: Vec<u8>, stall: bool, limit: Duration) -> Fixture {
        use std::{
            io::{Read, Write},
            net::TcpListener,
        };
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        let (headers_tx, headers) = mpsc::sync_channel(1);
        let (release, resume) = mpsc::sync_channel(1);
        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            stream
                .set_write_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            let mut request = Vec::new();
            while request.len() < 4096 && !request.ends_with(b"\r\n\r\n") {
                let mut byte = [0];
                if stream.read_exact(&mut byte).is_err() {
                    return;
                }
                request.push(byte[0]);
            }
            let _ = headers_tx.send(String::from_utf8(request).unwrap());
            let _ = stream.write_all(&response);
            if stall {
                let _ = resume.recv_timeout(Duration::from_secs(2));
            }
        });
        let (cancel, events) = mpsc::sync_channel(8);
        let context_events = cancel.clone();
        let (result_tx, result) = mpsc::sync_channel(1);
        let (closed_tx, closed) = mpsc::sync_channel(1);
        std::thread::spawn(move || {
            let context = Arc::new(Context {
                events: context_events,
                buffer: UnsafeCell::new([0; CHUNK]),
                closed: Some(closed_tx),
            });
            let answer = request_to(
                context,
                events,
                limit,
                Target {
                    host: w!("127.0.0.1"),
                    port,
                    path: w!("/fixture"),
                    flags: 0,
                },
            );
            let _ = result_tx.send(answer);
        });
        Fixture {
            cancel,
            result,
            closed,
            headers,
            release,
        }
    }
    fn response(status: &str, headers: &str, body: &[u8]) -> Vec<u8> {
        let mut bytes = format!(
            "HTTP/1.1 {status}\r\nContent-Length: {}\r\n{headers}Connection: close\r\n\r\n",
            body.len()
        )
        .into_bytes();
        bytes.extend_from_slice(body);
        bytes
    }
    fn answer(f: &Fixture) -> Result<CheckResult, Failure> {
        let result = f
            .result
            .recv_timeout(Duration::from_secs(3))
            .expect("worker must finish without the server closing");
        f.closed
            .recv_timeout(Duration::from_secs(3))
            .expect("request callback context/buffer must be released");
        result
    }
    #[test]
    fn winhttp_decodes_a_scripted_release_and_sends_only_fixed_metadata() {
        let body = br#"{"tag_name":"v99.0.0","html_url":"https://github.com/tjorim/ElevenClock-rs/releases/tag/v99.0.0","draft":false,"prerelease":false}"#;
        let f = fixture(response("200 OK", "", body), false, LIMIT);
        let headers = f
            .headers
            .recv_timeout(Duration::from_secs(3))
            .unwrap()
            .to_ascii_lowercase();
        assert!(headers.contains(&format!(
            "user-agent: elevenclock-rs/{}",
            env!("CARGO_PKG_VERSION")
        )));
        assert!(!headers.contains("cookie:"));
        assert!(!headers.contains("authorization:"));
        assert!(matches!(answer(&f), Ok(CheckResult::Available { .. })));
    }
    #[test]
    fn winhttp_overall_deadline_includes_a_partial_chunked_body() {
        let f = fixture(
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n1\r\n{\r\n".to_vec(),
            true,
            Duration::from_millis(250),
        );
        assert_eq!(answer(&f), Err(Failure::Timeout));
    }
    #[test]
    fn winhttp_cancellation_closes_handles_without_waiting_for_the_peer() {
        let f = fixture(Vec::new(), true, LIMIT);
        f.headers.recv_timeout(Duration::from_secs(3)).unwrap();
        f.cancel.send(Completion::Cancel).unwrap();
        assert_eq!(answer(&f), Err(Failure::Cancelled));
    }
    #[test]
    fn winhttp_rejects_redirects_rate_limits_and_streamed_overflow() {
        let f = fixture(
            response("302 Found", "Location: https://example.invalid/\r\n", b""),
            false,
            LIMIT,
        );
        assert_eq!(answer(&f), Err(Failure::BadResponse));
        let before = now();
        let f = fixture(
            response("429 Too Many Requests", "Retry-After: 120\r\n", b""),
            false,
            LIMIT,
        );
        assert!(matches!(answer(&f), Err(Failure::RateLimited { until }) if until >= before + 120));
        let f = fixture(
            response("200 OK", "", &vec![b' '; RESPONSE_LIMIT + 1]),
            false,
            LIMIT,
        );
        assert_eq!(answer(&f), Err(Failure::Oversized));
    }
}
