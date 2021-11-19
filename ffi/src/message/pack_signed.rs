use std::sync::Arc;

use didcomm::Message;
use didcomm::{error::ErrorKind, PackSignedMetadata};

use crate::common::{ErrorCode, EXECUTOR};
use crate::did::{did_resolver_adapter::FFIDIDResolverAdapter, FFIDIDResolver};
use crate::secrets::{secrets_resolver_adapter::FFISecretsResolverAdapter, FFISecretsResolver};

pub trait OnPackSignedResult: Sync + Send {
    fn success(&self, result: String, metadata: PackSignedMetadata);
    fn error(&self, err: ErrorKind, err_msg: String);
}

pub fn pack_signed(
    msg: &Message,
    sign_by: String,
    did_resolver: &Arc<dyn FFIDIDResolver>,
    secret_resolver: &Arc<dyn FFISecretsResolver>,
    cb: Box<dyn OnPackSignedResult>,
) -> ErrorCode {
    let msg = msg.clone();
    let did_resolver = FFIDIDResolverAdapter::new(Arc::clone(&did_resolver));
    let secret_resolver = FFISecretsResolverAdapter::new(Arc::clone(&secret_resolver));

    let future = async move {
        msg.pack_signed(&sign_by, &did_resolver, &secret_resolver)
            .await
    };

    EXECUTOR.spawn_ok(async move {
        match future.await {
            Ok((result, metadata)) => cb.success(result, metadata),
            Err(err) => cb.error(err.kind(), err.to_string()),
        }
    });

    ErrorCode::Success
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::{FFIDIDResolver, FFISecretsResolver};
    use crate::did::resolvers::ExampleFFIDIDResolver;
    use crate::message::pack_signed::pack_signed;
    use crate::message::test_helper::{get_pack_result, PackCallbackCreator};
    use crate::secrets::resolvers::ExampleFFISecretsResolver;
    use didcomm::Message;
    use serde_json::json;

    use crate::test_vectors::{ALICE_DID, ALICE_DID_DOC, ALICE_SECRETS, BOB_DID, BOB_DID_DOC};

    #[tokio::test]
    async fn test_pack_signed_works() {
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
        let secrets_resolver: Arc<dyn FFISecretsResolver> = Arc::new(ExampleFFISecretsResolver::new(ALICE_SECRETS.clone()));
        let test_cb = PackCallbackCreator::new().cb;
        let cb_id = test_cb.cb_id;

        pack_signed(
            &msg,
            String::from(ALICE_DID),
            &did_resolver,
            &secrets_resolver,
            test_cb,
        );

        let res = get_pack_result(cb_id).await;
        assert!(res.contains("payload"));
    }
}
