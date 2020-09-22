use crate::{api::claims::Claims, websocket::handlers::Args, DbPool, LemmyContext};
use actix_web::{web, web::Data};
use lemmy_db::{
  community::Community,
  community_view::CommunityUserBanView,
  post::Post,
  user::User_,
  Crud,
};
use lemmy_structs::{
  blocking,
  comment::*,
  community::*,
  post::*,
  site::*,
  user::*,
  websocket::{serialize_websocket_message, UserOperation},
};
use lemmy_utils::{APIError, ConnectionId, LemmyError};
use serde::Deserialize;

pub mod claims;
pub mod comment;
pub mod community;
pub mod post;
pub mod site;
pub mod user;

#[async_trait::async_trait(?Send)]
pub trait Perform {
  type Response: serde::ser::Serialize + Send;

  async fn perform(
    &self,
    context: &Data<LemmyContext>,
    websocket_id: Option<ConnectionId>,
  ) -> Result<Self::Response, LemmyError>;
}

pub(in crate::api) async fn is_mod_or_admin(
  pool: &DbPool,
  user_id: i32,
  community_id: i32,
) -> Result<(), LemmyError> {
  let is_mod_or_admin = blocking(pool, move |conn| {
    Community::is_mod_or_admin(conn, user_id, community_id)
  })
  .await?;
  if !is_mod_or_admin {
    return Err(APIError::err("not_a_mod_or_admin").into());
  }
  Ok(())
}
pub async fn is_admin(pool: &DbPool, user_id: i32) -> Result<(), LemmyError> {
  let user = blocking(pool, move |conn| User_::read(conn, user_id)).await??;
  if !user.admin {
    return Err(APIError::err("not_an_admin").into());
  }
  Ok(())
}

pub(in crate::api) async fn get_post(post_id: i32, pool: &DbPool) -> Result<Post, LemmyError> {
  match blocking(pool, move |conn| Post::read(conn, post_id)).await? {
    Ok(post) => Ok(post),
    Err(_e) => Err(APIError::err("couldnt_find_post").into()),
  }
}

pub(in crate::api) async fn get_user_from_jwt(
  jwt: &str,
  pool: &DbPool,
) -> Result<User_, LemmyError> {
  let claims = match Claims::decode(&jwt) {
    Ok(claims) => claims.claims,
    Err(_e) => return Err(APIError::err("not_logged_in").into()),
  };
  let user_id = claims.id;
  let user = blocking(pool, move |conn| User_::read(conn, user_id)).await??;
  // Check for a site ban
  if user.banned {
    return Err(APIError::err("site_ban").into());
  }
  Ok(user)
}

pub(in crate::api) async fn get_user_from_jwt_opt(
  jwt: &Option<String>,
  pool: &DbPool,
) -> Result<Option<User_>, LemmyError> {
  match jwt {
    Some(jwt) => Ok(Some(get_user_from_jwt(jwt, pool).await?)),
    None => Ok(None),
  }
}

pub(in crate::api) async fn check_community_ban(
  user_id: i32,
  community_id: i32,
  pool: &DbPool,
) -> Result<(), LemmyError> {
  let is_banned = move |conn: &'_ _| CommunityUserBanView::get(conn, user_id, community_id).is_ok();
  if blocking(pool, is_banned).await? {
    Err(APIError::err("community_ban").into())
  } else {
    Ok(())
  }
}

