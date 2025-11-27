import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Cmpgn } from "../target/types/cmpgn";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

describe("cmpgn", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.cmpgn as Program<Cmpgn>;

  const gameAuthority = provider.wallet;
  const campaignId = 1;

  let campaignPda: PublicKey;

  before(async () => {
    campaignPda = PublicKey.findProgramAddressSync(
      [Buffer.from("campaign"), Buffer.from([campaignId])],
      program.programId
    )[0];
  });

  describe("Initialize Campaign", () => {
    it("initializes a campaign", async () => {
      try {
        const sig = await program.methods
          .initialize(campaignId)
          .accounts({
            gameAuthority: gameAuthority.publicKey,
            campaign: campaignPda,
            SystemProgram: SystemProgram.programId,
          })
          .rpc();
      } catch (error: any) {
        console.error(`something went wrong: ${error}`);
        if (error.logs && Array.isArray(error.logs)) {
          console.log("Transaction Logs:");
          error.logs.forEach((log: string) => console.log(log));
        } else {
          console.log("No logs available in the error .");
        }
      }
    });
  });
});
