// This file is generated automatically using wasmcloud/weld-codegen and smithy model definitions
//

#[allow(unused_imports)]
use crate::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    RpcError, RpcResult, Timestamp,
};
#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};

pub const SMITHY_VERSION: &str = "1.0";

/// List of linked actors for a provider
pub type ActorLinks = Vec<LinkDefinition>;

// Decode ActorLinks from cbor input stream
// This is part of experimental cbor support
#[doc(hidden)]
pub fn decode_actor_links<'b>(d: &mut crate::cbor::Decoder<'b>) -> Result<ActorLinks, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<LinkDefinition> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(
                    decode_link_definition(d)
                        .map_err(|e| format!("decoding 'LinkDefinition': {}", e))?,
                )
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<LinkDefinition> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(crate::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_link_definition(d)
                            .map_err(|e| format!("decoding 'LinkDefinition': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
pub type ClusterIssuerKey = String;

// Decode ClusterIssuerKey from cbor input stream
// This is part of experimental cbor support
#[doc(hidden)]
pub fn decode_cluster_issuer_key<'b>(
    d: &mut crate::cbor::Decoder<'b>,
) -> Result<ClusterIssuerKey, RpcError> {
    let __result = { d.str()?.to_string() };
    Ok(__result)
}
pub type ClusterIssuers = Vec<ClusterIssuerKey>;

// Decode ClusterIssuers from cbor input stream
// This is part of experimental cbor support
#[doc(hidden)]
pub fn decode_cluster_issuers<'b>(
    d: &mut crate::cbor::Decoder<'b>,
) -> Result<ClusterIssuers, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<ClusterIssuerKey> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(
                    decode_cluster_issuer_key(d)
                        .map_err(|e| format!("decoding 'ClusterIssuerKey': {}", e))?,
                )
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<ClusterIssuerKey> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(crate::cbor::Type::Break) => break,
                    Ok(_) => arr.push(
                        decode_cluster_issuer_key(d)
                            .map_err(|e| format!("decoding 'ClusterIssuerKey': {}", e))?,
                    ),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// health check request parameter
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct HealthCheckRequest {}

// Decode HealthCheckRequest from cbor input stream
// This is part of experimental cbor support
#[doc(hidden)]
pub fn decode_health_check_request<'b>(
    d: &mut crate::cbor::Decoder<'b>,
) -> Result<HealthCheckRequest, RpcError> {
    let __result = {
        let is_array = match d.datatype()? {
            crate::cbor::Type::Array => true,
            crate::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct HealthCheckRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct HealthCheckRequest: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                d.skip()?;
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct HealthCheckRequest: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                d.str()?;
                d.skip()?;
            }
        }
        HealthCheckRequest {}
    };
    Ok(__result)
}
/// Return value from actors and providers for health check status
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct HealthCheckResponse {
    /// A flag that indicates the the actor is healthy
    #[serde(default)]
    pub healthy: bool,
    /// A message containing additional information about the actors health
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// Decode HealthCheckResponse from cbor input stream
// This is part of experimental cbor support
#[doc(hidden)]
pub fn decode_health_check_response<'b>(
    d: &mut crate::cbor::Decoder<'b>,
) -> Result<HealthCheckResponse, RpcError> {
    let __result = {
        let mut healthy: Option<bool> = None;
        let mut message: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            crate::cbor::Type::Array => true,
            crate::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct HealthCheckResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct HealthCheckResponse: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => healthy = Some(d.bool()?),
                    1 => {
                        message = if crate::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct HealthCheckResponse: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "healthy" => healthy = Some(d.bool()?),
                    "message" => {
                        message = if crate::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        HealthCheckResponse {
            healthy: if let Some(__x) = healthy {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HealthCheckResponse.healthy (#0)".to_string(),
                ));
            },
            message: message.unwrap(),
        }
    };
    Ok(__result)
}
/// initialization data for a capability provider
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct HostData {
    #[serde(default)]
    pub host_id: String,
    #[serde(default)]
    pub lattice_rpc_prefix: String,
    #[serde(default)]
    pub link_name: String,
    #[serde(default)]
    pub lattice_rpc_user_jwt: String,
    #[serde(default)]
    pub lattice_rpc_user_seed: String,
    #[serde(default)]
    pub lattice_rpc_url: String,
    #[serde(default)]
    pub provider_key: String,
    #[serde(default)]
    pub invocation_seed: String,
    pub env_values: HostEnvValues,
    #[serde(default)]
    pub instance_id: String,
    /// initial list of links for provider
    pub link_definitions: ActorLinks,
    /// list of cluster issuers
    pub cluster_issuers: ClusterIssuers,
    /// Optional configuration JSON sent to a given link name of a provider
    /// without an actor context
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_json: Option<String>,
}

// Decode HostData from cbor input stream
// This is part of experimental cbor support
#[doc(hidden)]
pub fn decode_host_data<'b>(d: &mut crate::cbor::Decoder<'b>) -> Result<HostData, RpcError> {
    let __result = {
        let mut host_id: Option<String> = None;
        let mut lattice_rpc_prefix: Option<String> = None;
        let mut link_name: Option<String> = None;
        let mut lattice_rpc_user_jwt: Option<String> = None;
        let mut lattice_rpc_user_seed: Option<String> = None;
        let mut lattice_rpc_url: Option<String> = None;
        let mut provider_key: Option<String> = None;
        let mut invocation_seed: Option<String> = None;
        let mut env_values: Option<HostEnvValues> = None;
        let mut instance_id: Option<String> = None;
        let mut link_definitions: Option<ActorLinks> = None;
        let mut cluster_issuers: Option<ClusterIssuers> = None;
        let mut config_json: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            crate::cbor::Type::Array => true,
            crate::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct HostData, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct HostData: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => host_id = Some(d.str()?.to_string()),
                    1 => lattice_rpc_prefix = Some(d.str()?.to_string()),
                    2 => link_name = Some(d.str()?.to_string()),
                    3 => lattice_rpc_user_jwt = Some(d.str()?.to_string()),
                    4 => lattice_rpc_user_seed = Some(d.str()?.to_string()),
                    5 => lattice_rpc_url = Some(d.str()?.to_string()),
                    6 => provider_key = Some(d.str()?.to_string()),
                    7 => invocation_seed = Some(d.str()?.to_string()),
                    8 => {
                        env_values = Some(
                            decode_host_env_values(d)
                                .map_err(|e| format!("decoding 'HostEnvValues': {}", e))?,
                        )
                    }
                    9 => instance_id = Some(d.str()?.to_string()),
                    10 => {
                        link_definitions = Some(
                            decode_actor_links(d)
                                .map_err(|e| format!("decoding 'ActorLinks': {}", e))?,
                        )
                    }
                    11 => {
                        cluster_issuers = Some(
                            decode_cluster_issuers(d)
                                .map_err(|e| format!("decoding 'ClusterIssuers': {}", e))?,
                        )
                    }
                    12 => {
                        config_json = if crate::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct HostData: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "hostId" => host_id = Some(d.str()?.to_string()),
                    "latticeRpcPrefix" => lattice_rpc_prefix = Some(d.str()?.to_string()),
                    "linkName" => link_name = Some(d.str()?.to_string()),
                    "latticeRpcUserJwt" => lattice_rpc_user_jwt = Some(d.str()?.to_string()),
                    "latticeRpcUserSeed" => lattice_rpc_user_seed = Some(d.str()?.to_string()),
                    "latticeRpcUrl" => lattice_rpc_url = Some(d.str()?.to_string()),
                    "providerKey" => provider_key = Some(d.str()?.to_string()),
                    "invocationSeed" => invocation_seed = Some(d.str()?.to_string()),
                    "envValues" => {
                        env_values = Some(
                            decode_host_env_values(d)
                                .map_err(|e| format!("decoding 'HostEnvValues': {}", e))?,
                        )
                    }
                    "instanceId" => instance_id = Some(d.str()?.to_string()),
                    "linkDefinitions" => {
                        link_definitions = Some(
                            decode_actor_links(d)
                                .map_err(|e| format!("decoding 'ActorLinks': {}", e))?,
                        )
                    }
                    "clusterIssuers" => {
                        cluster_issuers = Some(
                            decode_cluster_issuers(d)
                                .map_err(|e| format!("decoding 'ClusterIssuers': {}", e))?,
                        )
                    }
                    "configJson" => {
                        config_json = if crate::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        HostData {
            host_id: if let Some(__x) = host_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostData.host_id (#0)".to_string(),
                ));
            },

            lattice_rpc_prefix: if let Some(__x) = lattice_rpc_prefix {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostData.lattice_rpc_prefix (#1)".to_string(),
                ));
            },

            link_name: if let Some(__x) = link_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostData.link_name (#2)".to_string(),
                ));
            },

            lattice_rpc_user_jwt: if let Some(__x) = lattice_rpc_user_jwt {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostData.lattice_rpc_user_jwt (#3)".to_string(),
                ));
            },

            lattice_rpc_user_seed: if let Some(__x) = lattice_rpc_user_seed {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostData.lattice_rpc_user_seed (#4)".to_string(),
                ));
            },

            lattice_rpc_url: if let Some(__x) = lattice_rpc_url {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostData.lattice_rpc_url (#5)".to_string(),
                ));
            },

            provider_key: if let Some(__x) = provider_key {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostData.provider_key (#6)".to_string(),
                ));
            },

            invocation_seed: if let Some(__x) = invocation_seed {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostData.invocation_seed (#7)".to_string(),
                ));
            },

            env_values: if let Some(__x) = env_values {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostData.env_values (#8)".to_string(),
                ));
            },

            instance_id: if let Some(__x) = instance_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostData.instance_id (#9)".to_string(),
                ));
            },

            link_definitions: if let Some(__x) = link_definitions {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostData.link_definitions (#10)".to_string(),
                ));
            },

            cluster_issuers: if let Some(__x) = cluster_issuers {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field HostData.cluster_issuers (#11)".to_string(),
                ));
            },
            config_json: config_json.unwrap(),
        }
    };
    Ok(__result)
}
/// Environment settings for initializing a capability provider
pub type HostEnvValues = std::collections::HashMap<String, String>;

