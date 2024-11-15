### Крок 1: Підготовка проєкту та додавання залежностей

1. **Створіть новий проєкт**:

   ```bash
   cargo new calculator
   cd calculator
   ```

2. **Додайте залежності** в `Cargo.toml`:
   Відкрийте `Cargo.toml` і додайте бібліотеки `pest` та `pest_derive`:
   ```toml
   [dependencies]
   lazy_static = "1.4.0"
   pest = "2.6"
   pest_derive = "2.6"
   ```
3. **Запустіть команду збірки** для перевірки, чи все налаштовано правильно:
   ```bash
   cargo build
   ```
   Якщо помилок немає, ми готові перейти до наступного кроку.

---

### Крок 2: Визначення граматики для арифметичних виразів

**Example: Calculator**

У цьому прикладі розглядається практичний аспект використання синтаксичного аналізатора Пратта для розбору виразів з використанням `pest`.
Для ілюстрації ми створимо синтаксичний аналізатор для простих рівнянь і побудуємо абстрактне синтаксичне дерево.

**Перевага та асоціативність**

У простому рівнянні множення і ділення обчислюються першими, що означає, що вони мають вищий пріоритет.
Наприклад, `1 + 2 * 3` обчислюється як `1 + (2 * 3)`, якби пріоритет був рівним, то було б `(1 + 2) * 3`.
Для нашої системи маємо наступні операнди:

- найвищий пріоритет: множення та ділення
- найнижчий пріоритет: додавання та віднімання

У виразі `1 + 2 - 3` жоден оператор не є важливішим за інший.
Додавання, віднімання, множення та ділення обчислюються зліва направо,
наприклад, вираз `1 - 2 + 3` обчислюється як `(1 - 2) + 3`. Ми називаємо цю властивість лівою асоціативністю.
Оператори також можуть бути правою асоціативністю. Наприклад, ми зазвичай обчислюємо вираз `x = y = 1` спочатку
присвоюючи спочатку `y = 1`, а потім `x = 1` (або `x = y`).

Асоціативність має значення лише тоді, коли два оператори мають однаковий пріоритет, як у випадку з додаванням та відніманням, наприклад
наприклад, з додаванням та відніманням. Це означає, що якщо ми маємо вираз, який містить лише додавання і віднімання, ми можемо просто обчислити його зліва направо
зліва направо. Вираз `1 + 2 - 3` дорівнює `(1 + 2) - 3`. А `1 - 2 + 3` дорівнює `(1 - 2) + 3`.

Щоб перейти від плоского списку операндів, розділених операторами, достатньо визначити пріоритет та асоціативність для кожного
оператора. За допомогою цих визначень алгоритм, такий як синтаксичний аналіз Пратта, може побудувати відповідний
дерево виразів.

**Calculator example**

Ми хочемо, щоб наш калькулятор міг розбирати прості рівняння, які складаються з цілих чисел і простих бінарних операторів.
Крім того, ми хочемо підтримувати дужки та унарний мінус.
Наприклад:

```
1 + 2 * 3
-(2 + 5) * 16
```

**Grammar**

Ми починаємо з визначення наших атомів, бітів самодостатнього синтаксису, які не можна розділити на менші частини.
Для нашого калькулятора ми почнемо з простих цілих чисел:

```pest
// Пробіли між цифрами не допускаються
integer = @{ ASCII_DIGIT+ }

atom = _{ integer }
```

Далі, наші бінарні оператори:

```pest
bin_op = _{ add | subtract | multiply | divide }
	add = { "+" }
	subtract = { "-" }
	multiply = { "*" }
	divide = { "/" }
```

Відповідно до цього формату, ми визначаємо наше правило для виразів:

```pest
expr = { atom ~ (bin_op ~ atom)* }
```

І, нарешті, ми визначаємо наш `WHITESPACE` і правило рівняння:

```pest
WHITESPACE = _{ " " }

// Ми не можемо мати SOI та EOI безпосередньо на expr, тому що він використовується
// рекурсивно (наприклад, у круглих дужках)
equation = _{ SOI ~ expr ~ EOI }
```

Він визначає граматику, яка генерує необхідні вхідні дані для синтаксичного аналізатора Пратта.

---

### Крок 3: Абстрактне дерево синтаксису (AST)

Ми хочемо перетворити вхідні дані в абстрактне синтаксичне дерево.
Для цього ми визначимо наступні типи:

```rust
#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}
```

Зверніть увагу на `Box<Expr>`, оскільки Rust
[не допускає unboxed-рекурсивні типи](https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes).

Не існує окремого типу атома, будь-який атом також є допустимим виразом.

---

### Крок 4: Pratt parser

Пріоритет операцій визначається у синтаксичному аналізаторі Пратта.

