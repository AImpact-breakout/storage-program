import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Mappings } from "../target/types/mappings";
import { assert } from "chai";

describe("mappings", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.mappings as Program<Mappings>;
  const key = new anchor.BN(42);
  const domain = new anchor.BN(777);
  const value = new anchor.BN(100);

  const seeds = [domain.toArrayLike(Buffer, "le", 8), key.toArrayLike(Buffer, "le", 8)];

  const valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
    seeds,
    program.programId
  )[0];

  it("Initialize mapping storage", async () => {
    await program.methods.initialize(domain, key).accounts(valueAccount).rpc();
  });

  it("Should set value", async () => {
    await program.methods.set(domain, key, value).accounts({val: valueAccount}).rpc();
  });

  it("Should get value (direct memory access)", async () => {
    const retrievedValue = (await program.account.val.fetch(valueAccount)).value;
    assert.equal(retrievedValue.toString(), value.toString());

  });

   it("Should get value (via program method)", async () => {
    const retrievedValue = await program.methods.get(domain, key).accounts({val: valueAccount}).view();
    assert.equal(retrievedValue.toString(), value.toString());
  });
});
