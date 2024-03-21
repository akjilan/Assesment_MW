use serde_derive::{Deserialize, Serialize};
use serde_json;
use rand::{thread_rng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::{Write, Error as IOError};
use std::thread;
use std::time::Duration;

#[derive(Deserialize, Serialize, Debug)]
struct Monitor {
    monitor_id: Option<u64>,
    name: String,
    #[serde(rename = "type")]
    mytype: Option<String>,
    script: Option<String>,
    result: Option<result_for_output>,
    code: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Monitors {
    monitors: Vec<Monitor>
}

#[derive(Deserialize, Serialize, Debug)]
struct result_for_output {
    value: u64,
    processed_at: u64,
}

fn update_monitors(monitor_for_update: &mut Monitors) {
    let mut rng = thread_rng();
    let now = SystemTime::now();
    let seconds_since_epoch = now.duration_since(UNIX_EPOCH).expect("Failed to obtain current time").as_secs();

    for m in &mut monitor_for_update.monitors {
        let value = rng.gen_range(0..100);
        m.result = Some(result_for_output {
            value,
            processed_at: seconds_since_epoch,
        });
        println!("Monitor: {:?}", m);
    }
}

fn store_monitors(monitor_for_update: &Monitors) -> Result<(), IOError> {
    let now = SystemTime::now();
    let current_time = now.duration_since(UNIX_EPOCH).expect("Failed to obtain current time").as_secs();
    let filename = format!("D:/jilan/assesment/Assesment_MW/assets/{}_monitors.json", current_time);
    let json_output = serde_json::to_string_pretty(monitor_for_update)?;
    let mut file = File::create(filename)?;
    file.write_all(json_output.as_bytes())?;
    Ok(())
}

fn process_monitors(input_path: &str) -> Result<(), IOError> {
    let mut monitor_for_update = {
        let monitor = std::fs::read_to_string(input_path)?;
        serde_json::from_str::<Monitors>(&monitor)?
    };

    let five_mins = Duration::from_secs(300);
    let start_time = SystemTime::now();

    while start_time.elapsed().unwrap() < five_mins {
        update_monitors(&mut monitor_for_update);
        if start_time.elapsed().unwrap().as_secs() % 60 == 0 {
            store_monitors(&monitor_for_update)?; //sotring monitors after 1 min 
        }
        //  30 seconds break for each update
        thread::sleep(Duration::from_secs(30));
    }

    Ok(())
}

fn main() -> Result<(), IOError> {
    let input_path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Provide the input path as a command line argument.");
            return Ok(());
        }
    };
    
    let mut monitor_for_update = {
        let monitor = std::fs::read_to_string(&input_path)?;
        serde_json::from_str::<Monitors>(&monitor)?     // Load the Monitors structure
    };
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Failed to obtain current time");
    let seconds_since_epoch = duration_since_epoch.as_secs();
    let mut rng = thread_rng();
    let mut my_instance = result_for_output {
        value: 0,
        processed_at: 0,
    };
    let mut file = File::create("D:/jilan/assesment/Assesment_MW/assets/output.json").expect("Sorry , Failed to create file");
    let mut vector: Vec<serde_json::Value> = Vec::new();

    for m in &mut monitor_for_update.monitors {
        my_instance.value = rng.gen_range(0..100);
        my_instance.processed_at = seconds_since_epoch;

        let result_data = result_for_output {
            value: my_instance.value,
            processed_at: my_instance.processed_at,
        };
        m.result = Some(result_data);
        println!("Monitor: {:?}", m);

        vector.push(serde_json::to_value(m).unwrap());
    }
    
    let json_output = serde_json::to_string_pretty(&vector).unwrap();
    file.write_all(json_output.as_bytes()).expect("Sorry , Failed to write data to file");

    let process_monitor_thread = thread::spawn(move || {
        process_monitors(&input_path).unwrap_or_else(|err| eprintln!("Error processing monitors: {}", err));
    });

    process_monitor_thread.join().unwrap_or_else(|_| eprintln!("Monitoring process thread panicked"));

    Ok(())
}
