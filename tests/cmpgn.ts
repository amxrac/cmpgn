import * as anchor from "@coral-xyz/anchor";
import { Program, EventParser } from "@coral-xyz/anchor";
import { Cmpgn } from "../target/types/cmpgn";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";
import { MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";

describe("cmpgn", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.cmpgn as Program<Cmpgn>;

  const gameAuthority = provider.wallet;
  const campaignId = 1;
  const bugId = 1;
  const collection = Keypair.generate();
  const unauthorizedUser = Keypair.generate();
  const player = Keypair.generate();
  const asset = Keypair.generate();

  let campaignPda: PublicKey;
  let collectionAuthorityPda: PublicKey;
  let campaignCompletionPda: PublicKey;
  let playerProgressPda: PublicKey;

  before(async () => {
    const unauthorizedUserBalance = await provider.connection.getBalance(
      unauthorizedUser.publicKey
    );
    if (unauthorizedUserBalance < 1_000_000_000) {
      const tx = new anchor.web3.Transaction().add(
        SystemProgram.transfer({
          fromPubkey: gameAuthority.publicKey,
          toPubkey: unauthorizedUser.publicKey,
          lamports: 0.1 * anchor.web3.LAMPORTS_PER_SOL,
        })
      );
      await provider.sendAndConfirm(tx);
    }

    const playerBalance = await provider.connection.getBalance(
      player.publicKey
    );
    if (playerBalance < 1_000_000_000) {
      const tx = new anchor.web3.Transaction().add(
        SystemProgram.transfer({
          fromPubkey: gameAuthority.publicKey,
          toPubkey: player.publicKey,
          lamports: 0.1 * anchor.web3.LAMPORTS_PER_SOL,
        })
      );
      await provider.sendAndConfirm(tx);
    }

    campaignPda = PublicKey.findProgramAddressSync(
      [Buffer.from("campaign"), Buffer.from([campaignId])],
      program.programId
    )[0];

    collectionAuthorityPda = PublicKey.findProgramAddressSync(
      [Buffer.from("collection"), collection.publicKey.toBuffer()],
      program.programId
    )[0];

    campaignCompletionPda = PublicKey.findProgramAddressSync(
      [
        Buffer.from("completion"),
        Buffer.from([campaignId]),
        player.publicKey.toBuffer(),
        Buffer.from([bugId]),
      ],
      program.programId
    )[0];

    playerProgressPda = PublicKey.findProgramAddressSync(
      [
        Buffer.from("progress"),
        Buffer.from([campaignId]),
        player.publicKey.toBuffer(),
      ],
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
            systemProgram: SystemProgram.programId,
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

      const campaignAccount = await program.account.campaign.fetch(campaignPda);
      expect(campaignAccount.gameAuthority.toString()).to.equal(
        gameAuthority.publicKey.toString()
      );
    });
  });

  describe("Create Collection", () => {
    it("creates a collection", async () => {
      const args = {
        name: "Test Collection",
        uri: "https://devnet.irys.xyz/yourhashhere",
        nftName: "Test NFT",
        nftUri: "https://gateway.irys.xyz/yourhashhere",
      };
      try {
        const sig = await program.methods
          .createCollection(campaignId, args)
          .accounts({
            creator: gameAuthority.publicKey,
            collection: collection.publicKey,
            collectionAuthority: collectionAuthorityPda,
            campaign: campaignPda,
            coreProgram: MPL_CORE_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([collection])
          .rpc();
      } catch (error: any) {
        console.error(`something went wrong: ${error}`);
        if (error.logs && Array.isArray(error.logs)) {
          console.log("Transaction Logs:");
          error.logs.forEach((log: string) => console.log(log));
        } else {
          console.log("No logs available in the error.");
        }
        throw error;
      }

      const collectionAuthority =
        await program.account.collectionAuthority.fetch(collectionAuthorityPda);
      expect(collectionAuthority.creator.toString()).to.equal(
        gameAuthority.publicKey.toString()
      );
      expect(collectionAuthority.collection.toString()).to.equal(
        collection.publicKey.toString()
      );
      expect(collectionAuthority.nftName).to.equal(args.nftName);
      expect(collectionAuthority.nftUri).to.equal(args.nftUri);
    });

    it("fails to create a collection with an unauthorized signer", async () => {
      const newCollection = Keypair.generate();

      let newCollectionAuthorityPda = PublicKey.findProgramAddressSync(
        [Buffer.from("collection"), newCollection.publicKey.toBuffer()],
        program.programId
      )[0];

      const args = {
        name: "Test Collection",
        uri: "https://devnet.irys.xyz/yourhashhere",
        nftName: "Test NFT",
        nftUri: "https://gateway.irys.xyz/yourhashhere",
      };
      try {
        const sig = await program.methods
          .createCollection(campaignId, args)
          .accounts({
            creator: unauthorizedUser.publicKey,
            collection: newCollection.publicKey,
            collectionAuthority: newCollectionAuthorityPda,
            campaign: campaignPda,
            coreProgram: MPL_CORE_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([newCollection, unauthorizedUser])
          .rpc();
        expect.fail("Should fail with unauthorized creator");
      } catch (error: any) {
        expect(error.error.errorCode.code).to.equal("NotAuthorized");
      }
    });
  });

  describe("Start Campaign", () => {
    it("starts a campaign with a valid campaign campaign id and bug id", async () => {
      try {
        const sig = await program.methods
          .startCampaign(campaignId, bugId)
          .accounts({
            player: player.publicKey,
            campaignCompletion: campaignCompletionPda,
            campaign: campaignPda,
            systemProgram: SystemProgram.programId,
          })
          .signers([player])
          .rpc();
      } catch (error: any) {
        console.error(`something went wrong: ${error}`);
        if (error.logs && Array.isArray(error.logs)) {
          console.log("Transaction Logs:");
          error.logs.forEach((log: string) => console.log(log));
        } else {
          console.log("No logs available in the error.");
        }
      }

      const campaignCompletion = await program.account.campaignCompletion.fetch(
        campaignCompletionPda
      );
      const campaign = await program.account.campaign.fetch(campaignPda);
      const now = Math.floor(Date.now() / 1000);

      expect(campaignCompletion.bugId).to.equal(bugId);
      expect(campaignCompletion.campaignId).to.equal(campaignId);
      expect(campaignCompletion.campaignId).to.equal(campaign.campaignId);
      expect(campaignCompletion.campaignStart.toNumber()).to.be.closeTo(
        now,
        30
      );
      expect(campaignCompletion.campaignEnd).to.be.null;
      expect(campaignCompletion.timestamp).to.be.null;
      expect(campaignCompletion.nftMintAddress).to.be.null;
    });

    it("fails to start a campaign with an invalid campaign id", async () => {
      const invalidCampaignId = 6;
      const invalidCampaignPda = PublicKey.findProgramAddressSync(
        [Buffer.from("campaign"), Buffer.from([invalidCampaignId])],
        program.programId
      )[0];
      const invalidCampaignCompletionPda = PublicKey.findProgramAddressSync(
        [
          Buffer.from("completion"),
          Buffer.from([invalidCampaignId]),
          player.publicKey.toBuffer(),
          Buffer.from([bugId]),
        ],
        program.programId
      )[0];

      try {
        const sig = await program.methods
          .startCampaign(invalidCampaignId, bugId)
          .accounts({
            player: player.publicKey,
            campaignCompletion: invalidCampaignCompletionPda,
            campaign: invalidCampaignPda,
            systemProgram: SystemProgram.programId,
          })
          .signers([player])
          .rpc();
        expect.fail("Should fail with uninitialized campaign");
      } catch (error: any) {
        const errorCode = error.error?.errorCode?.code || error.message;
        expect(errorCode).to.satisfy(
          (code: string) =>
            code.includes("AccountNotInitialized") ||
            code.includes("Account does not exist")
        );
      }
    });

    it("fails to starts a campaign with an invalid bug id", async () => {
      const InvalidBugId = 200;

      const invalidCampaignCompletionPda = PublicKey.findProgramAddressSync(
        [
          Buffer.from("completion"),
          Buffer.from([campaignId]),
          player.publicKey.toBuffer(),
          Buffer.from([InvalidBugId]),
        ],
        program.programId
      )[0];
      try {
        const sig = await program.methods
          .startCampaign(campaignId, InvalidBugId)
          .accounts({
            player: player.publicKey,
            campaignCompletion: invalidCampaignCompletionPda,
            campaign: campaignPda,
            systemProgram: SystemProgram.programId,
          })
          .signers([player])
          .rpc();
        expect.fail("Should fail with invalid bug id");
      } catch (error: any) {
        expect(error.error.errorCode.code).to.equal("InvalidBugId");
      }
    });
  });

  describe("Record Campaign Completion", () => {
    it("records campaign with valid campaign id and bug id", async () => {
      let existingCompletion;
      try {
        existingCompletion = await program.account.campaignCompletion.fetch(
          campaignCompletionPda
        );
      } catch {
        existingCompletion = null;
      }

      if (existingCompletion?.campaignEnd) {
        console.log("Campaign already completed.");
        return;
      }
      try {
        const sig = await program.methods
          .recordCampaignCompletion(campaignId, bugId)
          .accounts({
            player: player.publicKey,
            campaignCompletion: campaignCompletionPda,
            playerProgress: playerProgressPda,
            campaign: campaignPda,
            systemProgram: SystemProgram.programId,
          })
          .signers([player])
          .rpc();
      } catch (error: any) {
        console.error(`something went wrong: ${error}`);
        if (error.logs && Array.isArray(error.logs)) {
          console.log("Transaction Logs:");
          error.logs.forEach((log: string) => console.log(log));
        } else {
          console.log("No logs available in the error.");
        }
      }

      const campaignCompletion = await program.account.campaignCompletion.fetch(
        campaignCompletionPda
      );
      const campaign = await program.account.campaign.fetch(campaignPda);
      const playerProgress = await program.account.playerProgress.fetch(
        playerProgressPda
      );
      const now = Math.floor(Date.now() / 1000);

      expect(campaignCompletion.bugId).to.equal(bugId);
      expect(campaignCompletion.campaignId).to.equal(campaignId);
      expect(campaignCompletion.campaignId).to.equal(campaign.campaignId);
      expect(campaignCompletion.campaignEnd.toNumber()).to.be.closeTo(now, 60);
      expect(campaignCompletion.timestamp.toNumber()).to.be.closeTo(now, 60);
      expect(campaignCompletion.nftMintAddress).to.be.null;
      expect(playerProgress.player.toString()).to.equal(
        player.publicKey.toString()
      );
      expect(playerProgress.campaignId).to.equal(campaignId);
    });

    it("fails to record a campaign with an invalid campaign id", async () => {
      const invalidCampaignId = 200;

      const invalidCampaignPda = PublicKey.findProgramAddressSync(
        [Buffer.from("campaign"), Buffer.from([invalidCampaignId])],
        program.programId
      )[0];

      const invalidCampaignCompletionPda = PublicKey.findProgramAddressSync(
        [
          Buffer.from("completion"),
          Buffer.from([invalidCampaignId]),
          player.publicKey.toBuffer(),
          Buffer.from([bugId]),
        ],
        program.programId
      )[0];

      const invalidPlayerProgressPda = PublicKey.findProgramAddressSync(
        [
          Buffer.from("progress"),
          Buffer.from([invalidCampaignId]),
          player.publicKey.toBuffer(),
        ],
        program.programId
      )[0];

      try {
        const sig = await program.methods
          .recordCampaignCompletion(invalidCampaignId, bugId)
          .accounts({
            player: player.publicKey,
            campaignCompletion: invalidCampaignCompletionPda,
            playerProgress: invalidPlayerProgressPda,
            campaign: invalidCampaignPda,
            systemProgram: SystemProgram.programId,
          })
          .signers([player])
          .rpc();

        expect.fail("Should fail with uninitialized campaign");
      } catch (error: any) {
        expect(error.error.errorCode.code).to.be.oneOf([
          "AccountNotInitialized",
          "ConstraintSeeds",
        ]);
      }
    });
  });

  describe("Mint NFT", () => {
    it("mints an NFT", async () => {
      const nftName = "test";
      const nftUri = "https://gateway.irys.xyz/yourhashhere";
      try {
        const sig = await program.methods
          .mintNft(campaignId, bugId, nftName, nftUri)
          .accounts({
            player: player.publicKey,
            asset: asset.publicKey,
            collection: collection.publicKey,
            collectionAuthority: collectionAuthorityPda,
            campaignCompletion: campaignCompletionPda,
            coreProgram: MPL_CORE_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([player, asset])
          .rpc();
      } catch (error: any) {
        console.error(`something went wrong: ${error}`);
        if (error.logs && Array.isArray(error.logs)) {
          console.log("Transaction Logs:");
          error.logs.forEach((log: string) => console.log(log));
        } else {
          console.log("No logs available in the error.");
        }
        throw error;
      }
      const collectionAuthority =
        await program.account.collectionAuthority.fetch(collectionAuthorityPda);
      const campaignCompletion = await program.account.campaignCompletion.fetch(
        campaignCompletionPda
      );

      expect(collectionAuthority.creator.toString()).to.equal(
        gameAuthority.publicKey.toString()
      );
      expect(campaignCompletion.nftMintAddress.toString()).to.equal(
        asset.publicKey.toString()
      );
    });

    it("fails to mint an NFT with an invalid collection", async () => {
      const invalidCollection = Keypair.generate();
      const invalidAsset = Keypair.generate();
      const nftName = "test";
      const nftUri = "https://gateway.irys.xyz/yourhashhere";

      try {
        const sig = await program.methods
          .mintNft(campaignId, bugId, nftName, nftUri)
          .accounts({
            player: player.publicKey,
            asset: invalidAsset.publicKey,
            collection: invalidCollection.publicKey,
            collectionAuthority: collectionAuthorityPda,
            campaignCompletion: campaignCompletionPda,
            coreProgram: MPL_CORE_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([player, invalidAsset])
          .rpc();

        expect.fail("Should fail with invalid collection");
      } catch (error: any) {
        expect(error.error.errorCode.code).to.equal("InvalidCollection");
      }
    });
  });

  describe("Get current player progress", () => {
    it("gets the current player progress", async () => {
      try {
        const sig = await program.methods
          .getPlayerProgress(campaignId)
          .accounts({
            player: player.publicKey,
            playerProgress: playerProgressPda,
          })
          .signers([player])
          .rpc({ commitment: "confirmed" });

        const tx = await provider.connection.getTransaction(sig, {
          commitment: "confirmed",
          maxSupportedTransactionVersion: 0,
        });

        expect(tx?.meta?.logMessages).to.exist;

        const eventParser = new EventParser(program.programId, program.coder);
        const events = [];
        for (const event of eventParser.parseLogs(tx.meta.logMessages)) {
          events.push(event);
        }

        expect(events).to.have.lengthOf.at.least(1);
        expect(events[0].name).to.equal("playerProgressEvent");
        expect(events[0].data.player.toString()).to.equal(
          player.publicKey.toString()
        );
        expect(events[0].data.campaignId).to.equal(campaignId);
      } catch (error: any) {
        console.error(`something went wrong: ${error}`);
        if (error.logs && Array.isArray(error.logs)) {
          console.log("Transaction Logs:");
          error.logs.forEach((log: string) => console.log(log));
        } else {
          console.log("No logs available in the error.");
        }
      }
    });
  });
});
