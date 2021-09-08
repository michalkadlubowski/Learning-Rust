
use crate::http::QueryString;
use crate::http::QueryStringValue as QueryStringValue;

use super::{http::{Response, StatusCode, Method}, server::Handler};
use std::{fs};

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self{
        WebsiteHandler { public_path }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}\\{}", self.public_path, file_path);
        println!("{}", path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(fs::canonicalize(&self.public_path).unwrap()) {
                    fs::read_to_string(path).ok()
                }
                else {
                    println!("Dir traversal attack");
                    None
                }
            }
            Err(_) => None
        }
    }

    pub fn try_sum2(&self, query_str: Option<&QueryString>) -> Option<i32> {
        query_str
            .and_then(|x| x.get("nums"))
            .map(|x|  match x {
                QueryStringValue::Single(s) => vec!(*s),
                QueryStringValue::Multiple(m) => m.to_vec(),
            })
            .and_then(|x| 
                x.iter()
                .map(|s| s.parse::<i32>().ok())
                .fold(Some(0i32), |acc, e| match (acc,e) {
                    (Some(a), Some(b)) => a.checked_add(b),
                    (_,_) => None
                })
            )
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> crate::http::Response {
        println!("{}",request.path());
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                "/best" => Response::new(StatusCode::Ok, Some("<h1>Best</h1>".to_string())),
                "/sum" => match self.try_sum2(request.query_string()) {
                    Some(result) => Response::new(StatusCode::Ok, Some(format!("<h1>Sum is: {}</h1>", result))),
                    None => Response::new(StatusCode::BadRequest, None),
                },
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::Ok, Some(content)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            },
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}