// Decode HostEnvValues from cbor input stream
// This is part of experimental cbor support
#[doc(hidden)]
pub fn decode_host_env_values<'b>(
    d: &mut crate::cbor::Decoder<'b>,
) -> Result<HostEnvValues, RpcError> {
    let __result = {
        {
            let mut m: std::collections::HashMap<String, String> =
                std::collections::HashMap::default();
            if let Some(n) = d.map()? {
                for _ in 0..(n as usize) {
                    let k = d.str()?.to_string();
                    let v = d.str()?.to_string();
                    m.insert(k, v);
                }
            } else {
                return Err(RpcError::Deser("indefinite maps not supported".to_string()));
            }
            m
        }
    };
    Ok(__result)
}
/// RPC message to capability provider
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Invocation {
    pub origin: WasmCloudEntity,
    pub target: WasmCloudEntity,
    #[serde(default)]
    pub operation: String,
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub msg: Vec<u8>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub encoded_claims: String,
    #[serde(default)]
    pub host_id: String,
}

// Decode Invocation from cbor input stream
// This is part of experimental cbor support
#[doc(hidden)]
pub fn decode_invocation<'b>(d: &mut crate::cbor::Decoder<'b>) -> Result<Invocation, RpcError> {
    let __result = {
        let mut origin: Option<WasmCloudEntity> = None;
        let mut target: Option<WasmCloudEntity> = None;
        let mut operation: Option<String> = None;
        let mut msg: Option<Vec<u8>> = None;
        let mut id: Option<String> = None;
        let mut encoded_claims: Option<String> = None;
        let mut host_id: Option<String> = None;

        let is_array = match d.datatype()? {
            crate::cbor::Type::Array => true,
            crate::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Invocation, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct Invocation: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        origin = Some(
                            decode_wasm_cloud_entity(d)
                                .map_err(|e| format!("decoding 'WasmCloudEntity': {}", e))?,
                        )
                    }
                    1 => {
                        target = Some(
                            decode_wasm_cloud_entity(d)
                                .map_err(|e| format!("decoding 'WasmCloudEntity': {}", e))?,
                        )
                    }
                    2 => operation = Some(d.str()?.to_string()),
                    3 => msg = Some(d.bytes()?.to_vec()),
                    4 => id = Some(d.str()?.to_string()),
                    5 => encoded_claims = Some(d.str()?.to_string()),
                    6 => host_id = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct Invocation: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "origin" => {
                        origin = Some(
                            decode_wasm_cloud_entity(d)
                                .map_err(|e| format!("decoding 'WasmCloudEntity': {}", e))?,
                        )
                    }
                    "target" => {
                        target = Some(
                            decode_wasm_cloud_entity(d)
                                .map_err(|e| format!("decoding 'WasmCloudEntity': {}", e))?,
                        )
                    }
                    "operation" => operation = Some(d.str()?.to_string()),
                    "msg" => msg = Some(d.bytes()?.to_vec()),
                    "id" => id = Some(d.str()?.to_string()),
                    "encodedClaims" => encoded_claims = Some(d.str()?.to_string()),
                    "hostId" => host_id = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        Invocation {
            origin: if let Some(__x) = origin {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Invocation.origin (#0)".to_string(),
                ));
            },

            target: if let Some(__x) = target {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Invocation.target (#1)".to_string(),
                ));
            },

            operation: if let Some(__x) = operation {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Invocation.operation (#2)".to_string(),
                ));
            },

            msg: if let Some(__x) = msg {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Invocation.msg (#3)".to_string(),
                ));
            },

            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Invocation.id (#4)".to_string(),
                ));
            },

            encoded_claims: if let Some(__x) = encoded_claims {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Invocation.encoded_claims (#5)".to_string(),
                ));
            },

            host_id: if let Some(__x) = host_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Invocation.host_id (#6)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Response to an invocation
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvocationResponse {
    /// serialize response message
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub msg: Vec<u8>,
    /// id connecting this response to the invocation
    #[serde(default)]
    pub invocation_id: String,
    /// optional error message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// Decode InvocationResponse from cbor input stream
// This is part of experimental cbor support
#[doc(hidden)]
pub fn decode_invocation_response<'b>(
    d: &mut crate::cbor::Decoder<'b>,
) -> Result<InvocationResponse, RpcError> {
    let __result = {
        let mut msg: Option<Vec<u8>> = None;
        let mut invocation_id: Option<String> = None;
        let mut error: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            crate::cbor::Type::Array => true,
            crate::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct InvocationResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct InvocationResponse: indefinite array not supported"
                        .to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => msg = Some(d.bytes()?.to_vec()),
                    1 => invocation_id = Some(d.str()?.to_string()),
                    2 => {
                        error = if crate::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct InvocationResponse: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "msg" => msg = Some(d.bytes()?.to_vec()),
                    "invocationId" => invocation_id = Some(d.str()?.to_string()),
                    "error" => {
                        error = if crate::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        InvocationResponse {
            msg: if let Some(__x) = msg {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field InvocationResponse.msg (#0)".to_string(),
                ));
            },

            invocation_id: if let Some(__x) = invocation_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field InvocationResponse.invocation_id (#1)".to_string(),
                ));
            },
            error: error.unwrap(),
        }
    };
    Ok(__result)
}
/// Link definition for binding actor to provider
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LinkDefinition {
    /// actor public key
    #[serde(default)]
    pub actor_id: String,
    /// provider public key
    #[serde(default)]
    pub provider_id: String,
    /// link name
    #[serde(default)]
    pub link_name: String,
    /// contract id
    #[serde(default)]
    pub contract_id: String,
    pub values: LinkSettings,
}

