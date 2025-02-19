
use std::{error::Error, io::stdin, thread::sleep, *};
use winreg::{RegKey, enums::*};

fn current_user() -> Result<(), Box<dyn Error>> {
    println!("Getting User Info...\n");
    //let hkcu = RegKey::predef(HKEY_CURRENT_USER).enum_keys().map(|x| x.unwrap()).collect();
    for k in RegKey::predef(HKEY_CURRENT_USER).enum_keys().map(|x| x.unwrap()) {
        println!("{}", k);
    }
    Ok(())
}

fn performance_data() -> Result<(), Box<dyn Error>> {
    println!("Getting performance data...\n");
    for k in RegKey::predef(HKEY_PERFORMANCE_DATA).enum_keys().map(|x| x.unwrap()) {
        println!("{}", k);
    }
    Ok(())
}

fn local_machine() -> Result<(), Box<dyn Error>> {
    println!("Reading System Info...\n");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("HARDWARE\\DESCRIPTION\\System");
    for (name, value) in hklm?.enum_values().map(|x| x.unwrap()) {
        println!("{} = {:?}", name, value);
    }
    Ok(())
}

fn classes_root() -> Result<(), Box<dyn Error>> {
    println!("Getting Info...\n");
    for k in RegKey::predef(HKEY_CLASSES_ROOT).enum_keys().map(|x| x.unwrap()) {
        println!("{}", k);
    }
    Ok(())
}

fn current_config() -> Result<(), Box<dyn Error>> {
    println!("Getting Hardware Profile Info...\n");
    for k in RegKey::predef(HKEY_CURRENT_CONFIG).enum_keys().map(|x| x.unwrap()) {
        println!("{}", k);
    } 
    Ok(())
}

fn main() {

    loop {
        println!("\nExplore Windows Registry.\n");
        println!("Enter number corresponding to the key:\n");
        println!("
                    1) HKEY_CURRENT_USER
                    2) HKEY_PERFORMANCE_DATA
                    3) HKEY_LOCAL_MACHINE
                    4) HKEY_CLASSES_ROOT
                    5) HKEY_CURRENT_CONFIG
                    6) EXIT
                ");
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap_err();
        
        match user_input.as_str() {

            "1" => {let _ = current_user();}

            "2" => {let _ = performance_data();}

            "3" => {let _ = local_machine();}

            "4" => {let _ = classes_root();}

            "5" => {let _ = current_config();}

            "6" => {
                println!("\nExiting program...");
                sleep(time::Duration::from_millis(20));
                process::exit(1);
            }

            _ => {println!("Error! invalid user input.");}
        }
    }

}
