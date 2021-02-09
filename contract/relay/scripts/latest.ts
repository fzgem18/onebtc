import { ethers } from "@nomiclabs/buidler";
import { TestRelayFactory } from '../typechain/TestRelayFactory'

async function main(address: string) {
	let signers = await ethers.signers();
  const factory = new TestRelayFactory(signers[0]);
  let contract = factory.attach(address);
  let height = (await contract.bestHeight()).toNumber();
  console.log(`Current height: ${height}`);
}

main("0x151eA753f0aF1634B90e1658054C247eFF1C2464")
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });