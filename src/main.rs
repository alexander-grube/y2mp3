fn main() {
    let video_url = std::env::args().nth(1).expect("No video URL provided");
    
    let cmd = std::process::Command::new("yt-dlp_min.exe")
        .arg("-f")
        .arg("ba")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg(&video_url)
        .stdout(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");
    cmd.wait_with_output()
        .expect("Failed to read command output");
}
