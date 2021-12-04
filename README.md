# Payment microservice 
Microservicio encargado de los medios de pago, está en Rust

## DOCKER
Para ejecutar los contenedores del microservicio y la base de datos en la carpeta `payment_ms` ejecutar:

    docker-compose up -d // tarda 7 mins aprox y pesa 5GB.


Una vez que los contenedores están en ejecución hay que ejecutar la migración:

    docker exec -it container_id // id del contenedor payment_ms
    diesel migration run

Queda pendiente la optimización del tamaño del contenedor (reducir las 5GB).

## Instalación

### Instalar Rust
Para poder usar Rust es necesario instalar **Rustup**, el cual es un instalador y un gestor de versiones de Rust, esto se hace mediante el siguiente comando:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Para comprobar que **Rustup** quedó instalado correctamente:

    rustup --version

Como Rocket.rs usa la versión nightly de rust es necesario ejecutar:

    rustup default nightly

### Instalar Cargo Watch
Rustup incluye también rustc, que es el compilador, y cargo, el cual es el package manager de Rust. Pero para poder visualizar los cambios en la aplicación, es decir, que la aplicación se compile y ejecute cada vez que se realiza un cambio es necesario instalar cargo watch

    cargo install cargo-watch

### Instalar Diesel
Diesel es un ORM para Rust que también proporciona un CLI. (En este caso es necesario tener instalado PostgreSQL)

    cargo install diesel_cli --no-default-features --features postgres

#### Posible error al instalar Diesel_cli
En caso de que se presente el siguiente error al ejecutar el comando anterior

    ...
    error: linking with `cc` failed exit code: 1

    ...

    error: aborting due to previous error
    error: failed to compile `diesel_cli v1.4.4`

Para solucionarlo es necesario ejecutar

    sudo apt install libpq-dev


# Ejecución
Teniendo el repositorio clonado y estando en la carpeta `api_payment/` ejecutar en consola

    cargo build
    cargo watch -x run

    // También se puede de la siguiente manera, pero no se reflejarán los cambios cada vez que se guarde
    cargo build
    cargo run

La aplicación se ejecutará en `localhost:8000`.  
Nota: `cargo build` es necesario la primera vez que se ejecuta el proyecto o también cada vez que se añade una nueva dependencia en la sección `[dependencies]` del archivo `Cargo.toml`
