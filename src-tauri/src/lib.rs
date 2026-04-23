mod discovery;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PortDataEvent {
    port: String,
    data: Vec<u8>,
    timestamp: u64,
    text: String,
    direction: String, // "rx" (from device) or "tx" (to device)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BaudProbeResult {
    baud: u32,
    score: f32,
    bytes_read: usize,
    printable_count: usize,
    sample_ascii: String,
    error: Option<String>,
}

struct PortRegistration {
    active: bool,
    write_tx: Sender<Vec<u8>>,
}

type MonitorRegistry = Arc<Mutex<HashMap<String, PortRegistration>>>;

fn is_active(registry: &MonitorRegistry, port: &str) -> bool {
    registry
        .lock()
        .unwrap()
        .get(port)
        .map(|r| r.active)
        .unwrap_or(false)
}

#[tauri::command]
fn discover_devices() -> Vec<discovery::DiscoveredDevice> {
    discovery::list_devices()
}

#[tauri::command]
async fn start_port_monitor(
    app: AppHandle,
    registry: tauri::State<'_, MonitorRegistry>,
    port: String,
    baud_rate: u32,
) -> Result<(), String> {
    let registry = registry.inner().clone();

    // Create write channel synchronously so send_bytes_to_port calls race-free
    let (write_tx, write_rx) = channel::<Vec<u8>>();

    {
        let mut map = registry.lock().map_err(|e| e.to_string())?;
        if map.get(&port).map(|r| r.active).unwrap_or(false) {
            return Err(format!("{} is already being monitored", port));
        }
        map.insert(
            port.clone(),
            PortRegistration {
                active: true,
                write_tx,
            },
        );
    }

    let port_name = port.clone();
    let reg = registry.clone();

    std::thread::spawn(move || {
        let mut serial = match serialport::new(&port_name, baud_rate)
            .timeout(std::time::Duration::from_millis(100))
            .open()
        {
            Ok(p) => p,
            Err(e) => {
                let _ = app.emit(
                    "port:error",
                    serde_json::json!({ "port": &port_name, "error": e.to_string() }),
                );
                reg.lock().unwrap().remove(&port_name);
                return;
            }
        };

        let mut buf = vec![0u8; 512];

        loop {
            if !is_active(&reg, &port_name) {
                break;
            }

            // Drain any pending writes first (non-blocking)
            while let Ok(data) = write_rx.try_recv() {
                match serial.write_all(&data) {
                    Ok(_) => {
                        let _ = serial.flush();
                        let timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as u64;
                        let text = String::from_utf8_lossy(&data).into_owned();
                        let _ = app.emit(
                            "port:data",
                            PortDataEvent {
                                port: port_name.clone(),
                                data: data.clone(),
                                timestamp,
                                text,
                                direction: "tx".into(),
                            },
                        );
                    }
                    Err(e) => {
                        let _ = app.emit(
                            "port:error",
                            serde_json::json!({
                                "port": &port_name,
                                "error": format!("Write failed: {}", e),
                            }),
                        );
                    }
                }
            }

            // Read incoming data
            match serial.read(&mut buf) {
                Ok(0) => {}
                Ok(n) => {
                    let data = buf[..n].to_vec();
                    let text = String::from_utf8_lossy(&data).into_owned();
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64;
                    let _ = app.emit(
                        "port:data",
                        PortDataEvent {
                            port: port_name.clone(),
                            data,
                            timestamp,
                            text,
                            direction: "rx".into(),
                        },
                    );
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {}
                Err(_) => break,
            }
        }

        reg.lock().unwrap().remove(&port_name);
        let _ = app.emit("port:closed", &port_name);
    });

    Ok(())
}

#[tauri::command]
fn send_bytes_to_port(
    registry: tauri::State<'_, MonitorRegistry>,
    port: String,
    data: Vec<u8>,
) -> Result<(), String> {
    if data.is_empty() {
        return Err("Cannot send empty payload".into());
    }
    let map = registry.lock().map_err(|e| e.to_string())?;
    let state = map
        .get(&port)
        .ok_or_else(|| format!("{} is not being monitored", port))?;
    if !state.active {
        return Err(format!("{} monitor is not active", port));
    }
    state
        .write_tx
        .send(data)
        .map_err(|e| format!("Write channel closed: {}", e))?;
    Ok(())
}

fn is_printable_byte(b: u8) -> bool {
    (0x20..=0x7E).contains(&b) || b == b'\n' || b == b'\r' || b == b'\t'
}

fn probe_single_sync(port: &str, baud: u32) -> BaudProbeResult {
    let mut serial = match serialport::new(port, baud)
        .timeout(std::time::Duration::from_millis(50))
        .open()
    {
        Ok(p) => p,
        Err(e) => {
            return BaudProbeResult {
                baud,
                score: 0.0,
                bytes_read: 0,
                printable_count: 0,
                sample_ascii: String::new(),
                error: Some(e.to_string()),
            };
        }
    };

    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(1000);
    let mut buf = vec![0u8; 1024];
    let mut bytes_read = 0usize;
    let mut printable_count = 0usize;
    let mut sample: Vec<u8> = Vec::with_capacity(80);

    while std::time::Instant::now() < deadline {
        match serial.read(&mut buf) {
            Ok(0) => {}
            Ok(n) => {
                bytes_read += n;
                for &b in &buf[..n] {
                    if is_printable_byte(b) {
                        printable_count += 1;
                    }
                }
                if sample.len() < 80 {
                    let take = std::cmp::min(80 - sample.len(), n);
                    sample.extend_from_slice(&buf[..take]);
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {}
            Err(_) => break,
        }
    }

    let score = if bytes_read > 0 {
        printable_count as f32 / bytes_read as f32
    } else {
        0.0
    };

    BaudProbeResult {
        baud,
        score,
        bytes_read,
        printable_count,
        sample_ascii: String::from_utf8_lossy(&sample)
            .chars()
            .map(|c| {
                if c.is_control() && c != '\n' && c != '\r' && c != '\t' {
                    '·'
                } else {
                    c
                }
            })
            .collect(),
        error: None,
    }
}

#[tauri::command]
async fn probe_single_baud(
    registry: tauri::State<'_, MonitorRegistry>,
    port: String,
    baud: u32,
) -> Result<BaudProbeResult, String> {
    {
        let map = registry.lock().map_err(|e| e.to_string())?;
        if map.get(&port).map(|r| r.active).unwrap_or(false) {
            return Err(format!(
                "{} is currently being monitored — stop it first",
                port
            ));
        }
    }

    tauri::async_runtime::spawn_blocking(move || probe_single_sync(&port, baud))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn stop_port_monitor(
    registry: tauri::State<'_, MonitorRegistry>,
    port: String,
) -> Result<(), String> {
    let mut map = registry.lock().map_err(|e| e.to_string())?;
    if let Some(reg) = map.get_mut(&port) {
        reg.active = false;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let registry: MonitorRegistry = Arc::new(Mutex::new(HashMap::new()));

    tauri::Builder::default()
        .manage(registry)
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_serialplugin::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            discover_devices,
            start_port_monitor,
            stop_port_monitor,
            probe_single_baud,
            send_bytes_to_port,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
