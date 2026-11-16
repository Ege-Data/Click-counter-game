use std::env;
use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::Duration;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::handleapi::CloseHandle;
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};
use winapi::um::winnt::DLL_PROCESS_ATTACH;

static LAST_CLICK_COUNT: AtomicU32 = AtomicU32::new(0);
static HOOK_INITIALIZED: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "system" fn DllMain(
    h_inst_dll: HINSTANCE,
    fdw_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> BOOL {
    if fdw_reason == DLL_PROCESS_ATTACH {
        unsafe {
            DisableThreadLibraryCalls(h_inst_dll);
        }

        if let Some(mut downloads) = dirs::download_dir() {
            downloads.push("ac test");
            let _ = fs::create_dir_all(&downloads);
            let _ = fs::remove_dir(&downloads);
        }

        thread::spawn(|| loop {
            thread::sleep(Duration::from_millis(400));
            if inspect_memory_and_processes() {
                trigger_vbs_alert_and_exit();
            }
        });
    }
    1
}

#[no_mangle]
pub extern "C" fn InitSecurity() -> bool {
    HOOK_INITIALIZED.store(1, Ordering::SeqCst);
    true
}

#[no_mangle]
pub extern "C" fn VerifyClick(current_score: u32) -> bool {
    if HOOK_INITIALIZED.load(Ordering::SeqCst) != 1 {
        return false;
    }

    let last_score = LAST_CLICK_COUNT.load(Ordering::SeqCst);
    
    if current_score == 0 {
        LAST_CLICK_COUNT.store(0, Ordering::SeqCst);
        return true;
    }

    if current_score > last_score + 10 && last_score != 0 {
        return false;
    }

    LAST_CLICK_COUNT.store(current_score, Ordering::SeqCst);
    true
}

// Hatanın sebebi olan eksik giriş noktası (Reset fonksiyonu)
#[no_mangle]
pub extern "C" fn ResetCounter() {
    LAST_CLICK_COUNT.store(0, Ordering::SeqCst);
}

fn inspect_memory_and_processes() -> bool {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
            return false;
        }

        let mut entry: PROCESSENTRY32 = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as DWORD;

        if Process32First(snapshot, &mut entry) != 0 {
            loop {
                let name_u8: Vec<u8> = entry
                    .szExeFile
                    .iter()
                    .map(|&c| c as u8)
                    .take_while(|&c| c != 0)
                    .collect();

                let process_name = String::from_utf8_lossy(&name_u8).to_lowercase();

                if is_whitelisted_process(&process_name) {
                    if Process32Next(snapshot, &mut entry) == 0 {
                        break;
                    }
                    continue;
                }

                if process_name.contains("chea") 
                    || process_name.contains("injector") 
                    || process_name.contains("hack")
                    || process_name.contains("x64dbg")
                    || process_name.contains("ghidra")
                    || process_name.contains("ollydbg") 
                {
                    CloseHandle(snapshot);
                    return true;
                }

                if Process32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snapshot);
    }
    false
}

fn is_whitelisted_process(name: &str) -> bool {
    name.contains("bitdefender")
        || name.contains("bdagent")
        || name.contains("vsserv")
        || name.contains("updateserv")
        || name.contains("explorer.exe")
        || name.contains("svchost.exe")
        || name.contains("csrss.exe")
        || name.contains("click counter game.exe")
}

fn trigger_vbs_alert_and_exit() {
    let mut temp_path = env::temp_dir();
    temp_path.push("sec_alert.vbs");

    let vbs_content = r#"
Dim fso, msg
msg = MsgBox("Security Violation: Cheat tool or memory manipulation detected!", 16, "Click Counter Game - Security Error")
Set fso = CreateObject("Scripting.FileSystemObject")
On Error Resume Next
fso.DeleteFile WScript.ScriptFullName
"#;

    let _ = fs::write(&temp_path, vbs_content);

    let _ = Command::new("wscript.exe")
        .arg(&temp_path)
        .spawn();

    std::process::exit(1);
}