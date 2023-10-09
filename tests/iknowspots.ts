import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Iknowspots } from "../target/types/iknowspots";

describe("iknowspots", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Iknowspots as Program<Iknowspots>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
