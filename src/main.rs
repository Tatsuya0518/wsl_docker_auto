use std::path;
use std::process::Command;


fn main() {
    let target_path = "./";
    let target = path::PathBuf::from(target_path);
    let files = target.read_dir().expect("このパスは存在しません");
    for dir_entry in files {
        // dir_entry(Result<DirEntry, Error>型)をfile_path(PathBuf型)に変換する
        let file_path = dir_entry.unwrap().path();
        println!("{:?}", file_path);
    }
    // Command::new("tokei")
    // .args(&["-f"])
    // .spawn()
    // .expect("failed to start `tokei`");
    
    Command::new("wsl")
    .args(&["--exec"])
    .args(&["docker"])
    .args(&["start"])
    .args(&["{Your_Dokcer_ContainerID or name}"]) //WSLにあるDockerの起動したいコンテナ名 {}は外して記述する。
    .spawn()
    .expect("failed to start `wsl`");
    
    // Command::new("docker")
    //  .args(&["start strange_davinci"])
    //  .spawn()
    //  .expect("failed to start `docker start`");

}
