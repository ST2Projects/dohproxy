mod response;

#[macro_use] extern crate rocket;

use std::fmt::format;
use std::io::{Error, ErrorKind};
use std::net::UdpSocket;
use std::str::FromStr;
use std::time::Duration;
use rocket::http::hyper::body::HttpBody;
use crate::response::{Answer, Question};
use rustdns::{Class, Extension, Message, MX, Type};
use uuid::Uuid;

#[get("/resolve?<name>&<type>&<do>")]
async fn resolve(name: &str, r#type: &str, r#do: u8  ) -> Option<String> {

    info!("Got request for [{}] of type [{}] and do [{}]", name, r#type, r#do);

    let dns_msg = make_dns_request(name, r#type).ok()?;

    let question = Question::new(name.to_string(), r#type.to_string());

    let mut response = response::Response::from_dns_msg(dns_msg, question);

    let comment = format!("RequestId {} Served by DOH Proxy", Uuid::new_v4());
    response.comment = comment;

    serde_json::to_string(&response).ok()
}

fn make_dns_request(name: &str, r#type: &str) -> Result<Message, std::io::Error> {

    info!("Inside make_dns_request");
    let mut message = Message::default();
    message.add_question(name, type_from_request(r#type)?, Class::Internet);
    message.add_extension(Extension {   // Optionally add a EDNS extension
        payload_size: 4096,       // which supports a larger payload size.
        ..Default::default()
    });

    let sock = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind");
    sock.set_read_timeout(Some(Duration::new(5, 0)))?;
    sock.connect("192.168.1.4:53").expect("Failed to connect to host");

    let q = message.to_vec()?;
    sock.send(&q);

    let mut resp = [0; 4096];
    let len = sock.recv(&mut resp)?;

    let resp = Message::from_slice(&resp[0.. len])?;

    Ok(resp)
}

fn type_from_request(r#type: &str) -> Result<Type, Error> {
    match r#type.to_lowercase().as_str() {
        "a" => Ok(Type::A),
        "ns" => Ok(Type::NS),
        "cname" => Ok(Type::CNAME),
        "soa" => Ok(Type::SOA),
        "ptr" => Ok(Type::PTR),
        "mx" => Ok(Type::MX),
        "txt" => Ok(Type::TXT),
        "aaaa" => Ok(Type::AAAA),
        "srv" => Ok(Type::SRV),
        "opt" => Ok(Type::OPT),
        "spf" => Ok(Type::SPF),
        "any" => Ok(Type::ANY),
        _ => {
            let msg = format!("Unknown request type [{}]", r#type);
            error!("{}", msg);
            Err(Error::new(ErrorKind::NotFound, msg))
        }
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![resolve])
}
