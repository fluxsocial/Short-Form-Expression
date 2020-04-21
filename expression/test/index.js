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

const orchestrator = new Orchestrator()
const dna = Config.dna(dnaPath, 'ShortFormDNA')
const conductorConfig = Config.gen({ShortFormDNA: dna})

orchestrator.registerScenario("test create and get public expression", async (s, t) => {

  const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig}, true)
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const result  = await alice.call("ShortFormDNA", "expression", "create_public_expression", {content : {background: ["bg1", ""], body: "Test ShortForm Expression"}})
  console.log(result)
  // Wait for all network activity to settle
  await s.consistency()
})

orchestrator.run()
