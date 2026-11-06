#![allow(async_fn_in_trait)]

pub trait DatabaseRepository: Sized {
    type Id: Clone + Copy;
    type New;
    type Model;
    async fn create(&self, insertable: Self::New) -> anyhow::Result<()>;
    async fn delete(&self, id: Self::Id) -> anyhow::Result<()>;
    async fn update(&self, id: Self::Id, updateable: Self::New) -> anyhow::Result<()>;
    async fn read(&self, id: Self::Id) -> anyhow::Result<Self::Model>;
    async fn read_all(&self) -> anyhow::Result<Vec<Self::Model>>;
}

/// ($model_repository:ty,$model:ty, $table: ident, $id:ty,$new:ty ,$changes:ty) => {
#[macro_export]
macro_rules! impl_repository {
    ($model_repository:ty,$model:ty, $table: ident, $id:ty ,$new:ty ) => {
        impl $crate::core::database::repository::DatabaseRepository for $model_repository {
            type Id = $id;
            type New = $new;
            type Model = $model;

            async fn create(&self, new: $new) -> anyhow::Result<()> {
                use diesel::prelude::*;
                $crate::core::database::connection::interact_with_connection(&self.pool, |conn| {
                    diesel::insert_into($table::table)
                        .values(new)
                        .execute(conn)
                        .map(|_| ())
                })
                .await
            }
            async fn read(&self, id: $id) -> anyhow::Result<$model> {
                use diesel::prelude::*;
                let id = id.clone();
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        $table::table
                            .find(id)
                            .select(<$model>::as_select())
                            .first(conn)
                    },
                )
                .await
            }

            async fn read_all(&self) -> anyhow::Result<Vec<$model>> {
                use diesel::prelude::*;
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        $table::table
                            .select(<$model>::as_select())
                            .get_results(conn)
                    },
                )
                .await
            }
            async fn update(&self, id: $id, changes: $new) -> anyhow::Result<()> {
                use diesel::prelude::*;
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        diesel::update($table::table.find(id))
                            .set(changes)
                            .returning(<$model>::as_returning())
                            .execute(conn)
                            .map(|_| ())
                    },
                )
                .await
            }
            async fn delete(&self, id: $id) -> anyhow::Result<()> {
                use diesel::prelude::*;
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        diesel::delete($table::table.find(id))
                            .execute(conn)
                            .map(|_| ())
                    },
                )
                .await
            }
        }
    };
    // Es factible que tenga que definir otro trait, ya que perdes control fino, quiero borrar
    // siempre todo? o quiero controlar uno por uno, que actualizo? que tipo seria?
    ($model_repository:ty,$model:ty, $table: ident, $id:ty ,$new:ty, $dependency:ty ) => {
        impl $crate::core::database::repository::DatabaseRepository for $model_repository {
            type Id = $id;
            type New = $new;
            type Model = $model;

            async fn create(&self, new: $new) -> anyhow::Result<()> {
                use diesel::prelude::*;
                $crate::core::database::connection::interact_with_connection(&self.pool, |conn| {
                    diesel::insert_into($table::table)
                        .values(new)
                        .execute(conn)
                        .map(|_| ())
                })
                .await
            }
            async fn read(&self, id: $id) -> anyhow::Result<$model> {
                use diesel::prelude::*;
                let id = id.clone();
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        $table::table
                            .find(id)
                            .select(<$model>::as_select())
                            .first(conn)
                    },
                )
                .await
            }

            async fn update(&self, id: $id, changes: $new) -> anyhow::Result<()> {
                use diesel::prelude::*;
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        diesel::update($table::table.find(id))
                            .set(changes)
                            .returning(<$model>::as_returning())
                            .execute(conn)
                            .map(|_| ())
                    },
                )
                .await
            }
            async fn delete(&self, id: $id) -> anyhow::Result<()> {
                use diesel::prelude::*;
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        diesel::delete($table::table.find(id))
                            .execute(conn)
                            .map(|_| ())
                    },
                )
                .await
            }
        }
    };
}
