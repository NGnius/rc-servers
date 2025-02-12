use std::collections::HashMap;

use polariton::operation::{Typed, ParameterTable, OperationResponse, Dict};
use polariton_server::operations::{Operation, OperationCode};

pub struct QualityConfigTeller;

impl Operation for QualityConfigTeller {
    type State = ();
    type User = crate::UserTy;

    fn handle(&self, _: ParameterTable, _: &mut Self::State, _: &Self::User) -> OperationResponse {
        let quality_levels = Typed::HashMap(vec![
            (Typed::Str("extremLow".into()), Typed::Dict(Dict {
                key_ty: 115,
                val_ty: 42,
                items: vec![
                    (Typed::Str("Level".into()), Typed::Long(0)),
                    (Typed::Str("default".into()), Typed::Float(0.0)),
                ],
            })),
            (Typed::Str("low".into()), Typed::Dict(Dict {
                key_ty: 115,
                val_ty: 42,
                items: vec![
                    (Typed::Str("Level".into()), Typed::Long(1)),
                    (Typed::Str("default".into()), Typed::Float(0.0)),
                ],
            })),
            (Typed::Str("normal".into()), Typed::Dict(Dict {
                key_ty: 115,
                val_ty: 42,
                items: vec![
                    (Typed::Str("Level".into()), Typed::Long(2)),
                    (Typed::Str("default".into()), Typed::Float(0.0)),
                ],
            })),
            (Typed::Str("beautiful".into()), Typed::Dict(Dict {
                key_ty: 115,
                val_ty: 42,
                items: vec![
                    (Typed::Str("Level".into()), Typed::Long(3)),
                    (Typed::Str("default".into()), Typed::Float(0.0)),
                ],
            })),
            (Typed::Str("fantastic".into()), Typed::Dict(Dict {
                key_ty: 115,
                val_ty: 42,
                items: vec![
                    (Typed::Str("Level".into()), Typed::Long(4)),
                    (Typed::Str("default".into()), Typed::Float(f32::MAX)),
                ],
            })),
        ].into());
        let mem_thresholds = Typed::HashMap(vec![
            (Typed::Str("low".into()), Typed::Int(69)),
            (Typed::Str("extremeLow".into()), Typed::Int(42)),
        ].into());
        let mut resp_params = HashMap::new();
        resp_params.insert(1 /* dict<string, hashtable> */, Typed::Dict(
            Dict {
                key_ty: 115, // str
                val_ty: 104, // hash table
                items: vec![
                    (Typed::Str("qualityLevels".into()), quality_levels),
                    (Typed::Str("systemMemoryThresholds".into()), mem_thresholds),
                ],
            }));
        OperationResponse {
            code: 104,
            return_code: 0,
            message: Typed::Null,
            params: resp_params.into(),
        }
    }
}

impl OperationCode for QualityConfigTeller {
    fn op_code() -> u8 {
        104
    }
}
