import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StorageProgram } from "../target/types/storage_program";
import { Address, address, getProgramDerivedAddress } from "@solana/kit";
import { assert } from "chai";
import { logger } from "./logger";

describe("Storage program tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.storageProgram as Program<StorageProgram>;
  const seed = anchor.utils.bytes.utf8.encode("storage");
  let storagePub: Address<string>;

  before(async () => {
    const [storageAccount, _] = await getProgramDerivedAddress({
      programAddress: address(program.programId.toString()),
      seeds: [seed],
    });
    storagePub = storageAccount;
  });
  it("Is initialized!", async () => {
    const transaction = await program.methods.initialize().rpc();
    console.log("Your transaction signature", transaction);
  });
  it("Sets the key", async () => {
    const key = Buffer.from("exampleKey");
    const value = Buffer.from("exampleValue");

    const transaction = await program.methods
      .setValue(key, value)
      .accounts({ storage: storagePub })
      .rpc();
    logger.info({ transaction });
  });
  it("Gets the value", async () => {
    const storage = await program.account.storage.fetch(storagePub);
    assert.ok(storage.storage.length > 0, "Not modifired the storage");
  });
});
