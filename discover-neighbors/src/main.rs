extern crate pnet;
extern crate juju;
extern crate log;

use std::str::FromStr;
use std::net::Ipv4Addr;
use std::str;
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use std::thread::sleep;

mod networking;


fn main() {
    let unit_id = env::var("JUJU_UNIT_NAME").unwrap_or("".to_string());
    let unit = parse_unit_into_relation(unit_id);

    let ready_status: String = juju::relation_get("ready").unwrap().to_string();
    let ready_status = ready_status.trim_matches('\n').trim();
    let finished_status: String =
        juju::relation_get_by_unit("finished", &unit).unwrap().to_string();
    let finished_status = finished_status.trim_matches('\n').trim();
    let results: String;




    println!("Ready status: {}", ready_status);
    if (ready_status == "1") && (finished_status != "1") {
        println!("Starting network discovery");
        results = network_discovery(unit.name.clone()).trim_matches('\n').trim().to_string();
        println!("Results: {}", results);

        juju::relation_set("neighbors", &results);

        let mut finished: bool = false;
        let mut count = 0;
        while !finished && count < 10 {
            let test_set = juju::relation_get_by_unit("neighbors", &unit).unwrap();
            let test_set = test_set.trim_matches('\n').trim();
            if test_set == results {
                juju::status_set(juju::Status {
                    status_type: juju::StatusType::Waiting,
                    message: "Finished network discovery".to_string(),
                });
                juju::relation_set("finished", "1");
                finished = true;

            } else {
                juju::relation_set("neighbors", &results);
                count += 1;
                sleep(Duration::new(5, 0));
            }
        }
    }



}

fn network_discovery(unit: String) -> String {

    let juju_unit_list: String = juju::relation_get("related-units").unwrap();
    println!("Unit list: {}", juju_unit_list);

    let mut juju_machine_ids_with_ip: HashMap<String, Ipv4Addr> = HashMap::new();

    for unit in juju_unit_list.split_whitespace() {

        println!("Unit to decompose: {}", unit);

        let relation = parse_unit_into_relation(unit.to_string());
        let ip = juju::relation_get_by_unit("private-address", &relation).unwrap();
        let hostname = juju::relation_get_by_unit("hostname", &relation).unwrap();
        let ip = ip.trim();
        juju_machine_ids_with_ip.insert(hostname, Ipv4Addr::from_str(&ip).unwrap());
    }
    juju_machine_ids_with_ip.remove(&unit);
    println!("Known IPs: {:?}", juju_machine_ids_with_ip);

    // Get list of neighbor IPs using arping
    let neighbor_list = networking::send_and_receive(juju_machine_ids_with_ip);
    let mut neighbors_formatted: String = "".to_string();

    for (machine, ip) in neighbor_list {
        let trimmed_machine = &machine.trim_matches('\n').trim();
        neighbors_formatted = format!("{} {}",
                                      &neighbors_formatted.trim_matches('\n')
                                          .trim()
                                          .to_string(),
                                      &trimmed_machine);
    }


    let unit_id = env::var("JUJU_UNIT_NAME").unwrap_or("".to_string());
    let unit = parse_unit_into_relation(unit_id);

    neighbors_formatted

}

fn parse_unit_into_relation(unit: String) -> juju::Relation {
    let v: Vec<&str> = unit.split('/').collect();
    let id: usize = v[1].parse::<usize>().unwrap();
    let parsed_unit = juju::Relation {
        name: v[0].to_string(),
        id: id,
    };
    parsed_unit
}
