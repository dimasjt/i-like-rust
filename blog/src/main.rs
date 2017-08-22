extern crate iron;
extern crate logger;
extern crate env_logger;
extern crate router;

use iron::prelude::*;
use iron::status;
use logger::Logger;
use router::Router;

fn main() {
    env_logger::init().unwrap();

    let (logger_before, logger_after) = Logger::new(None);

    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", handler, "query");


    let mut chain = Chain::new(router);

    chain.link_before(logger_before);
    chain.link_after(logger_after);

    match Iron::new(chain).http("localhost:3000") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => println!("{:?}", err),
    }
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("home");
    Ok(Response::with((status::Ok, *query)))
}
