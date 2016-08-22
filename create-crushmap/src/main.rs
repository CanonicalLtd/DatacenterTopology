extern crate crushtool;
extern crate juju;
extern crate log;

use log::LogLevel;
use std::collections::{HashMap, HashSet};
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

// Here is where the controller takes input from the subordinate services,
// determines which nodes are in the same rack, and finally
// creates the crushmap from those clusters.
//


fn main() {

    let machines = grab_relation_data();
    let racks = generate_racks(machines);

    let crush_result = match generate_crushmap(racks) {
        Ok(_) => {
            juju::status_set(juju::Status {
                status_type: juju::StatusType::Maintenance,
                message: "Crushmap generated in /tmp. Please examine crushmap with Ceph before \
                          use."
                    .to_string(),
            });
        }
        Err(e) => {
            let message = format!("Failed to create crushmap with error: {}", e);
            juju::log(message.clone(), Some(LogLevel::Error));
            juju::status_set(juju::Status {
                status_type: juju::StatusType::Maintenance,
                message: message,
            });
        }
    };
    juju::log(format!("{:?}", crush_result), Some(LogLevel::Info));
    println!("{:?}", crush_result);

}

fn grab_relation_data() -> HashMap<String, Vec<String>> {
    let juju_relation_ids = match juju::relation_ids_by_identifier("controller") {
        Ok(ids) => ids,
        Err(_) => {
            juju::log("Failed at grabbing relation IDs.", Some(LogLevel::Error));
            panic!("Failed at grabbing relation IDs.");
        }
    };
    let relation_id = &juju_relation_ids[0];
    let controller_id = match env::var("JUJU_UNIT_NAME") {
        Ok(id) => id,
        Err(_) => {
            juju::log("Failed to grab controller id from JUJU.", Some(LogLevel::Error));
            panic!("Failed to grab controller id from JUJU.");
        }

    };

    let controller = parse_unit_into_relation(controller_id);

    let juju_related_units =
    match juju::relation_get_by_id("related-units", &relation_id, &controller) {
        Ok(units) => units,
        Err(_) => {
            juju::log("Failed to grab related units from juju relation.", Some(LogLevel::Error));
            panic!("Failed to grab related units from juju relation.");
        }

    };

    let mut juju_parsed_units: Vec<juju::Relation> = Vec::new();

    for unit in juju_related_units.split_whitespace() {
        juju_parsed_units.push(parse_unit_into_relation(unit.to_string()));
    }

    let mut machines: HashMap<String, Vec<String>> = HashMap::new();

    for unit in juju_parsed_units {
        let hostname = match juju::relation_get_by_id("hostname", &relation_id, &unit) {
            Ok(h) => h,
            Err(_) => {
                juju::log(format!("Failed to grab hostname from {:?}.", unit), Some(LogLevel::Error));
                panic!("Failed to grab hostname from {:?}.", unit);
            }

        };
        let neighbors_raw = match juju::relation_get_by_id("neighbors", &relation_id, &unit) {
            Ok(n) => n,
            Err(_) => {
                juju::log(format!("Failed to grab neighbors from {:?}.", unit), Some(LogLevel::Error));
                panic!("Failed to grab neighbors from {:?}.", unit);
            }

        };
        let hostname_trimmed = hostname.trim_matches('\n').trim();
        let neighbors_trimmed = neighbors_raw.trim_matches('\n').trim();

        let neighbors: Vec<String> = neighbors_trimmed.split_whitespace()
            .map(|item| item.to_owned())
            .collect();

        println!("Hostname:{}, Neighbors:{:?}",
                 hostname_trimmed,
                 neighbors_trimmed);
        machines.insert(hostname_trimmed.to_owned(), neighbors);
    }

    machines
}

fn generate_racks(machines: HashMap<String, Vec<String>>) -> HashSet<Vec<String>>{

    let mut racks: HashSet<Vec<String>> = HashSet::new();
    let mut racked_machines: HashSet<String> = HashSet::new();
    let mut potential_racks: Vec<Vec<String>> = Vec::new();

    for (machine, neighbors) in machines {
        let mut members: Vec<String> = Vec::new();
        members.push(machine.clone());
        members.extend(neighbors.clone());
        members.sort();
        members.dedup();
        potential_racks.push(members);
    }
    potential_racks.sort_by(|a, b| b.len().cmp(&a.len()));
    potential_racks.dedup();

    println!("Potential racks: {:?}", potential_racks);

    'rack: for rack in potential_racks.clone() {
        // If we have no racks, add it to the rack list.
        if racks.is_empty() {
            racks.insert(rack.clone());
            racked_machines.extend(rack.clone());
            continue 'rack;
        }
        // for each rack, check to see if any of its members is already in another rack
        for machine in rack.clone() {
            println!("Checking {}", machine);
            if racked_machines.contains(&machine) {
                println!("Machine found in list: {:?}", racked_machines);
                continue 'rack;
            }
        }
        racks.insert(rack.clone());
        racked_machines.extend(rack.clone());
    }

    println!("Racks: {:?}", racks);

    racks
}

