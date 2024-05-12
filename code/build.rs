use std::error::Error;
use dbus_codegen::GenOpts;

const MUTTER_DBUS_INTERFACE_XML: &str = "dbus-interfaces/display-config.xml";
const MUTTER_DBUS_GEN_OUTPUT: &str = "src/mutter_dbus.rs";

fn main() {
    if let Err(e) = run() {
        panic!("{}", e);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=src/mutter_dbus.rs");
    let dbus_xml = std::fs::read_to_string(MUTTER_DBUS_INTERFACE_XML)?;

    let gen_opts = GenOpts {
      methodtype: None,
      ..GenOpts::default()
    };

    let gen_result =  dbus_codegen::generate(dbus_xml.as_str(), &gen_opts)?;
    std::fs::write(MUTTER_DBUS_GEN_OUTPUT, gen_result)?;

    Ok(())
}
