mod Manual_Mode;


//virus
fn main() {
    let mut sel = 0;
    let max = 3;
    menu(0);
    /*
    for key in Keyboard::new() {
        match key {
            Keys::Down | Keys::Char('s')   => {if sel != max { sel+=1 };menu(sel)},
            Keys::Up | Keys::Char('w')=> {if sel != 0 { sel -= 1 };menu(sel)},
            Keys::Enter => select(sel),
            _ => {}
        }
    }
    */
     select(0)
    
}

fn menu(operation: u8) {
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd").args(&["/C", "cls"]).status().unwrap();

    #[cfg(target_os = "linux")]
    std::process::Command::new("clear").status().unwrap();
    let mut op_msgs: Vec<String> = [
        "Manual Mode",
        "Hold Mode",
        "Auto Mode",
        "Exit",
    ].iter().map(|s| s.to_string()).collect();

    for (i,msg) in op_msgs.clone().iter().enumerate() {
        if i == operation as usize {
            op_msgs[i] = format!("[*] {}", msg);
        } else {
            op_msgs[i] = format!("[ ] {}", msg);
        }
    }
    for msg in op_msgs {
        println!("{}", msg);
    }

}


fn select(selectrion:u8) {
    match selectrion {
        0 => Manual_Mode::manual_mode(),
        3 => std::process::exit(0),
        _ => {}
    }
}