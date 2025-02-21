# 🚀 Quick Start Guide for Track 2: Automated DDC Account Top-Up System

This guide will help you get started with developing the **Automated DDC Account Top-Up System** for the hackathon. Follow the steps below to set up your environment, understand the deliverables, and begin contributing.

---

## **Step 1: Fork the Repository** 🍴

1. Locate the repository provided for this track.
2. Click on the "Fork" button in GitHub to create a copy of the repository under your account.
3. Clone the forked repository to your local machine using:

   ```bash
   git clone https://github.com/your-username/repository-name.git
   cd repository-name
   ```

---

## **Step 2: Install Substrate Dependencies** 🛠️

1. Install the required dependencies for Substrate development by following the official guide:
   - Visit [Polkadot-SDK Installation Guide](https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/).
   - Ensure you have Rust (with `nightly` toolchain), Node.js, Yarn, and other prerequisites installed.

2. Verify installation by running:

   ```bash
   rustup show
   cargo --version
   ```

---

## **Step 3: Build and Test the Project** 🏗️

1. Build the project to ensure all dependencies are correctly installed:

   ```bash
   cargo build --release
   ```

2. Run existing tests to verify the setup:

   ```bash
   cargo test
   ```

---

## **Step 4: Add Mandate Extrinsics in DDC Customer Pallet** 📝

1. Navigate to the **DDC Customer Pallet** directory in your project.
2. Add new extrinsics for mandate creation, updating, and revocation. These extrinsics should handle:
   - User-defined balance thresholds.
   - Maximum top-up amounts.

3. Refer to this tutorial for guidance on adding extrinsics:
   - [Substrate Runtime Development](https://docs.substrate.io/tutorials).

4. Update logic to trigger automatic top-ups when balance falls below thresholds.

---

## **Step 5: Extend DDC Customer Pallet** 🔧

1. Integrate your new extrinsics with the existing DDC Customer Pallet features.
2. Ensure compatibility with current account management functions.

---

## **Step 6: Write and Run Tests** 🧪

1. Write unit tests for all newly added functionality using Substrate's test framework.
2. Add integration tests to verify seamless operation with existing pallet features.
3. Use Polkadot.js to create end-to-end tests simulating real-world scenarios.

4. Refer to this guide for testing:
   - [Substrate Testing Framework](https://docs.polkadot.com/develop/parachains/testing/).

5. Run tests using:

   ```bash
   cargo test
   ```

---

## **Step 7: Deploy and Validate** 🚀

1. Deploy your updated pallet on a local Substrate node or testnet.
2. Validate that:
   - Mandates are created, updated, and revoked successfully.
   - Automated top-ups trigger correctly when thresholds are breached.
3. Record mandates on the CERE Mainnet blockchain as part of final integration.

---

## **Additional Notes** 📌

- Use **Polkadot.js** for interacting with your blockchain during testing and validation.
- Follow best practices for Rust development and Substrate runtime coding.
- Collaborate effectively if working in a team by assigning roles (e.g., Rust Developer, Frontend Developer).

Good luck with your hackathon project! 🍀🚀