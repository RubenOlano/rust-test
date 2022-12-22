pub mod times;

use cronjob::CronJob;
use dotenv::from_filename;
use reqwest::{
    blocking::{self, Response},
    Error,
};
use std::{collections::HashMap, env, fs::File, io::Write};
use times::Times;

type Res = (Response, i64);
type Body = HashMap<String, String>;

#[derive(Debug)]
enum PingError {
    RequestError(Error),
    VarError(env::VarError, String),
    FileError(std::io::Error),
    ParseError(Error),
    BodyError(String),
}

thread_local! {
   static TIMES: Times = Times::new();
}

fn main() {
    let res = from_filename(".env");
    match res {
        Ok(a) => println!("Loaded .env file {}", a.to_string_lossy()),
        Err(e) => panic!("No .env file found {}", e.to_string()),
    }
    let mut cron = CronJob::new("Test Cron", run);
    cron.seconds("*/20");
    cron.start_job();
}

fn run(_: &str) {
    let req_status = time_request();

    let (res, diff) = match req_status {
        Ok(a) => a,
        Err(e) => {
            println!("Error: {:#?}", e);
            return;
        }
    };

    if !significant_vary(diff) {
        return;
    }
    match print_response_to_file(res, diff) {
        Ok(_) => (),
        Err(e) => println!("Error: {:#?}", e),
    }
}

fn time_request() -> Result<Res, PingError> {
    let start_time = chrono::Local::now();
    let res = send_request()?;

    let diff = chrono::Local::now() - start_time;
    let diff = diff.num_milliseconds();
    Ok((res, diff))
}

fn send_request() -> Result<Response, PingError> {
    let url = env::var("URL");

    let url = match url {
        Ok(a) => a,
        Err(e) => return Err(PingError::VarError(e, "URL not found".to_string())),
    };

    let secret_key = env::var("SECRET");

    let secret_key = match secret_key {
        Ok(a) => a,
        Err(e) => return Err(PingError::VarError(e, "SECRET not found".to_string())),
    };

    let client = blocking::Client::new();
    let res = client.get(url).header("secret-key", secret_key).send();

    match res {
        Ok(a) => return Ok(a),
        Err(e) => return Err(PingError::RequestError(e)),
    };
}

fn print_response_to_file(res: Response, diff: i64) -> Result<(), PingError> {
    let string = get_string_format(res, diff)?;
    let path = env::var("FILE_LOCATION");

    let path = match path {
        Ok(a) => a,
        Err(e) => {
            return Err(PingError::VarError(
                e,
                "FILE_LOCATION not found".to_string(),
            ))
        }
    };

    let file = File::options().create(true).append(true).open(path);
    let mut file = match file {
        Ok(a) => a,
        Err(e) => return Err(PingError::FileError(e)),
    };

    let file_err = file.write_all(string.as_bytes());
    match file_err {
        Ok(_) => (),
        Err(e) => return Err(PingError::FileError(e)),
    }
    Ok(())
}

fn get_string_format(res: Response, diff: i64) -> Result<String, PingError> {
    let status = res.status();
    let body = res.json::<Body>();

    let body = match body {
        Ok(a) => a,
        Err(e) => return Err(PingError::ParseError(e)),
    };

    let time = body.get("time");

    let time = match time {
        Some(a) => a,
        None => return Err(PingError::BodyError("time not found".to_string())),
    };

    let date_time = chrono::Local::now().format("%Y-%m-%d %H:%M");

    let formatted = format!(
        "[{}]: Status: {}, Time: {}, Response Time: {}ms\n______________________________\n",
        date_time, status, time, diff
    );
    Ok(formatted)
}

fn significant_vary(diff: i64) -> bool {
    let len = TIMES.with(|t| t.len());
    if len <= 3 {
        TIMES.with(|t| t.add_time(diff));
        return true;
    }
    let avg = TIMES.with(|t| t.avg_time());
    if (diff as f64 - avg).abs() > 500.0 {
        return true;
    }
    TIMES.with(|t| t.remove_larger_times());
    TIMES.with(|t| t.add_time(diff));
    false
}
