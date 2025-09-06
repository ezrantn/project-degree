import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Degree } from "../target/types/degree";
import { PublicKey, Transaction } from "@solana/web3.js";
import { expect } from "chai";

describe("degree", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Degree as Program<Degree>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const walletPublicKey = provider.wallet.publicKey;

  let diplomaRegistryPDA: PublicKey;
  let diplomaRegistryBump: number;

  const testDiplomaId = "TEST-DIPLOMA-12345";
  const testIpfsHash = "QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco"; 
  let diplomaPDA: PublicKey;
  let diplomaBump: number;

  // For gas measurement, we will use a different diploma ID
  const gasDiplomaId = "GAS-TEST-DIPLOMA-12345";
  let gasDiplomaPDA: PublicKey;

  before(async () => {
    // Find the PDA for the diploma registry
    const [registryPDA, registryBump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("diploma-registry")],
      program.programId
    );

    diplomaRegistryPDA = registryPDA;
    diplomaRegistryBump = registryBump;

    // Find the PDA for the test diploma
    const [dipPDA, dipBump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("diploma"), Buffer.from(testDiplomaId)],
      program.programId
    );

    diplomaPDA = dipPDA;
    diplomaBump = dipBump;
  });

  it("initialize diploma registry", async () => {
    // Initialize the diploma registry
    const tx = await program.methods
      .initialize()
      .accounts({
        diplomaRegistry: diplomaRegistryPDA,
        authority: walletPublicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Transaction signature:", tx);

    // Verify the diploma registry was created correctly
    const diplomaRegistryAccount = await program.account.diplomaRegistry.fetch(
      diplomaRegistryPDA
    );
    expect(diplomaRegistryAccount.authority.toString()).to.equal(
      walletPublicKey.toString()
    );
    expect(diplomaRegistryAccount.count.toNumber()).to.equal(0);
  });

  it("add a diploma", async () => {
    // Add a diploma to the registry
    const tx = await program.methods
      .addDiploma(testDiplomaId, testIpfsHash)
      .accounts({
        diplomaRegistry: diplomaRegistryPDA,
        diploma: diplomaPDA,
        authority: walletPublicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Add a diploma transaction signature:", tx);

    // Verify the diploma was added correctly
    const diplomaAccount = await program.account.diploma.fetch(diplomaPDA);
    expect(diplomaAccount.diplomaId).to.equal(testDiplomaId);
    expect(diplomaAccount.verified).to.equal(true);
    expect(diplomaAccount.authority.toString()).to.equal(
      walletPublicKey.toString()
    );

    // Verify the diploma registry count was incremented
    const diplomaRegistryAccount = await program.account.diplomaRegistry.fetch(
      diplomaRegistryPDA
    );
    expect(diplomaRegistryAccount.count.toNumber()).to.equal(1);
  });

  it("verify a diploma exists", async () => {
    // This test shows how to verify a diploma exists on-chain
    try {
      const diplomaAccount = await program.account.diploma.fetch(diplomaPDA);
      expect(diplomaAccount.diplomaId).to.equal(testDiplomaId);
      expect(diplomaAccount.verified).to.equal(true);
      console.log("Diploma is valid!");
    } catch (error) {
      console.error("Diploma does not exist!");
      throw error;
    }
  });

  it("revoke a diploma", async () => {
    // Revoke the diploma
    const tx = await program.methods
      .revokeDiploma()
      .accounts({
        diplomaRegistry: diplomaRegistryPDA,
        diploma: diplomaPDA,
        authority: walletPublicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Revoke diploma transaction signature:", tx);

    // Verify the diploma was revoked correctly
    const diplomaAccount = await program.account.diploma.fetch(diplomaPDA);
    expect(diplomaAccount.verified).to.equal(false);

    // Verify the diploma registry count was decremented
    const diplomaRegistryAccount = await program.account.diplomaRegistry.fetch(
      diplomaRegistryPDA
    );
    expect(diplomaRegistryAccount.count.toNumber()).to.equal(0);
  });

  it("verify a revoked diploma is invalid", async () => {
    // This test shows how to check if a diploma is still valid
    const diplomaAccount = await program.account.diploma.fetch(diplomaPDA);
    expect(diplomaAccount.verified).to.equal(false);
    console.log("Diploma exists but is revoked!");
  });

  it("try to verify a non-existent diploma", async () => {
    // This test demonstrates how to handle non-existent diplomas
    const fakeDiplomaId = "FAKE-DIPLOMA-12345";
    const [fakeDiplomaPDA] = await PublicKey.findProgramAddressSync(
      [Buffer.from("diploma"), Buffer.from(fakeDiplomaId)],
      program.programId
    );

    try {
      await program.account.diploma.fetch(fakeDiplomaPDA);
      // If we get here, the diploma exists (which it shouldn't)
      expect.fail("Should not find a non-existent diploma");
    } catch (error) {
      // Expected behavior - account not found
      console.log("Correctly identified non-existent diploma");
    }
  });
});
