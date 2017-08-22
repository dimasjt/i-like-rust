extern crate iron;
extern crate logger;
extern crate env_logger;
extern crate router;
#[macro_use] extern crate tera;

use iron::prelude::*;
use iron::status;
use logger::Logger;
use router::Router;
use tera::Tera;

fn main() {
    env_logger::init().unwrap();

    let (logger_before, logger_after) = Logger::new(None);

    let mut router = Router::new();
    router.get("/", handler_index, "index");
    router.get("/posts/:id", handler_show, "show");

    let tera = compile_templates!("templates/**/*");

    let mut chain = Chain::new(router);

    chain.link_before(logger_before);
    chain.link_after(logger_after);

    match Iron::new(chain).http("localhost:3000") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => println!("{:?}", err),
    }
}

fn handler_index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Index")))
}

fn handler_show(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Show")))
}
