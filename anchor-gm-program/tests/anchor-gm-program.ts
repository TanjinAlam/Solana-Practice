import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorGmProgram } from "../target/types/anchor_gm_program";

describe("anchor-gm-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorGmProgram as Program<AnchorGmProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
