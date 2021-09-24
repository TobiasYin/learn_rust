use std::collections::HashMap;
use std::net::TcpStream;
use crate::http::result::Res;
use crate::http::buf_reader::BufferReader;
use std::time::Duration;


#[derive(Debug)]
pub struct Request {
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub version: String,
    pub method: String,
    pub uri: String,
    pub keep_alive: bool,
    content_len: usize,
    response: Option<Response>,
}

#[allow(dead_code)]
impl Request {
    pub fn set_response(&mut self, resp: Option<Response>) {
        self.response = resp;
    }

    fn init_response(&mut self) {
        if let None = self.response {
            self.response = Some(Response::new())
        }
    }

    fn update_response<T>(&mut self, f: T)
        where T: FnOnce(&mut Response) -> ()
    {
        self.init_response();
        let refs = self.response.as_mut().unwrap();
        f(refs);
    }

    pub fn set_response_body(&mut self, body: &[u8]) {
        self.update_response(|r| r.body = Vec::from(body));
    }

    pub fn set_status_code(&mut self, status: i32) {
        self.update_response(|r| r.status_code = status);
    }

    pub fn set_response_headers(&mut self, header: HashMap<String, String>) {
        self.update_response(|r| r.headers = header);
    }

    pub fn set_response_header(&mut self, k: &str, v: &str) {
        self.update_response(|r| {
            r.headers.insert(k.to_string(), v.to_string());
        })
    }

    pub fn borrow_response_mut(&mut self) -> &mut Response {
        self.init_response();
        self.response.as_mut().unwrap()
    }

    pub fn get_response(&self) ->  &Option<Response>{
        &self.response
    }

    pub fn move_response(mut self) ->  Response{
        self.init_response();
        self.response.unwrap()
    }
}


#[derive(Debug)]
pub struct Response {
    pub body: Vec<u8>,
    pub status_code: i32,
    pub headers: HashMap<String, String>,
    pub version: String,
}

impl Response {
    pub(crate) fn new() -> Response {
        Response {
            body: vec![],
            status_code: 0,
            headers: Default::default(),
            version: "HTTP/1.1".to_string(),
        }
    }
}


pub fn read_request(conn: &mut TcpStream) -> Res<Request> {
    let mut req = Request {
        body: vec![],
        headers: HashMap::new(),
        version: String::new(),
        method: String::new(),
        uri: String::new(),
        keep_alive: false,
        content_len: 0,
        response: None,
    };
    let pointer = conn as *mut TcpStream;
    let mut buf_reader = BufferReader::new(conn);
    buf_reader.set_line_sep("\r\n");

    unsafe {
        (*pointer).set_read_timeout(None)?;
    }


    let first_line = buf_reader.read_line()?;

    // 第一条阻塞读，用于等待请求到来
    unsafe {
        (*pointer).set_read_timeout(Some(Duration::from_secs(10)))?;
    }

    let items: Vec<&str> = first_line.split(" ").collect();
    if items.len() != 3 {
        return err_bad_request();
    }
    req.method.push_str(items[0]);
    req.uri.push_str(items[1]);
    req.version.push_str(items[2]);


    let mut line = buf_reader.read_line()?;
    while line.len() != 0 {
        let (k, v) = line.split_once(":").unwrap_or_else(|| (&line, ""));
        let k = k.trim();
        let v = v.trim();
        handle_special_header(&mut req, k, v)?;
        req.headers.insert(String::from(k), String::from(v));
        line = buf_reader.read_line()?;
    }

    req.body = buf_reader.read_size(req.content_len)?;

    Ok(req)
}


fn err_bad_request<T>() -> Res<T> {
    Err(Box::from("Bad Request"))
}


fn handle_special_header(req: &mut Request, k: &str, v: &str) -> Res<()> {
    let k = k.to_lowercase();
    let v = v.to_lowercase();
    if k == "content-length" {
        req.content_len = v.parse()?;
    }
    if k == "connection" {
        if v == "keep-alive" {
            req.keep_alive = true;
        }
    }

    Ok(())
}

