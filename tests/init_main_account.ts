import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GradingSystem } from "../target/types/grading_system";
import { SystemProgram, PublicKey } from "@solana/web3.js";

describe("Create Main Account", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.GradingSystem as Program<GradingSystem>;
  const programId = program.programId;

  it("Is initialized!", async () => {
    const mainAccount = PublicKey.findProgramAddressSync(
      [Buffer.from("Main Account", "utf8")],
      programId
    )[0];

    const tx = await program.methods
      .schoolMainAccount()
      .accounts({
        mainAccount: mainAccount,
        user: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("Transaction signature", tx);
  });
});
