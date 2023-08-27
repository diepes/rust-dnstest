use chrono;
use chrono_tz; //::Pacific::Auckland;

pub fn get_timestamp_now(pad: &str) -> String {
    let nz_now = chrono::Utc::now().with_timezone(&chrono_tz::Pacific::Auckland);
    format!(
        "{pad}Time:{t}{pad}",
        t = nz_now.format("%Y%m%dH%H:%M:%S%.3f")
    )
}