fn generate_crushmap(racks: HashSet<Vec<String>>) -> Result<(), String> {
    // This generates a crushmap using the information gathered during network discovery.
    //
    // First it loads the current crushmap generated in the begin-discovery action.
    // Then it picks apart that map to get the name_map, buckets, and current index. From there we
    // take our list of racks, take each item in the rack and match it up to an item in the name
    // map for further use. We take those racks again and create a bucket for each one which holds
    // the machine and the associated OSD. Finally put those buckets back into a crushmap and
    // encode it with Crushtool, and write the bytes to a file that Ceph can use.


    // Open that map and read the bytes to a var, then decode those bytes to a crushmap object
    let mut path = env::temp_dir();
    path.push("currentmap");
    let mut some_crushmap_file = try!(File::open(path).map_err(|e| e.to_string()));
    let mut crushmap_bytes: Vec<u8> = Vec::new();
    some_crushmap_file.read_to_end(&mut crushmap_bytes);
    // The actual Ceph crushmap pulled from our active cluster
    let current_map: crushtool::CrushMap = try!(crushtool::decode_crushmap(&crushmap_bytes[..]));

    // Grab the current lowest index. For bucket objects like racks these are always negative
    let mut current_index: i32 = match current_map.name_map.iter().min() {
        Some(&(index, _)) => index,
        None => {
            return Err("Cannot proceed due to error: Could not find current index.
                    Either no bucket items are present or map decode failed to generate meaningful
                    buckets."
                .to_string());
        }
    };
    // Set the current index for the next item
    current_index -= 1;
    println!("Current index: {}", current_index);

    // Name map and current buckets are pulled out
    let mut name_map: HashMap<String, i32> = HashMap::new();
    for (index, name) in current_map.name_map {
        name_map.insert(name, index);
    }
    println!("name_map:{:?}", name_map);


    let mut machines: HashSet<String> = HashSet::new();
    for members in racks.clone() {
        machines.extend(members);
    }
    println!("machines: {:?}", machines);

    let mut machines_map: HashMap<String, i32> = HashMap::new();
    let mut machine_ids: Vec<i32> = Vec::new();
    for (id, index) in name_map.clone() {
        if machines.contains(&id) {
            machines_map.insert(id.clone(), index.clone());
            machine_ids.push(index);
        }
    }
    println!("machines_map: {:?}", machines_map);

    // Name map for the new crushmap, list of tuples of an index and a name.
    let mut final_name_map: Vec<(i32, String)> = Vec::new();

    for (name, index) in name_map.clone() {
        if machine_ids.contains(&index) || index >= 0 {
            final_name_map.push((index, name));
        }
    }

    // Matching the name map items to buckets and pulling those buckets into a list.
    // We are only concerned with hosts and OSDs here, because all other buckets will
    // be thrown out when we make the new map

    let mut carryover_buckets: HashSet<crushtool::BucketTypes> = HashSet::new();
    let mut new_rack_buckets: Vec<crushtool::BucketTypes> = Vec::new();
    println!("Old buckets: {:?}", current_map.buckets.clone());

    for bucket in current_map.buckets.clone() {
        let id: i32;
        match bucket {
            crushtool::BucketTypes::Uniform(ref uniform) => {
                id = uniform.bucket.id;
            }
            crushtool::BucketTypes::List(ref list) => {
                id = list.bucket.id;
            }
            crushtool::BucketTypes::Tree(ref tree) => {
                id = tree.bucket.id;
            }
            crushtool::BucketTypes::Straw(ref straw) => {
                id = straw.bucket.id;
            }
            crushtool::BucketTypes::Straw2(ref straw2) => {
                id = straw2.bucket.id;
            }
            crushtool::BucketTypes::Unknown => {
                id = 65536;
            }
        };
        if machine_ids.contains(&id) {
            carryover_buckets.insert(bucket.clone());
        }
    }
    println!("Carryover buckets: {:?}", carryover_buckets);

    // This holds the current buckets pulled from Ceph. Machines and OSDs already have an index
    // assigned
    // by Ceph, and instead of making a new bucket for each item, we simply make a new rack bucket
    // using the
    // current buckets' indexes. Since the machine is the "top" bucket for each OSD, we grab that
    // index, keeping the tree below untouched.

    let mut new_rack_items: Vec<(i32, Option<String>)> = Vec::new();
    let mut bucket_name: i32 = 0;
    // For each group of machines in our racks var we make a bucket
    for members in racks {
        let mut bucket_items: Vec<(i32, Option<String>)> = Vec::new();

        // For each machine in the machines map we grab the bucket items from out machines map.
        // These are matched by the machine's ID
        for machine in members.clone() {
            let index: i32 = match machines_map.get(&machine) {
                Some(index) => *index,
                None => return Err("Could not match bucket items to machine index".to_string()),
            };
            // Again, since we're only concerned with the index of the machine
            // (the root of our machine/osd tree)
            // we only push that index into our bucket items list, along with the corresponding
            // machine name
            bucket_items.push((index, Some(machine.to_string())));
        }

        // Make a new bucket, put the items matched above into it, then push it to our rack buckets
        let bucket = crushtool::BucketTypes::Straw(crushtool::CrushBucketStraw {
            bucket: crushtool::Bucket {
                id: current_index,
                bucket_type: crushtool::OpCode::ChooseIndep,
                alg: crushtool::BucketAlg::Straw,
                hash: crushtool::CrushHash::RJenkins1,
                weight: 0,
                size: members.len() as u32,
                items: bucket_items.clone(),
                perm_n: 0,
                perm: members.len() as u32,
            },
            item_weights: vec![(0, 0); bucket_items.len()],
        });
        final_name_map.push((current_index, bucket_name.to_string()));
        new_rack_items.push((current_index, Some(bucket_name.to_string())));
        new_rack_buckets.push(bucket);
        current_index -= 1;
        bucket_name += 1;
    }
    // Make a new default bucket
    let new_default_bucket = crushtool::BucketTypes::Straw(crushtool::CrushBucketStraw {
        bucket: crushtool::Bucket {
            id: -1,
            bucket_type: crushtool::OpCode::SetChooseLocalTries,
            alg: crushtool::BucketAlg::Straw,
            hash: crushtool::CrushHash::RJenkins1,
            weight: 0,
            size: new_rack_buckets.len() as u32,
            items: new_rack_items.clone(),
            perm_n: 0,
            perm: new_rack_buckets.len() as u32,
        },
        item_weights: vec![(0, 0); new_rack_items.len()],
    });
    final_name_map.push((-1, "default".to_string()));
    final_name_map.sort();
    println!("Final name map:{:?}", final_name_map);

    let mut final_buckets: Vec<crushtool::BucketTypes> = Vec::new();
    final_buckets.extend(carryover_buckets.clone());
    final_buckets.extend(new_rack_buckets);
    final_buckets.push(new_default_bucket);

    create_crushmap(final_buckets, machines_map.len() as i32, final_name_map);

    Ok(())
}

