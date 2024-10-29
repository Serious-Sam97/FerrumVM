# FerrumVM

**FerrumVM** is an Ethereum Virtual Machine (EVM) implementation written in Rust, aiming to provide a reliable, modular, and high-performance environment for executing smart contracts. The project leverages Rust's safety features and efficiency to handle essential EVM operations, such as managing accounts, executing opcodes, and processing transaction states.

## Project Structure

- **Account**: Represents Ethereum accounts, holding balance, nonce, storage, and contract bytecode.
- **Execution Context**: Stores transaction-specific data, such as the caller’s address, transaction value, gas limit, and input data.
- **Stack and Memory**: Core components for handling stack-based and memory-based operations in the EVM.
- **Opcodes**: Implements Ethereum opcodes (e.g., `ADD`, `PUSH`, `POP`) to execute contract logic.
- **Gas Management**: Tracks gas usage and ensures transactions halt if gas runs out.

## Features

- **Account Management**: Initialize and manipulate accounts with balance, nonce, storage, and bytecode.
- **Execution Context**: Define and manage the environment for each transaction, including gas and caller data.
- **Modular Design**: Organized with modular code for maintainability and scalability.

---