use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "Get" => Method::Get,
            "Post" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub resource: Resource,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = "";
        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            } else if line.contains(":") {
                let (key, value) = process_req_headers(line);
                parsed_headers.insert(key, value);
            } else if line.is_empty() {
                // process blank line, ignore it;
            } else {
                // process body;
                parsed_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            resource: parsed_resource,
            version: parsed_version,
            headers: parsed_headers,
            msg_body: parsed_body.to_string(),
        }
    }
}

fn process_req_headers(line: &str) -> (String, String) {
    let mut seg_iter = line.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = seg_iter.next() {
        key.push_str(k);
    }
    if let Some(v) = seg_iter.next() {
        value.push_str(v);
    }
    (key, value)
}

fn process_req_line(line: &str) -> (Method, Resource, Version) {
    let mut seg_iter = line.split_whitespace();
    let method = seg_iter.next().unwrap();
    let resource = seg_iter.next().unwrap();
    let version = seg_iter.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

#[cfg(test)]
mod httprequest_testsuite {

    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "Get".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s = String::from("Get /greeting HTTP/1.1\r\nHost: localhost\r\nAccept: text/html\r\n\r\nthis is request body");
        let http_request: HttpRequest = s.into();

        let mut expected_headers = HashMap::<String, String>::new();
        // expected_headers.insert(String::from("Host"), String::from(" localhost"));
        expected_headers.insert("Host".into(), " localhost".into());
        expected_headers.insert(String::from("Accept"), String::from(" text/html"));

        assert_eq!(Method::Get, http_request.method);
        assert_eq!(
            Resource::Path("/greeting".to_string()),
            http_request.resource
        );
        assert_eq!(Version::V1_1, http_request.version);
        assert_eq!(expected_headers, http_request.headers);
    }
}
