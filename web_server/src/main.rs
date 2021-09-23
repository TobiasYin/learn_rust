use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, self};
use std::time::Duration;
use std::collections::HashMap;
use std::error::Error;
use std::thread;

#[macro_use]
extern crate lazy_static;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move ||{
            handle_connection(stream);
        });
    }
}

struct BufferReader<'a, T>
    where T: Read
{
    reader: &'a mut T,
    buf: Vec<u8>,
    pos: usize,
    line_sep: Vec<u8>,
    truck_size: usize,
    truck_buf: Vec<u8>,
}

impl<'a, T: Read> BufferReader<'a, T> {
    fn new(read: &mut T) -> BufferReader<T> {
        let default_truck_size = 1024usize;
        BufferReader {
            reader: read,
            buf: vec![],
            pos: 0,
            line_sep: vec!['\n' as u8],
            truck_size: default_truck_size,
            truck_buf: {
                let mut v = Vec::with_capacity(default_truck_size);
                v.resize(default_truck_size, 0);
                v
            },
        }
    }

    fn inner_read(&mut self) -> io::Result<()> {
        let size = self.reader.read(&mut self.truck_buf)?;
        self.buf.extend_from_slice(&self.truck_buf[0..size]);
        Ok(())
    }

    fn inner_read_with_size(&mut self, mut size: usize) -> io::Result<()> {
        if size > self.truck_size {
            size = self.truck_size
        }
        let size = self.reader.read(&mut self.truck_buf[0..size])?;
        self.buf.extend_from_slice(&self.truck_buf[0..size]);
        Ok(())
    }

    fn next_byte(&mut self) -> io::Result<u8> {
        if self.buf.len() <= self.pos {
            self.inner_read()?;
        }
        let c = self.buf[self.pos];
        self.pos += 1;
        Ok(c)
    }

    pub fn set_line_sep(&mut self, sep: &str) {
        self.line_sep = Vec::from(sep);
    }

    pub fn read_line(&mut self) -> Res<String> {
        let start = self.pos;
        if self.line_sep.len() == 0 {
            return Ok(String::from_utf8(vec![self.next_byte()?])?);
        }
        let mut sep_pos = 0;
        loop {
            let next = self.next_byte()?;
            if next == self.line_sep[sep_pos] {
                sep_pos += 1;
                if sep_pos >= self.line_sep.len() {
                    break;
                }
            } else {
                sep_pos = 0;
            }
        }

        Ok(String::from(std::str::from_utf8(&self.buf[start..(self.pos - self.line_sep.len())])?))
    }

    pub fn read_size(&mut self, size: usize) -> io::Result<Vec<u8>> {
        let mut left = self.buf.len() - self.pos;
        while left < size {
            self.inner_read_with_size(left)?;
            left = self.buf.len() - self.pos;
        }

        Ok(Vec::from(&self.buf[self.pos..(self.pos + size)]))
    }
}

fn err_bad_request<T>() -> Res<T> {
    Err(Box::from("Bad Request"))
}

type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Request {
    body: Vec<u8>,
    headers: HashMap<String, String>,
    version: String,
    method: String,
    uri: String,
    keep_alive: bool,
    content_len: usize,
    response: Option<Response>,
}

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
}

#[derive(Debug)]
pub struct Response {
    body: Vec<u8>,
    status_code: i32,
    headers: HashMap<String, String>,
    version: String,
}

impl Response {
    fn new() -> Response {
        Response {
            body: vec![],
            status_code: 0,
            headers: Default::default(),
            version: "HTTP/1.1".to_string(),
        }
    }
}

