# 📝 To-Do List CLI

A robust, feature-rich command-line To-Do List application built with **Rust**. Manage your tasks efficiently with persistent storage, advanced filtering, and intuitive search capabilities.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## ✨ Features

- ➕ **Add Tasks** – Create new tasks with titles and descriptions
- 🗑️ **Remove Tasks** – Delete tasks by ID
- 📋 **View All Tasks** – Display your complete task list with status indicators
- ✅ **Complete Tasks** – Mark individual tasks as done
- 💾 **Persistent Storage** – Automatically save/load tasks using JSON (via `serde_json`)
- 🔀 **Sort Tasks** – Organize tasks by completion status
- 📅 **Filter by Due Date** – View tasks within specific date ranges
- ⚡ **Bulk Complete** – Mark all tasks as completed in one command
- 🔍 **Smart Search** – Case-insensitive search across titles and descriptions
- 🛡️ **Error Handling** – Graceful error management with informative messages

## 🚀 Quick Start

### Prerequisites

Ensure you have **Rust 1.70+** installed. Install or update Rust via [rustup](https://www.rust-lang.org/tools/install):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/abhishekgautam95/ToDo_List.git
   cd ToDo_List
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the application:**
   ```bash
   cargo run --release
   ```

## 📖 Usage

Upon launching, you'll be presented with an interactive menu. Simply enter the corresponding number to perform actions:

```
=== To-Do List Manager ===
1. Add Task
2. Remove Task
3. View Tasks
4. Mark Task Complete
5. Sort Tasks
6. Filter by Due Date
7. Mark All Complete
8. Search Tasks
9. Exit
```

### Examples

**Add a new task:**
```
Enter choice: 1
Enter task title: Finish Rust project
Enter task description: Complete the README and code improvements
Enter due date (YYYY-MM-DD): 2024-12-31
```

**Search for tasks:**
```
Enter choice: 8
Enter search term: rust
```

**View all tasks:**
```
Enter choice: 3

ID: 1 | Status: [ ] | Title: Finish Rust project
     Description: Complete the README and code improvements
     Due: 2024-12-31
```

## 🏗️ Project Structure

```
todo_list/
├── Cargo.toml          # Project manifest & dependencies
├── src/
│   └── main.rs         # Main application logic
├── tasks.json          # Persistent task storage (auto-generated)
└── README.md           # This file
```

## 📦 Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `serde` | 1.0 | Serialization framework |
| `serde_json` | 1.0 | JSON parsing & generation |

## 🛠️ Development

### Build from Source

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Check Code Quality

```bash
cargo clippy -- -D warnings
cargo fmt --check
```

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using [Rust](https://www.rust-lang.org/)
- Powered by [`serde`](https://serde.rs/) for reliable data serialization

---

**Happy Task Managing! 🎯**
