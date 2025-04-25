import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Degree } from "../target/types/degree";
import { Keypair, PublicKey} from "@solana/web3.js";
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
  let diplomaPDA: PublicKey;
  let diplomaBump: number;

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
      const diplomaRegistryAccount = await program.account.diplomaRegistry.fetch(diplomaRegistryPDA);
      expect(diplomaRegistryAccount.authority.toString()).to.equal(walletPublicKey.toString());
      expect(diplomaRegistryAccount.count.toNumber()).to.equal(0);
    })

  
  it("add a diploma", async () => {
    // Add a diploma to the registry
    const tx = await program.methods
      .addDiploma(testDiplomaId)
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
    expect(diplomaAccount.authority.toString()).to.equal(walletPublicKey.toString());

    // Verify the diploma registry count was incremented
    const diplomaRegistryAccount = await program.account.diplomaRegistry.fetch(diplomaRegistryPDA);
    expect(diplomaRegistryAccount.count.toNumber()).to.equal(1);
  });
});
