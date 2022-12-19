use chrono::Duration;
use cronjob::CronJob;
use dotenv::dotenv;
use reqwest::{
    blocking::{self, Response},
    Error,
};
use std::{collections::HashMap, env, fs::File, io::Write};

thread_local! {
    // create a vector to store the different times
    // Don't print to file if time varies by less that 0.5 seconds
    static TIMES: std::cell::RefCell<Vec<f64>> = std::cell::RefCell::new(Vec::new());
}

fn main() {
    dotenv().ok();
    let mut cron = CronJob::new("Test Cron", run);
    cron.minutes("*/20");
    cron.start_job();
}

fn run(_: &str) {
    let res = make_request();
    if let Ok((res, diff)) = res {
        print_response_to_file(res, diff);
    }
}

fn make_request() -> Result<(Response, Duration), Error> {
    // Get the url and secret key from the environment
    let url = env::var("URL").expect("URL must be set");
    let secret_key = env::var("SECRET").expect("Secret key must be set");

    let start_time = chrono::Local::now();
    let client = blocking::Client::new();
    let res = client.get(url).header("secret-key", secret_key).send()?;
    let end_time = chrono::Local::now();
    let diff = end_time - start_time;
    Ok((res, diff))
}

fn print_response_to_file(res: Response, diff: Duration) {
    if !significant_vary(diff) {
        return;
    }

    let status = res.status();
    let body = res
        .json::<HashMap<String, String>>()
        .expect("Unable to parse json");

    let time = body.get("time").unwrap();
    let date_time = chrono::Local::now();
    let date_time = date_time.format("%Y-%m-%d %H:%M");
    let res_time = diff.num_milliseconds();

    let string = format!(
        "[{}]: Status: {}, Time: {}, Response Time: {}ms\n______________________________\n",
        date_time, status, time, res_time
    );

    let path = env::var("FILE_LOCATION").expect("File location must be set");
    let mut file = File::options()
        .append(true)
        .create(true)
        .open(path)
        .expect("Unable to create file");

    file.write_all(string.as_bytes())
        .expect("Unable to write data");
}

fn significant_vary(diff: Duration) -> bool {
    let diff = diff.num_milliseconds() as f64;
    let len = TIMES.with(|t| t.borrow().len());
    if len <= 3 {
        TIMES.with(|t| t.borrow_mut().push(diff));
        return true;
    }
    let avg = average_time();
    if (diff - avg).abs() > 500.0 {
        return true;
    }
    remove_large_times();
    TIMES.with(|t| {
        let mut times = t.borrow_mut();
        times.push(diff);
    });
    false
}

fn average_time() -> f64 {
    let avg = TIMES.with(|t| {
        let times = t.borrow();
        let sum: f64 = times.iter().sum();
        sum / times.len() as f64
    });
    avg
}

fn remove_large_times() {
    TIMES.with(|t| {
        let mut times = t.borrow_mut();
        let max = times.iter().max_by(|a, b| a.partial_cmp(b).unwrap());
        if let Some(max) = max {
            let index = times.iter().position(|x| x == max).unwrap();
            times.remove(index);
        }
    });
}
