#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use wmi::{COMLibrary, WMIConnection, Variant, FilterValue};
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Hardware {
    ProcessId: String,
    InstanceId: String,
    Identifier: String,
    HardwareType: String,
    Name: String,
    Parent: String,
}

#[derive(Deserialize, Debug)]
struct Sensor {
    SensorType: String,
    Identifier: String,
    InstanceId: String,
    Value: f32,
    ProcessId: String,
    Parent: String,
    Max: f32,
    Min: f32,
    Index: i32,
}

#[derive(Debug)]
struct MonitorGroup {
    hardware: Hardware,
    sensors: Vec<Sensor>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::with_namespace_path("root\\OpenHardwareMonitor", com_con.into())?;

    let mut monitor_groups = vec![];

    let hw_results: Vec<Hardware> = wmi_con.raw_query("SELECT * FROM Hardware")?;
    for hardware in hw_results {

        monitor_groups.push(MonitorGroup {
            hardware: hardware,
            sensors: Vec::new()
        });
    }

    let sensor_results: Vec<Sensor> = wmi_con.raw_query("SELECT * FROM Sensor")?;
    for sensor in sensor_results {
        for hw in &mut monitor_groups {
            if sensor.Identifier.starts_with(&hw.hardware.Identifier) {
                hw.sensors.push(sensor);
                break;
            }
        }
    }

    println!("{:#?}", monitor_groups);

    


    Ok(())

}
