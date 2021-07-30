use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(RootConfig) });
}

struct RootConfig;

impl Context for RootConfig {}

impl RootContext for RootConfig {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::StreamContext)
    }

    fn create_stream_context(&self, _context_id: u32) -> Option<Box<dyn StreamContext>> {
        Some(Box::new(Network))
    }
}

struct Network;

impl Context for Network {
    fn on_done(&mut self) -> bool {
        info!("connection completed");
        true
    }
}

impl StreamContext for Network {
    fn on_new_connection(&mut self) -> Action {
        info!("new connection");
        Action::Continue
    }

    fn on_downstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        if _data_size == 0 {
            return Action::Continue;
        }

        let data = self.get_downstream_data(0, _data_size).unwrap_or_else(|| panic!("failed to get downstream data"));
        
        info!(">>>>>> downstream data received >>>>>>\n{}", String::from_utf8_lossy(&data));
        Action::Continue
    }

    fn on_downstream_close(&mut self, _peer_type: PeerType) {
        info!("downstream connection closed");
    }

    fn on_upstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        if _data_size == 0 {
            return Action::Continue;
        }

        let data = self.get_upstream_data(0, _data_size).unwrap_or_else(|| panic!("failed to get upstream data"));

        info!(">>>>>> upstream data received >>>>>>\n{}", String::from_utf8_lossy(&data));
        Action::Continue
    }

    fn on_upstream_close(&mut self, _peer_type: PeerType) {
        info!("upstream connection closed");
    }
}