Простим підходом є визначення PrattParser як глобального за допомогою [`lazy_static`](https://docs.rs/lazy_static/1.4.0/lazy_static/).

Дотримуючись стандартних правил арифметики,
ми визначимо, що додавання і віднімання мають нижчий пріоритет, ніж множення і ділення,
і зробимо всі оператори лівими асоціативними.

```rust
lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Пріоритет визначається від найнижчого до найвищого
        PrattParser::new()
            // Додавання та віднімання мають однаковий пріоритет
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
    };
}
```

Ми майже на місці, залишилося лише використати наш парсер Пратта.
Для цього використовуються функції `map_primary`, `map_infix` та `parse`, перші дві з яких приймають функції, а третя - ітератор над парами.
`map_primary` виконується для кожного первинника (атома), а `map_infix` виконується для кожного BinOp з його новою лівою частиною
та правою частиною відповідно до правил пріоритету, визначених раніше.
У цьому прикладі ми створюємо AST в синтаксичному аналізаторі Pratt.

```rust
pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule)
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .parse(pairs)

}
```

Ось приклад використання парсеру.

```rust
fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match CalculatorParser::parse(Rule::equation, &line?) {
            Ok(mut pairs) => {
                println!(
                    "Parsed: {:#?}",
                    parse_expr(
                        pairs.next().unwrap().into_inner()
                    )
                );
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
    }
    Ok(())
}

```

За допомогою цього ми можемо проаналізувати наступне просте рівняння:

```
> 1 * 2 + 3 / 4
Parsed: BinOp {
    lhs: BinOp {
        lhs: Integer( 1 ),
        op: Multiply,
        rhs: Integer( 2 ),
    },
    op: Add,
    rhs: BinOp {
        lhs: Integer( 3 ),
        op: Divide,
        rhs: Integer( 4 ),
    },
}
```

---

### Крок 4: Унарний мінус та дужки

Поки що наш калькулятор може розбирати досить складні вирази, але він дасть збій, якщо зустріне явні круглі дужки
або унарний знак мінус. Давайте це виправимо.

**Дужки**

Розглянемо вираз `(1 + 2) * 3`. Очевидно, що видалення дужок дасть інший результат, тому ми повинні
підтримувати розбір таких виразів. На щастя, це може бути простим доповненням до нашого правила `atom`:

```diff
- atom = _{ integer }
+ atom = _{ integer | "(" ~ expr ~ ")" }
```

Раніше ми говорили, що атоми повинні бути простими послідовностями токенів, які не можуть бути розбиті на частини, але тепер атом може містити
довільні вирази! Причина, по якій нас це влаштовує, полягає в тому, що круглі дужки позначають чіткі межі для
виразу, і не буде неоднозначно зрозуміло, які оператори належать до внутрішнього виразу, а які - до зовнішнього.

**Унарний мінус**

Наразі ми можемо розбирати лише натуральні числа, наприклад `16` або `2342`. Але ми також хочемо виконувати обчислення з від'ємними числами.
Для цього ми введемо унарний мінус, таким чином ми зможемо робити `-4` і `-(8 + 15)`.
Нам потрібна наступна зміна у граматиці:

```diff
+ unary_minus = { "-" }
+ primary = _{ integer | "(" ~ expr ~ ")" }
- atom = _{ integer | "(" ~ expr ~ ")" }
+ atom = _{ unary_minus? ~ primary }
```

Для цих останніх змін ми робимо невеликі зміни в AST і логіці парсингу (з використанням `map_prefix`).

```rust
PrattParser::new()
    .op(Op::infix(add, Left) | Op::infix(subtract, Left))
    .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
    .op(Op::prefix(unary_minus))
```

```rust
#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}
```

```rust
.map_primary(|primary| match primary.as_rule() {
    Rule::integer => Expr::Integer(primary.as_str().parse::<i32>().unwrap()),
    Rule::expr => parse_expr(primary.into_inner()),
    rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
})
```

```rust
.map_infix(|lhs, op, rhs| {
    let op = match op.as_rule() {
        Rule::add => Op::Add,
        Rule::subtract => Op::Subtract,
        Rule::multiply => Op::Multiply,
        Rule::divide => Op::Divide,
        Rule::modulo => Op::Modulo,
        rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
    };
    Expr::BinOp {
        lhs: Box::new(lhs),
        op,
        rhs: Box::new(rhs),
    }
})
```

```rust
.map_prefix(|op, rhs| match op.as_rule() {
    Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
    _ => unreachable!(),
})
```

```rust
#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
```

```rust
match CalculatorParser::parse(Rule::equation, &line?) {
    Ok(mut pairs) => {
        println!(
            "Parsed: {:#?}",

            parse_expr(pairs.next().unwrap().into_inner())
        );
    }
    Err(e) => {
        eprintln!("Parse failed: {:?}", e);
    }
}
```

Щоб реалізувати математичні обчислення для виразів, які парсить наш калькулятор, потрібно написати функцію, яка буде рекурсивно обчислювати значення для дерева синтаксичного розбору `Expr`.

---

### Крок 5: Додати функцію `evaluate`

```rust
impl Expr {
    pub fn evaluate(&self) -> i32 {
        match self {
            Expr::Integer(value) => *value,
            Expr::UnaryMinus(expr) => -expr.evaluate(),
            Expr::BinOp { lhs, op, rhs } => {
                let left = lhs.evaluate();
                let right = rhs.evaluate();
                match op {
                    Op::Add => left + right,
                    Op::Subtract => left - right,
                    Op::Multiply => left * right,
                    Op::Divide => left / right,
                    Op::Modulo => left % right,
                }
            }
        }
    }
}
```

---

### Крок 6: Викликати обчислення в `main`

Після парсингу виразу у функції `main`, додайте обчислення результату і його вивід:

```rust
fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match CalculatorParser::parse(Rule::equation, &line?) {
            Ok(mut pairs) => {
                let expr = parse_expr(pairs.next().unwrap().into_inner());
                println!("Result: {}", expr.evaluate());
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
    }
    Ok(())
}
```

### Як це працює:

1. `Expr::evaluate` обробляє всі типи вузлів дерева:
   - Якщо це `Integer`, повертає його значення.
   - Якщо це `UnaryMinus`, обчислює значення дочірнього вузла і змінює його знак.
   - Якщо це `BinOp`, обчислює значення лівого і правого дочірніх вузлів, а потім застосовує відповідну операцію.
2. У `main` ви виводите як дерево розбору, так і обчислений результат.

---

### Результат

Коли ви запустите програму з виразом:

```
-(2 + 5) * 16
```

Вивід буде:

```
Result: -112
```
