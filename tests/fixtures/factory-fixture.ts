import { fromWorkspace, LiteSVMProvider } from "anchor-litesvm";
import { Program, Wallet } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import deepmerge from "deepmerge";
import { TokenFactory } from "../../target/types/token_factory";
import IDL from "../../target/idl/token_factory.json";
import { PERMISSIONS } from "../../shared/token-factory.constants";

interface FixtureOptions {
  createConfig?: { createFee: bigint } | false;
  createCollateral?: boolean;
}

const getDefaultOptions = () => {
  return {
    createConfig: {
      createFee: 100n,
    },
    createCollateral: true,
  };
};

export const NOW = new Date("2015-08-27T08:00:00.000Z");

export const createFixture = async (options?: FixtureOptions) => {
  options = deepmerge(getDefaultOptions(), options || {});
  const client = fromWorkspace(".");
  const provider = new LiteSVMProvider(client);
  const program = new Program<TokenFactory>(IDL, provider);

  const clock = client.getClock();
  clock.unixTimestamp = BigInt(NOW.getTime()) / 1000n;
  client.setClock(clock);

  const deployer = {
    keypair: provider.wallet.payer,
    provider,
    program,
    publicKey: provider.wallet.publicKey,
  };

  const users = await Promise.all(
    Array.from({ length: 20 }, (i) => i).map(async () => {
      const keypair = Keypair.generate();
      client.airdrop(keypair.publicKey, 10_000000000n);
      const userProvider = new LiteSVMProvider(client, new Wallet(keypair));

      return {
        keypair,
        provider: userProvider,
        program: new Program<TokenFactory>(IDL, userProvider),
        publicKey: keypair.publicKey,
      };
    })
  );

  const admin = users.shift()!;

  if (options.createConfig) {
    await program.methods.initialize().rpc();
    await program.methods
      .grantPermissionAdmin(PERMISSIONS.WHITELIST)
      .accounts({
        to: admin.publicKey,
      })
      .rpc();
  }

  return {
    deployer,
    client,
    provider,
    program,
    users,
    admin,
  };
};
