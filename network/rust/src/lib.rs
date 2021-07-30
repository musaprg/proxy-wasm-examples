use log::info;
use std::str;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(Network) });
}

struct Network;

impl Context for Network {
    fn on_done(&mut self) -> bool {
        info!("connection completed");
        true
    }
}

impl RootContext for Network {}

impl HttpContext for Network {}

impl StreamContext for Network {
    fn on_new_connection(&mut self) -> Action {
        info!("new connection");
        Action::Continue
    }

    fn on_downstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        if _data_size == 0 {
            return Action::Continue
        }

        let data = self.get_downstream_data(0, _data_size).unwrap_or_else(|| panic!("failed to get downstream data"));
        
        info!(">>>>>> downstream data received >>>>>>\n{}", str::from_utf8(&data).unwrap());
        Action::Continue
    }

    fn on_downstream_close(&mut self, _peer_type: PeerType) {
        info!("downstream connection closed");
    }

    fn on_upstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        if _data_size == 0 {
            return Action::Continue
        }

        let data = self.get_upstream_data(0, _data_size).unwrap_or_else(|| panic!("failed to get upstream data"));

        info!(">>>>>> upstream data received >>>>>>\n{}", str::from_utf8(&data).unwrap());
        Action::Continue
    }

    fn on_upstream_close(&mut self, _peer_type: PeerType) {
        info!("upstream connection closed");
    }
}