pub(super) async fn do_user_operation<'a, 'b, Data>(args: Args<'b>) -> Result<String, LemmyError>
where
  for<'de> Data: Deserialize<'de> + 'a,
  Data: Perform,
{
  let Args {
    context,
    id,
    op,
    data,
  } = args;

  do_websocket_operation::<Data>(context, data, id, op).await
}

pub async fn do_websocket_operation<'a, 'b, Data>(
  context: LemmyContext,
  data: &'a str,
  id: ConnectionId,
  op: UserOperation,
) -> Result<String, LemmyError>
where
  for<'de> Data: Deserialize<'de> + 'a,
  Data: Perform,
{
  let parsed_data: Data = serde_json::from_str(&data)?;
  let res = parsed_data
    .perform(&web::Data::new(context), Some(id))
    .await?;
  serialize_websocket_message(&op, &res)
}

pub async fn xxx(args: Args<'_>) -> Result<String, LemmyError> {
  match args.op {
    // User ops
    UserOperation::Login => do_user_operation::<Login>(args).await,
    UserOperation::Register => do_user_operation::<Register>(args).await,
    UserOperation::GetCaptcha => do_user_operation::<GetCaptcha>(args).await,
    UserOperation::GetUserDetails => do_user_operation::<GetUserDetails>(args).await,
    UserOperation::GetReplies => do_user_operation::<GetReplies>(args).await,
    UserOperation::AddAdmin => do_user_operation::<AddAdmin>(args).await,
    UserOperation::BanUser => do_user_operation::<BanUser>(args).await,
    UserOperation::GetUserMentions => do_user_operation::<GetUserMentions>(args).await,
    UserOperation::MarkUserMentionAsRead => do_user_operation::<MarkUserMentionAsRead>(args).await,
    UserOperation::MarkAllAsRead => do_user_operation::<MarkAllAsRead>(args).await,
    UserOperation::DeleteAccount => do_user_operation::<DeleteAccount>(args).await,
    UserOperation::PasswordReset => do_user_operation::<PasswordReset>(args).await,
    UserOperation::PasswordChange => do_user_operation::<PasswordChange>(args).await,
    UserOperation::UserJoin => do_user_operation::<UserJoin>(args).await,
    UserOperation::PostJoin => do_user_operation::<PostJoin>(args).await,
    UserOperation::CommunityJoin => do_user_operation::<CommunityJoin>(args).await,
    UserOperation::SaveUserSettings => do_user_operation::<SaveUserSettings>(args).await,

    // Private Message ops
    UserOperation::CreatePrivateMessage => do_user_operation::<CreatePrivateMessage>(args).await,
    UserOperation::EditPrivateMessage => do_user_operation::<EditPrivateMessage>(args).await,
    UserOperation::DeletePrivateMessage => do_user_operation::<DeletePrivateMessage>(args).await,
    UserOperation::MarkPrivateMessageAsRead => {
      do_user_operation::<MarkPrivateMessageAsRead>(args).await
    }
    UserOperation::GetPrivateMessages => do_user_operation::<GetPrivateMessages>(args).await,

    // Site ops
    UserOperation::GetModlog => do_user_operation::<GetModlog>(args).await,
    UserOperation::CreateSite => do_user_operation::<CreateSite>(args).await,
    UserOperation::EditSite => do_user_operation::<EditSite>(args).await,
    UserOperation::GetSite => do_user_operation::<GetSite>(args).await,
    UserOperation::GetSiteConfig => do_user_operation::<GetSiteConfig>(args).await,
    UserOperation::SaveSiteConfig => do_user_operation::<SaveSiteConfig>(args).await,
    UserOperation::Search => do_user_operation::<Search>(args).await,
    UserOperation::TransferCommunity => do_user_operation::<TransferCommunity>(args).await,
    UserOperation::TransferSite => do_user_operation::<TransferSite>(args).await,
    UserOperation::ListCategories => do_user_operation::<ListCategories>(args).await,

    // Community ops
    UserOperation::GetCommunity => do_user_operation::<GetCommunity>(args).await,
    UserOperation::ListCommunities => do_user_operation::<ListCommunities>(args).await,
    UserOperation::CreateCommunity => do_user_operation::<CreateCommunity>(args).await,
    UserOperation::EditCommunity => do_user_operation::<EditCommunity>(args).await,
    UserOperation::DeleteCommunity => do_user_operation::<DeleteCommunity>(args).await,
    UserOperation::RemoveCommunity => do_user_operation::<RemoveCommunity>(args).await,
    UserOperation::FollowCommunity => do_user_operation::<FollowCommunity>(args).await,
    UserOperation::GetFollowedCommunities => {
      do_user_operation::<GetFollowedCommunities>(args).await
    }
    UserOperation::BanFromCommunity => do_user_operation::<BanFromCommunity>(args).await,
    UserOperation::AddModToCommunity => do_user_operation::<AddModToCommunity>(args).await,

    // Post ops
    UserOperation::CreatePost => do_user_operation::<CreatePost>(args).await,
    UserOperation::GetPost => do_user_operation::<GetPost>(args).await,
    UserOperation::GetPosts => do_user_operation::<GetPosts>(args).await,
    UserOperation::EditPost => do_user_operation::<EditPost>(args).await,
    UserOperation::DeletePost => do_user_operation::<DeletePost>(args).await,
    UserOperation::RemovePost => do_user_operation::<RemovePost>(args).await,
    UserOperation::LockPost => do_user_operation::<LockPost>(args).await,
    UserOperation::StickyPost => do_user_operation::<StickyPost>(args).await,
    UserOperation::CreatePostLike => do_user_operation::<CreatePostLike>(args).await,
    UserOperation::SavePost => do_user_operation::<SavePost>(args).await,

    // Comment ops
    UserOperation::CreateComment => do_user_operation::<CreateComment>(args).await,
    UserOperation::EditComment => do_user_operation::<EditComment>(args).await,
    UserOperation::DeleteComment => do_user_operation::<DeleteComment>(args).await,
    UserOperation::RemoveComment => do_user_operation::<RemoveComment>(args).await,
    UserOperation::MarkCommentAsRead => do_user_operation::<MarkCommentAsRead>(args).await,
    UserOperation::SaveComment => do_user_operation::<SaveComment>(args).await,
    UserOperation::GetComments => do_user_operation::<GetComments>(args).await,
    UserOperation::CreateCommentLike => do_user_operation::<CreateCommentLike>(args).await,
  }
}
