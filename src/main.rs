use wmi::{COMLibrary, WMIConnection, Variant};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::with_namespace_path("root\\OpenHardwareMonitor", com_con.into())?;

    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT * FROM Sensor")?;

    for os in results {
        println!("{:#?}", os);
    }


    Ok(())

}
