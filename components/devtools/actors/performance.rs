/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// TODO: Is this actor still relevant?
#![allow(dead_code)]

use serde::Serialize;
use serde_json::{Map, Value};

use crate::StreamId;
use crate::actor::{Actor, ActorError, ActorRegistry};
use crate::protocol::{ActorDescription, ClientRequest, Method};

pub struct PerformanceActor {
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PerformanceFeatures {
    with_markers: bool,
    with_memory: bool,
    with_ticks: bool,
    with_allocations: bool,
    #[serde(rename = "withJITOptimizations")]
    with_jitoptimizations: bool,
}

#[derive(Serialize)]
struct PerformanceTraits {
    features: PerformanceFeatures,
}

#[derive(Serialize)]
struct ConnectReply {
    from: String,
    traits: PerformanceTraits,
}

#[derive(Serialize)]
struct CanCurrentlyRecordReply {
    from: String,
    value: SuccessMsg,
}

#[derive(Serialize)]
struct SuccessMsg {
    success: bool,
    errors: Vec<Error>,
}

#[derive(Serialize)]
enum Error {}

impl Actor for PerformanceActor {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn handle_message(
        &self,
        request: ClientRequest,
        _registry: &ActorRegistry,
        msg_type: &str,
        _msg: &Map<String, Value>,
        _id: StreamId,
    ) -> Result<(), ActorError> {
        match msg_type {
            "connect" => {
                let msg = ConnectReply {
                    from: self.name(),
                    traits: PerformanceTraits {
                        features: PerformanceFeatures {
                            with_markers: true,
                            with_memory: true,
                            with_ticks: true,
                            with_allocations: true,
                            with_jitoptimizations: true,
                        },
                    },
                };
                request.reply_final(&msg)?
            },
            "canCurrentlyRecord" => {
                let msg = CanCurrentlyRecordReply {
                    from: self.name(),
                    value: SuccessMsg {
                        success: true,
                        errors: vec![],
                    },
                };
                request.reply_final(&msg)?
            },
            _ => return Err(ActorError::UnrecognizedPacketType),
        };
        Ok(())
    }
}

impl PerformanceActor {
    pub fn new(name: String) -> PerformanceActor {
        PerformanceActor { name }
    }

    pub fn description() -> ActorDescription {
        ActorDescription {
            category: "actor",
            type_name: "performance",
            methods: vec![Method {
                name: "canCurrentlyRecord",
                request: Value::Object(
                    vec![(
                        "type".to_owned(),
                        Value::String("canCurrentlyRecord".to_owned()),
                    )]
                    .into_iter()
                    .collect(),
                ),
                response: Value::Object(
                    vec![(
                        "value".to_owned(),
                        Value::Object(
                            vec![("_retval".to_owned(), Value::String("json".to_owned()))]
                                .into_iter()
                                .collect(),
                        ),
                    )]
                    .into_iter()
                    .collect(),
                ),
            }],
        }
    }
}
