import type { Expression, Agent, GetByAuthorAdapter, HolochainLanguageDelegate, LanguageContext } from "@perspect3vism/ad4m";
import { DNA_NICK } from "./dna";
export default class ShortFormAuthorAdapter implements GetByAuthorAdapter {
  #shortFormDNA: HolochainLanguageDelegate;

  constructor(context: LanguageContext) {
    this.#shortFormDNA = context.Holochain as HolochainLanguageDelegate;
  }

  ///Get expressions authored by a given Agent/Identity
  async getByAuthor(
    author: string,
    count: number,
    page: number
  ): Promise<Expression[]> {
    //TODO: resolve did
    const res = await this.#shortFormDNA.call(
      DNA_NICK,
      "shortform",
      "get_by_author",
      { author: author, page_size: count, page_number: page }
    );
    const out = [];
    res.forEach((expression) => {
      const ad4mExpression: Expression = Object.assign(
        expression.expression_data
      );
      out.push(ad4mExpression);
    });
    return out;
  }
}
