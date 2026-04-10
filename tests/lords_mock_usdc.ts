import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LordsMockUsdc } from "../target/types/lords_mock_usdc";
import { TOKEN_PROGRAM_ID, getMint } from "@solana/spl-token";
import { assert } from "chai";

describe("lords_mock_usdc", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.LordsMockUsdc as Program<LordsMockUsdc>;
    const signer = provider.wallet as anchor.Wallet;

    // Generate a new keypair for the Mint account itself
    const mintKeypair = anchor.web3.Keypair.generate();

    // Hardcoded Phantom Address from your Rust code
    const PHANTOM_FREEZE_AUTH = new anchor.web3.PublicKey("HpAYk14jYpomivS4F7oXySN81sdoPvTaHtFsPgiK2jzf");

    // PDA for Mint Authority
    const [mintAuthorityPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("mint_authority")],
        program.programId
    );

    const METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    const [metadataAddress] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("metadata"),
            METADATA_PROGRAM_ID.toBuffer(),
            mintKeypair.publicKey.toBuffer(),
        ],
        METADATA_PROGRAM_ID
    );

    it("Deploys the Mint and Assigns Authorities", async () => {
        console.log("Deployer (Signer): ", signer.publicKey.toBase58());
        console.log("Mint Address:      ", mintKeypair.publicKey.toBase58());
        console.log("Phantom Freeze:    ", PHANTOM_FREEZE_AUTH.toBase58());

        try {
            const tx = await program.methods
                .createMockUsdc()
                .accounts({
                    signer: signer.publicKey,
                    mint: mintKeypair.publicKey,
                    // mintAuthorityPda is auto-resolved by Anchor v0.30
                    metadataAccount: metadataAddress,
                    tokenMetadataProgram: METADATA_PROGRAM_ID,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                } as any)
                .signers([mintKeypair])
                .rpc();

            console.log("Transaction Signature:", tx);

            // Fetch mint data to verify logic
            const mintAccount = await getMint(provider.connection, mintKeypair.publicKey);

            // 1. Verify Mint Authority moved to PDA
            assert.strictEqual(
                mintAccount.mintAuthority?.toBase58(),
                mintAuthorityPda.toBase58(),
                "Mint authority should be the PDA"
            );

            // 2. Verify Freeze Authority is your Phantom Wallet
            assert.strictEqual(
                mintAccount.freezeAuthority?.toBase58(),
                PHANTOM_FREEZE_AUTH.toBase58(),
                "Freeze authority should be the Phantom Wallet"
            );

            console.log("✅ Verified: Mint Authority -> PDA");
            console.log("✅ Verified: Freeze Authority -> Phantom Wallet");

        } catch (err) {
            console.error("Deployment failed:", err);
            throw err;
        }
    });
});