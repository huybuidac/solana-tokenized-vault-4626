import { expect } from 'chai'
import { createFixture, Fixture, UserFixture } from './fixtures/tokenized-vault-fixture'
import BN from 'bn.js'

describe('tokenized-vault', () => {
  it('Mint', async () => {
    const { users, provider, sharesMint, assetMint, cmmAccounts } = await createFixture()
    const user = users[0]
    await expect(user.program.methods.mint(sharesMint.parseAmount(1)).accounts(cmmAccounts).rpc())
      .splBalancesHaveChanged(provider, sharesMint.publicKey, [
        { wallet: user.publicKey, amount: sharesMint.parseAmount(1) },
      ])
      .splBalancesHaveChanged(provider, assetMint.publicKey, [
        { wallet: user.publicKey, amount: -assetMint.parseAmount(1) },
      ])
  })
  it('Deposit', async () => {
    const { users, provider, sharesMint, assetMint, cmmAccounts } = await createFixture()
    const user = users[0]
    await expect(user.program.methods.deposit(assetMint.parseAmount(1)).accounts(cmmAccounts).rpc())
      .splBalancesHaveChanged(provider, sharesMint.publicKey, [
        { wallet: user.publicKey, amount: sharesMint.parseAmount(1) },
      ])
      .splBalancesHaveChanged(provider, assetMint.publicKey, [
        { wallet: user.publicKey, amount: -assetMint.parseAmount(1) },
      ])
  })
  it('Withdraw', async () => {
    const { users, provider, sharesMint, assetMint, cmmAccounts } = await createFixture()
    const user = users[0]
    await user.program.methods.deposit(assetMint.parseAmount(1)).accounts(cmmAccounts).rpc()
    await expect(user.program.methods.withdraw(assetMint.parseAmount(1)).accounts(cmmAccounts).rpc())
      .splBalancesHaveChanged(provider, sharesMint.publicKey, [
        { wallet: user.publicKey, amount: -sharesMint.parseAmount(1) },
      ])
      .splBalancesHaveChanged(provider, assetMint.publicKey, [
        { wallet: user.publicKey, amount: assetMint.parseAmount(1) },
      ])
  })
  it('Redeem', async () => {
    const { users, provider, sharesMint, assetMint, cmmAccounts } = await createFixture()
    const user = users[0]
    await user.program.methods.deposit(assetMint.parseAmount(1)).accounts(cmmAccounts).rpc()
    await expect(user.program.methods.redeem(sharesMint.parseAmount(1)).accounts(cmmAccounts).rpc())
      .splBalancesHaveChanged(provider, sharesMint.publicKey, [
        { wallet: user.publicKey, amount: -sharesMint.parseAmount(1) },
      ])
      .splBalancesHaveChanged(provider, assetMint.publicKey, [
        { wallet: user.publicKey, amount: assetMint.parseAmount(1) },
      ])
  })
  it('Complex scenario', async () => {
    const { users, provider, sharesMint, assetMint, cmmAccounts } = await createFixture()
    //    Scenario:
    // No Action   USDA   SUSDA    USDA_vault   SUSDA_supply    Price
    // 1  Stake    +1000  +1000    1000         1000           1
    // 2  Reward   +1000           2000         1000           2
    // 3  Stake    +2000  +1000    4000         2000           2
    // 4  Unstake  -1000  -500     3000         1500           2
    // 5  Reward   +3000           6000         1500           4
    // 6  Stake    +4000  +1000    10000        2500           4

    const DELTA = 1000n
    const user = users[0]

    // 1. Deposit 1000 assets
    const _1_assets = assetMint.parseAmount(1000)
    const _1_expected_shares = sharesMint.parseAmount(1000)
    await expect(user.program.methods.deposit(_1_assets).accounts(cmmAccounts).rpc())
      .splBalancesHaveChanged(provider, sharesMint.publicKey, [{ wallet: user.publicKey, amount: _1_expected_shares }])
      .splBalancesHaveChanged(provider, assetMint.publicKey, [{ wallet: user.publicKey, amount: -_1_assets }])

    // 2. Reward 1000 assets
    await assetMint.transferTo(provider, sharesMint.publicKey, BigInt(assetMint.parseAmount(1000).toString()))

    // 3. Stake 2000 assets
    const _3_assets = assetMint.parseAmount(2000)
    const _3_expected_shares = sharesMint.parseAmount(1000)
    await expect(user.program.methods.deposit(_3_assets).accounts(cmmAccounts).rpc()).splBalancesHaveChanged(
      provider,
      sharesMint.publicKey,
      [{ wallet: user.publicKey, amount: _3_expected_shares }],
      DELTA
    )

    // 4. Unstake 1000 assets
    const _4_shares = sharesMint.parseAmount(500)
    const _4_expected_assets = assetMint.parseAmount(1000)
    await expect(user.program.methods.redeem(_4_shares).accounts(cmmAccounts).rpc()).splBalancesHaveChanged(
      provider,
      assetMint.publicKey,
      [{ wallet: user.publicKey, amount: _4_expected_assets }],
      DELTA
    )

    // 5. Reward 3000 assets
    await assetMint.transferTo(provider, sharesMint.publicKey, BigInt(assetMint.parseAmount(3000).toString()))

    // 6. Stake 4000 assets
    const _6_assets = assetMint.parseAmount(4000)
    const _6_expected_shares = sharesMint.parseAmount(1000)
    await expect(user.program.methods.deposit(_6_assets).accounts(cmmAccounts).rpc()).splBalancesHaveChanged(
      provider,
      sharesMint.publicKey,
      [{ wallet: user.publicKey, amount: _6_expected_shares }],
      DELTA
    )
  })

  describe('Inflation attack: offset price by direct deposit of assets', () => {
    const _1_asset = 1_000000n
    const virtualAssets = 1n
    const virtualShares = 1000n
    const effectiveAssets = _1_asset + virtualAssets
    const effectiveShares = virtualShares
    let fixture: Fixture
    let user: UserFixture
    beforeEach(async () => {
      fixture = await createFixture()
      user = fixture.users[0]
      const { provider, sharesMint, assetMint } = fixture
      await assetMint.transferTo(provider, sharesMint.publicKey, _1_asset)
    })
    it('status', async () => {
      const { assetMint, sharesMint } = fixture
      expect(assetMint.balanceOf(sharesMint.publicKey)).to.equal(_1_asset)
      expect(sharesMint.supply()).to.equal(0n)
    })

    it('deposit: virtual assets/shares mitigate inflation attack', async () => {
      const { provider, sharesMint, assetMint, cmmAccounts, users } = fixture

      const depositAssets = _1_asset
      const expectedShares = (depositAssets * effectiveShares) / effectiveAssets

      await expect(user.program.methods.deposit(new BN(depositAssets)).accounts(cmmAccounts).rpc())
        .splBalancesHaveChanged(provider, assetMint.publicKey, [
          { wallet: user.publicKey, amount: -depositAssets },
          { wallet: sharesMint.publicKey, amount: depositAssets },
        ])
        .splBalancesHaveChanged(provider, sharesMint.publicKey, [{ wallet: user.publicKey, amount: expectedShares }])
    })

    it('mint: protects against inflation attack but makes minting expensive', async () => {
      const { users, provider, sharesMint, assetMint, cmmAccounts } = fixture

      const mintShares = sharesMint.parseAmountBigInt(1)
      const expectedAssets = (mintShares * effectiveAssets) / effectiveShares

      await expect(
        user.program.methods.mint(new BN(mintShares.toString())).accounts(cmmAccounts).rpc()
      ).splBalancesHaveChanged(provider, assetMint.publicKey, [
        { wallet: user.publicKey, amount: -expectedAssets },
        { wallet: sharesMint.publicKey, amount: expectedAssets },
      ])
    })
  })
})