// Decode LinkDefinition from cbor input stream
// This is part of experimental cbor support
#[doc(hidden)]
pub fn decode_link_definition<'b>(
    d: &mut crate::cbor::Decoder<'b>,
) -> Result<LinkDefinition, RpcError> {
    let __result = {
        let mut actor_id: Option<String> = None;
        let mut provider_id: Option<String> = None;
        let mut link_name: Option<String> = None;
        let mut contract_id: Option<String> = None;
        let mut values: Option<LinkSettings> = None;

        let is_array = match d.datatype()? {
            crate::cbor::Type::Array => true,
            crate::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct LinkDefinition, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct LinkDefinition: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => actor_id = Some(d.str()?.to_string()),
                    1 => provider_id = Some(d.str()?.to_string()),
                    2 => link_name = Some(d.str()?.to_string()),
                    3 => contract_id = Some(d.str()?.to_string()),
                    4 => {
                        values = Some(
                            decode_link_settings(d)
                                .map_err(|e| format!("decoding 'LinkSettings': {}", e))?,
                        )
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct LinkDefinition: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "actorId" => actor_id = Some(d.str()?.to_string()),
                    "providerId" => provider_id = Some(d.str()?.to_string()),
                    "linkName" => link_name = Some(d.str()?.to_string()),
                    "contractId" => contract_id = Some(d.str()?.to_string()),
                    "values" => {
                        values = Some(
                            decode_link_settings(d)
                                .map_err(|e| format!("decoding 'LinkSettings': {}", e))?,
                        )
                    }
                    _ => d.skip()?,
                }
            }
        }
        LinkDefinition {
            actor_id: if let Some(__x) = actor_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field LinkDefinition.actor_id (#0)".to_string(),
                ));
            },

            provider_id: if let Some(__x) = provider_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field LinkDefinition.provider_id (#1)".to_string(),
                ));
            },

            link_name: if let Some(__x) = link_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field LinkDefinition.link_name (#2)".to_string(),
                ));
            },

            contract_id: if let Some(__x) = contract_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field LinkDefinition.contract_id (#3)".to_string(),
                ));
            },

            values: if let Some(__x) = values {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field LinkDefinition.values (#4)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Settings associated with an actor-provider link
pub type LinkSettings = std::collections::HashMap<String, String>;

// Decode LinkSettings from cbor input stream
// This is part of experimental cbor support
#[doc(hidden)]
pub fn decode_link_settings<'b>(
    d: &mut crate::cbor::Decoder<'b>,
) -> Result<LinkSettings, RpcError> {
    let __result = {
        {
            let mut m: std::collections::HashMap<String, String> =
                std::collections::HashMap::default();
            if let Some(n) = d.map()? {
                for _ in 0..(n as usize) {
                    let k = d.str()?.to_string();
                    let v = d.str()?.to_string();
                    m.insert(k, v);
                }
            } else {
                return Err(RpcError::Deser("indefinite maps not supported".to_string()));
            }
            m
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct WasmCloudEntity {
    #[serde(default)]
    pub public_key: String,
    #[serde(default)]
    pub link_name: String,
    pub contract_id: crate::model::CapabilityContractId,
}

// Decode WasmCloudEntity from cbor input stream
// This is part of experimental cbor support
#[doc(hidden)]
pub fn decode_wasm_cloud_entity<'b>(
    d: &mut crate::cbor::Decoder<'b>,
) -> Result<WasmCloudEntity, RpcError> {
    let __result = {
        let mut public_key: Option<String> = None;
        let mut link_name: Option<String> = None;
        let mut contract_id: Option<crate::model::CapabilityContractId> = None;

        let is_array = match d.datatype()? {
            crate::cbor::Type::Array => true,
            crate::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct WasmCloudEntity, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.array()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct WasmCloudEntity: indefinite array not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => public_key = Some(d.str()?.to_string()),
                    1 => link_name = Some(d.str()?.to_string()),
                    2 => contract_id = Some(crate::model::decode_capability_contract_id(d)?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.map()?.ok_or_else(|| {
                RpcError::Deser(
                    "decoding struct WasmCloudEntity: indefinite map not supported".to_string(),
                )
            })?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "publicKey" => public_key = Some(d.str()?.to_string()),
                    "linkName" => link_name = Some(d.str()?.to_string()),
                    "contractId" => {
                        contract_id = Some(crate::model::decode_capability_contract_id(d)?)
                    }
                    _ => d.skip()?,
                }
            }
        }
        WasmCloudEntity {
            public_key: if let Some(__x) = public_key {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field WasmCloudEntity.public_key (#0)".to_string(),
                ));
            },

            link_name: if let Some(__x) = link_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field WasmCloudEntity.link_name (#1)".to_string(),
                ));
            },

            contract_id: if let Some(__x) = contract_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field WasmCloudEntity.contract_id (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Actor service
/// wasmbus.actorReceive
#[async_trait]
pub trait Actor {
    /// Perform health check. Called at regular intervals by host
    async fn health_request(
        &self,
        ctx: &Context,
        arg: &HealthCheckRequest,
    ) -> RpcResult<HealthCheckResponse>;
}

/// ActorReceiver receives messages defined in the Actor service trait
/// Actor service
#[doc(hidden)]
#[async_trait]
pub trait ActorReceiver: MessageDispatch + Actor {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "HealthRequest" => {
                let value: HealthCheckRequest =
                    crate::common::decode(&message.arg, &decode_health_check_request)
                        .map_err(|e| RpcError::Deser(format!("'HealthCheckRequest': {}", e)))?;
                let resp = Actor::health_request(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Actor.HealthRequest",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Actor::{}",
                message.method
            ))),
        }
    }
}

/// ActorSender sends messages to a Actor service
/// Actor service
/// client for sending Actor messages
#[derive(Debug)]
pub struct ActorSender<T: Transport> {
    transport: T,
}

impl<T: Transport> ActorSender<T> {
    /// Constructs a ActorSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> ActorSender<crate::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send crate::core::LinkDefinition) -> Self {
        Self {
            transport: crate::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl ActorSender<crate::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport = crate::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Actor for ActorSender<T> {
    #[allow(unused)]
    /// Perform health check. Called at regular intervals by host
    async fn health_request(
        &self,
        ctx: &Context,
        arg: &HealthCheckRequest,
    ) -> RpcResult<HealthCheckResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Actor.HealthRequest",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: HealthCheckResponse =
            crate::common::decode(&resp, &decode_health_check_response)
                .map_err(|e| RpcError::Deser(format!("'{}': HealthCheckResponse", e)))?;
        Ok(value)
    }
}
