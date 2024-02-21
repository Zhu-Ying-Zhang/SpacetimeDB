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
pub struct DeletePkBoolArgs {
    pub b: bool,
}

impl Reducer for DeletePkBoolArgs {
    const REDUCER_NAME: &'static str = "delete_pk_bool";
}

#[allow(unused)]
pub fn delete_pk_bool(b: bool) {
    DeletePkBoolArgs { b }.invoke();
}

#[allow(unused)]
pub fn on_delete_pk_bool(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &bool) + Send + 'static,
) -> ReducerCallbackId<DeletePkBoolArgs> {
    DeletePkBoolArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let DeletePkBoolArgs { b } = __args;
        __callback(__identity, __addr, __status, b);
    })
}

#[allow(unused)]
pub fn once_on_delete_pk_bool(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &bool) + Send + 'static,
) -> ReducerCallbackId<DeletePkBoolArgs> {
    DeletePkBoolArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let DeletePkBoolArgs { b } = __args;
        __callback(__identity, __addr, __status, b);
    })
}

#[allow(unused)]
pub fn remove_on_delete_pk_bool(id: ReducerCallbackId<DeletePkBoolArgs>) {
    DeletePkBoolArgs::remove_on_reducer(id);
}
