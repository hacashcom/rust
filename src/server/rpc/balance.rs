

defineQueryObject!{ Q8364,
    address_list, String, s!(""),
    unit, Option<String>, None,
}

async fn balances(State(ctx): State<ApiCtx>, q: Query<Q8364>) -> impl IntoResponse  {
    ctx_state!(ctx, state);
    let unit = q_unit!(q);
    let ads = q.address_list.replace(" ","").replace("\n","");
    let addrs: Vec<_> = ads.split(",").collect();
    let adrsz = addrs.len();
    if adrsz == 0 || (adrsz==1 && addrs[0].len()==0) {
        return api_error("address format error")
    }
    if adrsz > 200 {
        return api_error("address max 200")
    }
    let mut resbls = Vec::with_capacity(adrsz);
    for a in addrs {
        let adr = Address::form_readable(a);
        if let Err(e) = adr {
            return api_error(&format!("address {} format error", a))
        }
        let adr = adr.unwrap();
        let mut bls = state.balance(&adr);
        if let None = bls {
            bls = Some(Balance::new());
        }
        let bls = bls.unwrap();
        resbls.push(json!({
            "hacash": bls.hacash.to_unit_string(&unit),
            "diamond": bls.diamond.value().uint(),
            "satoshi": bls.satoshi.value().uint(),
        }));
    }
    // ok
    api_data_list(resbls)
}