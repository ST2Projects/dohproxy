use rustdns::Message;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Response {
    status: String,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    pub ad: bool,
    pub cd: bool,
    pub question: Vec<Question>,
    pub answer: Vec<Answer>,
    pub comment: String
}

impl Response {
    pub fn from_dns_msg(dns_msg: Message, question: Question) -> Self {
        let answers = dns_msg.answers.iter().map(|m| Answer{
            name: m.name.clone(),
            answer_type: m.class.to_string(),
            ttl: m.ttl.as_secs() as u32,
            data: m.resource.to_string()
        }).collect();
        Response {
            status: "".to_string(),
            tc: dns_msg.tc,
            rd: dns_msg.rd,
            ra: dns_msg.ra,
            ad: dns_msg.ad,
            cd: dns_msg.cd,
            question: vec!(question),
            answer: answers,
            comment: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Question {
    pub(crate) name: String,
    #[serde(alias="type")]
    pub(crate) question_type: String,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Answer {
    pub(crate) name: String,
    #[serde(alias="type")]
    pub(crate) answer_type: String,
    #[serde(alias="TTL")]
    pub(crate) ttl: u32,
    pub(crate) data: String
}

impl Question {
    pub fn new(name: String, question_type: String) -> Self {
        Question{
            name,
            question_type
        }
    }
}

impl Answer {
    pub fn new(name: String, answer_type: String, ttl: u32, data: String) -> Self {
        Answer {
            name,
            answer_type,
            ttl,
            data
        }
    }
}