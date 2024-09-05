use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command_string = match args.iter().find(|&x| x.starts_with("--cmd=")) {
        Some(arg) => arg.trim_start_matches("--cmd=").to_string(),
        None => {            
            eprintln!("Error: --cmd parameter is missing.");
            std::process::exit(-1);
        },
    };
    let arg_string = match args.iter().find(|&x| x.starts_with("--arg=")) {
        Some(arg) => arg.trim_start_matches("--arg=").to_string(),
        None => "".to_string(),
    };

    let seconds_to_check = match args.iter().find(|&x| x.starts_with("--sec=")) {
        Some(arg) => arg.trim_start_matches("--sec=").to_string().parse::<i32>().unwrap(),
        None => 5,
    };
    let seconds_to_kill = match args.iter().find(|&x| x.starts_with("--kill_sec=")) {
        Some(arg) => arg.trim_start_matches("--kill_sec=").to_string().parse::<i32>().unwrap(),
        None => -1,
    };
    
    let mut sleep_time_count = 0;
    let mut collector_process = Command::new(command_string.clone())
        .arg(arg_string.clone())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start program");

    loop {
        sleep(Duration::from_secs(seconds_to_check.try_into().unwrap()));
        sleep_time_count += 1;
        
        match collector_process.try_wait() {
            Ok(Some(_)) => {
                // Program stopped running
                // Restart the program
                collector_process = Command::new(command_string.clone())
                    .arg(arg_string.clone())
                    .stdin(Stdio::null())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("failed to start program");
                
                sleep_time_count = 0;
            }
            Ok(None) => {
                // Program still running                                
                if seconds_to_kill == -1 {
                    continue;
                }

                if ( seconds_to_check * sleep_time_count ) >= seconds_to_kill {
                    // Kill the program
                    let _ = collector_process.kill();
                    // Prevent zombie situation
                    let _ = collector_process.wait();

                    collector_process = Command::new(command_string.clone())
                        .arg(arg_string.clone())
                        .stdin(Stdio::null())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("failed to start program");

                    
                    sleep_time_count = 0;
                }
            }
            Err(e) => {
                // Error occurred                
                eprintln!("{}", format!("Error checking program status: {}", e));
            }
        }//match
    }

}
