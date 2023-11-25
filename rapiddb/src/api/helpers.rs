use warp::Filter;

pub fn with_db(
  db: std::sync::Arc<
    tokio::sync::RwLock<impl crate::traits::IAsyncDatabase + ?Sized>,
  >,
) -> impl Filter<
  Extract = (
    std::sync::Arc<
      tokio::sync::RwLock<impl crate::traits::IAsyncDatabase + ?Sized>,
    >,
  ),
  Error = std::convert::Infallible,
> + Clone {
  warp::any().map(move || db.clone())
}
