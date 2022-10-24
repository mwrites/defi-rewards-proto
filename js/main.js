const {
    Connection,
    sendAndConfirmTransaction,
    Keypair,
    Transaction,
    SystemProgram,
    PublicKey,
    TransactionInstruction,
} = require("@solana/web3.js");
const {
    getMint, getAccount,
    TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID,
} = require("@solana/spl-token");
const BN = require("bn.js");
const {Buffer} = require("buffer");
const {expect} = require("chai");

const programId = new PublicKey("D6uuFNBvjd31CUrd41eHcvUzAVSJTYSf1hSRVYw7nqa3");

const buildIx = (ixIndex, keys) => {
    return new TransactionInstruction({
        keys,
        data: Buffer.from([ixIndex]),
        programId: programId,
    });
};

const mintIx = async (payer, mintPDA, mintDestAtaTBC) => {
    const mintK = {pubkey: mintPDA, isSigner: false, isWritable: true}; // writable because we need to alter mint account when minting
    const userK = {pubkey: payer.publicKey, isSigner: true, isWritable: false}; // should be non writable? but from spl example it should be writable..
    const destAtaTBCK = {pubkey: mintDestAtaTBC, isSigner: false, isWritable: true};

    // TODO: DO WE NEEED TO PASS THE PROGRAMS AROUND? CAN THE PROGRAM PASS THEM FOR US?
    const sysProgK = {pubkey: SystemProgram.programId, isSigner: false, isWritable: false};
    const tokenProgK = {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false};
    const ataTokenProgK = {pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false};

    const ix = buildIx(
        1,
        [
            mintK,
            userK,
            destAtaTBCK,
            sysProgK,
            tokenProgK,
            ataTokenProgK, // if commented, CPI to create dest ata will complain that an account is missing
        ]
    );
    console.log("mintPDA ", mintPDA.toString());
    console.log("mintDestAtaTBC ", mintDestAtaTBC.toString());
    return ix;
}

const initializeMintIx = (payer, toBeCreated) => {
    const payerK = {pubkey: payer.publicKey, isSigner: true, isWritable: false};
    const toBeCreatedK = {pubkey: toBeCreated, isSigner: false, isWritable: true};
    // DO WE NEED TO PASS THE PROGRAMS?
    const sysProgK = {pubkey: SystemProgram.programId, isSigner: false, isWritable: false};
    const tokenProgK = {pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false};

    return buildIx(
        0,
        [
            payerK,
            toBeCreatedK,
            sysProgK,
            tokenProgK,
        ]
    );
}


const main = async () => {
    const args = process.argv.slice(2);
    if (args.length !== 1) {
        console.log("Please provide instruction number");
        return;
    }
    const ixIndex = parseInt(args[0]);
    console.log("Program: ", programId.toBase58());
    console.log(`ixIndex: ${ixIndex}`);

    const connection = new Connection("http://127.0.0.1:8899");
    const payer = Keypair.fromSecretKey(
        Uint8Array.from([
            174, 47, 154, 16, 202, 193, 206, 113, 199, 190, 53, 133, 169, 175, 31, 56,
            222, 53, 138, 189, 224, 216, 117, 173, 10, 149, 53, 45, 73, 251, 237, 246,
            15, 185, 186, 82, 177, 240, 148, 69, 241, 227, 167, 80, 141, 89, 240, 121,
            121, 35, 172, 247, 68, 251, 226, 218, 48, 63, 176, 109, 168, 89, 238, 135,
        ])
    );
    if ((await connection.getBalance(payer.publicKey)) < 0.1) {
        console.log("Requesting Airdrop of 1 SOL...");
        await connection.requestAirdrop(payer.publicKey, 2e9);
        console.log("Airdrop received");
    }
    console.log("payer: ", payer.publicKey.toString());

    if (ixIndex === 0) {
        const signers = [payer];

        const [mintPdaTBC, mintPdaTBCBump] = await PublicKey.findProgramAddress(
            [Buffer.from("beef_mint")],
            programId
        );
        console.log(`mint - {pda:${mintPdaTBC.toString()}, bump:${mintPdaTBCBump}`);

        const ix = await initializeMintIx(payer, mintPdaTBC);
        const tx = new Transaction().add(ix);

        let txHash = await sendAndConfirmTransaction(connection, tx, signers, {
            skipPreflight: true,
            preflightCommitment: "confirmed",
            commitment: "finalized",
        });

        const mint = await getMint(connection, mintPdaTBC);
        expect(mint.address).to.eql(mintPdaTBC);
        expect(mint.mintAuthority).to.eql(mintPdaTBC);
        // supply: bigint;
        // /** Number of base 10 digits to the right of the decimal place */
        // decimals: number;
        // /** Is this mint initialized */
        // isInitialized: boolean;
        // /** Optional authority to freeze token accounts */
        // freezeAuthority: PublicKey | null;
        // /** Additional data for extension */
        // tlvData: Buffer;

        console.log("(after) mint:", mint);
    } else if (ixIndex === 1) {
        const signers = [payer];

        const mintPDA = (await PublicKey.findProgramAddress(
            [Buffer.from("beef_mint")],
            // [payer.publicKey.toBuffer()],
            programId
        ))[0];

        const [mintDestAtaTBC] = await PublicKey.findProgramAddress(
            [payer.publicKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mintPDA.toBuffer()],
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        let beforeAmount = BigInt(0);
        try {
            const ata = await getAccount(connection, mintDestAtaTBC);
            beforeAmount = ata.amount;
        } catch {}
        console.log("(before) ataAmount:", beforeAmount);

        const ix = await mintIx(payer, mintPDA, mintDestAtaTBC);
        const tx = new Transaction().add(ix);

        const txHash = await sendAndConfirmTransaction(connection, tx, signers, {
            skipPreflight: true,
            preflightCommitment: "confirmed",
            commitment: "finalized",
        });

        const mintAmount = BigInt(42); // currently hardcoded in the program

        const afterAta = await getAccount(connection, mintDestAtaTBC);
        console.log("(after) ata:", afterAta);
        expect(afterAta.amount).to.eql(beforeAmount + mintAmount);
    } else {
        console.log("not supported");
        exit(1);
    }
}


main()
    .then(() => {
        console.log("Done");
    })
    .catch((e) => {
        console.error(e);
    });
