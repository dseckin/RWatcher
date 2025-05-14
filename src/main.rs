use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use std::env;
use chrono::Utc;
use std::io::Read;

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
    let mut called_process = Command::new(command_string.clone())
        .arg(arg_string.clone())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start program");


    // Stdout ve stderr için ayrı reader'lar al
    let mut stdout_reader = called_process.stdout.take().unwrap();
    let mut stderr_reader = called_process.stderr.take().unwrap();

    // Process durumunu kontrol etmek için döngü
    let mut output = Vec::new();
    let mut error = Vec::new();

    loop {
        // Process'in bitip bitmediğini kontrol et
        match called_process.try_wait() {
            Ok(Some(status)) => {
                // Process tamamlandı, çıktıları oku
                stdout_reader.read_to_end(&mut output).unwrap();
                stderr_reader.read_to_end(&mut error).unwrap();
                
                println!("Exit status: {}", status);
                break;
            }
            Ok(None) => {
                // Process hala çalışıyor, beklemeye devam et
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                // Hata durumu
                eprintln!("Error waiting: {}", e);
                break;
            }
        }
    }

    // Çıktıları yazdır
    println!("STDOUT: {}", String::from_utf8_lossy(&output));
    println!("STDERR: {}", String::from_utf8_lossy(&error));

    println!("{} - Program started for the first time.", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"));

    loop {
        sleep(Duration::from_secs(seconds_to_check.try_into().unwrap()));
        sleep_time_count += 1;
        
        match called_process.try_wait() {
            Ok(Some(_)) => {
                // Program stopped running
                // Restart the program
                called_process = Command::new(command_string.clone())
                    .arg(arg_string.clone())
                    .stdin(Stdio::null())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("failed to start program");
                
                // Stdout ve stderr için ayrı reader'lar al
                let mut stdout_reader = called_process.stdout.take().unwrap();
                let mut stderr_reader = called_process.stderr.take().unwrap();

                // Process durumunu kontrol etmek için döngü
                let mut output = Vec::new();
                let mut error = Vec::new();

                loop {
                    // Process'in bitip bitmediğini kontrol et
                    match called_process.try_wait() {
                        Ok(Some(status)) => {
                            // Process tamamlandı, çıktıları oku
                            stdout_reader.read_to_end(&mut output).unwrap();
                            stderr_reader.read_to_end(&mut error).unwrap();
                            
                            println!("Exit status: {}", status);
                            break;
                        }
                        Ok(None) => {
                            // Process hala çalışıyor, beklemeye devam et
                            std::thread::sleep(std::time::Duration::from_millis(100));
                        }
                        Err(e) => {
                            // Hata durumu
                            eprintln!("Error waiting: {}", e);
                            break;
                        }
                    }
                }

                // Çıktıları yazdır
                println!("STDOUT: {}", String::from_utf8_lossy(&output));
                println!("STDERR: {}", String::from_utf8_lossy(&error));

                
                sleep_time_count = 0;

                println!("{} - Program stopped running.", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"));
            }
            Ok(None) => {
                // Program still running                                
                if seconds_to_kill == -1 {
                    continue;
                }
                
                if ( seconds_to_check * sleep_time_count ) >= seconds_to_kill {
                    // Kill the program
                    let _ = called_process.kill();
                    // Prevent zombie situation
                    let _ = called_process.wait();

                    called_process = Command::new(command_string.clone())
                        .arg(arg_string.clone())
                        .stdin(Stdio::null())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("failed to start program");


                    // Stdout ve stderr için ayrı reader'lar al
                    let mut stdout_reader = called_process.stdout.take().unwrap();
                    let mut stderr_reader = called_process.stderr.take().unwrap();

                    // Process durumunu kontrol etmek için döngü
                    let mut output = Vec::new();
                    let mut error = Vec::new();

                    loop {
                        // Process'in bitip bitmediğini kontrol et
                        match called_process.try_wait() {
                            Ok(Some(status)) => {
                                // Process tamamlandı, çıktıları oku
                                stdout_reader.read_to_end(&mut output).unwrap();
                                stderr_reader.read_to_end(&mut error).unwrap();
                                
                                println!("Exit status: {}", status);
                                break;
                            }
                            Ok(None) => {
                                // Process hala çalışıyor, beklemeye devam et
                                std::thread::sleep(std::time::Duration::from_millis(100));
                            }
                            Err(e) => {
                                // Hata durumu
                                eprintln!("Error waiting: {}", e);
                                break;
                            }
                        }
                    }

                    // Çıktıları yazdır
                    println!("STDOUT: {}", String::from_utf8_lossy(&output));
                    println!("STDERR: {}", String::from_utf8_lossy(&error));


                    println!("{} - Program killed and started again.", Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"));

                    
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
