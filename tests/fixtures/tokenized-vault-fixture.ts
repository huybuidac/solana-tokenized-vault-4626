import { fromWorkspace, LiteSVMProvider } from 'anchor-litesvm'
import { Program, Wallet } from '@coral-xyz/anchor'
import { Keypair } from '@solana/web3.js'
import deepmerge from 'deepmerge'
import { TokenizedVault } from '../../target/types/tokenized_vault'
import IDL from '../../target/idl/tokenized_vault.json'
import { ADDRESSES, PERMISSIONS } from '../../shared/tokenized-vault.constants'
import { createTokenAndMint, MintWrapper, wrapMint } from './spl'
import { parseUnits } from 'viem'

interface FixtureOptions {
  createConfig?: { createFee: bigint } | false
  createCollateral?: boolean
}

const getDefaultOptions = () => {
  return {
    createConfig: {
      createFee: 100n,
    },
    createCollateral: true,
  }
}

export const NOW = new Date('2015-08-27T08:00:00.000Z')

export const createFixture = async (options?: FixtureOptions) => {
  options = deepmerge(getDefaultOptions(), options || {})
  const client = fromWorkspace('.')
  const provider = new LiteSVMProvider(client)
  const program = new Program<TokenizedVault>(IDL, provider)

  const clock = client.getClock()
  clock.unixTimestamp = BigInt(NOW.getTime()) / 1000n
  client.setClock(clock)

  const assetMint = await createTokenAndMint(provider, parseUnits('100000000000', 6))

  const deployer = {
    keypair: provider.wallet.payer,
    provider,
    program,
    publicKey: provider.wallet.publicKey,
  }

  const users = await Promise.all(
    Array.from({ length: 20 }, (i) => i).map(async () => {
      const keypair = Keypair.generate()
      client.airdrop(keypair.publicKey, 10_000000000n)
      const userProvider = new LiteSVMProvider(client, new Wallet(keypair))

      assetMint.transferTo(provider, keypair.publicKey, parseUnits('100000000', 6))

      return {
        keypair,
        provider: userProvider,
        program: new Program<TokenizedVault>(IDL, userProvider),
        publicKey: keypair.publicKey,
      }
    })
  )

  const admin = users.shift()!

  let sharesMint: MintWrapper

  if (options.createConfig) {
    await program.methods
      .initialize({
        name: 'Test Token',
        symbol: 'TEST',
        uri: 'https://test.com',
      })
      .accounts({
        assetMint: assetMint.publicKey,
        assetTokenProgram: assetMint.tokenProgramId,
      })
      .rpc()
    sharesMint = wrapMint(provider, ADDRESSES.SHARES_MINT)
  }

  let cmmAccounts = {
    assetTokenProgram: assetMint.tokenProgramId,
  }

  return {
    deployer,
    client,
    provider,
    program,
    users,
    admin,
    assetMint,
    sharesMint,
    cmmAccounts,
  }
}

export type Fixture = Awaited<ReturnType<typeof createFixture>>
export type UserFixture = Fixture['users'][number]
