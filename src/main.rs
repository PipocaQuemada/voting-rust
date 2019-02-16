#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[post("/election/approval", format = "application/json", data="<election>")]
fn create_election(election: Json<Election>) -> &'static str {
    "Ok"
}


fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

///////////////////

#[derive(Deserialize)]
struct Election {
    name: String,
    candidates: Vec<String>
}

struct ApprovalBallot {
    approvedCandidates: Vec<String>
}

trait ApprovalRepository {
    fn createElection(&self, election: Election) -> bool;
    fn vote(&self, ballot: ApprovalBallot) -> bool;
    fn results(&self, name: str);
}

