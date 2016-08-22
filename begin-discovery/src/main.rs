extern crate juju;

use std::process::Command;


fn main() {

    let juju_relation_ids = juju::relation_ids_by_identifier("controller").unwrap();
    let relation_id = &juju_relation_ids[0];

    juju::relation_set_by_id("ready", &"1", &relation_id);

    let message: juju::Status = juju::Status {
        status_type: juju::StatusType::Waiting,
        message: "Network discovery initiated".to_owned(),
    };
    juju::status_set(message);

    Command::new("ceph")
        .current_dir("/tmp")
        .args(&["osd", "getcrushmap", "-o", "/tmp/currentmap"])
        .spawn()
        .expect("failed to grab current cruhsmap");

    println!("Grabbed current crushmap.");

}
