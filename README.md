# RustLearning

Este repositorio contiene todos los miniproyectos de Rust que estoy construyendo como parte de mi aprendizaje y roadmap de Web3 y Cloud.

Cada proyecto es independiente y tiene su propio `Cargo.toml` y código fuente, pero todos siguen la misma estructura de aprendizaje: práctica de **ownership**, **structs**, **enums**, **pattern matching**, y preparación para **smart contracts** en Solana.

## Proyectos actuales

| Proyecto       | Descripción |
|----------------|------------|
| `structs_demo` | Demuestra cómo usar `structs` y métodos (`impl`) para modelar cuentas de usuario y operaciones como deposit, withdraw y transfer. |
| `enums_demo`   | Demuestra cómo usar `enums` y `match` para modelar transacciones y lógica condicional de manera segura y expresiva. |

## Estructura general

Cada subproyecto sigue esta estructura mínima:

```
/project_name
    Cargo.toml
    /src
        main.rs
```

## Cómo ejecutar un proyecto

1. Navegar al subproyecto:

```bash
cd structs_demo
```

2. Ejecutar:

```bash
cargo run
```

3. Ver salida en consola.

## Notas

- Todos los proyectos ignoran compilaciones y dependencias (`target/`) mediante `.gitignore`.  
- Cada proyecto está pensado para ser **autocontenido**, puedes ejecutar cualquiera sin necesidad de los demás.  

## Próximos proyectos

- `tx_enum_demo` – manejo de transacciones con enums y HashMap.  
- `mini_parser` – parser de logs y error handling avanzado.  
- `dapp_backend_demo` – integración inicial con un pequeño backend tipo dApp.