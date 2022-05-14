const assert = require("assert");
const anchor = require("@project-serum/anchor");

describe("quote-gif-program", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Myepicproject;

  const baseAccount = anchor.web3.Keypair.generate();

  it("Is initializes the program", async () => {
    console.log("🚀 Starting test...");

    const tx = await program.rpc.startStuffOff({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [baseAccount],
    });

    console.log("📝 Your transaction signature", tx);

    let account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );
    // assert.ok(account.totalGifs.eq(new anchor.BN(0)));
    console.log("Init")
  });

  // it("Adds a GIF", async () => {
  //   await program.rpc.addGif("insert_a_giphy_link_here", {
  //     accounts: {
  //       baseAccount: baseAccount.publicKey,
  //       user: provider.wallet.publicKey,
  //     },
  //   });

  //   let account = await program.account.baseAccount.fetch(
  //     baseAccount.publicKey
  //   );
  //   assert.ok(account.totalGifs.eq(new anchor.BN(1)));

  //   let gifs = account.gifList;
  //   assert.equal(gifs[0].gifLink, "insert_a_giphy_link_here");
  //   assert.ok(gifs[0].userAddress.equals(provider.wallet.publicKey));
  // });

  it("Upvotes a GIF", async () => {
    // upVote, downVote requires an item index (u64)
    // that should be provided as an anchor.BN()
    await program.rpc.updateHex(new anchor.BN(1), "ffffff", {
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    });

    let account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );

    console.log(account.hex_list);

    /*let hex_llist = account.hex_list;
    assert.ok(hex_llist.length > 0);
    console.log(hex_llist)*/
  });

  // it("Downvotes a GIF", async () => {
  //   // upVote, downVote requires an item index (u64)
  //   // that should be provided as an anchor.BN()
  //   await program.rpc.downVote(new anchor.BN(0), {
  //     accounts: {
  //       baseAccount: baseAccount.publicKey,
  //     },
  //   });

  //   let account = await program.account.baseAccount.fetch(
  //     baseAccount.publicKey
  //   );
  //   let gifs = account.gifList;
  //   assert.ok(gifs[0].votes.eq(new anchor.BN(0)));
  // });
});
