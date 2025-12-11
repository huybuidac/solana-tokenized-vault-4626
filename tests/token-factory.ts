import { expect } from "chai";
import { createFixture } from "./fixtures/factory-fixture";
import { PERMISSIONS } from "../shared/token-factory.constants";
import { Keypair } from "@solana/web3.js";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { toBN } from "../shared/utils";
import BN from "bn.js";

const tokenAgrs = {
  name: "Test Token",
  symbol: "TEST",
  uri: "https://test.com",
};

describe("token-factory", () => {
  it("Can create a token with fee", async () => {
    const { users, provider, deployer } = await createFixture();
    const user = users[0];
    const mintKp = Keypair.generate();
    await expect(
      user.program.methods
        .createToken(tokenAgrs)
        .accountsPartial({
          access: null,
          mint: mintKp.publicKey,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .signers([mintKp])
        .rpc()
    ).solBalanceHaveChanged(provider, [
      { account: deployer.publicKey, amount: toBN(0.1) },
    ]);
  });

  it("Can create a token without fee", async () => {
    const { users, admin, provider, deployer } = await createFixture();
    const user = users[0];
    const mintKp = Keypair.generate();
    // can not call if no permission
    await expect(
      user.program.methods
        .createTokenWhitelisted(tokenAgrs)
        .accountsPartial({
          access: null,
          mint: mintKp.publicKey,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .signers([mintKp])
        .rpc()
    ).rejectedWith(/UnauthorizedPermission/);
    // grant permission
    await admin.program.methods
      .grantPermission(PERMISSIONS.WHITELIST)
      .accounts({ to: user.publicKey })
      .rpc();
    // no fee
    await expect(
      user.program.methods
        .createTokenWhitelisted(tokenAgrs)
        .accountsPartial({
          mint: mintKp.publicKey,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .signers([mintKp])
        .rpc()
    ).solBalanceHaveChanged(provider, [
      { account: deployer.publicKey, amount: 0n },
    ]);
  });
  it("Can get fee", async () => {
    const { users } = await createFixture();
    const user = users[0];
    const fee: BN = await user.program.methods
      .getFee()
      .signers([user.keypair])
      .view();
    expect(fee.toString()).to.equal("100000000");
  });
});
