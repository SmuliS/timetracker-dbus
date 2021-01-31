use std::error::Error;
use std::process::exit;

use dbus::blocking::Connection;
use dbus_crossroads::Crossroads;

use crate::data::Timetracker;
use crate::handlers::build_timetracker_interface;

mod data;
mod handlers;

const BUS_NAME: &str = "fi.smuli.timetracker";
const INTERFACE_NAME: &str = "fi.smuli.timetracker.status";
const OBJECT_PATH: &str = "/fi/smuli/timetracker/status";

fn main() {
    if let Err(e) = run() {
        eprintln!("{:?}", e);
        exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let c = Connection::new_session()?;
    c.request_name(BUS_NAME, false, true, false)?;

    let mut cr = Crossroads::new();
    cr.set_add_standard_ifaces(true);
    let data = Timetracker::new();
    let iface = cr.register(INTERFACE_NAME, build_timetracker_interface);

    cr.insert(OBJECT_PATH, &[iface], data);

    cr.serve(&c)?;
    Ok(())
}
