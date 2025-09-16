import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { JobPlatform } from "../target/types/job_platform";
import { expect } from "chai";
import { Keypair } from "@solana/web3.js";
const fs = require("fs");

describe("Profile Creation Test - Real Wallets", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.JobPlatform as Program<JobPlatform>;

  // 실제 지갑들 로드
  let jobSeekerKeypair: Keypair;
  let recruiterKeypair: Keypair;

  before(async () => {
    console.log("Loading real wallets for testing...");

    // Job Seeker 지갑 로드
    const jobSeekerWallet = JSON.parse(
      fs.readFileSync("job-seeker-wallet.json", "utf8")
    );
    jobSeekerKeypair = Keypair.fromSecretKey(new Uint8Array(jobSeekerWallet));

    // Recruiter 지갑 로드
    const recruiterWallet = JSON.parse(
      fs.readFileSync("recruiter-wallet.json", "utf8")
    );
    recruiterKeypair = Keypair.fromSecretKey(new Uint8Array(recruiterWallet));

    console.log("Job Seeker Wallet:", jobSeekerKeypair.publicKey.toString());
    console.log("Recruiter Wallet:", recruiterKeypair.publicKey.toString());

    // 지갑 잔액 확인
    const jobSeekerBalance = await provider.connection.getBalance(jobSeekerKeypair.publicKey);
    const recruiterBalance = await provider.connection.getBalance(recruiterKeypair.publicKey);

    console.log("Job Seeker Balance:", jobSeekerBalance / anchor.web3.LAMPORTS_PER_SOL, "SOL");
    console.log("Recruiter Balance:", recruiterBalance / anchor.web3.LAMPORTS_PER_SOL, "SOL");
  });

  describe("Profile Creation", () => {
    it("Creates a job seeker profile", async () => {
      console.log("\n=== Job Seeker 프로필 생성 테스트 ===");

      // PDA 계산
      const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("profile"), jobSeekerKeypair.publicKey.toBuffer()],
        program.programId
      );

      console.log("Profile PDA:", profilePda.toString());

      try {
        // 프로필 생성 트랜잭션 (실제 프로그램 파라미터에 맞춤)
        const tx = await program.methods
          .createProfile(
            ["JavaScript", "TypeScript", "Solana", "React"], // skills: Vec<String>
            3, // experience_years: u16
            "서울, 대한민국", // region: String
            "풀스택 개발자입니다. Solana 생태계에 관심이 많습니다." // bio: String
          )
          .accounts({
            owner: jobSeekerKeypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([jobSeekerKeypair])
          .rpc();

        console.log("✅ Transaction signature:", tx);
        console.log("🔗 Explorer:", `https://explorer.solana.com/tx/${tx}?cluster=devnet`);

        // 생성된 프로필 확인
        const profile = await program.account.profile.fetch(profilePda);
        console.log("✅ Created profile:", {
          owner: profile.owner.toString(),
          skills: profile.skills,
          experienceYears: profile.experienceYears,
          region: profile.region,
          bio: profile.bio,
          createdAt: new Date(profile.createdAt.toNumber() * 1000),
          isPublic: profile.isPublic
        });

        // 검증
        expect(profile.owner.toString()).to.equal(jobSeekerKeypair.publicKey.toString());
        expect(profile.skills.length).to.equal(4);
        expect(profile.experienceYears).to.equal(3);

      } catch (error) {
        console.error("❌ Error creating profile:", error);
        throw error;
      }
    });

    it("Creates a recruiter profile", async () => {
      console.log("\n=== Recruiter 프로필 생성 테스트 ===");

      // PDA 계산
      const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("profile"), recruiterKeypair.publicKey.toBuffer()],
        program.programId
      );

      console.log("Profile PDA:", profilePda.toString());

      try {
        // 프로필 생성 트랜잭션
        const tx = await program.methods
          .createProfile(
            ["HR", "Developer Relations", "Technical Recruiting"], // skills: Vec<String>
            5, // experience_years: u16
            "서울, 대한민국", // region: String
            "스타트업에서 개발자 채용을 담당하고 있습니다." // bio: String
          )
          .accounts({
            owner: recruiterKeypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([recruiterKeypair])
          .rpc();

        console.log("✅ Transaction signature:", tx);
        console.log("🔗 Explorer:", `https://explorer.solana.com/tx/${tx}?cluster=devnet`);

        // 생성된 프로필 확인
        const profile = await program.account.profile.fetch(profilePda);
        console.log("✅ Created profile:", {
          owner: profile.owner.toString(),
          skills: profile.skills,
          experienceYears: profile.experienceYears,
          region: profile.region,
          bio: profile.bio,
          createdAt: new Date(profile.createdAt.toNumber() * 1000),
          isPublic: profile.isPublic
        });

        // 검증
        expect(profile.owner.toString()).to.equal(recruiterKeypair.publicKey.toString());
        expect(profile.skills.length).to.equal(3);
        expect(profile.experienceYears).to.equal(5);

      } catch (error) {
        console.error("❌ Error creating recruiter profile:", error);
        throw error;
      }
    });
  });
});