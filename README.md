# Transaction Engine

A command-line payment engine written in Rust that processes a stream of transactions 
from a CSV file, handles client accounts, disputes, and chargebacks, and outputs the 
final account balances.

## 🚀 Getting Started

### Prerequisites
- Rust (latest stable version recommended)
- Cargo

### Building and Running
To run the engine with a sample input file and just see the result on the terminal:

```bash
cargo run -- <csv_file_path>
```

To run the engine with a sample input file and persist the result into another CSV:

```bash
cargo run -- transactions.csv > accounts.csv
```

To run with cargo's release optimizations (recommended for large datasets):
```bash
cargo run --release -- transactions.csv > accounts.csv
```

### Running Tests
This project includes unit tests covering deposits, withdrawals, disputes, 
resolutions, chargebacks, and edge cases (locked accounts, duplicates).
```bash
cargo test
```

## 🏗 Architecture & Design Decisions
This solution was designed with **Safety**, **Correctness**, and **Efficiency** as 
primary goals.

### 1. Memory Efficiency & Streaming

Instead of loading the entire dataset into memory, the engine streams the 
input CSV row by row. This ensures the memory remains relatively constant regardless 
of the input file size (handling millions of transactions efficiently).

### 2. Data Integrity

Floating-point arithmetic (`f32/f64`) is prone to rounding errors and is unsuitable 
for financial calculations. This engine uses the `rust_decimal` crate to ensure 
precision up to 4 decimal places, guaranteeing mathematical correctness.

### 3. Idempotency & Duplicate Handling

Although the specification implies unique transaction IDs, robust systems must 
handle replay attacks or duplicate inputs.

- The engine tracks processed transaction IDs.
- **Optimization**: To save memory, `Withdrawal` transactions are stored in the history 
map as a simple Enum variant without payload data (`Transaction::Withdrawal`). 
Only `Deposit` transactions store the full metadata required for potential disputes. 
This significantly reduces heap usage.

### 4. Type Safety & Error Handling

- **Parse, don't validate:** The system converts raw CSV input into strongly typed 
domain structs (`Deposit`, `Account`) as early as possible. Invalid states are 
unrepresentable in the domain layer.

- **Resilience:** Malformed CSV rows are logged to `stderr` but do not crash the engine, 
allowing valid transactions to be processed.

- **Pattern Matching:** Rust's strict pattern matching ensures that operations like 
`Dispute` or `Resolve` only affect valid `Deposit` transactions, ignoring `Withdrawals` 
or non-existent IDs automatically.

## 📂 Project Structure
* `src/main.rs`: Entry point. Handles CLI arguments, CSV I/O streaming, and top-level 
error reporting.

* `src/engine.rs`: Core business logic. Manages the state of accounts and transaction 
history.

* `src/models.rs`: Domain entities (Account, Transaction types) and DTOs for 
serialization/deserialization.

## 📝 Assumptions
Based on the requirements, the following assumptions were made:

1. **Duplicate Transactions:** If a `deposit` or `withdrawal` arrives with an ID that has 
already been seen, it is ignored to preserve idempotency.

2. **Dispute Scope:** Only `deposits` can be disputed. Disputes referencing `withdrawals` 
or non-existent IDs are ignored.

3. **Locked Accounts:** Once an account is locked (due to a chargeback), it ignores all 
`deposit` and `withdrawal` operations but allows "admin" operations as `dispute`.

4. **Input format:** The input CSV is expected to follow the headers: `type, client, tx, amount`.

## 📦 Dependencies
* `csv`: Fast and flexible CSV parsing with streaming support.

* `serde`: Efficient serialization framework.

* `rust_decimal: Arbitrary precision decimal arithmetic for financial operations.

## 📈 Evolution (next steps)
Storing all deposits in memory into a HashMap allows fast O(1) dispute resolution but 
limits the dataset size to available RAM. For production systems processing massive 
datasets, I would use an external key-value store (like Redis) or a database to persist 
transaction history efficiently.

## 🤖 AI
This project involved consulting AI. Given my experience in `Java` and since 
this was my first contact with `Rust`, most of the questions were related to the 
"Rust way" compared to Java, real examples:

```
In Java, I would create an operations interface and an implementation for each operation 
following the strategy pattern. In Rust, would it be good practice to follow the same 
approach, or is there a more idiomatic way to do this?
```

```
In Java, when working with monetary values, we cannot rely on primitive types like float 
and double; instead, we must use BigDecimal. In Rust, can I rely on primitive types, or 
should I use a special type?
```

```
In Java, the package structure follows a pattern like com.company.project, and typically 
there's one class for each model, controller, and logic. What are the best practices for 
Rust project structure?
```

```
Is there a native function or library in Rust that performs the same parsing role as 
Jackson in Java?
```

```
Just like Java's Optional has an "orElse" statement, is there something similar in Rust?
```

And about nuances of Rust:
```
What is the difference between pub and pub(crate)?
```

```
Through research, I understand that it's good practice in Rust to keep unit tests in the 
same file as the main logic, but I still want to keep them separate. Is there another good 
practice for this?
```

```
Explain the concept of Borrow Checker, as my IDE frequently complains about it.
```

General help:
```
Below I describe how to use my Rust project, its structure and architecture, as well 
as the motivations behind each decision. Help me structure the following text into a 
readable and user-friendly README:
...
```