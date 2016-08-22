extern crate juju;


fn main() {

    let configured_num_units = juju::config_get("num-units").unwrap();
    let configured_num_units = configured_num_units.parse::<usize>().unwrap();
    let units = juju::relation_list().unwrap();

    let mut unit_list: String = "".to_string();

    for unit in &units {
        let list_add = format!("{}{}{} ", &unit.name, "/", &unit.id);
        unit_list.push_str(&list_add);
    }

    if units.len() == configured_num_units {

        juju::relation_set("related-units", &unit_list);
        let message: juju::Status = juju::Status {
            status_type: juju::StatusType::Waiting,
            message: "Ready to begin network discovery".to_owned(),
        };
        juju::status_set(message);

    }

}
