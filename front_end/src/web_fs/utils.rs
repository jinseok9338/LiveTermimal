use std::time::{SystemTime, UNIX_EPOCH};

pub fn system_time_to_ms(system_time: SystemTime) -> u64 {
    let duration_since_epoch = system_time
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime should be after UNIX_EPOCH");

    let milliseconds =
        duration_since_epoch.as_secs() * 1000 + u64::from(duration_since_epoch.subsec_millis());

    milliseconds
}
