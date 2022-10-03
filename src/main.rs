#![windows_subsystem = "windows"]

use winsafe::prelude::kernel_Hprocesslist;
use winsafe::prelude::HandleClose;

fn main() {
    let mut devmode = winsafe::DEVMODE::default();
    let _ = winsafe::EnumDisplaySettings(None, winsafe::GmidxEnum::Gmidx(4294967295u32), &mut devmode);
    let mut running: bool = false;

    loop {
        let hpl = winsafe::HPROCESSLIST::CreateToolhelp32Snapshot(winsafe::co::TH32CS::SNAPPROCESS, None).unwrap();
        if (hpl.iter_processes().any(|x| x.unwrap().szExeFile() == "nvstreamer.exe")) ^ running {
            running = !running;
            change_res(&mut devmode, running);
        }
        let _ = hpl.CloseHandle();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn change_res(devmode: &mut winsafe::DEVMODE, running: bool) {
    let flags = winsafe::co::CDS::SET_PRIMARY;
    if running {
        devmode.dmPelsWidth=3840;
        devmode.dmPelsHeight=2160;
        devmode.dmDisplayFrequency=60;
    }
    else {
        devmode.dmPelsWidth=3440;
        devmode.dmPelsHeight=1440;
        devmode.dmDisplayFrequency=144;
    }
    let _ = winsafe::ChangeDisplaySettings(Some(devmode), flags);
}
