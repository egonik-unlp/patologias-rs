//! `impl_repository!`: generates a `<Model>Repository` struct plus its
//! `DatabaseRepository` / `AsyncLayer` / `AsyncBelongsTo` /
//! `AsyncLayerWithHierarchy` impls. Exported at the crate root
//! (`crate::impl_repository!`).

#[macro_export]
macro_rules! impl_repository {
    (@repository, $model: ty ) => {
        pastey::paste!{
 #[derive(Debug, Clone)]
            pub struct [<$model Repository>] {
                pool: $crate::core::database::connection::DbPool
        }
            impl [<$model Repository>] {
                pub fn new(pool: $crate::core::database::connection::DbPool) -> Self {
                Self { pool }
        }
        }
        }
    };


    (@sync, $model:ident, $table:ident, $id:ty ,$new:ty ) => {
        pastey::paste!{
        impl_repository!(@repository, $model);
        impl $crate::core::database::repository::DatabaseRepository for [<$model Repository>] {
            type Id = $id;
            type New = $new;
            type Model = $model;

            fn _create(
                &self,
                new: $new,
                conn: &mut diesel::prelude::PgConnection,
            ) -> anyhow::Result<$model> {
                use diesel::prelude::*;
                 diesel::insert_into($table::table)
                    .values(new)
                    .returning(<$model>::as_returning())
                    .get_result(conn)
                    .inspect_err(|err| ::leptos::logging::log!("Error {}",err))
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
                    .inspect_err(|err| ::leptos::logging::log!("Error {}",err))
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
                    .inspect_err(|err| ::leptos::logging::log!("Error {}",err))
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
                    .inspect_err(|err| ::leptos::logging::log!("Error {}",err))
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
                    .inspect_err(|err| ::leptos::logging::log!("Error {}",err))
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
        impl $crate::core::database::repository::AsyncLayer for [<$model Repository>]{
        async fn create(&self,insertable: $new) -> anyhow::Result<$model> {
                let repo = self.clone();
                $crate::core::database::connection::interact(&self.pool, move |conn| {
                    use anyhow::Context;
                    use $crate::core::database::repository::DatabaseRepository;
                    repo._create(insertable, conn ).context("creating single")
                })
                .await
        }
        async fn delete(&self, id:$id) -> anyhow::Result<()> {
                let repo = self.clone();
                $crate::core::database::connection::interact(&self.pool, move |conn| {
                    use anyhow::Context;
                    use $crate::core::database::repository::DatabaseRepository;
                    repo._delete(id, conn).context("deleting single").map(|_|())
                })
                .await
        }

        async fn update(&self, id:$id, updateable: $new) -> anyhow::Result<$model> {
                let repo = self.clone();
                $crate::core::database::connection::interact(&self.pool, move |conn| {
                    use anyhow::Context;
                    use $crate::core::database::repository::DatabaseRepository;
                    repo._update(id, updateable, conn).context("updating single")
                })
                .await
        }

        async fn read(&self, id:$id ) -> anyhow::Result<$model> {
                let repo = self.clone();
                $crate::core::database::connection::interact(&self.pool, move |conn| {
                    use anyhow::Context;
                    use $crate::core::database::repository::DatabaseRepository;
                    repo._read(id, conn ).context("updating single")
                })
                .await
        }
        async fn read_all(&self  ) -> anyhow::Result<Vec<$model>> {
                let repo = self.clone();
                $crate::core::database::connection::interact(&self.pool, move |conn| {
                    use anyhow::Context;
                    use $crate::core::database::repository::DatabaseRepository;
                    repo._read_all( conn ).context("updating single")
                })
                .await
        }

        }
}
    };
    // Hijo con ciclo de vida propio (1:N):
    // impl_repository!(async, Session, sessions, i32, NewSession, belongs_to(user_id: i32));
    // Genera `SessionRepository` con el CRUD normal (AsyncLayer) más
    // `read_by_parent` / `delete_by_parent` (AsyncBelongsTo). El padre se
    // declara como un repositorio plano.
    ( async, $model:ident, $table:ident, $id:ty, $new:ty,
      belongs_to($fk:ident : $parent_id:ty) $(,)? ) => {
        impl_repository!(async, $model, $table, $id, $new);
        impl_repository!(@belongs_to, $model, $table, $fk, $parent_id);
    };

    (@belongs_to, $model:ident, $table:ident, $fk:ident, $parent_id:ty) => {
        pastey::paste! {
        impl $crate::core::database::repository::AsyncBelongsTo for [<$model Repository>] {
            type ParentId = $parent_id;

            async fn read_by_parent(&self, parent_id: $parent_id) -> anyhow::Result<Vec<$model>> {
                $crate::core::database::connection::interact(&self.pool, move |conn| {
                    use anyhow::Context;
                    use diesel::prelude::*;
                    $table::table
                        .filter($table::$fk.eq(parent_id))
                        .select(<$model>::as_select())
                        .load(conn)
                        .context("reading by parent")
                })
                .await
            }

            async fn delete_by_parent(&self, parent_id: $parent_id) -> anyhow::Result<usize> {
                $crate::core::database::connection::interact(&self.pool, move |conn| {
                    use anyhow::Context;
                    use diesel::prelude::*;
                    diesel::delete($table::table.filter($table::$fk.eq(parent_id)))
                        .execute(conn)
                        .context("deleting by parent")
                })
                .await
            }
        }
        }
    };
    // Padre con hijos:
    // impl_repository!(async, Study, studies, i32, NewStudy,
    //     children(study_id) {
    //         Citologia: citologias, NewCitologia;
    //         Histopatologia: histopatologias, NewHistopatologia;
    //     },
    //     kind = tipo: StudyType,   // opcional
    // );
    // Genera `StudyRepository` (AsyncLayerWithHierarchy) y los enums
    // `StudyDetail` / `NewStudyDetail`. `study_id` es la FK en cada tabla hija
    // y la completa el repositorio. Con `kind`, el campo `tipo` del padre se
    // fija desde la variante del hijo (las variantes de `StudyType` deben
    // llamarse como los modelos hijos).
    ( async, $model:ident, $table:ident, $id:ty, $new:ty,
      children($fk:ident) $children:tt
      $(, kind = $kind_field:ident : $kind_ty:ident)? $(,)? ) => {
        impl_repository!(@sync, $model, $table, $id, $new);
        impl_repository!(@children, $model, $id, $new, $fk, $children);
        impl_repository!(@kind, $model, $new, [$($kind_field $kind_ty)?], $children);
        impl_repository!(@hierarchy, $model, $id, $new);
    };

    // Enums de hijos + helpers síncronos sobre las tablas hijas.
    (@children, $model:ident, $id:ty, $new:ty, $fk:ident,
     { $($child:ident : $ctable:ident, $cnew:ty);+ $(;)? }) => {
        pastey::paste! {
            #[derive(Debug, serde::Serialize, serde::Deserialize)]
            pub enum [<$model Detail>] {
                $($child($child),)+
            }

            #[derive(Debug, serde::Serialize, serde::Deserialize)]
            pub enum [<New $model Detail>] {
                $($child($cnew),)+
            }

            impl [<$model Repository>] {
                fn _create_child(
                    conn: &mut diesel::prelude::PgConnection,
                    parent_id: $id,
                    child: [<New $model Detail>],
                ) -> anyhow::Result<[<$model Detail>]> {
                    use diesel::prelude::*;
                    Ok(match child {
                        $([<New $model Detail>]::$child(new) => [<$model Detail>]::$child(
                            diesel::insert_into($ctable::table)
                                .values((new, $ctable::$fk.eq(parent_id)))
                                .returning(<$child>::as_returning())
                                .get_result(conn)?,
                        ),)+
                    })
                }

                fn _delete_children(
                    conn: &mut diesel::prelude::PgConnection,
                    parent_id: $id,
                ) -> anyhow::Result<()> {
                    use diesel::prelude::*;
                    $(diesel::delete($ctable::table.filter($ctable::$fk.eq(parent_id)))
                        .execute(conn)?;)+
                    Ok(())
                }

                fn _read_child(
                    conn: &mut diesel::prelude::PgConnection,
                    parent_id: $id,
                ) -> anyhow::Result<[<$model Detail>]> {
                    use diesel::prelude::*;
                    $(if let Some(child) = $ctable::table
                        .filter($ctable::$fk.eq(parent_id))
                        .select(<$child>::as_select())
                        .first(conn)
                        .optional()?
                    {
                        return Ok([<$model Detail>]::$child(child));
                    })+
                    anyhow::bail!("{} {:?} has no child row", stringify!($model), parent_id)
                }

                fn _read_all_children(
                    conn: &mut diesel::prelude::PgConnection,
                ) -> anyhow::Result<std::collections::HashMap<$id, [<$model Detail>]>> {
                    use diesel::prelude::*;
                    let mut children = std::collections::HashMap::new();
                    $(for (parent_id, child) in $ctable::table
                        .select(($ctable::$fk, <$child>::as_select()))
                        .load::<($id, $child)>(conn)?
                    {
                        children.insert(parent_id, [<$model Detail>]::$child(child));
                    })+
                    Ok(children)
                }
            }
        }
    };

    // Sin `kind`: no hay campo que sincronizar.
    (@kind, $model:ident, $new:ty, [], $children:tt) => {
        pastey::paste! {
            impl [<$model Repository>] {
                fn _sync_kind(_new: &mut $new, _child: &[<New $model Detail>]) {}
            }
        }
    };
    (@kind, $model:ident, $new:ty, [$kind_field:ident $kind_ty:ident],
     { $($child:ident : $ctable:ident, $cnew:ty);+ $(;)? }) => {
        pastey::paste! {
            impl [<$model Repository>] {
                fn _sync_kind(new: &mut $new, child: &[<New $model Detail>]) {
                    new.$kind_field = match child {
                        $([<New $model Detail>]::$child(_) => $kind_ty::$child,)+
                    };
                }
            }
        }
    };

    (@hierarchy, $model:ident, $id:ty, $new:ty) => {
        pastey::paste! {
        impl $crate::core::database::repository::AsyncLayerWithHierarchy for [<$model Repository>] {
            type Child = [<$model Detail>];
            type NewChild = [<New $model Detail>];

            async fn create(
                &self,
                mut new: $new,
                child: [<New $model Detail>],
            ) -> anyhow::Result<($model, [<$model Detail>])> {
                Self::_sync_kind(&mut new, &child);
                let repo = self.clone();
                $crate::core::database::connection::interact(&self.pool, move |conn| {
                    use anyhow::Context;
                    use diesel::{Connection, Identifiable};
                    use $crate::core::database::repository::DatabaseRepository;
                    conn.transaction(|conn| {
                        let parent = repo._create(new, conn ).context("creating parent")?;
                        let id: $id = *(&parent).id();
                        let child = Self::_create_child(conn, id, child).context("creating child")?;
                        Ok((parent, child))
                    })
                })
                .await
            }

            async fn read(&self, id: $id) -> anyhow::Result<($model, [<$model Detail>])> {
                let repo = self.clone();
                $crate::core::database::connection::interact(&self.pool, move |conn| {
                    use anyhow::Context;
                    use diesel::Connection;
                    use $crate::core::database::repository::DatabaseRepository;
                    conn.transaction(|conn| {
                        let parent = repo._read(id, conn).context("reading parent")?;
                        let child = Self::_read_child(conn, id).context("reading child")?;
                        Ok((parent, child))
                    })
                })
                .await
            }

            async fn read_all(&self) -> anyhow::Result<Vec<($model, [<$model Detail>])>> {
                let repo = self.clone();
                $crate::core::database::connection::interact(&self.pool, move |conn| {
                    use anyhow::Context;
                    use diesel::{Connection, Identifiable};
                    use $crate::core::database::repository::DatabaseRepository;
                    conn.transaction(|conn| {
                        let parents = repo._read_all(conn).context("reading parents")?;
                        let mut children =
                            Self::_read_all_children(conn).context("reading children")?;
                        parents
                            .into_iter()
                            .map(|parent| {
                                let id: $id = *(&parent).id();
                                children
                                    .remove(&id)
                                    .map(|child| (parent, child))
                                    .with_context(|| {
                                        format!("{} {:?} has no child row", stringify!($model), id)
                                    })
                            })
                            .collect()
                    })
                })
                .await
            }

            async fn update(
                &self,
                id: $id,
                mut new: $new,
                child: [<New $model Detail>],
            ) -> anyhow::Result<($model, [<$model Detail>])> {
                Self::_sync_kind(&mut new, &child);
                let repo = self.clone();
                $crate::core::database::connection::interact(&self.pool, move |conn| {
                    use anyhow::Context;
                    use diesel::Connection;
                    use $crate::core::database::repository::DatabaseRepository;
                    conn.transaction(|conn| {
                        let parent = repo._update(id, new, conn).context("updating parent")?;
                        Self::_delete_children(conn, id).context("deleting old child")?;
                        let child = Self::_create_child(conn, id, child).context("creating child")?;
                        Ok((parent, child))
                    })
                })
                .await
            }

            async fn delete(&self, id: $id) -> anyhow::Result<()> {
                let repo = self.clone();
                $crate::core::database::connection::interact(&self.pool, move |conn| {
                    use anyhow::Context;
                    use diesel::Connection;
                    use $crate::core::database::repository::DatabaseRepository;
                    conn.transaction(|conn| {
                        Self::_delete_children(conn, id).context("deleting children")?;
                        repo._delete(id, conn).context("deleting parent")
                    })
                })
                .await
            }
        }
        }
    };
}
