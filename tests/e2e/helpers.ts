import { KeyringPair } from "@polkadot/keyring/types";
import { Keyring } from "@polkadot/keyring";
import { ApiPromise } from "@polkadot/api";
import Constructors from "../../typechain-generated/constructors/contract";
import Contract from "../../typechain-generated/contracts/contract";
import chai from "chai";
import chaiAsPromised from "chai-as-promised";

chai.use(chaiAsPromised);
chai.should();

const getSigner = (): KeyringPair => {
  return new Keyring({ type: "sr25519" }).addFromUri("//Alice");
};

export const setup = async (): Promise<{
  api: ApiPromise;
  contract: Contract;
}> => {
  const api = await ApiPromise.create();

  const alice = getSigner();

  const contractFactory = new Constructors(api, alice);
  const contractAddress = (await contractFactory.new()).address;
  const contract = new Contract(contractAddress, alice, api);

  return {
    api,
    contract,
  };
};
