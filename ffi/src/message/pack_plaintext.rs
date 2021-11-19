use std::sync::Arc;

use didcomm::error::ErrorKind;
use didcomm::Message;

use crate::common::{ErrorCode, EXECUTOR};
use crate::did::{did_resolver_adapter::FFIDIDResolverAdapter, FFIDIDResolver};

pub trait OnPackPlaintextResult: Sync + Send {
    fn success(&self, result: String);
    fn error(&self, err: ErrorKind, err_msg: String);
}

pub fn pack_plaintext(
    msg: &Message,
    did_resolver: &Arc<dyn FFIDIDResolver>,
    cb: Box<dyn OnPackPlaintextResult>,
) -> ErrorCode {
    let msg = msg.clone();
    let did_resolver = FFIDIDResolverAdapter::new(Arc::clone(&did_resolver));

    let future = async move { msg.pack_plaintext(&did_resolver).await };

    EXECUTOR.spawn_ok(async move {
        match future.await {
            Ok(result) => cb.success(result),
            Err(err) => cb.error(err.kind(), err.to_string()),
        }
    });

    ErrorCode::Success
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::FFIDIDResolver;
    use crate::did::resolvers::ExampleFFIDIDResolver;
    use crate::message::pack_plaintext;
    use crate::message::test_helper::{get_pack_result, PackCallbackCreator};
    use didcomm::Message;
    use serde_json::json;

    use crate::test_vectors::{ALICE_DID, ALICE_DID_DOC, BOB_DID, BOB_DID_DOC};

    #[tokio::test]
    async fn test_pack_plaintext_works() {
        let msg = Message::build(
            "example-1".to_owned(),
            "example/v1".to_owned(),
            json!("example-body"),
        )
        .to(BOB_DID.to_owned())
        .from(ALICE_DID.to_owned())
        .finalize();

        let did_resolver: Arc<dyn FFIDIDResolver> = Arc::new(ExampleFFIDIDResolver::new(vec![
            ALICE_DID_DOC.clone(),
            BOB_DID_DOC.clone(),
        ]));
        let test_cb = PackCallbackCreator::new().cb;
        let cb_id = test_cb.cb_id;

        pack_plaintext(&msg, &did_resolver, test_cb);

        let res = get_pack_result(cb_id).await;
        assert!(res.contains("body"));
    }
}
