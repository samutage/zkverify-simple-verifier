# zkVerify Simple Verifier

[![Rust Tests](https://github.com/samutage/zkverify-simple-verifier/actions/workflows/rust.yml/badge.svg)](https://github.com/samutage/zkverify-simple-verifier/actions)

This repository contains a **simple Substrate pallet** for zkVerify integration.  
It demonstrates how a verifier pallet could work in a dApp environment.

---

## 📌 Features
- Basic pallet with a `verify_proof` call.
- Fake verification (always accepts proof ✅).
- Unit tests included (`src/tests.rs`).
- GitHub Actions CI pipeline to run tests automatically.

---

## 🛠 How to Run Tests Locally
If you have Rust installed:
```bash
cargo test
