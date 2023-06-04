use std::os::windows::raw::HANDLE;
use std::process::Command;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::processthreadsapi::TerminateProcess;
use winapi::um::winnt::PROCESS_TERMINATE;
#[allow(dead_code)]
pub fn terminate_process(process_name: &str) {
    let output = Command::new("tasklist").output().unwrap();
    let tasklist = String::from_utf8(output.stdout).unwrap();
    println!("{}", tasklist);

    let pid = tasklist
        .lines()
        .find(|line| line.to_lowercase().contains(process_name))
        .and_then(|line| line.split_ascii_whitespace().nth(1))
        .and_then(|pid_str| pid_str.parse::<u32>().ok())
        .unwrap();

    unsafe {
        let handle: HANDLE = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if TerminateProcess(handle, 0) == 0 {
            println!("Failed to kill process.");
        }
        CloseHandle(handle);
    }
}
