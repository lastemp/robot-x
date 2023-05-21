import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RobotX } from "../target/types/robot_x";

describe("robot-x", () => {
  // Configure the client to use the local cluster.
  //anchor.setProvider(anchor.AnchorProvider.env());
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899")

  const program = anchor.workspace.RobotX as Program<RobotX>;
  const user = anchor.web3.Keypair.generate();

  let [action_state] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("action-state"),
    user.publicKey.toBuffer()
    ],
    program.programId);

  before(async () => {

    let res = await provider.connection.requestAirdrop(user.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);

    let latestBlockHash = await provider.connection.getLatestBlockhash()

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });

  });

  it("Is Created!", async () => {
    // Add your test here.
    //const tx = await program.methods.initialize().rpc();
    //console.log("Your transaction signature", tx);
    const tx = await program.methods.create()
      .accounts({
        actionState: action_state,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user]).rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.actionState.fetch(action_state);
    console.log("create: ", result);
  });

  it("Robot Walked!", async () => {
    // Add your test here.
    const tx = await program.methods.walk()
      .accounts({
        actionState: action_state,
        user: user.publicKey,
        //systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user]).rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.actionState.fetch(action_state);
    console.log("walk: ", result);
  });

  it("Robot Ran!", async () => {
    // Add your test here.
    const tx = await program.methods.run()
      .accounts({
        actionState: action_state,
        user: user.publicKey,
        //systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user]).rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.actionState.fetch(action_state);
    console.log("run: ", result);
  });

  it("Robot Jumped!", async () => {
    // Add your test here.
    const tx = await program.methods.jump()
      .accounts({
        actionState: action_state,
        user: user.publicKey,
        //systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user]).rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.actionState.fetch(action_state);
    console.log("jump: ", result);
  });

  it("Robot actions reset!", async () => {
    // Add your test here.
    const tx = await program.methods.reset()
      .accounts({
        actionState: action_state,
        user: user.publicKey,
        //systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([user]).rpc();
    console.log("Your transaction signature", tx);

    let result = await program.account.actionState.fetch(action_state);
    console.log("reset: ", result);
  });

});
