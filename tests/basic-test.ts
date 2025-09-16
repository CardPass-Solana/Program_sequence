import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { JobPlatform } from "../target/types/job_platform";
import { expect } from "chai";

describe("Job Platform Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.JobPlatform as Program<JobPlatform>;

  let profileOwner: anchor.web3.Keypair;
  let recruiter: anchor.web3.Keypair;

  before(async () => {
    // Generate test keypairs
    profileOwner = anchor.web3.Keypair.generate();
    recruiter = anchor.web3.Keypair.generate();

    // Airdrop SOL for testing
    const airdropSig1 = await provider.connection.requestAirdrop(
      profileOwner.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    const airdropSig2 = await provider.connection.requestAirdrop(
      recruiter.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );

    // Wait for airdrops to confirm
    await provider.connection.confirmTransaction(airdropSig1);
    await provider.connection.confirmTransaction(airdropSig2);
  });

  describe("Profile Management", () => {
    let profilePda: anchor.web3.PublicKey;

    it("Creates a profile", async () => {
      [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("profile"), profileOwner.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .createProfile(
          ["Rust", "Solana", "TypeScript"],
          3,
          "Seoul",
          "Experienced blockchain developer"
        )
        .accounts({
          profile: profilePda,
          owner: profileOwner.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([profileOwner])
        .rpc();

      const profile = await program.account.profile.fetch(profilePda);
      expect(profile.owner.toString()).to.equal(profileOwner.publicKey.toString());
      expect(profile.skills).to.deep.equal(["Rust", "Solana", "TypeScript"]);
      expect(profile.experienceYears).to.equal(3);
      expect(profile.isPublic).to.be.true;
    });

    it("Updates profile", async () => {
      await program.methods
        .updateProfile(
          ["Rust", "Solana", "TypeScript", "React"],
          "Updated bio description",
          null
        )
        .accounts({
          profile: profilePda,
          owner: profileOwner.publicKey,
        })
        .signers([profileOwner])
        .rpc();

      const profile = await program.account.profile.fetch(profilePda);
      expect(profile.skills).to.deep.equal(["Rust", "Solana", "TypeScript", "React"]);
      expect(profile.bio).to.equal("Updated bio description");
    });
  });

  describe("Job Management", () => {
    let jobPda: anchor.web3.PublicKey;
    let jobId: number;

    it("Creates a job posting", async () => {
      jobId = Math.floor(Date.now() / 1000); // Use seconds for consistency
      [jobPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("job"),
          recruiter.publicKey.toBuffer(),
          new anchor.BN(jobId).toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      );

      await program.methods
        .createJob(
          "Senior Blockchain Developer",
          "Looking for experienced Solana developer",
          ["Rust", "Solana"],
          new anchor.BN(80000),
          new anchor.BN(120000),
          30,
          new anchor.BN(jobId)
        )
        .accounts({
          job: jobPda,
          recruiter: recruiter.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([recruiter])
        .rpc();

      const job = await program.account.job.fetch(jobPda);
      expect(job.recruiter.toString()).to.equal(recruiter.publicKey.toString());
      expect(job.title).to.equal("Senior Blockchain Developer");
      expect(job.isActive).to.be.true;
      expect(job.applicationCount).to.equal(0);
    });
  });

  describe("Application System", () => {
    let profilePda: anchor.web3.PublicKey;
    let jobPda: anchor.web3.PublicKey;
    let applicationPda: anchor.web3.PublicKey;
    let jobId: number;

    before(async () => {
      // Setup profile for application test
      [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("profile"), profileOwner.publicKey.toBuffer()],
        program.programId
      );

      // Setup job for application test
      jobId = Math.floor(Date.now() / 1000) + 100; // Different job ID
      [jobPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("job"),
          recruiter.publicKey.toBuffer(),
          new anchor.BN(jobId).toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      );

      // Create job for testing applications
      await program.methods
        .createJob(
          "Frontend Developer",
          "React and TypeScript position",
          ["React", "TypeScript"],
          new anchor.BN(60000),
          new anchor.BN(90000),
          45,
          new anchor.BN(jobId)
        )
        .accounts({
          job: jobPda,
          recruiter: recruiter.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([recruiter])
        .rpc();
    });

    it("Applies to a job", async () => {
      [applicationPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("application"), jobPda.toBuffer(), profileOwner.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .applyToJob("I am very interested in this position and have relevant experience with React and TypeScript.")
        .accounts({
          application: applicationPda,
          job: jobPda,
          profile: profilePda,
          applicant: profileOwner.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([profileOwner])
        .rpc();

      const application = await program.account.application.fetch(applicationPda);
      expect(application.applicant.toString()).to.equal(profileOwner.publicKey.toString());
      expect(application.job.toString()).to.equal(jobPda.toString());
      expect(application.status).to.deep.equal({ pending: {} });

      // Check job application count updated
      const job = await program.account.job.fetch(jobPda);
      expect(job.applicationCount).to.equal(1);
    });

    it("Updates application status", async () => {
      await program.methods
        .updateApplicationStatus({ reviewing: {} })
        .accounts({
          application: applicationPda,
          job: jobPda,
          recruiter: recruiter.publicKey,
        })
        .signers([recruiter])
        .rpc();

      const application = await program.account.application.fetch(applicationPda);
      expect(application.status).to.deep.equal({ reviewing: {} });
    });
  });

  describe("Scout System", () => {
    let profilePda: anchor.web3.PublicKey;
    let scoutOfferPda: anchor.web3.PublicKey;

    before(async () => {
      [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("profile"), profileOwner.publicKey.toBuffer()],
        program.programId
      );
    });

    it("Sends a scout offer", async () => {
      [scoutOfferPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("scout"), recruiter.publicKey.toBuffer(), profilePda.toBuffer()],
        program.programId
      );

      const incentiveAmount = 0.01 * anchor.web3.LAMPORTS_PER_SOL; // 0.01 SOL

      await program.methods
        .sendScoutOffer(
          "We'd like to scout you for our company!",
          new anchor.BN(incentiveAmount)
        )
        .accounts({
          scoutOffer: scoutOfferPda,
          targetProfile: profilePda,
          recruiter: recruiter.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([recruiter])
        .rpc();

      const scoutOffer = await program.account.scoutOffer.fetch(scoutOfferPda);
      expect(scoutOffer.recruiter.toString()).to.equal(recruiter.publicKey.toString());
      expect(scoutOffer.targetProfile.toString()).to.equal(profilePda.toString());
      expect(scoutOffer.status).to.deep.equal({ pending: {} });
    });

    it("Responds to scout offer (accept)", async () => {
      const profileOwnerBalanceBefore = await provider.connection.getBalance(profileOwner.publicKey);

      await program.methods
        .respondToScout(true) // Accept the scout offer
        .accounts({
          scoutOffer: scoutOfferPda,
          profile: profilePda,
          profileOwner: profileOwner.publicKey,
          recruiter: recruiter.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([profileOwner])
        .rpc();

      const scoutOffer = await program.account.scoutOffer.fetch(scoutOfferPda);
      expect(scoutOffer.status).to.deep.equal({ accepted: {} });

      // Check that incentive was transferred
      const profileOwnerBalanceAfter = await provider.connection.getBalance(profileOwner.publicKey);
      expect(profileOwnerBalanceAfter).to.be.greaterThan(profileOwnerBalanceBefore);
    });
  });
});