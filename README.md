## sui-move-kit

A CLI toolkit for streamlined development, testing, and deployment of Move smart contracts on the [Sui blockchain](https://sui.io/), built with Rust.

![](./assets/cover.png)

### **✨ Features**

- 🚀 Initialize Sui Move contract projects with standard structure
- 🛠 Build and compile Move contracts using the Sui toolchain
- ✅ Run Move tests quickly in your local environment
- 🌐 Publish to the Sui network with a single command
- 🧩 Simple and extensible CLI interface



### **🔧 Installation**

```bash
cargo install --path .
```

Or, to clone and install directly:

```bash
git clone https://github.com/libaice/sui-move-kit.git
cd sui-move-kit
cargo install --path .
```



### **🖥 Usage**

```bash
sui-move-kit <COMMAND> [OPTIONS]
```

**Commands**

| **Command** | **Description**                        |
| ----------- | -------------------------------------- |
| init        | Initialize a new Move project          |
| build       | Compile the current Move package       |
| test        | Run tests for the current Move package |
| publish     | Publish the package to a Sui endpoint  |



**Example**

```bash
# Create a new Move project
sui-move-kit init my_project

# Build the Move project
cd my_project
sui-move-kit build

# Run tests
sui-move-kit test

# Publish to a Sui devnet endpoint
sui-move-kit publish https://fullnode.devnet.sui.io
```



### **📁 Project Structure**

```bash
sui-move-kit/
  ├── src/
  │   └── my_project.move
  ├── move.toml
  └── README.md
```



### **🧱 Built With**

- [Rust](https://www.rust-lang.org/)
- [Clap](https://docs.rs/clap/latest/clap/) – Command-line parsing
- [Sui Move](https://docs.sui.io/) – Smart contract platform



### **📌 Roadmap**

- Support workspace templates
- Integrate Sui SDK for publish/test automation
- Auto-detect local Sui CLI installation
- Add .sui-move-kit.toml config support





























