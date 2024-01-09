import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GradingSystem } from "../target/types/grading_system";
import { SystemProgram, PublicKey } from "@solana/web3.js";

describe("Set notes", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.GradingSystem as Program<GradingSystem>;
  const programId = program.programId;

  it("Notes set!", async () => {
    const mainAccount = PublicKey.findProgramAddressSync(
      [Buffer.from("Main Account", "utf8")],
      programId
    )[0];

    const school = PublicKey.findProgramAddressSync(
      [new anchor.BN(0).toBuffer("le", 4)],
      programId
    )[0];

    const student = PublicKey.findProgramAddressSync(
      [new anchor.BN(10).toBuffer("be", 8)],
      programId
    )[0];

    const grades = PublicKey.findProgramAddressSync(
      [new anchor.BN(10).toBuffer("be", 8), new anchor.BN(1).toBuffer("be", 8)],
      programId
    )[0];

    const tx = await program.methods
      .schoolNotes(
        new anchor.BN(7),
        new anchor.BN(8),
        new anchor.BN(9),
        new anchor.BN(8),
        new anchor.BN(7),
        new anchor.BN(8)
      )
      .accounts({
        mainAccount: mainAccount,
        school: school,
        student: student,
        grades: grades,
        user: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("Transaction signature", tx);
  });
});
