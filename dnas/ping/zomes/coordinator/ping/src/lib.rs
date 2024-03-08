use hdk::prelude::*;
use ping_integrity::*;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let zome_name = zome_info()?.name;
    let function = FunctionName::from("ping");
    let functions = GrantedFunctions::Listed(BTreeSet::from([(zome_name, function)]));
    let cap_grant_entry = ZomeCallCapGrant {
        tag: "".into(),
        access: CapAccess::Unrestricted,
        functions,
    };
    create_cap_grant(cap_grant_entry)?;
    Ok(InitCallbackResult::Pass)
}
#[hdk_extern]
pub fn ping_agent(agent: AgentPubKey) -> ExternResult<String> {
    let zome_name = zome_info()?.name;
    let fn_name = FunctionName::from("ping");
    let resp = call_remote(agent, zome_name, fn_name, None, ())?;
    match resp {
        ZomeCallResponse::CountersigningSession(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Unexpected countersigning session".into()
        ))),
        ZomeCallResponse::NetworkError(err) => Err(wasm_error!(WasmErrorInner::Guest(format!(
            "Network error: {}",
            err
        )))),
        ZomeCallResponse::Unauthorized(..) => {
            Err(wasm_error!(WasmErrorInner::Guest("Unauthorised".into())))
        }
        ZomeCallResponse::Ok(message) => message.decode().map_err(|_| {
            wasm_error!(WasmErrorInner::Guest(
                "Couldn't deserialise response".into()
            ))
        }),
    }
}
#[hdk_extern]
pub fn ping(_: ()) -> ExternResult<String> {
    Ok("pong".into())
}

/// Don't modify this enum if you want the scaffolding tool to generate appropriate signals for your entries and links
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Signal {}

/// Whenever an action is committed, we emit a signal to the UI elements to reactively update them
#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    /// Don't modify this loop if you want the scaffolding tool to generate appropriate signals for your entries and links
    for action in committed_actions {
        if let Err(err) = signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}

/// Don't modify this function if you want the scaffolding tool to generate appropriate signals for your entries and links
fn signal_action(action: SignedActionHashed) -> ExternResult<()> {
    Ok(())
}
