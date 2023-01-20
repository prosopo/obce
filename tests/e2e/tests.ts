import { assert, expect } from "chai";
import { Error } from "../../typechain-generated/types-returns/contract";
import { setup } from "./helpers";
import { WeightV2 } from "@polkadot/types/interfaces/runtime/types";

describe("OBCE_TESTS", () => {
  it("Can call successful method", async () => {
    const { api, contract } = await setup();

    await contract.query.successfulMethod();

    await api.disconnect();
  });

  it("Can call erroneous method", async () => {
    const { api, contract } = await setup();

    const res = await contract.query.erroneousMethod();
    expect(res.value.unwrap().err).to.be.eq(Error.nonCriticalError);

    await api.disconnect();
  });

  it("Can call critically erroneous method", async () => {
    const { api, contract } = await setup();

    expect(contract.tx.criticallyErroneousMethod()).to.eventually.throw();

    await api.disconnect();
  });

  it("Can call multi arg method", async () => {
    const { api, contract } = await setup();

    const res = await contract.query.multiArgMethod(100, 300);
    expect(res.value.unwrap().ok).to.be.eq(400);

    await api.disconnect();
  });

  it("Can correctly charge weight", async () => {
    const { api, contract } = await setup();

    const init = (await contract.query.weightLinearMethod(1))
      .gasConsumed as unknown as WeightV2;
    const increased = (await contract.query.weightLinearMethod(5))
      .gasConsumed as unknown as WeightV2;

    assert(increased.refTime.toNumber() - init.refTime.toNumber() == 4);

    await api.disconnect();
  });
});
