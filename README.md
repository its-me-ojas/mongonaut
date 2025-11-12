# 🗺️ Monjo-Kompass

**A blazingly fast TUI (Terminal User Interface) for MongoDB — Browse databases, explore collections, and query documents with ease.**

<p align="center">
  <img src="https://img.shields.io/badge/MongoDB-47A248?style=for-the-badge&logo=mongodb&logoColor=white" alt="MongoDB">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge" alt="License">
</p>

---

## ✨ Features

- 🚀 **Lightning Fast** — Built with Rust for maximum performance
- 🎨 **Intuitive TUI** — Beautiful terminal interface powered by Ratatui
- 🔍 **Smart Navigation** — Keyboard-driven workflow for power users
- 📊 **Document Viewer** — Browse and inspect MongoDB documents with syntax highlighting
- 🔌 **Easy Connection** — Connect to local or remote MongoDB instances
- 📂 **Database Explorer** — Navigate databases and collections effortlessly
- ⌨️ **Vim-like Controls** — Familiar keybindings for efficient navigation

---

## 📸 Screenshots

### Welcome Page
![Database Selection](assets/welcomePage.png)
*Connect using connection string and save that as well*

### Database Selection
![Database Selection](assets/dbs.png)
*Browse all available databases with collection counts*

### Collection Browser
![Collection Browser](assets/collections.png)
*View collections with document counts and indexes*

### Document Viewer
![Document Viewer](assets/documents.png)
*Inspect documents with formatted JSON display*

---

## 📥 Installation

### Build from Source

Requires [Rust](https://rustup.rs/) 1.70 or later.
```bash
```bash
# Clone the repository
git clone https://github.com/its-me-ojas/monjo-kompass.git
cd monjo-kompass

# Build in release mode
cargo build --release

# Run the binary
./target/release/monjo-kompass
```

---

## 🚀 Quick Start

1. **Launch Monjo-Kompass:**
```bash
   monjo-kompass
```

2. **Enter your MongoDB connection URI:**
```
   mongodb://localhost:27017
```
   Or for remote connections:
```
   mongodb+srv://username:password@cluster.mongodb.net/
```

3. **Navigate and explore:**
   - Use `↑/↓` or `j/k` to navigate
   - Press `Enter` to select
   - Press `Backspace` to go back
   - Press `q` to quit

---

## ⌨️ Keybindings

| Key | Action |
|-----|--------|
| `↑/↓` or `j/k` | Navigate up/down |
| `Enter` | Select database/collection/document |
| `Backspace` | Go back to previous view |
| `PgUp/PgDn` | Scroll document content |
| `f` | Search/filter (coming soon) |
| `r` | Refresh current view |
| `q` | Quit application |
| `Ctrl+C` | Force quit |

---

## 🎯 Usage Examples

### Connect to Local MongoDB
```bash
monjo-kompass
# Enter: mongodb://localhost:27017
```

### Connect with Authentication
```bash
monjo-kompass
# Enter: mongodb://username:password@localhost:27017/admin
```

### Connect to MongoDB Atlas
```bash
monjo-kompass
# Enter: mongodb+srv://user:pass@cluster.mongodb.net/
```

---


## 🗺️ Roadmap

- [x] Database and collection browsing
- [x] Document viewer with JSON formatting
- [x] Keyboard navigation
- [ ] Document editing
- [ ] Query builder
- [ ] Data export (JSON, CSV, BSON)
- [ ] Search and filtering
- [ ] Index management
- [ ] Connection profiles
- [ ] Aggregation pipeline builder
- [ ] Query history
- [ ] Dark/Light theme support

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. 🍴 Fork the repository
2. 🔨 Create a feature branch (`git checkout -b feature/amazing-feature`)
3. 💾 Commit your changes (`git commit -m 'Add amazing feature'`)
4. 📤 Push to the branch (`git push origin feature/amazing-feature`)
5. 🎉 Open a Pull Request

Please ensure your code follows Rust conventions and includes tests where applicable.

---

## 🐛 Bug Reports

Found a bug? Please open an issue with:
- Your OS and terminal emulator
- MongoDB version
- Steps to reproduce
- Expected vs actual behavior


---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- Inspired by [MongoDB Compass](https://www.mongodb.com/products/compass)
- Built with ❤️ using [Rust](https://www.rust-lang.org/)
- TUI powered by [Ratatui](https://ratatui.rs/)

---

## 📞 Support

- 📧 Email: [crestfallenwizard@gmail.com](mailto:crestfallenwizard@gmail.com)
- 🐦 Twitter: [@ojvznx](https://x.com/ojvznx)
- 💬 Discussions: [GitHub Discussions](https://github.com/its-me-ojas/monjo-kompass/discussions)

---

<p align="center">
  Made with 🦀 by <a href="https://github.com/its-me-ojas">its-me-ojas</a>
</p>

<p align="center">
  ⭐ Star this repo if you find it useful!
</p>
