extern crate serde;
extern crate serde_json;

#[macro_use] extern crate nickel;
#[macro_use]
extern crate serde_derive;

use nickel::Nickel;

#[derive(Serialize, Debug)]
struct ResponseStops {
    dep_sid: String,
    arr_sid: String,
    direct_bus_route: bool
}

#[derive(Deserialize, Debug)]
struct RequestStops {
    dep_sid: String,
    arr_sid: String
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "/api/direct/:dep_sid/:arr_sid" => |request, response| {
            let requested_stops = RequestStops {
                dep_sid: match request.param("dep_sid") {
                    Some(id) => String::from(id),
                    None => panic!("No departure SID specified")
                },
                arr_sid: match request.param("arr_sid") {
                    Some(id) => String::from(id),
                    None => panic!("No arrival SID specified")
                }
            };

            format!("{:?}", requested_stops)
        }
    });

    server.listen("127.0.0.1:1337");
}
