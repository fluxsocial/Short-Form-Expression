import type { Address, Agent, Expression, ExpressionAdapter, PublicSharing, HolochainLanguageDelegate, LanguageContext, AgentService } from "@perspect3vism/ad4m";
import { DNA_NICK } from "./dna";

class ShortFormPutAdapter implements PublicSharing {
  #agent: AgentService;
  #shortFormDNA: HolochainLanguageDelegate;

  constructor(context: LanguageContext) {
    this.#agent = context.agent;
    this.#shortFormDNA = context.Holochain as HolochainLanguageDelegate;
  }

  async createPublic(shortForm: object): Promise<Address> {
    const orderedShortFormData = Object.keys(shortForm)
      .sort()
      .reduce((obj, key) => {
        obj[key] = shortForm[key];
        return obj;
      }, {});
    const expression = this.#agent.createSignedExpression(orderedShortFormData);
    const expressionPostData = {
      author: expression.author,
      timestamp: expression.timestamp,
      data: JSON.stringify(expression.data),
      proof: expression.proof,
    };
    const res = await this.#shortFormDNA.call(
      DNA_NICK,
      "shortform",
      "create_public_expression",
      expressionPostData
    );
    return res.holochain_data.element.signed_header.header.hash.toString("hex");
  }
}

export default class ShortFormAdapter implements ExpressionAdapter {
  #shortFormDNA: HolochainLanguageDelegate;

  putAdapter: PublicSharing;

  constructor(context: LanguageContext) {
    this.#shortFormDNA = context.Holochain as HolochainLanguageDelegate;
    this.putAdapter = new ShortFormPutAdapter(context);
  }

  async get(address: Address): Promise<Expression> {
    const hash = Buffer.from(address, "hex");
    const expression = await this.#shortFormDNA.call(
      DNA_NICK,
      "shortform",
      "get_expression_by_address",
      hash
    );
    if (expression != null) {
      const ad4mExpression: Expression = Object.assign(
        expression.expression_data
      );
      return ad4mExpression;
    } else {
      return null;
    }
  }

  /// Send an expression to someone privately p2p
  send_private(to: Agent, content: object) {
    //@ts-ignore
    const obj = JSON.parse(content);

    this.#shortFormDNA.call(DNA_NICK, "shortform", "send_private", {
      to: to,
      data: JSON.stringify(obj),
    });
  }

  /// Get private expressions sent to you
  async inbox(filterFrom: void | Agent[]): Promise<Expression[]> {
    //TODO: add from & pages to inbox
    if (filterFrom != null) {
      filterFrom = filterFrom[0];
    }
    const res = await this.#shortFormDNA.call(
      DNA_NICK,
      "shortform",
      "get_inbox",
      { from: filterFrom, page_size: 0, page_number: 0 }
    );
    const out = [];
    res.forEach((expression) => {
      out.push({
        author: expression.creator,
        timestamp: expression.created_at,
        data: JSON.parse(expression),
        proof: undefined,
      });
    });
    return out;
  }
}
