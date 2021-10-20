
#[macro_use]            // seems to be useless idk why
extern crate rocket;
extern crate dashmap;
extern crate rand;


/************************************************************
 * Pointless use section, idk why 'extern crate' won't work *
 * Must fix ASAP because this looks so awful                *
 ************************************************************/

// dashmap
use dashmap::DashMap;

// rocket
use rocket::response::status::BadRequest;
use rocket::State;
use rocket::response::Redirect;
use rocket::response::status::NotFound;

// rand
use rand::thread_rng;
use crate::rand::Rng;


/****************************
 *  Actual code starts here *
 ****************************/


#[get("/")]                 // test
fn index() -> &'static str {
    "Hello, world!"
}


#[launch]
fn rocket() -> _ {          // sharing dashmap state across threads
    rocket::build()
        .manage(DashMap::<u32, String>::new())
        .mount("/", routes![index])
}


#[post("/api/shorten?<url>")]       // listen to POST requests of the form /api/shorten?url=___
fn shorten(url: String, state: &State<DashMap<u32, String>>) -> Result<String, BadRequest<&str>> {
    if url.is_empty() {
        Err(BadRequest(Some("URL is empty")))
    } else {
        let key: u32 = thread_rng().gen();      // generates random url to return
        state.insert(key, url);
        Ok(key.to_string())
    }
}


#[get("/<key>")]                    // listen to GET requests of the form /<key> (number)
fn redirect(key: u32, state: &State<DashMap<u32, String>>) -> Result<Redirect, NotFound<&str>> {
    state
        .get(&key)
        .map(|url| Redirect::to(url.clone()))
        .ok_or(NotFound("Invalid or expired link"))
}
