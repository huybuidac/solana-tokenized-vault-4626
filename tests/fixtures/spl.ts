import { web3 } from "@coral-xyz/anchor";
import {
  TOKEN_2022_PROGRAM_ID,
  MINT_SIZE,
  MULTISIG_SIZE,
  createInitializeMultisigInstruction,
  createInitializeMint2Instruction,
  getAssociatedTokenAddressSync,
  unpackAccount,
  createAssociatedTokenAccountIdempotentInstruction,
  createMintToInstruction,
  createTransferCheckedInstruction,
  unpackMint,
} from "@solana/spl-token";
import { LiteSVMProvider } from "anchor-litesvm";
import BN from "bn.js";
import {
  Keypair,
  SystemProgram,
  Transaction,
  PublicKey,
} from "@solana/web3.js";

export const createTokenAndMint = async (
  provider: LiteSVMProvider,
  supply: bigint,
  tokenProgramId = TOKEN_2022_PROGRAM_ID
) => {
  const decimals = 6;

  const { client, wallet } = provider;
  const mintKp = Keypair.generate();
  const mint = mintKp.publicKey;
  const multisigKp = Keypair.generate();

  const transaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: mint,
      space: MINT_SIZE,
      lamports: Number(
        provider.client.minimumBalanceForRentExemption(BigInt(MINT_SIZE))
      ),
      programId: tokenProgramId,
    }),
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: multisigKp.publicKey,
      space: MULTISIG_SIZE,
      lamports: Number(
        provider.client.minimumBalanceForRentExemption(BigInt(MULTISIG_SIZE))
      ),
      programId: tokenProgramId,
    }),
    createInitializeMultisigInstruction(
      multisigKp.publicKey,
      [provider.wallet.payer],
      1,
      tokenProgramId
    ),
    createInitializeMint2Instruction(
      mint,
      decimals,
      multisigKp.publicKey,
      null,
      tokenProgramId
    )
  );
  await provider.send(transaction, [mintKp, multisigKp]);

  const mintWrapper = wrapMint(provider, mint);
  if (supply > 0n) {
    await mintWrapper.mintTokenTo(wallet.publicKey, supply);
  }
  return mintWrapper;
};

export const wrapMint = (provider: LiteSVMProvider, mint: PublicKey) => {
  const mintAcc = provider.client.getAccount(mint) as any;
  const tokenProgramId = mintAcc.owner;
  const mintData = unpackMint(mint, mintAcc, tokenProgramId);
  const decimals = mintData.decimals;
  const mintAuthority = mintData.mintAuthority;
  const { wallet } = provider;
  const getAtaOf = (owner: PublicKey) =>
    getAssociatedTokenAddressSync(mint, owner, true, tokenProgramId);
  const balanceOf = (owner: PublicKey) => {
    const ata = getAtaOf(owner);
    try {
      const balance = unpackAccount(
        ata,
        provider.client.getAccount(ata) as any,
        tokenProgramId
      );
      return balance.amount;
    } catch (error) {
      return 0n;
    }
  };
  const balanceOfTokenAccount = (tokenAccount: PublicKey) => {
    try {
      const balance = unpackAccount(
        tokenAccount,
        provider.client.getAccount(tokenAccount) as any,
        tokenProgramId
      );
      return balance.amount;
    } catch (error) {
      return 0n;
    }
  };

  const createAtaOf = async (owner: PublicKey) => {
    const instruction = createAssociatedTokenAccountIdempotentInstruction(
      wallet.publicKey,
      getAtaOf(owner),
      owner,
      mint,
      tokenProgramId
    );
    const tx = new web3.Transaction().add(instruction);
    await provider.send(tx);
  };

  const mintTokenTo = async (owner: PublicKey, amount: bigint) => {
    await createAtaOf(owner);
    const ix = createMintToInstruction(
      mint,
      getAtaOf(owner),
      mintAuthority,
      amount,
      [wallet.payer],
      tokenProgramId
    );
    const tx = new web3.Transaction().add(ix);
    await provider.send(tx);
  };

  const transferTo = async (
    provider: LiteSVMProvider,
    to: PublicKey,
    amount: bigint
  ) => {
    const payer = provider.wallet.payer;
    await createAtaOf(to);
    const ix = createTransferCheckedInstruction(
      getAtaOf(wallet.publicKey),
      mint,
      getAtaOf(to),
      payer.publicKey,
      amount,
      decimals,
      [],
      tokenProgramId
    );
    const tx = new web3.Transaction().add(ix);
    await provider.send(tx);
  };

  const transferToTokenAccount = async (
    provider: LiteSVMProvider,
    toTokenAccount: PublicKey,
    amount: bigint | BN
  ) => {
    const payer = provider.wallet.payer;
    const ix = createTransferCheckedInstruction(
      getAtaOf(provider.wallet.publicKey),
      mint,
      toTokenAccount,
      payer.publicKey,
      BigInt(amount.toString()),
      decimals,
      [],
      tokenProgramId
    );
    await provider.send(new Transaction().add(ix));
  };

  return {
    publicKey: mint,
    getAtaOf,
    mintTokenTo,
    createAtaOf,
    transferTo,
    balanceOf,
    balanceOfTokenAccount,
    transferToTokenAccount,
    tokenProgramId,
    mintAuthority,
  };
};

export type MintWrapper = ReturnType<typeof wrapMint>;
