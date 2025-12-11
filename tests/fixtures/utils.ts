import { LiteSVMProvider } from "anchor-litesvm";
import BN from "bn.js";

export const jumpToTimestamp = (
  provider: LiteSVMProvider,
  timestamp: bigint | BN | number
) => {
  const clock = provider.client.getClock();
  clock.unixTimestamp = BigInt(timestamp.toString());
  provider.client.setClock(clock);
};
