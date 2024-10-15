### 1

Перевіряємо яка версія Rust встановлена.

```
rustc --version
```

### 2

Якщо у вас версія нижче `rustc 1.81.0`

То оновіть Rust до актуальної версії

```
rustup update
```

### 3

Створюємо новий проект

```
cargo new my_first_code
```

![Alt text](../assets/image-16.png)

### 4

Переходимо у папку з проектом

```
cd my_first_code
```

### 5

Можемо подивитись що знаходиться в файлі `Cargo.toml`

```
cat Cargo.toml
```

![Alt text](../assets/image-17.png)

### 6

Можемо подивитись що знаходиться в файлі `main.rs`

```
cat src/main.rs
```

![Alt text](../assets/image-18.png)

### 7

Запускаємо программу

```
cargo run
```

![Alt text](../assets/image-15.png)

### 8

```
cargo install cargo-watch
```

### 9

```
cargo watch -cx "run"
```

![Alt text](../assets/image-19.png)
