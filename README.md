# Sorority 💜

### **Interactive 3D Trust Network for Women's Safety**
*A high-performance bridge between Rust and Flutter designed to visualize and secure collective protection.*

---

## 🌟 The Vision
**Sorority** is more than a simulation; it is a blueprint for a decentralized safety net. 

My vision is to empower women's associations by providing them with a tool they can host on **local servers**. This allows for the creation of a vast, private "Network of Trust." Even if two women don't know each other personally, being part of the same verified network ensures that a cry for help is heard and acted upon by those nearby. 

By decentralizing the infrastructure, we ensure that the data—and the safety of our community—remains in our hands.

---

## 🛠️ Technical Architecture
To handle sensitive safety data, I chose a stack that prioritizes speed and cryptographic integrity:

* **Rust (The Engine):** Simulates the network and calculates alert propagation logic. Rust’s memory safety ensures that the core is robust and efficient.
* **Flutter (The Interface):** Renders a custom 3D projection and smooth animations, providing a human-centric UI for a complex spherical node distribution.
* **Dart FFI:** Seamlessly bridges both languages, allowing Flutter to call high-performance Rust functions with near-zero latency.

---

## 📽️ Demo

<div align="center">
  <video src="assets/demo.webm" width="800" controls autoplay loop muted>
    Tu navegador no soporta el formato de video.
  </video>
  <p><em>Visualización de la propagación de alertas en la Red de Confianza 3D.</em></p>
</div>

> **Note:** The 3D sphere represents the community. When a node turns red, it’s not just data; it’s a real-time alert rippling through a support system.

---

## 🚀 How It Works
1. **Network Simulation:** Rust calculates the optimal alert propagation path through the trust network.
2. **FFI Bridge:** Flutter calls Rust functions via FFI to retrieve real-time propagation steps.
3. **3D Animation:** Flutter renders the nodes in a 3D sphere and animates the alert path.
4. **Visual Feedback:** Nodes glow and scale dynamically; lines animate to show the "ripple effect" of the help request.
5. **Completion:** A secure notification confirms the alert has been successfully distributed across the network.

---

## 📦 Getting Started

### Prerequisites
* **Flutter SDK** (Stable channel)
* **Rust** (Cargo & Rustc)
* **Platform:** Currently optimized for **Linux** (Fedora/Ubuntu).

### Setup
1. **Clone the repository**
   ```bash
   git clone [https://github.com/](https://github.com/)<your-username>/Sorority.git
   cd Sororit
    ```
    Build Rust Library
    ```Bash

    cd q_verified_core
    cargo build --release
    ```
    The compiled library will be at target/release/libq_verified_core.so.

    Flutter Setup
    ```Bash

    cd ../alert_sim_app/alert_app
    flutter pub get
    ```
    Run the App
    ```Bash

    flutter run -d linux
    ```
    Ensure the libq_verified_core.so is in the path defined in rust_bridge.dart.

📂 Project Structure
```Plaintext

Sorority/
├─ alert_sim_app/      # Flutter UI + animations
│  ├─ lib/
│  │  ├─ screens/home_screen.dart
│  │  ├─ native/rust_bridge.dart
│  │  └─ native/simulacion.dart
│  └─ pubspec.yaml
├─ q_verified_core/    # Rust backend simulation
│  ├─ src/
│  │  ├─ lib.rs        # FFI Exports
│  │  ├─ simulation.rs # Core logic
│  └─ Cargo.toml
└─ README.md
```
🔮 Future Improvements

    [ ] Multi-platform: Add support for Windows, macOS, and Android (ARM64).

    [ ] Real Communication: Implement P2P communication between multiple devices.

    [ ] Advanced 3D: Add perspective depth, rotation gestures, and dynamic node adjustments.

    [ ] NGO Deployment: Create easy-to-use Docker guides for local server hosting.

🤝 Contributing

Contributions are welcome! If you want to help make this network stronger:

    Fork the repo.

    Create a feature branch (git checkout -b feature-name).

    Commit your changes (git commit -m 'Add new feature').

    Push to the branch (git push origin feature-name).

    Open a Pull Request.

📄 License

MIT License – Created with the belief that technology should be a tool for collective liberation and safety.






