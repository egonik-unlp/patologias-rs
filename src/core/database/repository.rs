#![allow(async_fn_in_trait)]

use std::collections::HashMap;

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

// Repositorio de un modelo padre con exactamente un hijo, que puede estar en
// una de varias tablas (p. ej. Study -> Citologia | Histopatologia).
// `Child` / `NewChild` son enums con una variante por tabla hija; los genera
// `impl_repository!` con la opción `children(...)`. Cada operación corre en
// una sola transacción.
pub trait AsyncLayerWithHierarchy: DatabaseRepository {
    type Child;
    type NewChild;

    async fn create(
        &self,
        new: Self::New,
        child: Self::NewChild,
    ) -> anyhow::Result<(Self::Model, Self::Child)>;
    async fn read(&self, id: Self::Id) -> anyhow::Result<(Self::Model, Self::Child)>;
    async fn read_all(&self) -> anyhow::Result<Vec<(Self::Model, Self::Child)>>;
    /// Actualiza el padre y reemplaza su hijo (que puede cambiar de tabla).
    async fn update(
        &self,
        id: Self::Id,
        new: Self::New,
        child: Self::NewChild,
    ) -> anyhow::Result<(Self::Model, Self::Child)>;
    async fn delete(&self, id: Self::Id) -> anyhow::Result<()>;
}

// Repositorio de un modelo sin hijos.
pub trait AsyncLayer: DatabaseRepository {
    async fn create(&self, insertable: Self::New) -> anyhow::Result<Self::Model>;
    async fn delete(&self, id: Self::Id) -> anyhow::Result<()>;
    async fn update(&self, id: Self::Id, updateable: Self::New) -> anyhow::Result<Self::Model>;
    async fn read(&self, id: Self::Id) -> anyhow::Result<Self::Model>;
    async fn read_all(&self) -> anyhow::Result<Vec<Self::Model>>;
}

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

/// Values that differ only in case, a leading dash, a trailing period or spaces
/// get the same key.
fn normalize_key(s: &str) -> String {
    s.trim_start_matches(|c: char| c == '-' || c.is_whitespace())
        .trim_end_matches(|c: char| c == '.' || c.is_whitespace())
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// `rows` = (exact value, times used). Returns the most common spelling
/// of each group, most-used groups first.
pub fn rank_suggestions<I>(rows: I, limit: usize) -> Vec<String>
where
    I: IntoIterator<Item = (String, i64)>,
{
    // key -> (spelling -> count)
    let mut groups: HashMap<String, HashMap<String, i64>> = HashMap::new();
    for (raw, n) in rows {
        let value = raw.trim();
        let key = normalize_key(value);
        if key.is_empty() {
            continue; // blank, "-", "." …
        }
        *groups
            .entry(key)
            .or_default()
            .entry(value.to_owned())
            .or_default() += n;
    }

    let mut ranked: Vec<(i64, String)> = groups
        .into_values()
        .map(|spellings| {
            let total = spellings.values().sum();
            let best = spellings
                .into_iter()
                .max_by(|(a, na), (b, nb)| na.cmp(nb).then_with(|| b.cmp(a)))
                .map(|(v, _)| v)
                .expect("a group always has at least one spelling");
            (total, best)
        })
        .collect();

    ranked.sort_by(|(ta, a), (tb, b)| tb.cmp(ta).then_with(|| a.cmp(b)));
    ranked.into_iter().take(limit).map(|(_, v)| v).collect()
}
#[macro_export]
macro_rules! suggestions {
    (unique, $conn:ident, $table_name:ident, $col:expr) => {{
        use ::diesel::{QueryDsl as _, RunQueryDsl as _};
        $table_name::table
            .group_by($col)
            .select(($col, ::diesel::dsl::count_star()))
            .load::<(String, i64)>($conn)
    }};
    ($conn:ident, $table_name:ident, $col:expr) => {{
        use ::diesel::{ExpressionMethods as _, QueryDsl as _, RunQueryDsl as _};
        $table_name::table
            .filter($col.is_not_null())
            .group_by($col)
            .select(($col, ::diesel::dsl::count_star()))
            .load::<(Option<String>, i64)>($conn)
    }};
}
