import BN from "bn.js";
import { parseUnits } from "viem";

export const DAY_SECONDS = 86400;

export function toBN(val: any, decimals: number = 9) {
  const decimalAmount = parseUnits(val.toString(), decimals);
  return new BN(decimalAmount.toString());
}
