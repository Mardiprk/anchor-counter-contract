import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";

describe("anchor-contract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;
  const counterKeypair = anchor.web3.Keypair.generate();

  it("Initialize", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        counter: counterKeypair.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counterKeypair])
      .rpc();

    console.log("Initialize transaction signature:", tx);
  });

  it("Increment", async () => {
    await program.methods
      .increment()
      .accounts({
        counter: counterKeypair.publicKey,
      })
      .rpc();
  });

  it("Increment", async () => {
    await program.methods
      .decrement()
      .accounts({
        counter: counterKeypair.publicKey,
      })
      .rpc();
  });

  it("Reset Counter", async () => {
    try {
      await program.methods
        .reset()
        .accounts({
          counter: counterKeypair.publicKey,
          authority: provider.wallet.publicKey,
        })
        .rpc();
    } catch (e) {
      console.log("Cannot decrement counter below zero: ", e);
    }
  });
});
