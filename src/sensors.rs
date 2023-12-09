use std::collections::BTreeMap;

use lm_sensors::prelude::*;

#[derive(Debug, PartialEq)]
pub struct RangedValue {
    pub current: f64,
    pub maximum: f64,
    pub critical: f64,
    pub minimum: f64,
}

pub fn get_temperatures() -> Result<BTreeMap<String, RangedValue>, Box<dyn std::error::Error>> {
    // Initialize LM sensors library.
    let sensors = lm_sensors::Initializer::default().initialize()?;
    let mut temp_map = BTreeMap::<String, RangedValue>::new();
    // Print all chips.
    for chip in sensors.chip_iter(None) {
        // Print all features of the current chip.
        for feature in chip.feature_iter() {
            // let name = feature.name().transpose()?.unwrap_or("N/A");
            let feature_name = feature.to_string();
            let mut temperature = RangedValue {
                current: -10.0,
                maximum: 0.0,
                critical: 0.0,
                minimum: 0.0,
            };
            // Print all sub-features of the current chip feature.
            for sub_feature in feature.sub_feature_iter() {
                if let Ok(value) = sub_feature.value() {
                    match value {
                        lm_sensors::Value::TemperatureMaximum(value) => {
                            temperature.maximum = value;
                        }
                        lm_sensors::Value::TemperatureCritical(value) => {
                            temperature.critical = value;
                        }
                        lm_sensors::Value::TemperatureMinimum(value) => {
                            temperature.minimum = value;
                        }
                        lm_sensors::Value::TemperatureInput(value) => {
                            temperature.current = value;
                        }
                        _ => {}
                    }
                }
            }
            if temperature
                == (RangedValue {
                    current: -10.0,
                    maximum: 0.0,
                    critical: 0.0,
                    minimum: 0.0,
                })
            {
                continue;
            }
            temp_map.insert(feature_name, temperature);
        }
    }
    Ok(temp_map)
}

pub fn get_fan_speeds() -> Result<BTreeMap<String, RangedValue>, Box<dyn std::error::Error>> {
    // Initialize LM sensors library.
    let sensors = lm_sensors::Initializer::default().initialize()?;
    let mut fan_map = BTreeMap::<String, RangedValue>::new();
    // Print all chips.
    for chip in sensors.chip_iter(None) {
        // Print all features of the current chip.
        for feature in chip.feature_iter() {
            // let name = feature.name().transpose()?.unwrap_or("N/A");
            let feature_name = feature.to_string();
            let mut fan_speed = RangedValue {
                current: -10.0,
                maximum: 0.0,
                critical: 0.0,
                minimum: 0.0,
            };
            // Print all sub-features of the current chip feature.
            for sub_feature in feature.sub_feature_iter() {
                if let Ok(value) = sub_feature.value() {
                    match value {
                        lm_sensors::Value::FanMaximum(value) => {
                            fan_speed.maximum = value;
                        }
                        lm_sensors::Value::FanMinimum(value) => {
                            fan_speed.minimum = value;
                        }
                        lm_sensors::Value::FanInput(value) => {
                            fan_speed.current = value;
                        }
                        _ => {}
                    }
                }
            }
            if fan_speed
                == (RangedValue {
                    current: -10.0,
                    maximum: 0.0,
                    critical: 0.0,
                    minimum: 0.0,
                })
            {
                continue;
            }
            fan_map.insert(feature_name, fan_speed);
        }
    }
    Ok(fan_map)
}
/* pub fn print_sensors() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize LM sensors library.
    let sensors = lm_sensors::Initializer::default().initialize()?;

    // Print all chips.
    for chip in sensors.chip_iter(None) {
        if let Some(path) = chip.path() {
            println!("chip: {} at {} ({})", chip, chip.bus(), path.display());
        } else {
            println!("chip: {} at {}", chip, chip.bus());
        }

        // Print all features of the current chip.
        for feature in chip.feature_iter() {
            let name = feature.name().transpose()?.unwrap_or("N/A");
            println!("    {}: {}", name, feature);

            // Print all sub-features of the current chip feature.
            for sub_feature in feature.sub_feature_iter() {
                if let Ok(value) = sub_feature.value() {
                    println!("        {}: {}", sub_feature, value);
                } else {
                    println!("        {}: N/A", sub_feature);
                }
            }
        }
    }
    Ok(())
} */
