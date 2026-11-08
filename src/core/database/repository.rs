#![allow(async_fn_in_trait)]

use diesel::{Identifiable, Insertable, PgConnection, Queryable};

pub trait DatabaseRepository: Sized {
    type Id: Clone + Copy;
    type Model: Identifiable + Queryable;
    type New: Insertable<Model>;
    fn create(
        &self,
        insertable: Self::New,
        conn: &mut PgConnection,
        parent_id: Option<Self::Id>,
    ) -> anyhow::Result<Model>;
    fn delete(&self, id: Self::Id, conn: &mut PgConnection) -> anyhow::Result<()>;
    fn update(
        &self,
        id: Self::Id,
        updateable: Self::New,
        conn: &mut PgConnection,
    ) -> anyhow::Result<Model>;
    fn read(&self, id: Self::Id, conn: &mut PgConnection) -> anyhow::Result<Self::Model>;
    fn read_all(&self, conn: &mut PgConnection) -> anyhow::Result<Vec<Self::Model>>;
}

// Uno por par
pub trait AsyncLayerWithHierarchy<P, C>: DatabaseRepository
where
    P: Parent,
    C: Child,
{
    type Parent = P;
    type Child = C;

    fn create(&self, parent: P, child: C, new: impl Insertable<P>) -> anyhow::Result<()>;
    fn delete(&self, id: Self::Id, parent: P, child: C) -> anyhow::Result<()>;
    fn update(
        &self,
        id: Self::Id,
        parent: P,
        child: C,
        new: impl Insertable<P>,
    ) -> anyhow::Result<()>;
    fn read(&self, id: Self::Id, parent: P) -> anyhow::Result<Self::Model>;
    fn read_all(&self) -> anyhow::Result<Vec<Self::Model>>;
}

pub trait AsyncLayer: DatabaseRepository {
    fn create(&self, insertable: Self::New) -> anyhow::Result;
    fn delete(&self, id: Self::Id, conn: &mut PgConnection) -> anyhow::Result<()>;
    fn update(
        &self,
        id: Self::Id,
        updateable: Self::New,
        conn: &mut PgConnection,
    ) -> anyhow::Result<()>;
    fn read(&self, id: Self::Id, conn: &mut PgConnection) -> anyhow::Result<Self::Model>;
    fn read_all(&self, conn: &mut PgConnection) -> anyhow::Result<Vec<Self::Model>>;
}

pub trait Parent {
    fn get_id(&self) -> i32;
    fn get_child(&self) -> impl Child;
    fn get_child_id(&self) -> i32 {
        self.get_child().get_id()
    }
}
pub trait Child {
    fn get_id(&mut self) -> i32;
}

#[macro_export]
macro_rules! impl_repository {
    (@sync, $model_repository:ty,$model:ty, $table: ident, $id:ty ,$new:ty ) => {
        impl $crate::core::database::repository::DatabaseRepository for $model_repository {
            type Id = $id;
            type New = $new;
            type Model = $model;

            fn create(
                &self,
                mut new: $new,
                conn: &mut diesel::prelude::PgConnection,
                parent_id: Option<$id>
            ) -> anyhow::Result<$model> {
                use diesel::prelude::*;

                 match parent_id {
                    Some(id) =>
                 diesel::insert_into($table::table)
                    .values((new, $table::study_id.eq(id)))
                    .execute(conn)
                    .map_err(Into::into)
                     ,
                    None =>
                 diesel::insert_into($table::table)
                    .values(new)
                    .select(<$model>::as_select())
                    .get_result(conn)
                    .map_err(Into::into)
                }
            }
            fn read(
                &self,
                id: $id,
                conn: &mut diesel::prelude::PgConnection,
            ) -> anyhow::Result<$model> {
                use diesel::prelude::*;
                let id = id.clone();
                $table::table
                    .find(id)
                    .select(<$model>::as_select())
                    .first(conn)
                    .map_err(Into::into)
            }

            fn read_all(
                &self,
                conn: &mut diesel::prelude::PgConnection,
            ) -> anyhow::Result<Vec<$model>> {
                use diesel::prelude::*;
                $table::table
                    .select(<$model>::as_select())
                    .get_results(conn)
                    .map_err(Into::into)
            }
            fn update(
                &self,
                id: $id,
                changes: $new,
                conn: &mut diesel::prelude::PgConnection,
            ) -> anyhow::Result<()> {
                use diesel::prelude::*;
                diesel::update($table::table.find(id))
                    .set(changes)
                    .returning(<$model>::as_returning())
                    .get_result(conn)
                    .map(|_| ())
                    .map_err(Into::into)
            }
            fn delete(
                &self,
                id: $id,
                conn: &mut diesel::prelude::PgConnection,
            ) -> anyhow::Result<()> {
                use diesel::prelude::*;
                diesel::delete($table::table.find(id))
                    .execute(conn)
                    .map(|_| ())
                    .map_err(Into::into)
            }
        }
    };

    (sync, $model_repository:ty,$model:ty, $table: ident, $id:ty ,$new:ty ) => {
        impl_repository!(@sync, $model_repository,$model, $table , $id,$new);
    };

    ( async,  $model_repository:ty,$model:ty, $table: ident, $id:ty ,$new:ty  )  => {

            impl_repository!(@sync, $model_repository,$model, $table  $id ,$new)

    };
    ( async,  $model_repository:ty,$model:ty, $table: ident, $id:ty ,$new:ty,  $parent: ty ,$(, $children:ty)* $(,)?)  => {

            impl_repository!(@sync, $model_repository,$model, $table  $id ,$new)
        impl $crate::core::database::repository::AsyncLayerWithHierarchy<$parent, *(
        $children
        ) >  for $model_repository {
            type Parent = $parent;
            type Child =  $($children)*;
            async fn create(&self, parent: $parent, child: $($children)*, new: impl Insertable<$parent>) -> anyhow::Result<()> {
                use diesel::prelude::*;
                $crate::core::database::connection::interact_with_connection(&self.pool, |conn| {
                    let parent = parent.create(new, conn, None).context("Creating parent")?;
                     child.create(parent.get_child(), conn, Some(parent.get_id())).context("Creating child").map(|_| () )
                })
                .await
            }
            async fn read(&self, id: $id, parent: $parent) -> anyhow::Result<$model> {
                use diesel::prelude::*;
                let id = id.clone();
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        parent.read(id, conn)
                    },
                )
                .await
            }
            async fn update(&self, id: $id, parent: $parent, child :$($children)*, new: impl Insertable<$parent>) -> anyhow::Result<()> {
                use diesel::prelude::*;
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        parent.update(id, new, conn).context("Updating parent")?;
                        child.update(id, new.get_child(), conn).context("Updating child")
                    },
                )
                .await
            }
            async fn delete(&self, id: $id, parent: $parent, child: $($children)*) -> anyhow::Result<()> {
                use diesel::prelude::*;
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        child.delete(parent.get_child_id(), conn).context("Deleting child")?;
                        parent.delete(id, conn).context("Deleting parent")
                    },
                )
                .await
            }
        }
    };
}
