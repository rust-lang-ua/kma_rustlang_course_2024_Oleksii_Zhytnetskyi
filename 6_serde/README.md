# Практична робота: Серіалізація та десеріалізація з `serde`

### Підготовка

1. **Налаштування проєкту**  
   Створіть новий проєкт у Rust:

   ```bash
   cargo new serde_practice --bin
   cd serde_practice
   ```

2. **Додайте залежності в `Cargo.toml`**
   ```toml
   [dependencies]
   chrono = { version = "0.4.38", features = ["serde"] }
   humantime-serde = "1.1.1"
   serde = { version = "1.0.214", features = ["derive"]}
   serde_json = "1.0.132"
   serde_yaml = "0.9.34"
   toml = "0.8.19"
   url = { version = "2.5.3", features = ["serde"] }
   uuid = { version = "1.11.0", features = ["serde"] }
   ```

---

### 1. Базова серіалізація та десеріалізація

**Завдання**: Створіть структуру `User` з полями `name`, `email` та `birthdate`. Виконайте серіалізацію в JSON і десеріалізацію назад у структуру.

```rust
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
    birthdate: String,
}

fn main() {
    // Створення екземпляра структури User
    let user = User {
        name: "Іван".to_string(),
        email: "ivan@example.com".to_string(),
        birthdate: "2000-01-01".to_string(),
    };

    // Серіалізація в JSON
    let json = serde_json::to_string(&user).expect("Помилка серіалізації");
    println!("Серіалізований JSON: {}", json);

    // Десеріалізація з JSON
    let deserialized_user: User = serde_json::from_str(&json).expect("Помилка десеріалізації");
    println!("Десеріалізований користувач: {:?}", deserialized_user);
}
```

---

### 2. Десеріалізація з файлу

**Завдання**: Створіть файл `request.json` з JSON-даними та прочитайте його в структуру `Request`.

**request.json**

```json
{
  "type": "success",
  "stream": {
    "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
    "is_private": false,
    "settings": 45345,
    "shard_url": "https://n3.example.com/sapi",
    "public_tariff": {
      "id": 1,
      "price": 100,
      "duration": "1h",
      "description": "test public tariff"
    },
    "private_tariff": {
      "client_price": 250,
      "duration": "1m",
      "description": "test private tariff"
    }
  },
  "gifts": [
    {
      "id": 1,
      "price": 2,
      "description": "Gift 1"
    },
    {
      "id": 2,
      "price": 3,
      "description": "Gift 2"
    }
  ],
  "debug": {
    "duration": "234ms",
    "at": "2019-06-28T08:35:46+00:00"
  }
}
```

**Код:**

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_yaml::to_string as to_yaml;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use toml::to_string as to_toml;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

fn main() {
    let mut file = File::open("request.json").unwrap();
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).unwrap();

    let request: Request = serde_json::from_str(&json_str).unwrap();


}


```

---

### 3. Підтримка різних форматів (JSON, YAML, TOML)

**Завдання**: Виконайте серіалізацію структури `Request` у формат YAML і TOML.

```rust
fn main() {
    let mut file = File::open("request.json").unwrap();
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).unwrap();

    let request: Request = serde_json::from_str(&json_str).unwrap();

    let yaml_str = to_yaml(&request).unwrap();
    println!("YAML:\n{}", yaml_str);

    let toml_str = to_toml(&request).unwrap();
    println!("TOML:\n{}", toml_str);
}
```

---

### 4. Тестування

**Завдання**: Доведіть правильність своєї реалізації за допомогою тестів.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test() {
        let mut file = File::open("request.json").unwrap();
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();

        let request: Request = serde_json::from_str(&json_str).unwrap();
        assert_eq!(request.stream.public_tariff.id, 1);
        assert_eq!(request.stream.private_tariff.client_price, 250);
        assert_eq!(request.gifts.len(), 2);
        assert_eq!(request.gifts[0].description, "Gift 1");
    }
}
```

---

### 5. Кастомна серіалізація та десеріалізація

**Завдання**: Використовуйте власну логіку серіалізації для формату дати.

```rust
use serde::{Serializer, Deserializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    date: String,
}

// Функція серіалізації дати
fn serialize_date<S>(date: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("Date: {}", date))
}

// Функція десеріалізації дати
fn deserialize_date<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.replace("Date: ", ""))
}

fn main() {
    let event = Event {
        name: "Концерт".to_string(),
        date: "2024-11-15".to_string(),
    };

    let json = serde_json::to_string(&event).expect("Помилка серіалізації");
    println!("Серіалізований JSON з кастомною датою: {}", json);

    let deserialized_event: Event = serde_json::from_str(&json).expect("Помилка десеріалізації");
    println!("Десеріалізована подія: {:?}", deserialized_event);
}
```
