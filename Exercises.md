# 🦀 crab-race-lab — Ejercicios de concurrencia en Rust

Progresión de ejercicios: de `std::thread` básico hasta patrones async más avanzados con Tokio.

---

## Bloque 1 — Sin Tokio (Rust puro, sync)

**1.1 Reloj bloqueante**
Imprime "tick" cada segundo, 5 veces, usando `std::thread::sleep`.

**1.2 Dos relojes "al mismo tiempo" (sin threads)**
Imprime "tick A" cada 1s y "tick B" cada 1.5s, ambos 5 veces, en el mismo hilo, sin threads. Vas a notar que uno bloquea al otro — todo sale en serie.

**1.3 Arreglarlo con threads**
Mismo ejercicio que 1.2 pero con `std::thread::spawn` para cada reloj. Usa `.join()` **después** de crear ambos threads (no dentro del mismo loop donde los creas).

**1.4 Contador compartido**
5 threads que incrementan un contador compartido 1000 veces cada uno. Usa `Arc<Mutex<i32>>`. Verifica que el resultado final sea 5000. Guarda los `JoinHandle` en un `Vec` y haz join fuera del loop de creación.

---

## Bloque 2 — Con Tokio (async, nivel básico)

Agrega a `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

**2.1 Hola tokio**
```rust
#[tokio::main]
async fn main() {
    println!("hola desde una task async");
}
```

**2.2 El mismo reloj, pero async**
Repite 1.1 con `tokio::time::sleep(...).await` en vez de `std::thread::sleep`.

**2.3 Los dos relojes, ahora sí concurrentes**
Repite 1.2 pero con `tokio::spawn` para cada reloj. Usa `tokio::join!` o guarda los `JoinHandle` y haz `.await` sobre ellos al final.

**2.4 Descargas simuladas**
Simula 3 "descargas" con `tokio::time::sleep` (3s, 1s, 2s). Lánzalas con `tokio::spawn` y mide con `tokio::join!` cuánto tarda todo. Si tokio corre concurrente de verdad, el total debería ser ~3s, no 6s.

**2.5 Servidor TCP mini**
Usa `tokio::net::TcpListener` para aceptar conexiones y responder "Hola, cliente!" a cada una. Prueba con `telnet localhost <puerto>` o `nc`.

---

## Bloque 3 — Tokio nivel intermedio

**3.1 Contador compartido, pero async**
Repite el ejercicio 1.4 (contador con `Arc<Mutex<i32>>`) pero usando `tokio::spawn` en vez de `std::thread::spawn`, y `tokio::sync::Mutex` en vez de `std::sync::Mutex`. Pregúntate: ¿por qué existe un `Mutex` específico para tokio? Pista: tiene que ver con qué pasa si un `.lock()` tarda mucho dentro de una task async.

**3.2 Canales entre tasks (`mpsc`)**
Crea un canal con `tokio::sync::mpsc::channel`. Lanza 3 tasks "productoras" que envíen números del 1 al 10 cada una (30 mensajes en total), y una task "consumidora" que los reciba todos e imprima la suma total. Esto reemplaza el patrón de "estado compartido con Mutex" por "pasar mensajes" — un estilo muy común en sistemas concurrentes.

**3.3 Timeout de una operación**
Simula una "operación lenta" con `tokio::time::sleep` de 5 segundos. Usa `tokio::time::timeout` para cancelarla si tarda más de 2 segundos, e imprime un mensaje distinto según si terminó a tiempo o hizo timeout.

**3.4 `select!` — la primera tarea que responda gana**
Lanza dos tareas simuladas (con distintos tiempos de sleep) que representan "dos servidores respondiendo a la misma petición". Usa `tokio::select!` para quedarte con la respuesta de la que termine primero, y cancela (implícitamente) la otra.

**3.5 Rate limiting simple**
Simula 10 "clientes" haciendo requests, pero solo permite que 3 corran al mismo tiempo (los demás deben esperar su turno). Usa `tokio::sync::Semaphore`.

---

## Bloque 4 — Tokio nivel avanzado (más cercano a un backend real)

**4.1 Servidor TCP con estado compartido**
Extiende el ejercicio 2.5: el servidor debe llevar un contador de "cuántos clientes se han conectado en total" (`Arc<Mutex<u32>>` o `Arc<tokio::sync::Mutex<u32>>`), y cada cliente nuevo recibe un mensaje tipo "Eres el cliente número N".

**4.2 Broadcast a múltiples clientes**
Usa `tokio::sync::broadcast` para simular un chat mínimo: varios "clientes" (tasks) están suscritos a un canal, y cuando cualquiera "envía un mensaje", todos los demás lo reciben. No necesitas sockets reales todavía — puedes simularlo todo con tasks dentro del mismo programa.

**4.3 Cancelación con `CancellationToken`**
Usando la crate `tokio-util`, lanza una task que hace trabajo repetido (como un loop con sleep), y desde `main` cancélala después de 3 segundos usando un `CancellationToken`. La task debe terminar limpiamente al detectar la cancelación, no de golpe.

**4.4 Reintentos con backoff exponencial**
Simula una función que "falla" las primeras 2 veces (puedes usar un contador o `rand`) y luego funciona a la tercera. Escribe una función async genérica que reintente con espera creciente (100ms, 200ms, 400ms...) usando `tokio::time::sleep`, hasta un máximo de intentos.

**4.5 Pool de workers con `mpsc` + `JoinSet`**
Crea un canal `mpsc` donde llegan 20 "trabajos" (simples, como calcular el cuadrado de un número). Lanza 4 tasks "worker" que consuman de ese canal y procesen los trabajos que les toquen. Usa `tokio::task::JoinSet` para manejar el grupo de workers y esperar a que todos terminen limpiamente cuando el canal se cierre.

**4.6 Mini servidor HTTP con `axum`**
Este ya se sale un poco de tokio "puro", pero es el paso natural: usa el framework `axum` (que corre sobre tokio) para crear un servidor con dos rutas: `GET /` que responda "Hola" y `GET /lento` que espere 2 segundos (con `tokio::time::sleep`) antes de responder. Prueba que mientras `/lento` está esperando, puedes seguir pegándole a `/` sin que se bloquee — la prueba definitiva de que el modelo async está funcionando de verdad.

---

## Progresión sugerida

Si el bloque 3 o 4 se sienten pesados, no pasa nada por tardarte varios días en cada uno — la idea es que cada ejercicio te obligue a usar una herramienta nueva (`Semaphore`, `select!`, `broadcast`, etc.) mientras ya tienes automatizado lo básico (`spawn`, `join`, `Arc<Mutex<T>>`).

**Orden recomendado si te sientes con ganas de saltar:**
1. Termina el bloque 1 y 2 completos.
2. Bloque 3 en orden (3.1 → 3.5), cada uno se apoya un poco en el anterior.
3. Bloque 4: 4.1 y 4.2 primero (extienden el servidor TCP que ya conoces), luego 4.3-4.5 (patrones de control más generales), y 4.6 al final como "graduación" — ya usando algo más parecido a un backend real en producción.