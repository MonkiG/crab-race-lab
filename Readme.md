# 🦀 crab-race-lab

Ejercicios de concurrencia en Rust: comparando threads nativos (`std::thread`) 
vs async con Tokio.

## Cómo correr los ejercicios

```bash
cargo run --bin ex1_3_threads
cargo run --bin ex2_4_simulated_downloads
```

## Bloque 1 — Sin Tokio (sync)

| Ejercicio | Descripción | Estado |
|---|---|---|
| `ex1_1_blocking_clock` | Reloj bloqueante con `thread::sleep` | ✅ |
| `ex1_2_two_clocks_sync` | Dos relojes en serie (sin threads) — muestra el bloqueo | ✅ |
| `ex1_3_threads` | Los mismos relojes, ahora concurrentes con `thread::spawn` | ✅ |
| `ex1_4_shared_counter` | Contador compartido con `Arc<Mutex<i32>>` | ✅ |

## Bloque 2 — Con Tokio (async)

| Ejercicio | Descripción |
|---|---|
| `ex2_1_hello_tokio` | Estructura mínima de un programa async |
| `ex2_2_async_clock` | Reloj con `tokio::time::sleep` |
| `ex2_3_concurrent_clocks_tokio` | Relojes concurrentes con `tokio::spawn` |
| `ex2_4_simulated_downloads` | Descargas simuladas corriendo en paralelo |
| `ex2_5_tcp_server` | Mini servidor TCP con `tokio::net::TcpListener` |
