import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GradingSystem } from "../target/types/grading_system";
import { SystemProgram, PublicKey } from "@solana/web3.js";

describe("Create School Account", () => {
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

    const school = PublicKey.findProgramAddressSync(
      [new anchor.BN(0).toBuffer("le", 4)],
      programId
    )[0];

    const tx = await program.methods
      .schoolRegister("Cool School", new anchor.BN(10))
      .accounts({
        mainAccount: mainAccount,
        school: school,
        user: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("Transaction signature", tx);
  });
});
