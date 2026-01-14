use std::time::Duration;
use sysinfo::System;
use tokio::time::sleep;

pub async fn wait_for_warframe_start() {
    log::info!("Waiting for Warframe to start...");
    let mut system = System::new();

    loop {
        system.refresh_all();

        let running = system.processes().values().any(|process| {
            let name_match = process
                .name()
                .to_string_lossy()
                .contains("Warframe.x64.exe");
            let cmd_match = process
                .cmd()
                .iter()
                .any(|arg| arg.to_string_lossy().contains("Warframe.x64.exe"));

            if name_match || cmd_match {
                // Check excludes: ignore launcher which uses Preprocess.log
                let is_launcher = process
                    .cmd()
                    .iter()
                    .any(|arg| arg.to_string_lossy().contains("Preprocess.log"));
                !is_launcher
            } else {
                false
            }
        });

        if running {
            log::info!("Warframe process detected.");
            break;
        }

        sleep(Duration::from_secs(5)).await;
    }
}
