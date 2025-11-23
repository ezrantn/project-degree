# A Blockchain and IPFS-Based Smart Contract Framework for Preventing Certificate Fraud in Academia

This repository contains the source code for a decentralized application (dApp) built on the Solana blockchain. It provides a secure and verifiable framework for issuing, managing, and verifying academic diplomas, leveraging the InterPlanetary File System (IPFS) for decentralized storage. This project serves as the practical implementation for the thesis titled, **"A Blockchain and InterPlanetary File System (IPFS) Based Smart Contract Framework for Preventing Certificate Fraud in Academia."**

This work was formally presented at the 2025 International Conference on Information Technology, Computer, and Electrical Engineering (ICITACEE), hosted by Diponegoro University on August 28, 2025.

## Abstract

This research presents a decentralized platform for academic credential verification using blockchain technology. The system allows students to store and share their school diplomas in a secure, tamper evident format. By leveraging smart contracts, diploma issuance and verification are automated, enabling educational qualifications to be independently verified without manual intervention. The proposed system significantly reduces verification time, requiring only 400 milliseconds for an individual diploma and less than 2 seconds for a complete document. These performance metrics demonstrate the system’s efficiency in real world use. Employers can validate job applicants credentials directly through the blockchain, reducing administrative workload and improving trust in academic records. The solution benefits students, educational institutions, and employers alike by ensuring the authenticity and integrity of issued diplomas through a decentralized and transparent infrastructure.

## Publication Status

The paper has been accepted and is currently undergoing the publication process. It will be available on IEEE Xplore soon.

- Status: Accepted
- Publisher: IEEE
- DOI: https://doi.org/10.1109/ICITACEE66165.2025.11233026

You can find the presentation slides in the `materials` directory.

## How It's Built

This project is a Solana smart contract developed using the [Anchor](https://www.anchor-lang.com/docs) framework in Rust. This combination provides a secure and high-performance foundation for the on-chain diploma registry. For off-chain data, it uses the InterPlanetary File System (IPFS). The testing and client-side interactions are handled with TypeScript and Node.js.

## Getting Started

Follow these instructions to set up the project on your local machine for development and testing.

**Prerequisites:**

- Node.js (v16 or higher)
- Rust and Cargo
- Solana Tool Suite
- Anchor Framework (`avm install latest`, `avm use latest`)

**Installation & Setup**

1. Clone the repository:

   ```bash
   git clone https://github.com/ezrantn/project-degree
   ```

2. Install Node.js dependencies:

   ```bash
   npm install
   ```

3. Build the Anchor program:

   ```bash
   anchor build
   ```

4. Run the tests:

   This command will start a local Solana validator and run the test suite to ensure everything is configured correctly.

   ```bash
   anchor test
   ```

## Troubleshooting

> [!CAUTION]
> Error: Unable to read keypair file

If you encounter this error, it means the Solana CLI cannot find a default keypair file in your system's configuration directory.

You can generate a new keypair to resolve this issue by running the following command in your terminal:

```bash
solana-keygen new
```
