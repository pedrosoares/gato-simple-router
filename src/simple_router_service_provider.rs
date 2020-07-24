use gato_core::kernel::Provider;
use gato_core::kernel::RouterHandler;
use crate::SimpleRouter;

pub struct SimpleRouterServiceProvider {

}

impl SimpleRouterServiceProvider {
    pub fn new() -> Box<Self> {
        return Box::new(SimpleRouterServiceProvider {});
    }
}

impl Provider for SimpleRouterServiceProvider {
    fn boot(&self) -> () {
        RouterHandler::set_driver(Box::new(SimpleRouter::new()));
    }
}
