use js_sys::{Array, Promise};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{error::JsResult, utils::set_panic_hook, DIDResolver, FromPrior, JsDIDResolver};

#[wasm_bindgen(skip_typescript)]
impl FromPrior {
    #[wasm_bindgen(skip_typescript)]
    pub fn unpack(from_prior: String, did_resolver: DIDResolver) -> Promise {
        // TODO: Better place?
        set_panic_hook();

        let did_resolver = JsDIDResolver(did_resolver);

        future_to_promise(async move {
            let (msg, metadata) = didcomm::FromPrior::unpack(&from_prior, &did_resolver)
                .await
                .as_js()?;

            let res = {
                let res = Array::new_with_length(2);
                res.set(0, FromPrior(Rc::new(msg)).into());
                res.set(1, metadata.into());
                res
            };

            Ok(res.into())
        })
    }
}

#[wasm_bindgen(typescript_custom_section)]
const FROM_PRIOR_UNPACK_TS: &'static str = r#"
export namespace FromPrior {
    function unpack(
        from_prior: string,
        did_resolver: DIDResolver,
    ): Promise<[FromPrior, string]>;
}
"#;
