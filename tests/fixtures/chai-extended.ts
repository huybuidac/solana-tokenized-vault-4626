import BN from 'bn.js'
import chai from 'chai'
import { PublicKey } from '@solana/web3.js'
import { LiteSVMProvider } from 'anchor-litesvm'
import { wrapMint } from './spl'
import chaiAsPromised from 'chai-as-promised'
import { isHex } from 'viem'
import { AnchorError } from '@coral-xyz/anchor'
import { bs58 } from '@coral-xyz/anchor/dist/cjs/utils/bytes'

chai.use(chaiAsPromised)

function getBasePromise(assertion) {
  return typeof assertion.then === 'function' ? assertion : assertion._obj
}

chai.use(function (chai, utils) {
  var Assertion = chai.Assertion
  Assertion.addMethod('closeToBN', function (value, delta) {
    const _obj = BigInt(this._obj.toString())
    const _value = BigInt(value.toString())
    delta = delta || 0n
    const _delta = BigInt(delta.toString())
    const result = _obj < _value ? _value - _obj <= _delta : _obj - _value <= _delta
    new Assertion(
      result,
      `Expected ${this._obj.toString()} to be close to ${value.toString()} within ${delta.toString()}`
    ).ok
  })
  Assertion.addMethod('log', function (provider: LiteSVMProvider, enabled: boolean = true) {
    const _promise = getBasePromise(this)

    const derivedPromise = _promise.then(
      (result: any) => {
        console.log('result', result)
        if (enabled) {
          let hash =
            [result, result?.tx, result?.txHash, result?.signature, result?.hash].find((x) => isHex(x)) || result
          console.log('hash', hash)
          let signature: Buffer
          if (hash && isHex(hash)) {
            signature = Buffer.from(hash, 'hex') //bs58.decode(hash)
          } else {
            signature = bs58.decode(hash)
          }
          if (signature) {
            const tx = provider.client.getTransaction(new Uint8Array(signature))
            console.log(tx?.toString())
          } else {
            console.log('No transaction hash found')
          }
        }
        return result
      },
      (error: AnchorError) => {
        console.error(error)
        throw error
      }
    )

    ;(this as any).then = derivedPromise.then.bind(derivedPromise)

    return this
  })
  Assertion.addMethod(
    'solBalanceHaveChanged',
    function (
      provider: LiteSVMProvider,
      changes: { amount: bigint | number | BN; account: PublicKey | string }[],
      diff = 0n
    ) {
      const _promise = getBasePromise(this)
      const befores = changes.map((change) => {
        const { account } = change as any
        return {
          ...change,
          before: provider.client.getBalance(new PublicKey(account)),
        }
      })
      const derivedPromise = _promise.then(
        (result: any) => {
          for (const changes of befores) {
            const af = provider.client.getBalance(new PublicKey(changes.account))
            const actualChange = af - changes.before
            const amount = BigInt(changes.amount.toString())
            const delta = actualChange > amount ? actualChange - amount : amount - actualChange
            new Assertion(
              delta <= diff,
              `For address "${changes.account}", expected balance to change by ${amount} (from ${changes.before} to ${
                changes.before + amount
              }), but got a change of ${actualChange} instead.`
            ).ok
          }
          return result
        },
        (error: any) => {
          console.error(error)
          throw error
        }
      )
      ;(this as any).then = derivedPromise.then.bind(derivedPromise)
      return this
    }
  )
  Assertion.addMethod(
    'splBalancesHaveChanged',
    function (
      provider: LiteSVMProvider,
      mint: PublicKey | string,
      changes: (
        | { amount: bigint | number | BN; wallet: PublicKey | string }
        | { amount: bigint | number | BN; tokenAccount: PublicKey | string }
      )[],
      diff = 0n
    ) {
      const _promise = getBasePromise(this)

      const wrapper = wrapMint(provider, new PublicKey(mint))

      const befores = changes.map((change) => {
        const { wallet, tokenAccount } = change as any
        const account = wallet ? wrapper.getAtaOf(new PublicKey(wallet)) : new PublicKey(tokenAccount)
        return {
          ...change,
          account,
          before: wrapper.balanceOfTokenAccount(account),
        }
      })

      const derivedPromise = _promise.then(
        (result: any) => {
          for (const changes of befores) {
            const af = wrapper.balanceOfTokenAccount(changes.account)
            const actualChange = af - changes.before
            const amount = BigInt(changes.amount.toString())
            const delta = actualChange > amount ? actualChange - amount : amount - actualChange
            new Assertion(
              delta <= diff,
              `For address "${changes.account}", expected balance to change by ${amount} (from ${changes.before} to ${
                changes.before + amount
              }), but got a change of ${actualChange} instead.`
            ).ok
          }
          return result
        },
        (error: any) => {
          console.error(error)
          throw error
        }
      )

      ;(this as any).then = derivedPromise.then.bind(derivedPromise)
      return this
    }
  )
})

declare global {
  export namespace Chai {
    interface Assertion {
      closeToBN(value: BN | number | bigint, delta?: BN | number | bigint): void
      splBalancesHaveChanged(
        provider: LiteSVMProvider,
        mint: PublicKey | string,
        changes: (
          | { amount: bigint | number | BN; wallet: PublicKey | string }
          | { amount: bigint | number | BN; tokenAccount: PublicKey | string }
        )[],
        diff?: bigint
      ): PromisedAssertion
      solBalanceHaveChanged(
        provider: LiteSVMProvider,
        changes: {
          amount: bigint | number | BN
          account: PublicKey | string
        }[],
        diff?: bigint
      ): PromisedAssertion
      log(provider: LiteSVMProvider, enabled?: boolean): PromisedAssertion
    }
    interface Eventually {
      splBalancesHaveChanged(
        provider: LiteSVMProvider,
        mint: PublicKey | string,
        changes: (
          | { amount: bigint | number | BN; wallet: PublicKey | string }
          | { amount: bigint | number | BN; tokenAccount: PublicKey | string }
        )[],
        diff?: bigint
      ): PromisedAssertion
      solBalanceHaveChanged(
        provider: LiteSVMProvider,
        changes: {
          amount: bigint | number | BN
          account: PublicKey | string
        }[],
        diff?: bigint
      ): PromisedAssertion
      log(provider: LiteSVMProvider, enabled?: boolean): PromisedAssertion
    }
  }
}