fn read_request(conn: &mut TcpStream) -> Res<Request> {
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


fn handle_connection(mut conn: TcpStream) {
    println!("connect: {}", conn.peer_addr().unwrap());
    loop {
        let req = read_request(&mut conn);
        let (method, uri) = if let Ok(req) = &req { (&req.method[..], &req.uri[..]) } else { ("unknown", "unknown") };
        println!("new request: {} {}", method, uri);
        let keep_alive = if let Ok(req) = &req {
            req.keep_alive
        } else { false };
        if let Err(e) = handle_request(&mut conn, req){
            println!("error on handle request: {}", e);
            break;
        }
        if !keep_alive {
            break;
        }
    }
    println!("disconnect: {}", conn.peer_addr().unwrap());
}

fn handle_request(mut conn: &mut TcpStream, req: Res<Request>) -> Res<()> {
    let resp = match req {
        Ok(mut req) => {
            let ret = BaseHandler::handle(&mut req);
            if let Err(err) = ret {
                error_response_for_request(&mut req, err)
            } else {
                req.update_response(|r| {
                    if r.status_code == 0 {
                        r.status_code = 200;
                    }
                })
            }
            req.response.unwrap()
        }
        Err(err) => {
            let mut resp = Response::new();
            error_response(&mut resp, err);
            resp
        }
    };

    write_response(&mut conn, &resp)?;

    Ok(())
}

fn error_response_for_request(req: &mut Request, err: Box<dyn Error>) {
    if let None = req.response {
        req.response = Some(Response::new());
    }
    let resp = req.response.as_mut().unwrap();
    error_response(resp, err);
}

fn error_response(resp: &mut Response, err: Box<dyn Error>) {
    if resp.status_code == 0 {
        resp.status_code = 502;
    }
    let err_info = format!("Server Error: {}", err);
    if resp.body.len() == 0 {
        resp.body = Vec::from(err_info.as_bytes());
    }
    println!("error found: {}", err_info);
}

fn write_response(conn: &mut TcpStream, resp: &Response) -> Res<()> {
    let status_info = STATUS_CODE.get(&resp.status_code).unwrap_or(&"Unknown Status");
    let status_line = format!("{} {} {}", resp.version, resp.status_code, status_info);

    let header = resp.headers.iter().map(|(k, v)| -> String {
        format!("{}:{}", k, v)
    }).reduce(|o, n| -> String{
        format!("{}\r\n{}", o, n)
    });
    // write status
    conn.write(status_line.as_bytes())?;
    // write header
    if let Some(h) = header {
        conn.write("\r\n".as_bytes())?;
        conn.write(h.as_bytes())?;
    }
    conn.write("\r\n\r\n".as_bytes())?;
    //write body
    conn.write(&resp.body)?;

    Ok(())
}

pub trait Handler {
    fn handle(req: &mut Request) -> Res<()>;
}


macro_rules! handler {
    ($name:ident  $code:block) => {
        struct $name();
        impl Handler for $name{
            fn handle(req: &mut Request) -> Res<()>{
                let f :fn(req: &mut Request) -> Res<()> = $code;
                f(req)
            }
        }
    };
}


handler! {
    BaseHandler {
        // TODO route
        |req| {
            let mut new_body = Vec::from("hello, from base handler   ".as_bytes());
            new_body.extend_from_slice(&req.body);
            req.set_response_body(&new_body);
            let version = req.version.clone();
            req.update_response(|r| {
                let len = r.body.len();
                if len != 0{
                    r.headers.insert("Content-Length".to_string(), len.to_string());
                }
                r.version = version;
            });

            Ok(())
        }

    }
}





macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($k, $v);
            )*
            map
        }
    };
}


lazy_static! {
    static ref STATUS_CODE: HashMap<i32, &'static str> = create_status();
}

fn create_status() -> HashMap<i32, &'static str> {
    map! {
        100 => "Continue",
        101 => "Switching Protocol",
        102 => "Processing",
        103 => "Early Hints",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        207 => "Multi-Status",
        208 => "Already Reported",
        226 => "IM Used",
        300 => "Multiple Choice",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        304 => "Not Modified",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        402 => "Payment Required",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        406 => "Not Acceptable",
        407 => "Proxy Authentication Required",
        408 => "Request Timeout",
        409 => "Conflict",
        410 => "Gone",
        411 => "Length Required",
        412 => "Precondition Failed",
        413 => "Payload Too Large",
        414 => "URI Too Long",
        415 => "Unsupported Media Type",
        416 => "Range Not Satisfiable",
        417 => "Expectation Failed",
        418 => "I'm a teapot",
        421 => "Misdirected Request",
        422 => "Unprocessable Entity",
        423 => "Locked",
        424 => "Failed Dependency",
        425 => "Too Early",
        426 => "Upgrade Required",
        428 => "Precondition Required",
        429 => "Too Many Requests",
        431 => "Request Header Fields Too Large",
        451 => "Unavailable For Legal Reasons",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        505 => "HTTP Version Not Supported",
        506 => "Variant Also Negotiates",
        507 => "Insufficient Storage",
        508 => "Loop Detected",
        510 => "Not Extended",
        511 => "Network Authentication Required",
    }
}