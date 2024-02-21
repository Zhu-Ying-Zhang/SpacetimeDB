// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UpdatePkStringArgs {
    pub s: String,
    pub data: i32,
}

impl Reducer for UpdatePkStringArgs {
    const REDUCER_NAME: &'static str = "update_pk_string";
}

#[allow(unused)]
pub fn update_pk_string(s: String, data: i32) {
    UpdatePkStringArgs { s, data }.invoke();
}

#[allow(unused)]
pub fn on_update_pk_string(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &String, &i32) + Send + 'static,
) -> ReducerCallbackId<UpdatePkStringArgs> {
    UpdatePkStringArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let UpdatePkStringArgs { s, data } = __args;
        __callback(__identity, __addr, __status, s, data);
    })
}

#[allow(unused)]
pub fn once_on_update_pk_string(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &String, &i32) + Send + 'static,
) -> ReducerCallbackId<UpdatePkStringArgs> {
    UpdatePkStringArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let UpdatePkStringArgs { s, data } = __args;
        __callback(__identity, __addr, __status, s, data);
    })
}

#[allow(unused)]
pub fn remove_on_update_pk_string(id: ReducerCallbackId<UpdatePkStringArgs>) {
    UpdatePkStringArgs::remove_on_reducer(id);
}
