

pub fn routes(mut ctx: ApiCtx) -> Router {
    Router::new().route("/", get(console))

    // query
    .route(&query("latest"), get(latest))
    .route(&query("balance"), get(balance))
    .route(&query("diamond"), get(diamond))
    .route(&query("block_intro"), get(block_intro))
    .route(&query("coin_transfer"), get(scan_coin_transfer))

    // create
    .route(&create("account"), get(account))
    .route(&create("coin_transfer"), get(create_coin_transfer))
    
    // submit
    .route(&submit("transaction"), post(submit_transaction))
    .route(&submit("block"), post(submit_block))

    // submit
    // ...

    // ctx
    .with_state(ctx)
}




// paths
fn query(p: &str) -> String {
    "/query/".to_owned() + p
}
fn create(p: &str) -> String {
    "/create/".to_owned() + p
}
fn submit(p: &str) -> String {
    "/submit/".to_owned() + p
}
fn operate(p: &str) -> String {
    "/operate/".to_owned() + p
}