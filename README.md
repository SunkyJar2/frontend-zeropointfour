Decentralized Password Manager with Shamir Secret Sharing

A research prototype of a decentralized password manager that implements Shamir Secret Sharing (SSS) to enhance password security. Unlike traditional centralized password managers, this system leverages a hybrid architecture combining self-hosted servers and public cloud databases to minimize single points of failure.

✨ Features

Decentralized design: Passwords stored on a private server, while master password shares are distributed across multiple cloud providers.

Shamir Secret Sharing (SSS): Splits master password into multiple shares; only a subset of shares is required for reconstruction.

AES-256 Encryption: Ensures secure transmission and storage of sensitive data.

🚧 Limitations

Prototype intended for research and educational purposes only.

Not recommended for production or commercial use.

Focuses on decentralization and SSS, not exhaustive security hardening (e.g., side-channel attacks, memory leaks).

🛠️ Tech Stack

Backend: Node (Javascript)

Frontend: Tauri

Database: Hybrid (Self-hosted + Public Cloud)

Encryption: AES-256, Hashing

Algorithm: Shamir Secret Sharing

📖 Research Goals

Analyze user understanding of password security.

Evaluate the security of a decentralized password manager.

Compare Shamir Secret Sharing to conventional centralized password manager storage.

📌 Disclaimer

This project is a research prototype created for the OPSI 2025 competition and should not be used to secure real-world sensitive data at this early phase.
