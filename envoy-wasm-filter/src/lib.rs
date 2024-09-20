use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|context_id,
 _| -> Box<dyn HttpContext> {
        Box::new(MyHttpContext { context_id })
    });
}

struct MyHttpContext {
    context_id: u32,
}

impl Context for MyHttpContext {}

impl HttpContext for MyHttpContext {
    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        self.add_http_response_header("x-wasm-filter", "hello");
        Action::Continue
    }
}