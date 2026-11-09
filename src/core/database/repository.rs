#![allow(async_fn_in_trait)]

use diesel::PgConnection;
// Traits para repository

pub trait DatabaseRepository: Sized {
    type Id: Clone + Copy;
    type Model;
    type New;
    fn _create(
        &self,
        insertable: Self::New,
        conn: &mut PgConnection,
        parent_id: Option<Self::Id>,
    ) -> anyhow::Result<Self::Model>;
    fn _delete(&self, id: Self::Id, conn: &mut PgConnection) -> anyhow::Result<()>;
    fn _update(
        &self,
        id: Self::Id,
        updateable: Self::New,
        conn: &mut PgConnection,
    ) -> anyhow::Result<Self::Model>;
    fn _read(&self, id: Self::Id, conn: &mut PgConnection) -> anyhow::Result<Self::Model>;
    fn _read_all(&self, conn: &mut PgConnection) -> anyhow::Result<Vec<Self::Model>>;
}
pub trait ParentRepo {}
pub trait ChildRepo {}
// P no se menciona explicitamente en la invocacion, es el modelo
// Igualmente se le solicita a Modelo implementar Parent y a
// los children implementar Child. Esa implementación la hace el programador
pub trait AsyncLayerWithHierarchy: DatabaseRepository {
    type P: ParentRepo;
    type C: ChildRepo;
    type N: ParentWithChildNew;

    async fn create(&self, parent: Self::P, child: Self::C, new: Self::N) -> anyhow::Result<()>;
    async fn delete(&self, id: Self::Id, parent: Self::P, child: Self::C) -> anyhow::Result<()>;
    async fn update(
        &self,
        id: Self::Id,
        parent: Self::P,
        child: Self::C,
        new: Self::N,
    ) -> anyhow::Result<()>;
    async fn read(&self, id: Self::Id, parent: Self::P) -> anyhow::Result<Self::Model>;
    async fn read_all(&self) -> anyhow::Result<Vec<Self::Model>>;
}

// Acá podemos pasar directamente el modelo (no se requiere implementar parent).
// Como no se requiere implementar Parent ni child pasa directo sin decir nada.
pub trait AsyncLayer: DatabaseRepository {
    async fn create(&self, insertable: Self::New) -> anyhow::Result<Self::Model>;
    async fn delete(&self, id: Self::Id) -> anyhow::Result<()>;
    async fn update(&self, id: Self::Id, updateable: Self::New) -> anyhow::Result<Self::Model>;
    async fn read(&self, id: Self::Id) -> anyhow::Result<Self::Model>;
    async fn read_all(&self) -> anyhow::Result<Vec<Self::Model>>;
}

