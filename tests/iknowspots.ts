import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Iknowspots } from "../target/types/iknowspots";
import { BN } from "bn.js";

describe("iknowspots", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Iknowspots as Program<Iknowspots>;


  const provider = anchor.AnchorProvider.local();
  // Get the wallet instance
  const wallet = provider.wallet as anchor.Wallet;


  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is event created!", async () => {
    // Add your test here.
      let event_id = 1;
      let price = 10000000;
      let supply = 100;
      let date = 4348374;
      const kpid = anchor.web3.Keypair.generate();
      let [eventAccount, eventAccountBumb] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("event-data"), new BN(event_id).toArrayLike(Buffer,"le",8)],
        program.programId
      );
    const tx = await program.methods.eventCreation(
      new anchor.BN(event_id),
      new anchor.BN(price),
      new anchor.BN(supply),
      new anchor.BN(date)
      ).accounts(
      {
        authority : wallet.publicKey,
        eventAccount : eventAccount,
        tokenMint : kpid.publicKey
      }
    ).rpc();
    console.log("Your transaction signature", tx);

    // Fetch the escrow account data
    let eventData = await program.account.eventAccount.fetch(eventAccount);
    console.log(eventData);

  });

});
