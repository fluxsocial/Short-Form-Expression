/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require('path')

const { Orchestrator, Config, combine, singleConductor, localOnly, tapeExecutor } = require('@holochain/tryorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/expression.dna.json")
const dna = Config.dna(dnaPath, 'ShortFormDNA')

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require('tape')),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly,
  ),
  //   waiter: {
  //   hardTimeout: 100000,
  //   strict: true,
  // }
});

const conductorConfig = Config.gen(
  {
    ShortFormExpression: dna,
  },
  {
    logger: {
      type: 'debug',
      state_dump: false,
      rules: {
          rules: [{ exclude: true, pattern: ".*" }]
      }
    },
    network: {
      type: 'sim2h',
      sim2h_url: 'ws://localhost:9000'
    }
  }
)

orchestrator.registerScenario("test create and get public expression", async (s, t) => {
  const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig}, true)
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const result  = await alice.call("ShortFormExpression", "expression", "create_public_expression", {content : JSON.stringify({background: ["bg1", "bg2"], body: "Test ShortForm Expression"})})
  t.deepEqual(result.hasOwnProperty("Ok"), true)
  await s.consistency() 

  const get = await bob.call("ShortFormExpression", "expression", "get_by_author", {"author": alice.instance('ShortFormExpression').agentAddress, page_size: 10, page_number: 0})
  t.deepEqual(get.hasOwnProperty("Ok"), true)
  t.deepEqual(get.Ok.length, 1)
})

orchestrator.registerScenario("test send and receive private", async (s, t) => {
  const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig}, true)
  await s.consistency() 

  const send = await alice.call("ShortFormExpression", "expression", "send_private", {to: bob.instance('ShortFormExpression').agentAddress, content : JSON.stringify({background: ["bg1", "bg2"], body: "Test Private P2P ShortForm Expression"})})
  t.deepEqual(result.hasOwnProperty("Ok"), true)
  await s.consistency() 

  const get_inbox = await bob.call("ShortFormExpression", "expression", "inbox", {from: null, page_size: 10, page_number: 0})
  t.deepEqual(get.hasOwnProperty("Ok"), true)
  t.deepEqual(get.Ok.length, 1)
})

orchestrator.run()
