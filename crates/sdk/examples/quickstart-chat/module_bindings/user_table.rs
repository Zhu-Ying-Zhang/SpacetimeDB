// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use super::user_type::User;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `user`.
///
/// Obtain a handle from the [`UserTableAccess::user`] method on [`super::RemoteTables`],
/// like `ctx.db.user()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.user().on_insert(...)`.
pub struct UserTableHandle<'ctx> {
    imp: __sdk::TableHandle<User>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `user`.
///
/// Implemented for [`super::RemoteTables`].
pub trait UserTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`UserTableHandle`], which mediates access to the table `user`.
    fn user(&self) -> UserTableHandle<'_>;
}

impl UserTableAccess for super::RemoteTables {
    fn user(&self) -> UserTableHandle<'_> {
        UserTableHandle {
            imp: self.imp.get_table::<User>("user"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct UserInsertCallbackId(__sdk::CallbackId);
pub struct UserDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for UserTableHandle<'ctx> {
    type Row = User;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = User> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = UserInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UserInsertCallbackId {
        UserInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: UserInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = UserDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UserDeleteCallbackId {
        UserDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: UserDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<User>("user");
    _table.add_unique_constraint::<__sdk::Identity>("identity", |row| &row.identity);
}
pub struct UserUpdateCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::TableWithPrimaryKey for UserTableHandle<'ctx> {
    type UpdateCallbackId = UserUpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> UserUpdateCallbackId {
        UserUpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: UserUpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<User>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<User>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}

/// Access to the `identity` unique index on the table `user`,
/// which allows point queries on the field of the same name
/// via the [`UserIdentityUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.user().identity().find(...)`.
pub struct UserIdentityUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<User, __sdk::Identity>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> UserTableHandle<'ctx> {
    /// Get a handle on the `identity` unique index on the table `user`.
    pub fn identity(&self) -> UserIdentityUnique<'ctx> {
        UserIdentityUnique {
            imp: self.imp.get_unique_constraint::<__sdk::Identity>("identity"),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> UserIdentityUnique<'ctx> {
    /// Find the subscribed row whose `identity` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &__sdk::Identity) -> Option<User> {
        self.imp.find(col_val)
    }
}