// Traits para modelos
pub trait ParentWithChildNew {
    fn get_child(&self) -> impl Child;
    fn get_parent(&self) -> impl Parent;
}
pub trait Child {
    fn set_id(&mut self, val: i32) {}
}
pub trait Parent {}
#[macro_export]
macro_rules! impl_repository {
    (@repository, $model: ty ) => {
        pastey::paste!{
 #[derive(Debug, Clone)]
            pub struct [<$model Repostory>] {
                pool: $crate::core::database::connection::DbPool
        }
            impl [<$model Repostory>] {
                fn new(pool: $crate::core::database::connection::DbPool) -> Self {
                Self { pool }
        }
        }
        }
    };


    // Acá hay un problema, tiene que haber una de estas para los plain y otra
    // para los children-parent. No es necesario que sea una opcion
    (@sync, $model:ident, $table:ident, $id:ty ,$new:ty ) => {
        pastey::paste!{
        impl_repository!(@repository, $model);
        impl $crate::core::database::repository::DatabaseRepository for [<$model Repostory>] {
            type Id = $id;
            type New = $new;
            type Model = $model;

            fn _create(
                &self,
                mut new: $new,
                conn: &mut diesel::prelude::PgConnection,
                parent_id: Option<$id>
            ) -> anyhow::Result<$model> {
                use diesel::prelude::*;
                 diesel::insert_into($table::table)
                    .values(new)
                    .get_result(conn)
                    .map_err(Into::into)
            }
            fn _read(
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

            fn _read_all(
                &self,
                conn: &mut diesel::prelude::PgConnection,
            ) -> anyhow::Result<Vec<$model>> {
                use diesel::prelude::*;
                $table::table
                    .select(<$model>::as_select())
                    .get_results(conn)
                    .map_err(Into::into)
            }
            fn _update(
                &self,
                id: $id,
                changes: $new,
                conn: &mut diesel::prelude::PgConnection,
            ) -> anyhow::Result<$model> {
                use diesel::prelude::*;
                diesel::update($table::table.find(id))
                    .set(changes)
                    .returning(<$model>::as_returning())
                    .get_result(conn)
                    .map_err(Into::into)
            }
            fn _delete(
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
    }
    };

    (@sync, $model:ident, $table:ident, $id:ty ,$new:ty ,  $parent: ty $(, $children:ty)* $(,)?) => {
        pastey::paste!{
        impl_repository!(@repository, $model);
        impl $crate::core::database::repository::DatabaseRepository for [<$model Repostory>] {
            type Id = $id;
            type New = $new;
            type Model = $model;

            fn _create(
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
                    .returning(<$model>::as_returning())
                    .get_result(conn)
                    .map_err(Into::into)
                     ,
                    None =>
                 diesel::insert_into($table::table)
                    .values(new)
                    .returning(<$model>::as_returning())
                    .get_result(conn)
                    .map_err(Into::into)
                }
            }
            fn _read(
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

            fn _read_all(
                &self,
                conn: &mut diesel::prelude::PgConnection,
            ) -> anyhow::Result<Vec<$model>> {
                use diesel::prelude::*;
                $table::table
                    .select(<$model>::as_select())
                    .get_results(conn)
                    .map_err(Into::into)
            }
            fn _update(
                &self,
                id: $id,
                changes: $new,
                conn: &mut diesel::prelude::PgConnection,
            ) -> anyhow::Result<$model> {
                use diesel::prelude::*;
                diesel::update($table::table.find(id))
                    .set(changes)
                    .returning(<$model>::as_returning())
                    .get_result(conn)
                    .map_err(Into::into)
            }
            fn _delete(
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
    }
    };
    (sync, $model:ty, $table: ident, $id:ty ,$new:ty ) => {
        impl_repository!(@sync, $model, $table , $id, $new);
    };

    ( async,  $model:ident, $table: ident, $id:ty ,$new:ty  )  => {
        impl_repository!(@sync, $model, $table,  $id ,$new);
    pastey::paste!{
        impl $crate::core::database::repository::AsyncLayer for [<$model Repostory>]{
        async fn create(&self,insertable: $new) -> anyhow::Result<$model> {
                $crate::core::database::connection::interact_with_connection(&self.pool, |conn| {
                    self._create(insertable, conn, None).context("creating single")
                })
                .await
        }
        async fn delete(&self, id:$id) -> anyhow::Result<()> {
                $crate::core::database::connection::interact_with_connection(&self.pool, |conn| {
                    self._delete(id, conn).context("deleting single").map(|_|())
                })
                .await
        }

        async fn update(&self, id:$id, updateable: $new) -> anyhow::Result<$model> {
                $crate::core::database::connection::interact_with_connection(&self.pool, |conn| {
                    self._update(id, conn, updateable).context("updating single")
                })
                .await
        }

        async fn read(&self, id:$id ) -> anyhow::Result<$model> {
                $crate::core::database::connection::interact_with_connection(&self.pool, |conn| {
                    self._read(id, conn ).context("updating single")
                })
                .await
        }
        async fn read_all(&self  ) -> anyhow::Result<Vec<$model>> {
                $crate::core::database::connection::interact_with_connection(&self.pool, |conn| {
                    self._read_all( conn ).context("updating single")
                })
                .await
        }

        }
}
    };
    ( async,  $model:ident, $table: ident, $id:ty ,$new:ty,  $parent: ty $(, $children:ty)* $(,)?)  => {
        use diesel::prelude::*;
            impl_repository!(@sync, $model, $table , $id ,$new, $parent, $($children,)*);

        pastey::paste!{
       $( impl $crate::core::database::repository::AsyncLayerWithHierarchy for [<$model Repostory>]{
            // type Parent = $parent;
            // type Child =  $children;
            async fn create(&self, parent: $parent, child: $children, new: impl diesel::prelude::Insertable<$parent>) -> anyhow::Result<()> {
                $crate::core::database::connection::interact_with_connection(&self.pool, |conn| {
                    let parent = self._create(new, conn, None).context("Creating parent")?;
                     self._create(parent.get_child(), conn, Some(parent.get_id())).context("Creating child").map(|_| () )
                })
                .await
            }
            async fn read(&self, id: $id, parent: $parent) -> anyhow::Result<$model> {
                let id = id.clone();
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        self._read(id, conn)
                    },
                )
                .await
            }
            async fn update(&self, id: $id, parent: $parent, child :$children, new: impl Insertable<$parent>  ) -> anyhow::Result<()> {
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        parent._update(id, new, conn).context("Updating parent")?;
                        child._update(id, new.get_child(), conn).context("Updating child")
                    },
                )
                .await
            }
            async fn delete(&self, id: $id, parent: $parent, child: $children) -> anyhow::Result<()> {
                use diesel::prelude::*;
                $crate::core::database::connection::interact_with_connection(
                    &self.pool,
                    move |conn| {
                        child._delete(parent.get_child_id(), conn).context("Deleting child")?;
                        parent._delete(id, conn).context("Deleting parent")
                    },
                )
                .await
            }
        })*
    }

    };
}
