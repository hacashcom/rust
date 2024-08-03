
use crate::mint::coinbase::*;

defineQueryObject!{ Q9364,
    __nnn_, Option<bool>, None,
}

async fn supply(State(ctx): State<ApiCtx>, q: Query<Q9364>) -> impl IntoResponse {
    ctx_state!(ctx, state);
    ctx_mintstate!(ctx, mintstate);
    //
    let lasthei = ctx.engine.latest_block().objc().height().uint();
    let lastdia = mintstate.latest_diamond();
    //
    
    // return data
    let mut data = jsondata!{
        "block_reward", cumulative_block_reward(lasthei),
        "minted_diamond", lastdia.number.uint(),
    };
    api_data(data)
}

