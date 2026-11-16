# Click Counter Game & Rust Anticheat

A lightweight 32-bit C# Windows Forms clicker game protected by a custom, fast-scanning Rust (`Anticheat.dll`) security library. Developed for educational purposes to demonstrate C#/Rust P/Invoke interoperability, atomic score verification, binary persistence, and basic process/memory integrity checks.

## Features

* **Click Counter & Binary Persistence:** Tracks user clicks and saves progress locally in a binary file (`click_score.bin`).
* **Reset Protection:** Includes an English confirmation dialog and synchronized atomic state resets between the C# frontend and Rust backend.
* **Rust Anticheat Engine (`Anticheat.dll`):**
  * **Atomic Verification:** Validates score increments using atomic variables (`AtomicU32`) to prevent sudden memory-editing spikes.
  * **Fast Process Scanner:** Scans active processes every 400ms for known debugging or memory-manipulation tools while whitelisting essential Windows and antivirus services.
  * **Autonomous Security Alerts & Instant Termination:** Triggers an independent VBScript alert and immediately halts execution via `Environment.FailFast` upon detecting unauthorized tampering.

## Quick Start (Pre-compiled Releases)

If you do not want to compile the source code yourself, you can download the ready-to-run binaries from the **[Releases](../../releases)** page:

1. Download the latest `ClickCounterGame-v1.0.zip` archive.
2. Extract both `Click Counter Game.exe` and `Anticheat.dll` into the **same folder** (keeping them in the same directory is required for P/Invoke to load the DLL).
3. Run `Click Counter Game.exe`.

> **SmartScreen / Antivirus Notice:** Since the release binaries are compiled locally without a paid code-signing certificate, Windows SmartScreen or antivirus software may show an "Unknown Publisher" or false-positive warning. You can inspect the full source code in this repository and build it yourself if you prefer.

## Build from Source

### Prerequisites
* Windows OS (32-bit or 64-bit)
* [.NET Framework](https://dotnet.microsoft.com/) (4.7.2 or higher) & Visual Studio
* [Rust & Cargo](https://www.rust-lang.org/) with the `i686-pc-windows-msvc` target installed

### 1. Build the Rust Anticheat DLL
Navigate to the Rust library directory and compile it as a 32-bit Windows DLL:
```bash
rustup target add i686-pc-windows-msvc
cargo build --target i686-pc-windows-msvc --release

2. Build the C# Game
Open the .sln solution file in Visual Studio.

Ensure the build platform target is set to x86 (32-bit) to match Anticheat.dll.

Build and run the project (F5).

Disclaimer & License
This project is provided strictly for educational and research purposes. The software is provided "AS IS", without warranty of any kind, express or implied. The author is not responsible for any misuse, false-positive antivirus detections, or unintended behavior resulting from running, downloading, or modifying this software.

Distributed under the MIT License. See LICENSE for more information.
