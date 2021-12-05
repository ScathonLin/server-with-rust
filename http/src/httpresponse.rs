use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(resp: HttpResponse<'a>) -> Self {
        let cloned_resp = resp.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &cloned_resp.version(),
            &cloned_resp.status_code(),
            &cloned_resp.status_text(),
            &cloned_resp.headers(),
            // todo
            resp.body.unwrap().len(),
            &cloned_resp.body(),
        )
    }
}

impl<'a> HttpResponse<'a> {
    fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        }
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::<&'a str, &'a str>::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found",
            "500" => "Internel Server Error".into(),
            _ => "Unknown Error".into(),
        };
        response.body = body;
        response
    }
    pub fn send_resp(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let headers = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        // 对header进行排序
        let mut header_keys = headers.keys().collect::<Vec<&&str>>();
        header_keys.sort();
        for hk in header_keys {
            header_string = format!("{}{}:{}\r\n", header_string, hk, headers[hk]);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod http_response_testsuite {
    use super::*;
    #[test]
    fn test_response_struct_creation_200() {
        let actual_resp = HttpResponse::new("200", None, Some("xxx".into()));
        let expected_resp = HttpResponse {
            status_code: "200",
            status_text: "OK",
            version: "HTTP/1.1",
            headers: {
                let mut hd = HashMap::new();
                hd.insert("Content-Type", "text/html");
                Some(hd)
            },
            body: Some("xxx".into()),
        };
        assert_eq!(actual_resp, expected_resp);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new("404", None, Some("yyyy".into()));
        let response_expected = HttpResponse {
            status_code: "404",
            status_text: "Not Found",
            version: "HTTP/1.1",
            headers: {
                let mut hd = HashMap::new();
                hd.insert("Content-Type", "text/html");
                Some(hd)
            },
            body: Some("yyyy".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
    #[test]
    fn test_response_creation() {
        let expected_response = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut hd = HashMap::new();
                hd.insert("Content-Type", " application/json");
                hd.insert("Host", " localhost");
                hd.insert("Cookie", " name=linhuadong");
                Some(hd)
            },
            body: Some("this is a json response".into()),
        };
        let expected_response_string = "HTTP/1.1 200 OK\r\n\
        Content-Type: application/json\r\n\
        Cookie: name=linhuadong\r\n\
        Host: localhost\r\n\
        Content-Length: 23\r\n\
        \r\n\
        this is a json response";
        let actual_response_string: String = expected_response.into();
        assert_eq!(expected_response_string, actual_response_string);
    }
}
