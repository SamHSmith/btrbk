use std::env;
use std::process::Command;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);


    let _full_cont =fs::read_to_string("./.btrbk")
        .expect("Something went wrong reading the file");
    let contents : Vec<&str> = _full_cont.split("\n").collect();

    let mut from : String = "".to_string();
    let mut to   : String = "".to_string();

    for c in contents {
        let pair : Vec<&str>= c.split(":").collect();
        if pair.len() > 1 && pair[0].contains("backup_from") {
            from = pair[1].replace(" ", "");
        }
        if pair.len() > 1 && pair[0].contains("backup_to") {
            to = pair[1].replace(" ", "");
        }
    }

    if from.len() == 0 {
        println!("backup_from must be set in .btrbk");
        return;
    }
    if to.len() == 0 {
        println!("backup_to must be set in .btrbk");
        return;
    }

    let cmd = format!("cp -rp {} {}/{}", from, to, chrono::Utc::now().to_rfc2822().replace(" ", "_").replace(",", ""));

    exec_cmd(&cmd);
}

fn exec_cmd(cmd : &str){

    let output =Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    print!("{}", String::from_utf8(output.stdout).expect("Can't convert stdout to utf8 string."));
}
