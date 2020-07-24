use gato_core::kernel::{Router, Request, Response, Logger};
use std::collections::HashMap;

struct Endpoint {
    uri: String,
    method: String,
    handler: &'static dyn Fn(&Request) -> Response
}

impl Endpoint {
    pub fn new(uri: &str, method: &str, handler: &'static dyn Fn(&Request) -> Response) -> Self {
        return Endpoint { uri: uri.to_string(), handler, method: method.to_string() };
    }
}

pub struct SimpleRouter { }

impl SimpleRouter {
    pub fn new() -> Self {
        return SimpleRouter { }
    }
    pub fn get(endpoint: &str, f: &'static dyn Fn(&Request) -> Response) {
        unsafe {
            ENDPOINTS.push(Endpoint::new(endpoint, "GET", f));
        }
    }
    pub fn post(endpoint: &str, f: &'static dyn Fn(&Request) -> Response) {
        unsafe {
            ENDPOINTS.push(Endpoint::new(endpoint, "POST", f));
        }
    }
    pub fn put(endpoint: &str, f: &'static dyn Fn(&Request) -> Response) {
        unsafe {
            ENDPOINTS.push(Endpoint::new(endpoint, "PUT", f));
        }
    }
    pub fn patch(endpoint: &str, f: &'static dyn Fn(&Request) -> Response) {
        unsafe {
            ENDPOINTS.push(Endpoint::new(endpoint, "PATCH", f));
        }
    }
    pub fn delete(endpoint: &str, f: &'static dyn Fn(&Request) -> Response) {
        unsafe {
            ENDPOINTS.push(Endpoint::new(endpoint, "DELETE", f));
        }
    }
}

impl SimpleRouter {
    fn match_route_name(&self, uri: &str, router: &str, request: &mut Request) -> bool {
        if uri == router {
            return true;
        }
        let mut params : HashMap<String, String> = HashMap::new();
        let uri_piece : Vec<&str> = uri.split("/").collect();
        let route_piece : Vec<&str> = router.split("/").collect();
        let limit = uri_piece.len();
        for i in 0..limit {
            if route_piece.len() <= i {
                return false;
            }
            let is_param =
                uri_piece[i].find("{").is_some() &&
                uri_piece[i].find("}").is_some();

            if route_piece[i] != uri_piece[i] && !is_param {
                return false;
            } else if is_param && uri_piece[i] == "" {
                return false;
            } else if is_param {
                let param = uri_piece[i].to_string();
                params.insert(param.replace("{", "").replace("}", ""), route_piece[i].to_string());
            }
        }

        if uri_piece.len() == route_piece.len() {
            request.add_params(params);
            return true;
        }

        return false;
    }
}

impl Router for SimpleRouter {
    fn boot(&self) -> () {
        Logger::info("SimpleRouter[boot]");
    }
    fn handle(&self, request: &mut Request) -> Response {
        Logger::info("SimpleRouter[handle]");

        let endpoints = unsafe { ENDPOINTS.iter() };

        for endpoint in endpoints {
            let did_match_route : bool = self.match_route_name(
                endpoint.uri.as_str(), request.get_uri().as_str(), request
            );
            if did_match_route && endpoint.method == request.get_method() {
                return (endpoint.handler)(request);
            }
        }

        let mut response = Response::json(serde_json::json!({
            "error": "Page Not Found"
        }));
        response.set_code(404);
        return response;
    }
}

const fn new_vec() -> Vec<Endpoint> {
    return Vec::new();
}

static mut ENDPOINTS : Vec<Endpoint> = new_vec();
