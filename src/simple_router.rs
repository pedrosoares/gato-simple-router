use gato_core::kernel::{Router, Request, Response, Logger, RequestBuilder};
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
    pub fn any(endpoint: &str, f: &'static dyn Fn(&Request) -> Response) {
        unsafe {
            ENDPOINTS.push(Endpoint::new(endpoint, "ANY", f));
        }
    }
    pub fn options(endpoint: &str, f: &'static dyn Fn(&Request) -> Response) {
        unsafe {
            ENDPOINTS.push(Endpoint::new(endpoint, "OPTIONS", f));
        }
    }
    pub fn head(endpoint: &str, f: &'static dyn Fn(&Request) -> Response) {
        unsafe {
            ENDPOINTS.push(Endpoint::new(endpoint, "HEAD", f));
        }
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
    fn match_route_name(&self, uri: &str, router: &str, request_builder: &mut RequestBuilder) -> bool {
        if uri == "*" || uri == router {
            return true;
        }
        let mut params : HashMap<String, String> = HashMap::new();
        
        // Remove QueryString
        let p: Vec<&str> = router.split("?").collect();
        
        // Get ROUTER Folders
        let route_piece : Vec<&str> = p[0].split("/").filter(|&x| x != "").collect();
        
        // Get URL Folders
        let uri_piece : Vec<&str> = uri.split("/").filter(|&x| x != "").collect();
        
        // Get URL QueryString Variables
        let url_query: Vec<&str> = if p.len() > 1 { p[1].split("&").collect() } else { vec!() };
        
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
            request_builder.add_params(params);
            let querystring: HashMap<_, _> = url_query.iter().map(|&params| {
                let p: Vec<_> = params.split("=").collect();
                return if p.len() == 2 {
                    (p[0].to_string(), p[1].to_string())
                } else {
                    (p[0].to_string(), String::new())
                }
            }).collect();
            request_builder.add_querystring(querystring);
            return true;
        }

        return false;
    }
}

impl Router for SimpleRouter {
    fn boot(&self) -> () {
        Logger::info("SimpleRouter[boot]");
    }
    fn handle(&self, request_builder: &mut RequestBuilder) -> Response {
        Logger::info("SimpleRouter[handle]");

        let endpoints = unsafe { ENDPOINTS.iter() };
        let request = request_builder.get_request();

        for endpoint in endpoints {
            let did_match_route : bool = self.match_route_name(
                endpoint.uri.as_str(), request.get_uri().as_str(), request_builder
            );
            if did_match_route {
                let req_method = request.get_method();
                if endpoint.method == "ANY" || endpoint.method == req_method {
                    let request = request_builder.get_request();
                    return (endpoint.handler)(&request);
                } else if req_method == "OPTIONS" || req_method == "HEAD" {
                    return Response::new().status(200).raw("");
                }
            }
        }

        return Response::new().status(404).json(serde_json::json!({
            "error": "Page Not Found"
        }));
    }
}

const fn new_vec() -> Vec<Endpoint> {
    return Vec::new();
}

static mut ENDPOINTS : Vec<Endpoint> = new_vec();
