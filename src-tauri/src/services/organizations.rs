use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    TransactionTrait,
};
use uuid::Uuid;

use crate::{
    entity::{
        organization_members::{self, ActiveModel as OrganizationMemberActiveModel},
        organizations::{self, ActiveModel as OrganizationActiveModel},
    },
    enums::organization_roles::OrganizationRole,
    errors::AppError,
    inputs::organizations::{CreateOrganizationInput, UpdateOrganizationInput},
    responses::{organizations::Organization, text::Response},
};

pub async fn create_organization(
    user_id: String,
    input: CreateOrganizationInput,
    db: &DatabaseConnection,
) -> Result<Organization, AppError> {
    let txn = db.begin().await?;

    let new_organization = OrganizationActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(input.name),
        creator_id: Set(Uuid::parse_str(&user_id)?),
        created_at: Set(Utc::now().timestamp()),
        updated_at: Set(Utc::now().timestamp()),
    };

    let organization = new_organization.insert(&txn).await?;

    let new_organization_member = OrganizationMemberActiveModel {
        id: Set(Uuid::new_v4()),
        role: Set(OrganizationRole::Owner.to_string()),
        organization_id: Set(organization.id),
        user_id: Set(Uuid::parse_str(&user_id)?),
        created_at: Set(Utc::now().timestamp()),
    };

    new_organization_member.insert(&txn).await?;

    txn.commit().await?;

    Ok(Organization {
        id: organization.id.to_string(),
        name: organization.name,
        creator_id: organization.creator_id.to_string(),
    })
}

pub async fn update_organization(
    input: UpdateOrganizationInput,
    db: &DatabaseConnection,
) -> Result<Response, AppError> {
    let organization_model = organizations::Entity::find_by_id(Uuid::parse_str(&input.id)?)
        .one(db)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    let mut organization: OrganizationActiveModel = organization_model.into();

    organization.name = Set(input.name);

    organization.update(db).await?;

    Ok(Response {
        message: "Update successfully".to_string(),
    })
}

pub async fn delete_organization(
    organization_id: String,
    db: &DatabaseConnection,
) -> Result<Response, AppError> {
    let organization_model = organizations::Entity::find_by_id(Uuid::parse_str(&organization_id)?)
        .one(db)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    OrganizationActiveModel::from(organization_model)
        .delete(db)
        .await?;

    Ok(Response {
        message: "Organization deleted successfully".to_string(),
    })
}

pub async fn get_organization_for_user(
    user_id: String,
    db: &DatabaseConnection,
) -> Result<Vec<Organization>, AppError> {
    let organizations = organizations::Entity::find()
        .inner_join(organization_members::Entity)
        .filter(organization_members::Column::UserId.eq(Uuid::parse_str(&user_id)?))
        .all(db)
        .await?;

    let result = organizations
        .into_iter()
        .map(|o| Organization {
            id: o.id.to_string(),
            name: o.name,
            creator_id: o.creator_id.to_string(),
        })
        .collect();

    Ok(result)
}

pub async fn get_organization(
    organization_id: String,
    db: &DatabaseConnection,
) -> Result<Organization, AppError> {
    let organization = organizations::Entity::find_by_id(Uuid::parse_str(&organization_id)?)
        .one(db)
        .await?
        .ok_or_else(|| AppError::Forbidden)?;

    Ok(Organization {
        id: organization.id.to_string(),
        name: organization.name,
        creator_id: organization.creator_id.to_string(),
    })
}
