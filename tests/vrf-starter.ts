import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VrfStarter } from "../target/types/vrf_starter";
import fs from "fs";
import glob from "glob";

const { SystemProgram } = anchor.web3;
const vrfPubkey = new anchor.web3.PublicKey(
  "3maGWtnB3Uo6oBKbi4WSpSiz1hNch7K6bJqYWmddiQtw"
);

describe("vrf-starter", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.VrfStarter as Program<VrfStarter>;
  it("requests random data", async () => {
    let signature = await program.rpc.getRandom({
      accounts: {
        vrf: vrfPubkey,
        systemProgram: SystemProgram.programId,
      },
    });
    let logPath = ".anchor/program-logs/**/*.log";
    glob(logPath, function (err, files) {
      let logdata = [];
      fs.readFile(files[0], "utf8", (err, data) => {
        let lines = data.split("\n");
        lines.slice(lines.length - 6, lines.length).forEach((line) => {
          logdata.push(line);
        });
        console.table({ msg: "request random data", signature });
        console.log(logdata);
      });
    });
  });
});
