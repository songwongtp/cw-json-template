use std::fmt::Debug;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Addr, Api, CustomQuery, DepsMut, MessageInfo, Response, StdError, StdResult, Storage,
};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use thiserror::Error;

/// The finite states that are possible
#[cw_serde]
pub struct OwnerState {
    pub owner: Addr,
    pub status: Option<String>,
}

/// Errors returned from Owner state transitions
#[derive(Error, Debug, PartialEq)]
pub enum OwnerError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Caller is not owner")]
    NotOwner {},
}

type OwnerResult<T> = Result<T, OwnerError>;

#[cw_serde]
pub enum OwnerUpdate {
    /// Proposes a new owner to take role. Only current owner can execute.
    UpdateOwner {
        // new_owner - String
        new_owner: String,
    },
    /// Change owner status.
    UpdateStatus {
        // new_status - string
        new_status: String,
    },
    /// Clear owner status.
    ClearStatus,
}

/// Returned from Owner.query()
#[cw_serde]
pub struct OwnerResponse {
    pub owner: String,
    pub status: Option<String>,
}

/// A struct designed to help facilitate a two-step transition between contract owners safely.
/// It implements a finite state machine with dispatched events to manage state transitions.
/// State machine visualization: https://stately.ai/registry/editor/b7e5dbac-2d33-47f7-a84b-e38dff5694ad?machineId=f8d99cd1-dd55-4506-961b-e2542480be68&mode=Simulate
pub struct Owner<'a>(Item<'a, OwnerState>);

impl<'a> Owner<'a> {
    pub const fn new(namespace: &'a str) -> Self {
        Self(Item::new(namespace))
    }

    fn state(&self, storage: &'a dyn Storage) -> StdResult<OwnerState> {
        self.0.load(storage)
    }

    //--------------------------------------------------------------------------------------------------
    // Queries
    //--------------------------------------------------------------------------------------------------
    pub fn current(&self, storage: &'a dyn Storage) -> StdResult<Addr> {
        Ok(self.state(storage)?.owner)
    }

    pub fn status(&self, storage: &'a dyn Storage) -> StdResult<Option<String>> {
        Ok(self.state(storage)?.status)
    }

    pub fn is_owner(&self, storage: &'a dyn Storage, addr: &Addr) -> StdResult<bool> {
        Ok(self.current(storage)? == addr)
    }

    pub fn query(&self, storage: &'a dyn Storage) -> StdResult<OwnerResponse> {
        Ok(OwnerResponse {
            owner: self.current(storage)?.into(),
            status: self.status(storage)?.map(Into::into),
        })
    }

    //--------------------------------------------------------------------------------------------------
    // Mutations
    //--------------------------------------------------------------------------------------------------
    /// Execute inside instantiate fn
    pub fn initialize(
        &self,
        storage: &'a mut dyn Storage,
        api: &'a dyn Api,
        owner: String,
        status: Option<String>,
    ) -> OwnerResult<()> {
        let validated = api.addr_validate(&owner)?;
        let new_state = OwnerState {
            owner: validated,
            status,
        };
        self.0.save(storage, &new_state)?;
        Ok(())
    }

    /// Composes execute responses for owner state updates
    pub fn update<C, Q: CustomQuery>(
        &self,
        deps: DepsMut<Q>,
        info: MessageInfo,
        update: OwnerUpdate,
    ) -> OwnerResult<Response<C>>
    where
        C: Clone + Debug + PartialEq + JsonSchema,
    {
        let new_state = self.transition_state(deps.storage, deps.api, &info.sender, update)?;
        self.0.save(deps.storage, &new_state)?;

        let res = self.query(deps.storage)?;
        Ok(Response::new()
            .add_attribute("action", "update_owner")
            .add_attribute("owner", res.owner)
            .add_attribute("status", res.status.unwrap_or_else(|| "".to_string()))
            .add_attribute("sender", info.sender))
    }

    /// Executes owner state transitions
    fn transition_state(
        &self,
        storage: &'a mut dyn Storage,
        api: &'a dyn Api,
        sender: &Addr,
        event: OwnerUpdate,
    ) -> OwnerResult<OwnerState> {
        self.assert_owner(storage, sender)?;

        let new_state = match event {
            OwnerUpdate::UpdateOwner { new_owner } => {
                let validated = api.addr_validate(&new_owner)?;
                OwnerState {
                    owner: validated,
                    status: self.status(storage)?,
                }
            }
            OwnerUpdate::UpdateStatus { new_status } => OwnerState {
                owner: self.current(storage)?,
                status: Some(new_status),
            },
            OwnerUpdate::ClearStatus => OwnerState {
                owner: self.current(storage)?,
                status: None,
            },
        };
        Ok(new_state)
    }

    //--------------------------------------------------------------------------------------------------
    // Assertions
    //--------------------------------------------------------------------------------------------------
    /// Similar to is_owner() except it raises an exception if caller is not current owner
    pub fn assert_owner(&self, storage: &'a dyn Storage, caller: &Addr) -> OwnerResult<()> {
        if !self.is_owner(storage, caller)? {
            Err(OwnerError::NotOwner {})
        } else {
            Ok(())
        }
    }
}
