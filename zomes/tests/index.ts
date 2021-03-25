import { Orchestrator, Config, InstallAgentsHapps } from '@holochain/tryorama'
import { TransportConfigType, ProxyAcceptConfig, ProxyConfigType } from '@holochain/tryorama'
import { HoloHash, InstallAppRequest } from '@holochain/conductor-api'
import path from 'path'

const network = {
    transport_pool: [{
      type: TransportConfigType.Proxy,
      sub_transport: {type: TransportConfigType.Quic},
      proxy_config: {
        type: ProxyConfigType.LocalProxyServer,
        proxy_accept_config: ProxyAcceptConfig.AcceptAll
      }
    }],
    bootstrap_service: "https://bootstrap.holo.host"
};
//const conductorConfig = Config.gen({network});
const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const shortForm = path.join(__dirname, '../../workdir/shortform-expression.dna')

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [shortForm] // contains 1 dna, the "shortform" dna
  ]
]

const orchestrator = new Orchestrator()

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

orchestrator.registerScenario("create and get public expression", async (s, t) => {
  const [alice, bob] = await s.players([conductorConfig, conductorConfig])
  const [[alice_happ]] = await alice.installAgentsHapps(installation)
  //const [[bob_happ]] = await bob.installAgentsHapps(installation)

  //Create a public expression from alice
  const create_exp = await alice_happ.cells[0].call("shortform", "create_public_expression", 
    {data: JSON.stringify({background: [], body: "A test expression"}), author: {did: "did://alice", name: null, email: null}, timestamp: "ISO8601", proof: {key: "key", signature: "sig"}})
  console.log("Created expression", create_exp);
  t.notEqual(create_exp.expression_data, undefined);
  
  sleep(10000);
  //Create another time index
  var dateOffset = (24*60*60*1000) / 2; //12 hr ago
  var date = new Date();
  date.setTime(date.getTime() - dateOffset);

  let current = new Date().toISOString();
  console.log("Getting date", current);
  //Get agent alice expressions from bob
  const get_exps = await alice_happ.cells[0].call("shortform", "get_by_author", {author: "did://alice", from: date.toISOString(), until: new Date().toISOString()})
  console.log("Got expressions for alice: ", get_exps);
  t.equal(get_exps.length, 1);

  //Try and get the expression by address
  const get_exp = await alice_happ.cells[0].call("shortform", "get_expression_by_address", create_exp.holochain_data.element.signed_header.header.hash)
  console.log("Got exp by address", get_exp);
  t.notEqual(get_exp.expression_data, undefined);
})

orchestrator.registerScenario("test send and receive private", async (s, t) => {
  const [alice, bob] = await s.players([conductorConfig, conductorConfig])
  const [[alice_happ]] = await alice.installAgentsHapps(installation)
  const [[bob_happ]] = await bob.installAgentsHapps(installation)

  await s.shareAllNodes([alice, bob])

  const send = await alice_happ.cells[0].call("shortform", "send_private", {to: bob_happ.agent, expression: {data: JSON.stringify({background: [], body: "A private test expression"}), author: {did: "did://alice", name: null, email: null}, timestamp: "ISO8601", proof: {key: "key", signature: "sig"}}})
  console.log("Created expression", send);
  t.ok(send);

  const get_inbox = await bob_happ.cells[0].call("shortform", "inbox", {from: null, page_size: 10, page_number: 0})
  console.log("get inbox", get_inbox);
  t.deepEqual(get_inbox.length, 1);

  const get_inbox_from = await bob_happ.cells[0].call("shortform", "inbox", {from: "did://alice", page_size: 10, page_number: 0})
  t.deepEqual(get_inbox_from.length, 1)
})

// Run all registered scenarios as a final step, and gather the report,
// if you set up a reporter
const report = orchestrator.run()

// Note: by default, there will be no report
console.log(report)