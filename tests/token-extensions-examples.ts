import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenExtensionsExamples } from "../target/types/token_extensions_examples";

import { createAccount, TOKEN_2022_PROGRAM_ID, mintToChecked, getAccount, burnChecked } from "@solana/spl-token";


describe("token-extensions-examples", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TokenExtensionsExamples as Program<TokenExtensionsExamples>;
  const connection = anchor.AnchorProvider.env().connection;

  const wallet = anchor.web3.Keypair.generate();
  before(async () => {

    await connection.confirmTransaction(
      await connection.requestAirdrop(wallet.publicKey, 10000000000),
      "confirmed"
    );

  });

  it("Is initialized!", async () => {
    // Add your test here.
    const delegate =  anchor.web3.Keypair.generate();
    const freezeAuth =  anchor.web3.Keypair.generate();
    const mintAuth =  anchor.web3.Keypair.generate();
    
    const tx = await program.methods.initializeWithExt(10).accounts({delegate: delegate.publicKey, tokenProgram: TOKEN_2022_PROGRAM_ID, freezeAuth: freezeAuth.publicKey, mintAuth: mintAuth.publicKey}).rpcAndKeys();
    console.log("Your transaction signature", tx.signature);

    const mint = tx.pubkeys.mint;

    const mintAmount = BigInt(1_000_000_000);
    const owner = anchor.web3.Keypair.generate();
    const ta = await createAccount(
        connection,
        wallet,
        mint,
        owner.publicKey,
        undefined,
        undefined,
        TOKEN_2022_PROGRAM_ID
    );

    await mintToChecked(
      connection,
      wallet,
      mint,
      ta,
      mintAuth,
      mintAmount,
      10,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID

    );

    let acc = await getAccount(connection, ta, undefined, TOKEN_2022_PROGRAM_ID);

    console.log("Balance after mint: ", acc.amount);

    await burnChecked(connection,
      wallet,
      ta,
      mint,
      delegate,
      mintAmount,
      10,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID);

    acc = await getAccount(connection, ta, undefined, TOKEN_2022_PROGRAM_ID);

    console.log("Balance after burn from delegate: ", acc.amount);
  });
});
