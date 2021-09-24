use std::net::TcpStream;
use crate::http::request::{read_request, Request, Response};
use crate::http::result::Res;
use std::error::Error;
use std::io::Write;
use crate::http::handler::*;
use crate::http::status_code::*;
use std::io;

pub fn handle_connection<T: Handler>(mut conn: TcpStream, handler: T) {
    let base_handler = BaseHandler { handler };
    println!("connect: {}", conn.peer_addr().unwrap());
    loop {
        let req = read_request(&mut conn);

        if let Err(e) = &req {
            if let Some(err) = e.downcast_ref::<io::Error>(){
                if err.kind() == io::ErrorKind::UnexpectedEof {
                    break;
                }
            }
        }

        let (method, uri) = if let Ok(req) = &req { (&req.method[..], &req.uri[..]) } else { ("unknown", "unknown") };
        println!("new request: {} {}", method, uri);
        let keep_alive = if let Ok(req) = &req {
            req.keep_alive
        } else { false };
        if let Err(e) = handle_request(&mut conn, req, base_handler) {
            println!("error on handle request: {}", e);
            break;
        }
        if !keep_alive {
            break;
        }
    }
    println!("disconnect: {}", conn.peer_addr().unwrap());
}

fn handle_request<T: Handler>(mut conn: &mut TcpStream, req: Res<Request>, handler: T) -> Res<()> {
    let resp = match req {
        Ok(mut req) => {
            let ret = handler.handle(&mut req);
            if let Err(err) = ret {
                error_response_for_request(&mut req, err)
            } else {
                let r = req.borrow_response_mut();
                if r.status_code == 0 {
                    r.status_code = 200;
                }
            }
            req.move_response()
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
    let resp = req.borrow_response_mut();
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