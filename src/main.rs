use std::env;
use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _full_cont =fs::read_to_string("./.btrbk")
        .expect("Can't read from ./btrbk");
    let contents : Vec<&str> = _full_cont.split("\n").collect();

    let mut from : String = "".to_string();
    let mut to   : String = "".to_string();
    let mut btrfs : bool = false;

    for c in contents {
        let pair : Vec<&str>= c.split(":").collect();
        if pair.len() > 1 && pair[0].contains("backup_from") {
            from = pair[1].replace(" ", "");
        }
        if pair.len() > 1 && pair[0].contains("backup_to") {
            to = pair[1].replace(" ", "");
        }
        if pair.len() > 1 && pair[0].contains("btrfs") {
            let btrstr = pair[1].replace(" ", "");
            if btrstr.contains("yes"){
                btrfs = true;
            }
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

    let pretime = chrono::Utc::now().to_rfc2822().replace(" ", "_").replace(",", "").split("+").collect::<Vec<&str>>()[0].to_string();
    let mut index = 0;

    let from_path = Path::new(&from);
    let mut to_path;
    let mut _to_path_string;

    loop {
        _to_path_string = format!("{}/{}_{}", to, pretime, index);
        if btrfs {
            use std::ops::Add;
            _to_path_string = _to_path_string.add("_btrfs");
        }
        to_path = Path::new(&_to_path_string);
        if !to_path.exists() {
            break;
        }
        index+=1;
    }

    let _verbose_string = "verbose".to_string();

    if btrfs {
        btrfs_cp(&from_path, to_path)
    }else {
        cp(&from_path, &to_path, args.contains(&_verbose_string));
    }
}

fn btrfs_cp(from : &Path, to : &Path){
    let cmd = format!("btrfs subv snapshot {} {}", from.to_str().unwrap(), to.to_str().unwrap());
    let error = exec_cmd(&cmd);
    if error.is_some() {
        eprint!("{}", error.unwrap());
        eprintln!("For btrfs backups to work, both backup_from and backup_to have to reside on a btrfs filesystem and backup_from needs to be a btrfs subvolume.");
        eprintln!("You create a btrfs subvolume by using the command: btrfs subvolume create {{subvolume name}}");
    }
}

fn cp(from : &Path, to: &Path, verbose: bool){
    let cmd = format!("cp -rp{} {} {}", if verbose {"v"} else {""} , from.to_str().unwrap(), to.to_str().unwrap());
    let err = exec_cmd(&cmd);
    if err.is_some() {
        eprint!("ERROR: {}", err.unwrap());
    }
}

fn exec_cmd(cmd : &str) -> Option<String> {

    let output =Command::new("sh")
        .stdout(std::process::Stdio::inherit())
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    let error = String::from_utf8(output.stderr).expect("Can't convert stderr to utf8 string.");
    if error.len() > 0 {
        return Some(error);
    } else {
        return None;
    }
}
