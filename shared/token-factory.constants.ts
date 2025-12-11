import { address as programAddress } from "../target/idl/token_factory.json";
import { PublicKey } from "@solana/web3.js";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { BN } from "@coral-xyz/anchor";
import { maxUint64 } from "viem";

export const PERMISSIONS = {
  ALL: new BN(maxUint64),
  WHITELIST: new BN(1 << 0),
};

export const SEEDS = {
  CONFIG: "config",
  ACCESS: "access",
};

export const ADDRESSES = {
  CONFIG: PublicKey.findProgramAddressSync(
    [utf8.encode(SEEDS.CONFIG)],
    new PublicKey(programAddress)
  )[0],
  ACCESS: (account: PublicKey) =>
    PublicKey.findProgramAddressSync(
      [utf8.encode(SEEDS.ACCESS), account.toBuffer()],
      new PublicKey(programAddress)
    )[0],
};