fn create_crushmap(final_buckets: Vec<crushtool::BucketTypes>,
                   devices: i32, final_name_map: Vec<(i32, String)>) -> Result<(), String> {

    let mut new_crushmap: crushtool::CrushMap = crushtool::CrushMap {
        magic: 65536,
        max_buckets: final_buckets.len() as i32,
        max_rules: 1,
        max_devices: devices,
        buckets: final_buckets,
        rules: vec![Some(crushtool::Rule {
                        mask: crushtool::CrushRuleMask {
                            ruleset: 0,
                            rule_type: crushtool::RuleType::Replicated,
                            min_size: 1,
                            max_size: 10,
                        },
                        steps: vec![crushtool::CrushRuleStep {
                                        op: crushtool::OpCode::Take,
                                        arg1: (-1, None),
                                        arg2: (0, None),
                                    },
                                    crushtool::CrushRuleStep {
                                        op: crushtool::OpCode::ChooseLeafFirstN,
                                        arg1: (0, None),
                                        arg2: (1, None),
                                    },
                                    crushtool::CrushRuleStep {
                                        op: crushtool::OpCode::Emit,
                                        arg1: (0, None),
                                        arg2: (0, None),
                                    }],
                    })],
        type_map: vec![(0, "osd".to_string()),
                       (1, "host".to_string()),
                       (2, "chassis".to_string()),
                       (3, "rack".to_string()),
                       (4, "row".to_string()),
                       (5, "pdu".to_string()),
                       (6, "pod".to_string()),
                       (7, "room".to_string()),
                       (8, "datacenter".to_string()),
                       (9, "region".to_string()),
                       (10, "root".to_string())],

        name_map: final_name_map,
        rule_name_map: vec![(0, "replicated_ruleset".to_string())],
        choose_local_tries: Some(0),
        choose_local_fallback_tries: Some(0),
        choose_total_tries: Some(50),
        chooseleaf_descend_once: Some(1),
        chooseleaf_vary_r: Some(0),
        straw_calc_version: Some(1),
        allowed_bucket_algorithms: Some(0),
        chooseleaf_stable: Some(0),
    };
    println!("New Crushmap: {:?}", new_crushmap);
    crushtool::set_tunables_jewel(&mut new_crushmap);
    let encoded_crushmap = try!(crushtool::encode_crushmap(new_crushmap)
        .map_err(|e| e.to_string()));
    let mut path = env::temp_dir();
    path.push("dct_crushmap");
    let mut finished_map = try!(File::create(path).map_err(|e| e.to_string()));

    try!(finished_map.write_all(&encoded_crushmap[..]).map_err(|e| e.to_string()));

    Ok(())
}

// Parses unit strings from Juju into relations that Crushtool can understand
fn parse_unit_into_relation(unit: String) -> juju::Relation {
    let v: Vec<&str> = unit.split('/').collect();
    let id: usize = match v[1].parse::<usize>() {
        Ok(i) => i,
        Err(_) => {juju::log(format!("Could not parse {} into relation.", unit), Some(LogLevel::Error));
            panic!("Could not parse {} into relation.", unit)},

    };

    let parsed_unit = juju::Relation {
        name: v[0].to_string(),
        id: id,
    };
    parsed_unit
}