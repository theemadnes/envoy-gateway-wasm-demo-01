use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(AddHeaderRoot) 
 });
}

struct AddHeaderRoot;

impl Context for AddHeaderRoot {}

impl RootContext for AddHeaderRoot {
    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(AddHeaderContext {}))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

struct AddHeaderContext;

impl Context for AddHeaderContext {}

impl HttpContext for AddHeaderContext {
    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        self.add_http_response_header("x-custom-header", "added-by-wasm-plugin");
        Action::Continue
    }
}