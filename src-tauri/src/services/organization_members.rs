use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

use crate::{
    entity::{
        organization_members::{self, ActiveModel as OrganizationMemberActiveModel},
        users,
    },
    enums::organization_roles::OrganizationRole,
    errors::AppError,
    inputs::organization_members::CreateOrganizationMemberInput,
    responses::{organization_members::OrganizationMembers, text::Response, users::User},
};

pub async fn create_organization_member(
    user_id: String,
    input: CreateOrganizationMemberInput,
    db: &DatabaseConnection,
) -> Result<OrganizationMembers, AppError> {
    let new_organization_member = OrganizationMemberActiveModel {
        id: Set(Uuid::new_v4()),
        organization_id: Set(Uuid::parse_str(&input.organization_id)?),
        role: Set(input.role.to_string()),
        user_id: Set(Uuid::parse_str(&user_id)?),
        created_at: Set(Utc::now().timestamp()),
    };

    let organization_member = new_organization_member.insert(db).await?;

    Ok(OrganizationMembers {
        id: organization_member.id.to_string(),
        organization_id: organization_member.organization_id.to_string(),
        user_id: organization_member.user_id.to_string(),
        role: organization_member.role.parse()?,
    })
}

pub async fn change_organization_member_role(
    user_id: String,
    role: OrganizationRole,
    db: &DatabaseConnection,
) -> Result<Response, AppError> {
    let organization_member_model = organization_members::Entity::find()
        .filter(
            Condition::any()
                .add(organization_members::Column::UserId.eq(Uuid::parse_str(&user_id)?)),
        )
        .one(db)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    let mut organization_member: OrganizationMemberActiveModel = organization_member_model.into();

    organization_member.role = Set(role.to_string());

    organization_member.update(db).await?;

    Ok(Response {
        message: "Role changed successfully".to_string(),
    })
}

pub async fn delete_organization_member(
    user_id: String,
    db: &DatabaseConnection,
) -> Result<Response, AppError> {
    let organization_member_model = organization_members::Entity::find()
        .filter(
            Condition::any()
                .add(organization_members::Column::UserId.eq(Uuid::parse_str(&user_id)?)),
        )
        .one(db)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    OrganizationMemberActiveModel::from(organization_member_model)
        .delete(db)
        .await?;

    Ok(Response {
        message: "Organization member deleted successfully".to_string(),
    })
}

pub async fn get_all_organization_member(
    organization_id: String,
    db: &DatabaseConnection,
) -> Result<Vec<User>, AppError> {
    let members_with_users = organization_members::Entity::find()
        .filter(organization_members::Column::OrganizationId.eq(Uuid::parse_str(&organization_id)?))
        .find_also_related(users::Entity)
        .all(db)
        .await?;

    let users = members_with_users
        .into_iter()
        .filter_map(|(_, maybe_user)| maybe_user)
        .map(|u| User {
            id: u.id.to_string(),
            username: u.username,
            email: u.email,
        })
        .collect();

    Ok(users)
}

pub async fn get_organization_member(
    organization_member_id: String,
    db: &DatabaseConnection,
) -> Result<User, AppError> {
    let (_member, user) =
        organization_members::Entity::find_by_id(Uuid::parse_str(&organization_member_id)?)
            .find_also_related(users::Entity)
            .one(db)
            .await?
            .ok_or_else(|| AppError::Forbidden)?;

    let user = user.ok_or_else(|| AppError::NotFound("user".to_string()))?;

    Ok(User {
        id: user.id.to_string(),
        email: user.email,
        username: user.username,
    })
}
