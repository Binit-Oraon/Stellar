# 🎓 NFT Certificates on Soroban (Stellar)

## 📌 Project Description

This project implements a basic **NFT Certificate system** using **Soroban**, the smart contract platform on the Stellar network.

The contract allows authorized issuers to create unique, non-fungible digital certificates that are permanently stored on-chain. These certificates can represent achievements such as academic degrees, course completions, or professional credentials.

The system is designed to be simple, verifiable, and easily extendable for real-world use cases.

---

## 🚀 What It Does

The smart contract enables the creation and management of NFT-based certificates.

### Core functionality:

- Mint unique certificates tied to a recipient’s wallet address  
- Store certificate details directly on-chain  
- Assign a unique ID to each certificate  
- Allow retrieval of certificate data using its ID  

Each certificate contains:

- Certificate ID  
- Issuer address  
- Recipient address  
- Title (e.g., "Blockchain Fundamentals")  
- Metadata (description, link, or additional info)  

This ensures that certificates are:

- Tamper-proof  
- Publicly verifiable  
- Permanently accessible  

---

## ✨ Features

- 🎓 **NFT-style Certificates**  
  Each certificate is unique and non-fungible  

- 🔐 **Issuer Authorization**  
  Only the transaction invoker (issuer) can mint certificates  

- 🆔 **Auto-Incremented IDs**  
  Every certificate gets a unique identifier  

- 👤 **Ownership Model**  
  Certificates are assigned to recipient addresses  

- 🧾 **On-Chain Metadata**  
  Certificate details are stored securely on-chain  

- ⚡ **Lightweight Design**  
  Built using Rust with `no_std` for efficiency  

- 🧩 **Extensible Architecture**  
  Easy to expand with advanced NFT features  

---

## 🛠 Tech Stack

- Rust (`no_std`)  
- Soroban SDK  
- Stellar Network  

---

## 🔮 Future Improvements

- Certificate verification function  
- Revocation mechanism  
- Transferable NFT support  
- Metadata stored via IPFS  
- Event emission for indexing  
- Role-based issuer permissions  
- Frontend UI for minting and viewing certificates  

---

## 📜 License

MIT License