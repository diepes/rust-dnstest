use chrono;
use chrono_tz; //::Pacific::Auckland;

pub fn get_timestamp_now() -> String {
    let nz_now = chrono::Utc::now().with_timezone(&chrono_tz::Pacific::Auckland);
    format!(" TimeStamp:__{}__ ", nz_now)
}
