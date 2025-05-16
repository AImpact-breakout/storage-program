# Solana Key-Value Storage Program

A simple, efficient key-value storage solution built on Solana blockchain using the Anchor framework.

## Overview

This program provides a persistent key-value storage mechanism on Solana with the following features:

- Initialize storage accounts using Program Derived Addresses (PDAs)
- Store arbitrary data as key-value pairs (both keys and values are byte arrays)
- Update existing values
- Retrieve stored values by key

## Getting Started

### Prerequisites

- Solana CLI (latest version)
- Anchor Framework
- Node.js and npm

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd storage-program
   ```

2. Install dependencies:
   ```bash
   yarn
   ```

### Building

Build the Solana program:
```bash
anchor build
```

### Testing

Run tests:
```bash
anchor test
```

## Usage

### Program Instructions

The program provides the following instructions:

1. **Initialize**
   ```typescript
   // Create a new storage account
   await program.methods.initialize().rpc();
   ```

2. **Set Value**
   ```typescript
   // Set or update a key-value pair
   const key = Buffer.from("exampleKey");
   const value = Buffer.from("exampleValue");
   
   await program.methods
     .setValue(key, value)
     .accounts({ storage: storagePDA })
     .rpc();
   ```

3. **Get Value**
   ```typescript
   // Fetch stored data
   const storage = await program.account.storage.fetch(storagePDA);
   ```

### Creating a Storage PDA

```typescript
// Generate a storage PDA (Program Derived Address)
const seed = anchor.utils.bytes.utf8.encode("storage");
const [storageAccount, _] = await getProgramDerivedAddress({
  programAddress: address(program.programId.toString()),
  seeds: [seed],
});
```

## Program Structure

- **Storage Account**: Maintains a vector of key-value entries
- **Storage Entry**: Represents a single key-value pair

## Deployment

To deploy to devnet:
```bash
anchor deploy --provider.cluster devnet
```
