pub mod _module {
    
}
/// account/Account.dfy(48,1)
pub mod KipioAccountAccount {
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountIdentity::Identity;
    pub use crate::KipioAccountAuthorizationState::AuthorizationState;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// account/Account.dfy(105,3)
        pub fn AccountIdOf(account: &Rc<Account>) -> Sequence<u8> {
            account.id().clone()
        }
        /// account/Account.dfy(119,3)
        pub fn AccountSovereignIdentity(account: &Rc<Account>) -> Rc<Identity> {
            account.identity().clone()
        }
        /// account/Account.dfy(132,3)
        pub fn AccountAuthorizationState(account: &Rc<Account>) -> Rc<AuthorizationState> {
            account.authorizationState().clone()
        }
        /// account/Account.dfy(153,3)
        pub fn ValidAccount(account: &Rc<Account>) -> bool {
            crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::AccountIdOf(account)) && crate::KipioAccountIdentity::_default::ValidIdentity(&_default::AccountSovereignIdentity(account)) && crate::KipioAccountAuthorizationState::_default::ValidAuthorizationState(&_default::AccountAuthorizationState(account))
        }
        /// account/Account.dfy(177,3)
        pub fn SameAccount(left: &Rc<Account>, right: &Rc<Account>) -> bool {
            _default::AccountIdOf(left) == _default::AccountIdOf(right)
        }
    }

    /// account/Account.dfy(92,3)
    #[derive(Clone)]
    pub enum Account {
        Account {
            id: Sequence<u8>,
            identity: Rc<Identity>,
            authorizationState: Rc<AuthorizationState>
        }
    }

    impl Account {
        /// Returns a borrow of the field id
        pub fn id(&self) -> &Sequence<u8> {
            match self {
                Account::Account{id, identity, authorizationState, } => id,
            }
        }
        /// Returns a borrow of the field identity
        pub fn identity(&self) -> &Rc<Identity> {
            match self {
                Account::Account{id, identity, authorizationState, } => identity,
            }
        }
        /// Returns a borrow of the field authorizationState
        pub fn authorizationState(&self) -> &Rc<AuthorizationState> {
            match self {
                Account::Account{id, identity, authorizationState, } => authorizationState,
            }
        }
    }

    impl Debug
        for Account {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for Account {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                Account::Account{id, identity, authorizationState, } => {
                    write!(_formatter, "KipioAccountAccount.Account.Account(")?;
                    DafnyPrint::fmt_print(id, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(identity, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(authorizationState, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for Account {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (Account::Account{id, identity, authorizationState, }, Account::Account{id: _2_id, identity: _2_identity, authorizationState: _2_authorizationState, }) => {
                    id == _2_id && identity == _2_identity && authorizationState == _2_authorizationState
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for Account {}

    impl Hash
        for Account {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                Account::Account{id, identity, authorizationState, } => {
                    Hash::hash(id, _state);
                    Hash::hash(identity, _state);
                    Hash::hash(authorizationState, _state)
                },
            }
        }
    }

    impl AsRef<Account>
        for Account {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// account/AccountTransitions.dfy(513,1)
pub mod KipioAccountAccountTransitions {
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountAccount::Account;
    pub use crate::KipioAccountCredential::Credential;
    pub use crate::KipioAccountCredential::CredentialStatus;
    pub use crate::KipioAccountSession::Session;
    pub use crate::KipioAccountSession::SessionStatus;
    pub use crate::KipioAccountDelegation::Delegation;
    pub use crate::KipioAccountDelegation::DelegationStatus;
    pub use crate::KipioAccountAuthorizationState::AuthorizationState;
    pub use ::dafny_runtime::Sequence;
    pub use ::dafny_runtime::set;
    pub use ::dafny_runtime::Map;
    pub use crate::KipioAccountAuthorizationState::RestrictionTarget;
    pub use crate::KipioAccountRestriction::Restriction;
    pub use crate::KipioAccountCapability::Capability;
    pub use ::dafny_runtime::MapBuilder;
    pub use crate::KipioAccountPolicyEffect::OptionalPolicyScope;
    pub use ::dafny_runtime::Set;
    pub use ::dafny_runtime::SetBuilder;
    pub use ::dafny_runtime::int;
    pub use crate::KipioAccountIdentity::Identity;

    pub struct _default {}

    impl _default {
        /// account/AccountTransitions.dfy(534,3)
        pub fn AccountTransitionPreservesIdentity(before: &Rc<Account>, after: &Rc<Account>) -> bool {
            crate::KipioAccountAccount::_default::AccountIdOf(before) == crate::KipioAccountAccount::_default::AccountIdOf(after) && crate::KipioAccountAccount::_default::AccountSovereignIdentity(before) == crate::KipioAccountAccount::_default::AccountSovereignIdentity(after)
        }
        /// account/AccountTransitions.dfy(590,3)
        pub fn ValidCredentialStatusTransition(credential: &Rc<Credential>, newStatus: &Rc<CredentialStatus>) -> bool {
            crate::KipioAccountCredential::_default::CredentialIsActive(credential) && (newStatus.clone() == Rc::new(CredentialStatus::Active {}) || newStatus.clone() == Rc::new(CredentialStatus::Suspended {}) || newStatus.clone() == Rc::new(CredentialStatus::Revoked {})) || crate::KipioAccountCredential::_default::CredentialIsSuspended(credential) && (newStatus.clone() == Rc::new(CredentialStatus::Active {}) || newStatus.clone() == Rc::new(CredentialStatus::Suspended {}) || newStatus.clone() == Rc::new(CredentialStatus::Revoked {})) || crate::KipioAccountCredential::_default::CredentialIsRevoked(credential) && newStatus.clone() == Rc::new(CredentialStatus::Revoked {})
        }
        /// account/AccountTransitions.dfy(635,3)
        pub fn ValidSessionStatusTransition(session: &Rc<Session>, newStatus: &Rc<SessionStatus>) -> bool {
            crate::KipioAccountSession::_default::SessionIsActive(session) && (newStatus.clone() == Rc::new(SessionStatus::Active {}) || newStatus.clone() == Rc::new(SessionStatus::Revoked {})) || crate::KipioAccountSession::_default::SessionIsRevoked(session) && newStatus.clone() == Rc::new(SessionStatus::Revoked {})
        }
        /// account/AccountTransitions.dfy(654,3)
        pub fn ValidDelegationStatusTransition(delegation: &Rc<Delegation>, newStatus: &Rc<DelegationStatus>) -> bool {
            crate::KipioAccountDelegation::_default::DelegationIsActive(delegation) && (newStatus.clone() == Rc::new(DelegationStatus::Active {}) || newStatus.clone() == Rc::new(DelegationStatus::Revoked {})) || crate::KipioAccountDelegation::_default::DelegationIsRevoked(delegation) && newStatus.clone() == Rc::new(DelegationStatus::Revoked {})
        }
        /// account/AccountTransitions.dfy(673,3)
        pub fn ValidReplayKeyConsumption(state: &Rc<AuthorizationState>, replayKey: &Sequence<u8>) -> bool {
            crate::KipioAccountDomainPrimitives::_default::ValidId(replayKey) && !crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(state).contains(replayKey)
        }
        /// account/AccountTransitions.dfy(685,3)
        pub fn ConsumeReplayKey(account: &Rc<Account>, replayKey: &Sequence<u8>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state),
                                credentials: crate::KipioAccountAuthorizationState::_default::StateCredentials(&state),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state),
                                sessions: crate::KipioAccountAuthorizationState::_default::StateSessions(&state),
                                delegations: crate::KipioAccountAuthorizationState::_default::StateDelegations(&state),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state),
                                restrictionMap: crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state).merge(&set!{replayKey.clone()})
                            })
                })
        }
        /// account/AccountTransitions.dfy(1144,3)
        pub fn RemoveRestrictionsForCapability(restrictionMap: &Map<Rc<RestrictionTarget>, Rc<Restriction>>, capability: &Rc<Capability>) -> Map<Rc<RestrictionTarget>, Rc<Restriction>> {
            (&({
                let mut restrictionMap = restrictionMap.clone();
                let mut capability = capability.clone();
                Rc::new(move || -> Map<Rc<RestrictionTarget>, Rc<Restriction>> {
            let mut _coll0: MapBuilder<Rc<RestrictionTarget>, Rc<Restriction>> = MapBuilder::<Rc<RestrictionTarget>, Rc<Restriction>>::new();
            for __compr_0 in (&restrictionMap.keys()).iter().cloned() {
                let mut target: Rc<RestrictionTarget> = __compr_0.clone();
                if restrictionMap.keys().contains(&target) && crate::KipioAccountAuthorizationState::_default::RestrictionTargetCapability(&target) != capability.clone() {
                    _coll0.add(&target, &restrictionMap.get(&target))
                }
            }
            _coll0.build()
        }) as Rc<dyn ::std::ops::Fn() -> Map<Rc<RestrictionTarget>, Rc<Restriction>>>
            }))()
        }
        /// account/AccountTransitions.dfy(1156,3)
        pub fn RemoveRestrictionTarget(restrictionMap: &Map<Rc<RestrictionTarget>, Rc<Restriction>>, capability: &Rc<Capability>, scope: &Rc<OptionalPolicyScope>) -> Map<Rc<RestrictionTarget>, Rc<Restriction>> {
            let mut target: Rc<RestrictionTarget> = Rc::new(RestrictionTarget::RestrictionTarget {
                        capability: capability.clone(),
                        scope: scope.clone()
                    });
            (&({
                let mut restrictionMap = restrictionMap.clone();
                let mut target = target.clone();
                Rc::new(move || -> Map<Rc<RestrictionTarget>, Rc<Restriction>> {
            let mut _coll0: MapBuilder<Rc<RestrictionTarget>, Rc<Restriction>> = MapBuilder::<Rc<RestrictionTarget>, Rc<Restriction>>::new();
            for __compr_0 in (&restrictionMap.keys()).iter().cloned() {
                let mut existingTarget: Rc<RestrictionTarget> = __compr_0.clone();
                if restrictionMap.keys().contains(&existingTarget) && existingTarget.clone() != target.clone() {
                    _coll0.add(&existingTarget, &restrictionMap.get(&existingTarget))
                }
            }
            _coll0.build()
        }) as Rc<dyn ::std::ops::Fn() -> Map<Rc<RestrictionTarget>, Rc<Restriction>>>
            }))()
        }
        /// account/AccountTransitions.dfy(1175,3)
        pub fn SetRestrictionInMap(restrictionMap: &Map<Rc<RestrictionTarget>, Rc<Restriction>>, capability: &Rc<Capability>, scope: &Rc<OptionalPolicyScope>, restriction: &Rc<Restriction>) -> Map<Rc<RestrictionTarget>, Rc<Restriction>> {
            restrictionMap.update_index(&Rc::new(RestrictionTarget::RestrictionTarget {
                        capability: capability.clone(),
                        scope: scope.clone()
                    }), restriction)
        }
        /// account/AccountTransitions.dfy(1356,3)
        pub fn RegisterCredential(account: &Rc<Account>, credential: &Rc<Credential>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state),
                                credentials: crate::KipioAccountAuthorizationState::_default::StateCredentials(&state).merge(&set!{credential.clone()}),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state).update_index(&crate::KipioAccountCredential::_default::CredentialId(credential), &set!{}),
                                sessions: crate::KipioAccountAuthorizationState::_default::StateSessions(&state),
                                delegations: crate::KipioAccountAuthorizationState::_default::StateDelegations(&state),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state),
                                restrictionMap: crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state)
                            })
                })
        }
        /// account/AccountTransitions.dfy(1594,3)
        pub fn ExistingSessionsRemainWithinCredentialAuthority(state: &Rc<AuthorizationState>, credentialId: &Sequence<u8>, authority: &Set<Rc<Capability>>) -> bool {
            (&crate::KipioAccountAuthorizationState::_default::StateSessions(state)).iter().all(({
                    let mut credentialId = credentialId.clone();
                    let mut state = state.clone();
                    let mut authority = authority.clone();
                    Rc::new(move |__forall_var_0: &Rc<Session>| -> bool{
            let mut session: Rc<Session> = __forall_var_0.clone();
            !(crate::KipioAccountAuthorizationState::_default::StateSessions(&state).contains(&session) && crate::KipioAccountSession::_default::SessionCredentialId(&session) == credentialId.clone()) || crate::KipioAccountSession::_default::SessionAuthorityWithinCredentialAuthority(&session, &authority)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// account/AccountTransitions.dfy(1611,3)
        pub fn ValidCredentialAuthorityUpdate(state: &Rc<AuthorizationState>, credential: &Rc<Credential>, authority: &Set<Rc<Capability>>) -> bool {
            crate::KipioAccountAuthorizationState::_default::CredentialRecognizedInState(state, credential) && crate::KipioAccountCredentialAuthority::_default::ValidCredentialAuthority(authority) && _default::ExistingSessionsRemainWithinCredentialAuthority(state, &crate::KipioAccountCredential::_default::CredentialId(credential), authority)
        }
        /// account/AccountTransitions.dfy(1629,3)
        pub fn SetCredentialAuthority(account: &Rc<Account>, credential: &Rc<Credential>, authority: &Set<Rc<Capability>>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state),
                                credentials: crate::KipioAccountAuthorizationState::_default::StateCredentials(&state),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state).update_index(&crate::KipioAccountCredential::_default::CredentialId(credential), authority),
                                sessions: crate::KipioAccountAuthorizationState::_default::StateSessions(&state),
                                delegations: crate::KipioAccountAuthorizationState::_default::StateDelegations(&state),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state),
                                restrictionMap: crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state)
                            })
                })
        }
        /// account/AccountTransitions.dfy(1882,3)
        pub fn UpdateCredentialStatus(account: &Rc<Account>, credential: &Rc<Credential>, status: &Rc<CredentialStatus>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            let mut updatedCredential: Rc<Credential> = Rc::new(Credential::Credential {
                        id: crate::KipioAccountCredential::_default::CredentialId(credential),
                        status: status.clone()
                    });
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state),
                                credentials: (&({
                                        let mut state = state.clone();
                                        let mut credential = credential.clone();
                                        Rc::new(move || -> Set<Rc<Credential>> {
            let mut _coll0: SetBuilder<Rc<Credential>> = SetBuilder::<Rc<Credential>>::new();
            for __compr_0 in (&crate::KipioAccountAuthorizationState::_default::StateCredentials(&state)).iter().cloned() {
                let mut registeredCredential: Rc<Credential> = __compr_0.clone();
                if crate::KipioAccountAuthorizationState::_default::StateCredentials(&state).contains(&registeredCredential) && crate::KipioAccountCredential::_default::CredentialId(&registeredCredential) != crate::KipioAccountCredential::_default::CredentialId(&credential) {
                    _coll0.add(&registeredCredential)
                }
            }
            _coll0.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Rc<Credential>>>
                                    }))().merge(&set!{updatedCredential.clone()}),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state),
                                sessions: crate::KipioAccountAuthorizationState::_default::StateSessions(&state),
                                delegations: crate::KipioAccountAuthorizationState::_default::StateDelegations(&state),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state),
                                restrictionMap: crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state)
                            })
                })
        }
        /// account/AccountTransitions.dfy(2124,3)
        pub fn AddCapability(account: &Rc<Account>, capability: &Rc<Capability>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state).merge(&set!{capability.clone()}),
                                credentials: crate::KipioAccountAuthorizationState::_default::StateCredentials(&state),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state),
                                sessions: crate::KipioAccountAuthorizationState::_default::StateSessions(&state),
                                delegations: crate::KipioAccountAuthorizationState::_default::StateDelegations(&state),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state),
                                restrictionMap: crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state)
                            })
                })
        }
        /// account/AccountTransitions.dfy(2244,3)
        pub fn RemoveCapability(account: &Rc<Account>, capability: &Rc<Capability>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state).subtract(&set!{capability.clone()}),
                                credentials: crate::KipioAccountAuthorizationState::_default::StateCredentials(&state),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state),
                                sessions: crate::KipioAccountAuthorizationState::_default::StateSessions(&state),
                                delegations: crate::KipioAccountAuthorizationState::_default::StateDelegations(&state),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state),
                                restrictionMap: _default::RemoveRestrictionsForCapability(&crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state), capability),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state)
                            })
                })
        }
        /// account/AccountTransitions.dfy(2440,3)
        pub fn SetRestriction(account: &Rc<Account>, capability: &Rc<Capability>, scope: &Rc<OptionalPolicyScope>, restriction: &Rc<Restriction>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state),
                                credentials: crate::KipioAccountAuthorizationState::_default::StateCredentials(&state),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state),
                                sessions: crate::KipioAccountAuthorizationState::_default::StateSessions(&state),
                                delegations: crate::KipioAccountAuthorizationState::_default::StateDelegations(&state),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state),
                                restrictionMap: _default::SetRestrictionInMap(&crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state), capability, scope, restriction),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state)
                            })
                })
        }
        /// account/AccountTransitions.dfy(2735,3)
        pub fn RemoveRestriction(account: &Rc<Account>, capability: &Rc<Capability>, scope: &Rc<OptionalPolicyScope>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state),
                                credentials: crate::KipioAccountAuthorizationState::_default::StateCredentials(&state),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state),
                                sessions: crate::KipioAccountAuthorizationState::_default::StateSessions(&state),
                                delegations: crate::KipioAccountAuthorizationState::_default::StateDelegations(&state),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state),
                                restrictionMap: _default::RemoveRestrictionTarget(&crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state), capability, scope),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state)
                            })
                })
        }
        /// account/AccountTransitions.dfy(2962,3)
        pub fn RegisterSession(account: &Rc<Account>, session: &Rc<Session>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state),
                                credentials: crate::KipioAccountAuthorizationState::_default::StateCredentials(&state),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state),
                                sessions: crate::KipioAccountAuthorizationState::_default::StateSessions(&state).merge(&set!{session.clone()}),
                                delegations: crate::KipioAccountAuthorizationState::_default::StateDelegations(&state),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state),
                                restrictionMap: crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state)
                            })
                })
        }
        /// account/AccountTransitions.dfy(3168,3)
        pub fn UpdateSessionStatus(account: &Rc<Account>, session: &Rc<Session>, status: &Rc<SessionStatus>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            let mut updatedSession: Rc<Session> = Rc::new(Session::Session {
                        id: crate::KipioAccountSession::_default::SessionId(session),
                        credentialId: crate::KipioAccountSession::_default::SessionCredentialId(session),
                        capabilities: crate::KipioAccountSession::_default::SessionCapabilities(session),
                        validFrom: crate::KipioAccountSession::_default::SessionValidFrom(session),
                        validUntil: crate::KipioAccountSession::_default::SessionValidUntil(session),
                        restrictions: crate::KipioAccountSession::_default::SessionRestrictions(session),
                        status: status.clone()
                    });
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state),
                                credentials: crate::KipioAccountAuthorizationState::_default::StateCredentials(&state),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state),
                                sessions: (&({
                                        let mut state = state.clone();
                                        let mut session = session.clone();
                                        Rc::new(move || -> Set<Rc<Session>> {
            let mut _coll0: SetBuilder<Rc<Session>> = SetBuilder::<Rc<Session>>::new();
            for __compr_0 in (&crate::KipioAccountAuthorizationState::_default::StateSessions(&state)).iter().cloned() {
                let mut registeredSession: Rc<Session> = __compr_0.clone();
                if crate::KipioAccountAuthorizationState::_default::StateSessions(&state).contains(&registeredSession) && crate::KipioAccountSession::_default::SessionId(&registeredSession) != crate::KipioAccountSession::_default::SessionId(&session) {
                    _coll0.add(&registeredSession)
                }
            }
            _coll0.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Rc<Session>>>
                                    }))().merge(&set!{updatedSession.clone()}),
                                delegations: crate::KipioAccountAuthorizationState::_default::StateDelegations(&state),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state),
                                restrictionMap: crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state)
                            })
                })
        }
        /// account/AccountTransitions.dfy(3447,3)
        pub fn ParentDelegationsContainCapability(state: &Rc<AuthorizationState>, parentDelegationIds: &Set<Sequence<u8>>, capability: &Rc<Capability>) -> bool {
            (&crate::KipioAccountAuthorizationState::_default::StateDelegations(state)).iter().any(({
                    let mut parentDelegationIds = parentDelegationIds.clone();
                    let mut state = state.clone();
                    let mut capability = capability.clone();
                    Rc::new(move |__exists_var_0: &Rc<Delegation>| -> bool{
            let mut parent: Rc<Delegation> = __exists_var_0.clone();
            crate::KipioAccountAuthorizationState::_default::StateDelegations(&state).contains(&parent) && parentDelegationIds.contains(&crate::KipioAccountDelegation::_default::DelegationId(&parent)) && crate::KipioAccountDelegation::_default::DelegationCapabilities(&parent).contains(&capability)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// account/AccountTransitions.dfy(3462,3)
        pub fn ParentDelegationsBoundNewDelegation(state: &Rc<AuthorizationState>, delegation: &Rc<Delegation>, parentDelegationIds: &Set<Sequence<u8>>) -> bool {
            !parentDelegationIds.contains(&crate::KipioAccountDelegation::_default::DelegationId(delegation)) && int!(0) < parentDelegationIds.cardinality() && parentDelegationIds.iter().all(({
                    let mut parentDelegationIds = parentDelegationIds.clone();
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Sequence<u8>| -> bool{
            let mut parentId: Sequence<u8> = __forall_var_0.clone();
            !parentDelegationIds.contains(&parentId) || crate::KipioAccountAuthorizationState::_default::DelegationIdRecognizedInState(&state, &parentId)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref()) && (&crate::KipioAccountDelegation::_default::DelegationCapabilities(delegation)).iter().all(({
                    let mut parentDelegationIds = parentDelegationIds.clone();
                    let mut state = state.clone();
                    let mut delegation = delegation.clone();
                    Rc::new(move |__forall_var_1: &Rc<Capability>| -> bool{
            let mut capability: Rc<Capability> = __forall_var_1.clone();
            !crate::KipioAccountDelegation::_default::DelegationCapabilities(&delegation).contains(&capability) || _default::ParentDelegationsContainCapability(&state, &parentDelegationIds, &capability)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// account/AccountTransitions.dfy(3491,3)
        pub fn ParentDelegationsProvideAuthorityToChildSource(state: &Rc<AuthorizationState>, identities: &Set<Rc<Identity>>, delegation: &Rc<Delegation>, parentDelegationIds: &Set<Sequence<u8>>, delegateCapability: &Rc<Capability>, delegatableAuthority: &Set<Rc<Capability>>, sourceEffectiveAuthority: &Set<Rc<Capability>>) -> bool {
            _default::ParentDelegationsBoundNewDelegation(state, delegation, parentDelegationIds) && crate::KipioAccountDelegation::_default::SourceHasDelegationCapability(sourceEffectiveAuthority, delegateCapability) && _default::ParentDelegationsContainCapability(state, parentDelegationIds, delegateCapability) && delegatableAuthority.iter().all(({
                    let mut parentDelegationIds = parentDelegationIds.clone();
                    let mut delegatableAuthority = delegatableAuthority.clone();
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Capability>| -> bool{
            let mut capability: Rc<Capability> = __forall_var_0.clone();
            !delegatableAuthority.contains(&capability) || _default::ParentDelegationsContainCapability(&state, &parentDelegationIds, &capability)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref()) && delegatableAuthority.clone() <= sourceEffectiveAuthority.clone() && parentDelegationIds.iter().all(({
                    let mut parentDelegationIds = parentDelegationIds.clone();
                    let mut state = state.clone();
                    let mut delegation = delegation.clone();
                    let mut identities = identities.clone();
                    Rc::new(move |__forall_var_1: &Sequence<u8>| -> bool{
            let mut parentId: Sequence<u8> = __forall_var_1.clone();
            !parentDelegationIds.contains(&parentId) || (&crate::KipioAccountAuthorizationState::_default::StateDelegations(&state)).iter().any(({
                    let mut state = state.clone();
                    let mut parentId = parentId.clone();
                    let mut delegation = delegation.clone();
                    let mut identities = identities.clone();
                    Rc::new(move |__exists_var_0: &Rc<Delegation>| -> bool{
            let mut parent: Rc<Delegation> = __exists_var_0.clone();
            crate::KipioAccountAuthorizationState::_default::StateDelegations(&state).contains(&parent) && crate::KipioAccountDelegation::_default::DelegationId(&parent) == parentId.clone() && crate::KipioAccountAuthorizationState::_default::ParentDelegationSupportsChildSource(&identities, &parent, &delegation)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// account/AccountTransitions.dfy(3551,3)
        pub fn ValidRootDelegationRegistration(account: &Rc<Account>, delegation: &Rc<Delegation>, delegateCapability: &Rc<Capability>, delegatableAuthority: &Set<Rc<Capability>>, sourceEffectiveAuthority: &Set<Rc<Capability>>) -> bool {
            crate::KipioAccountAccount::_default::ValidAccount(account) && crate::KipioAccountDelegation::_default::ValidDelegation(delegation) && crate::KipioAccountDelegation::_default::DelegationSourceIdentityId(delegation) == crate::KipioAccountIdentity::_default::IdentityId(&crate::KipioAccountAccount::_default::AccountSovereignIdentity(account)) && crate::KipioAccountDelegation::_default::SourceHasDelegationCapability(sourceEffectiveAuthority, delegateCapability) && crate::KipioAccountDelegation::_default::DelegationAuthorityWithinDelegatableAuthority(delegation, delegatableAuthority) && crate::KipioAccountDelegation::_default::DelegatableAuthorityWithinEffectiveAuthority(delegatableAuthority, sourceEffectiveAuthority) && !crate::KipioAccountAuthorizationState::_default::DelegationIdRecognizedInState(&crate::KipioAccountAccount::_default::AccountAuthorizationState(account), &crate::KipioAccountDelegation::_default::DelegationId(delegation))
        }
        /// account/AccountTransitions.dfy(3589,3)
        pub fn RegisterDelegation(account: &Rc<Account>, delegation: &Rc<Delegation>, delegateCapability: &Rc<Capability>, delegatableAuthority: &Set<Rc<Capability>>, sourceEffectiveAuthority: &Set<Rc<Capability>>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state),
                                credentials: crate::KipioAccountAuthorizationState::_default::StateCredentials(&state),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state),
                                sessions: crate::KipioAccountAuthorizationState::_default::StateSessions(&state),
                                delegations: crate::KipioAccountAuthorizationState::_default::StateDelegations(&state).merge(&set!{delegation.clone()}),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state).update_index(&crate::KipioAccountDelegation::_default::DelegationId(delegation), &set!{}),
                                restrictionMap: crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state)
                            })
                })
        }
        /// account/AccountTransitions.dfy(4062,3)
        pub fn ValidTransitiveDelegationRegistration(account: &Rc<Account>, delegation: &Rc<Delegation>, delegateCapability: &Rc<Capability>, delegatableAuthority: &Set<Rc<Capability>>, sourceEffectiveAuthority: &Set<Rc<Capability>>, parentDelegationIds: &Set<Sequence<u8>>, identities: &Set<Rc<Identity>>) -> bool {
            crate::KipioAccountAccount::_default::ValidAccount(account) && crate::KipioAccountDelegation::_default::ValidDelegation(delegation) && !crate::KipioAccountAuthorizationState::_default::DelegationIdRecognizedInState(&crate::KipioAccountAccount::_default::AccountAuthorizationState(account), &crate::KipioAccountDelegation::_default::DelegationId(delegation)) && _default::ParentDelegationsProvideAuthorityToChildSource(&crate::KipioAccountAccount::_default::AccountAuthorizationState(account), identities, delegation, parentDelegationIds, delegateCapability, delegatableAuthority, sourceEffectiveAuthority) && crate::KipioAccountDelegation::_default::DelegationAuthorityWithinDelegatableAuthority(delegation, delegatableAuthority)
        }
        /// account/AccountTransitions.dfy(4098,3)
        pub fn RegisterTransitiveDelegation(account: &Rc<Account>, delegation: &Rc<Delegation>, delegateCapability: &Rc<Capability>, delegatableAuthority: &Set<Rc<Capability>>, sourceEffectiveAuthority: &Set<Rc<Capability>>, parentDelegationIds: &Set<Sequence<u8>>, identities: &Set<Rc<Identity>>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state),
                                credentials: crate::KipioAccountAuthorizationState::_default::StateCredentials(&state),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state),
                                sessions: crate::KipioAccountAuthorizationState::_default::StateSessions(&state),
                                delegations: crate::KipioAccountAuthorizationState::_default::StateDelegations(&state).merge(&set!{delegation.clone()}),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state).update_index(&crate::KipioAccountDelegation::_default::DelegationId(delegation), parentDelegationIds),
                                restrictionMap: crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state)
                            })
                })
        }
        /// account/AccountTransitions.dfy(4734,3)
        pub fn UpdateDelegationStatus(account: &Rc<Account>, delegation: &Rc<Delegation>, status: &Rc<DelegationStatus>) -> Rc<Account> {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            let mut updatedDelegation: Rc<Delegation> = Rc::new(Delegation::Delegation {
                        id: crate::KipioAccountDelegation::_default::DelegationId(delegation),
                        delegatorIdentityId: crate::KipioAccountDelegation::_default::DelegationSourceIdentityId(delegation),
                        delegatee: crate::KipioAccountDelegation::_default::DelegationDelegatee(delegation),
                        capabilities: crate::KipioAccountDelegation::_default::DelegationCapabilities(delegation),
                        validFrom: crate::KipioAccountDelegation::_default::DelegationValidFrom(delegation),
                        validUntil: crate::KipioAccountDelegation::_default::DelegationValidUntil(delegation),
                        restrictions: crate::KipioAccountDelegation::_default::DelegationRestrictions(delegation),
                        metadata: crate::KipioAccountDelegation::_default::DelegationMetadata(delegation),
                        status: status.clone()
                    });
            Rc::new(Account::Account {
                    id: crate::KipioAccountAccount::_default::AccountIdOf(account),
                    identity: crate::KipioAccountAccount::_default::AccountSovereignIdentity(account),
                    authorizationState: Rc::new(AuthorizationState::AuthorizationState {
                                capabilities: crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state),
                                credentials: crate::KipioAccountAuthorizationState::_default::StateCredentials(&state),
                                credentialAuthorities: crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state),
                                sessions: crate::KipioAccountAuthorizationState::_default::StateSessions(&state),
                                delegations: (&({
                                        let mut state = state.clone();
                                        let mut delegation = delegation.clone();
                                        Rc::new(move || -> Set<Rc<Delegation>> {
            let mut _coll0: SetBuilder<Rc<Delegation>> = SetBuilder::<Rc<Delegation>>::new();
            for __compr_0 in (&crate::KipioAccountAuthorizationState::_default::StateDelegations(&state)).iter().cloned() {
                let mut registeredDelegation: Rc<Delegation> = __compr_0.clone();
                if crate::KipioAccountAuthorizationState::_default::StateDelegations(&state).contains(&registeredDelegation) && crate::KipioAccountDelegation::_default::DelegationId(&registeredDelegation) != crate::KipioAccountDelegation::_default::DelegationId(&delegation) {
                    _coll0.add(&registeredDelegation)
                }
            }
            _coll0.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Rc<Delegation>>>
                                    }))().merge(&set!{updatedDelegation.clone()}),
                                delegationProvenance: crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state),
                                restrictionMap: crate::KipioAccountAuthorizationState::_default::StateRestrictionMap(&state),
                                policyEffects: crate::KipioAccountAuthorizationState::_default::StatePolicyEffects(&state),
                                consumedReplayKeys: crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(&state)
                            })
                })
        }
    }
}
/// authorization/Authorization.dfy(172,1)
pub mod KipioAccountAuthorization {
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountExecutionTarget::ExecutionTarget;
    pub use crate::KipioAccountDomainAction::DomainAction;
    pub use crate::KipioAccountScope::Scope;
    pub use crate::KipioAccountChain::Chain;
    pub use ::dafny_runtime::Set;
    pub use crate::KipioAccountCapability::Capability;
    pub use crate::KipioAccountRestriction::Restriction;
    pub use ::dafny_runtime::DafnyInt;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// authorization/Authorization.dfy(249,3)
        pub fn AuthorizationAccountId(authorization: &Rc<Authorization>) -> Sequence<u8> {
            authorization.accountId().clone()
        }
        /// authorization/Authorization.dfy(261,3)
        pub fn AuthorizationExecutionTarget(authorization: &Rc<Authorization>) -> ExecutionTarget {
            authorization.executionTarget().clone()
        }
        /// authorization/Authorization.dfy(273,3)
        pub fn AuthorizationDomainAction(authorization: &Rc<Authorization>) -> DomainAction {
            authorization.domainAction().clone()
        }
        /// authorization/Authorization.dfy(285,3)
        pub fn AuthorizationScope(authorization: &Rc<Authorization>) -> Rc<Scope> {
            authorization.scope().clone()
        }
        /// authorization/Authorization.dfy(296,3)
        pub fn AuthorizationChain(authorization: &Rc<Authorization>) -> Chain {
            authorization.chain().clone()
        }
        /// authorization/Authorization.dfy(312,3)
        pub fn AuthorizationCredentialId(authorization: &Rc<Authorization>) -> Sequence<u8> {
            authorization.credentialId().clone()
        }
        /// authorization/Authorization.dfy(328,3)
        pub fn AuthorizationRequestedAuthority(authorization: &Rc<Authorization>) -> Set<Rc<Capability>> {
            authorization.requestedAuthority().clone()
        }
        /// authorization/Authorization.dfy(338,3)
        pub fn AuthorizationRequestsCapability(authorization: &Rc<Authorization>, capability: &Rc<Capability>) -> bool {
            _default::AuthorizationRequestedAuthority(authorization).contains(capability)
        }
        /// authorization/Authorization.dfy(352,3)
        pub fn AuthorizationRestrictions(authorization: &Rc<Authorization>) -> Set<Rc<Restriction>> {
            authorization.restrictions().clone()
        }
        /// authorization/Authorization.dfy(362,3)
        pub fn AuthorizationContainsRestriction(authorization: &Rc<Authorization>, restriction: &Rc<Restriction>) -> bool {
            _default::AuthorizationRestrictions(authorization).contains(restriction)
        }
        /// authorization/Authorization.dfy(379,3)
        pub fn AuthorizationValidFrom(authorization: &Rc<Authorization>) -> DafnyInt {
            authorization.validFrom().clone()
        }
        /// authorization/Authorization.dfy(388,3)
        pub fn AuthorizationValidUntil(authorization: &Rc<Authorization>) -> DafnyInt {
            authorization.validUntil().clone()
        }
        /// authorization/Authorization.dfy(407,3)
        pub fn AuthorizationReplayKey(authorization: &Rc<Authorization>) -> Sequence<u8> {
            authorization.replayKey().clone()
        }
        /// authorization/Authorization.dfy(631,3)
        pub fn ValidRequestedAuthority(authority: &Set<Rc<Capability>>) -> bool {
            authority.iter().all(({
                    let mut authority = authority.clone();
                    Rc::new(move |__forall_var_0: &Rc<Capability>| -> bool{
            let mut capability: Rc<Capability> = __forall_var_0.clone();
            !authority.contains(&capability) || crate::KipioAccountCapability::_default::ValidCapability(&capability)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authorization/Authorization.dfy(657,3)
        pub fn ValidAuthorization(authorization: &Rc<Authorization>) -> bool {
            crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::AuthorizationCredentialId(authorization)) && _default::ValidRequestedAuthority(&_default::AuthorizationRequestedAuthority(authorization)) && (&_default::AuthorizationRestrictions(authorization)).iter().all(({
                    let mut authorization = authorization.clone();
                    Rc::new(move |__forall_var_0: &Rc<Restriction>| -> bool{
            let mut restriction: Rc<Restriction> = __forall_var_0.clone();
            !_default::AuthorizationRestrictions(&authorization).contains(&restriction) || crate::KipioAccountRestriction::_default::ValidRestriction(&restriction)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref()) && _default::AuthorizationValidFrom(authorization) <= _default::AuthorizationValidUntil(authorization) && crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::AuthorizationReplayKey(authorization)) && crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::AuthorizationAccountId(authorization)) && crate::KipioAccountScope::_default::ValidScope(&_default::AuthorizationScope(authorization))
        }
        /// authorization/Authorization.dfy(778,3)
        pub fn RequestedAuthorityWithinEffectiveAuthority(authorization: &Rc<Authorization>, effectiveAuthority: &Set<Rc<Capability>>) -> bool {
            _default::AuthorizationRequestedAuthority(authorization) <= effectiveAuthority.clone()
        }
    }

    /// authorization/Authorization.dfy(223,3)
    #[derive(Clone)]
    pub enum Authorization {
        Authorization {
            credentialId: Sequence<u8>,
            requestedAuthority: Set<Rc<Capability>>,
            restrictions: Set<Rc<Restriction>>,
            validFrom: DafnyInt,
            validUntil: DafnyInt,
            replayKey: Sequence<u8>,
            accountId: Sequence<u8>,
            executionTarget: ExecutionTarget,
            domainAction: DomainAction,
            scope: Rc<Scope>,
            chain: Chain
        }
    }

    impl Authorization {
        /// Returns a borrow of the field credentialId
        pub fn credentialId(&self) -> &Sequence<u8> {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => credentialId,
            }
        }
        /// Returns a borrow of the field requestedAuthority
        pub fn requestedAuthority(&self) -> &Set<Rc<Capability>> {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => requestedAuthority,
            }
        }
        /// Returns a borrow of the field restrictions
        pub fn restrictions(&self) -> &Set<Rc<Restriction>> {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => restrictions,
            }
        }
        /// Returns a borrow of the field validFrom
        pub fn validFrom(&self) -> &DafnyInt {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => validFrom,
            }
        }
        /// Returns a borrow of the field validUntil
        pub fn validUntil(&self) -> &DafnyInt {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => validUntil,
            }
        }
        /// Returns a borrow of the field replayKey
        pub fn replayKey(&self) -> &Sequence<u8> {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => replayKey,
            }
        }
        /// Returns a borrow of the field accountId
        pub fn accountId(&self) -> &Sequence<u8> {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => accountId,
            }
        }
        /// Returns a borrow of the field executionTarget
        pub fn executionTarget(&self) -> &ExecutionTarget {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => executionTarget,
            }
        }
        /// Returns a borrow of the field domainAction
        pub fn domainAction(&self) -> &DomainAction {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => domainAction,
            }
        }
        /// Returns a borrow of the field scope
        pub fn scope(&self) -> &Rc<Scope> {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => scope,
            }
        }
        /// Returns a borrow of the field chain
        pub fn chain(&self) -> &Chain {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => chain,
            }
        }
    }

    impl Debug
        for Authorization {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for Authorization {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => {
                    write!(_formatter, "KipioAccountAuthorization.Authorization.Authorization(")?;
                    DafnyPrint::fmt_print(credentialId, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(requestedAuthority, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(restrictions, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(validFrom, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(validUntil, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(replayKey, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(accountId, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(executionTarget, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(domainAction, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(scope, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(chain, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for Authorization {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, }, Authorization::Authorization{credentialId: _2_credentialId, requestedAuthority: _2_requestedAuthority, restrictions: _2_restrictions, validFrom: _2_validFrom, validUntil: _2_validUntil, replayKey: _2_replayKey, accountId: _2_accountId, executionTarget: _2_executionTarget, domainAction: _2_domainAction, scope: _2_scope, chain: _2_chain, }) => {
                    credentialId == _2_credentialId && requestedAuthority == _2_requestedAuthority && restrictions == _2_restrictions && validFrom == _2_validFrom && validUntil == _2_validUntil && replayKey == _2_replayKey && accountId == _2_accountId && executionTarget == _2_executionTarget && domainAction == _2_domainAction && scope == _2_scope && chain == _2_chain
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for Authorization {}

    impl Hash
        for Authorization {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                Authorization::Authorization{credentialId, requestedAuthority, restrictions, validFrom, validUntil, replayKey, accountId, executionTarget, domainAction, scope, chain, } => {
                    Hash::hash(credentialId, _state);
                    Hash::hash(requestedAuthority, _state);
                    Hash::hash(restrictions, _state);
                    Hash::hash(validFrom, _state);
                    Hash::hash(validUntil, _state);
                    Hash::hash(replayKey, _state);
                    Hash::hash(accountId, _state);
                    Hash::hash(executionTarget, _state);
                    Hash::hash(domainAction, _state);
                    Hash::hash(scope, _state);
                    Hash::hash(chain, _state)
                },
            }
        }
    }

    impl AsRef<Authorization>
        for Authorization {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// authorization/AuthorizationAcceptanceComposition.dfy(26,1)
pub mod KipioAccountAuthorizationAcceptanceComposition {
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountAuthorization::Authorization;
    pub use crate::KipioAccountAccount::Account;
    pub use crate::KipioAccountAuthorizationState::AuthorizationState;
    pub use ::dafny_runtime::Set;
    pub use crate::KipioAccountCapability::Capability;
    pub use ::dafny_runtime::DafnyInt;

    pub struct _default {}

    impl _default {
        /// authorization/AuthorizationAcceptanceComposition.dfy(33,3)
        pub fn AuthorizationCanBeAccepted(authorization: &Rc<Authorization>, account: &Rc<Account>, state: &Rc<AuthorizationState>, effectiveAuthority: &Set<Rc<Capability>>, now: &DafnyInt, proofVerified: bool) -> bool {
            crate::KipioAccountAuthorizationValidation::_default::AuthorizationIsStructurallyValid(authorization) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationCredentialIsUsable(authorization, state) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationContextMatchesAccount(authorization, account) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationIsTemporallyValidAt(authorization, now) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationReplayIsFresh(authorization, state) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationAuthorityIsEffective(authorization, effectiveAuthority) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationProofIsVerified(proofVerified)
        }
    }
}
/// authorization/AuthorizationAcceptanceTransition.dfy(104,1)
pub mod KipioAccountAuthorizationAcceptanceTransition {
    
}
/// authorization/AuthorizationAcceptanceWithProvenance.dfy(54,1)
pub mod KipioAccountAuthorizationAcceptanceWithProvenance {
    pub use ::dafny_runtime::Set;
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountCapability::Capability;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountEffectiveAuthority::AuthoritySourceReference;
    pub use ::dafny_runtime::integer_range;
    pub use ::dafny_runtime::int;
    pub use ::dafny_runtime::DafnyInt;
    pub use crate::KipioAccountAuthorizationState::AuthorizationState;
    pub use crate::KipioAccountAccount::Account;
    pub use crate::KipioAccountIdentity::Identity;
    pub use ::dafny_runtime::set;
    pub use crate::KipioAccountEffectiveAuthority::AuthoritySourceKind;
    pub use ::dafny_runtime::SetBuilder;
    pub use crate::KipioAccountSession::Session;
    pub use crate::KipioAccountDelegation::Delegation;
    pub use crate::KipioAccountEffectiveAuthority::AuthoritySourceKind::AccountAuthoritySource;
    pub use crate::KipioAccountEffectiveAuthority::AuthoritySourceKind::CredentialAuthoritySource;
    pub use crate::KipioAccountEffectiveAuthority::AuthoritySourceKind::SessionAuthoritySource;
    pub use crate::KipioAccountAuthorization::Authorization;
    pub use ::std::default::Default;

    pub struct _default {}

    impl _default {
        /// authorization/AuthorizationAcceptanceWithProvenance.dfy(79,5)
        pub fn EffectiveAuthorityDerivedFromAttributedSourcesExec(effectiveAuthority: &Set<Rc<Capability>>, sourceReferences: &Sequence<Rc<AuthoritySourceReference>>, sourceAuthorities: &Sequence<Set<Rc<Capability>>>, contributions: &Sequence<Set<Rc<Capability>>>) -> bool {
            sourceReferences.cardinality() == sourceAuthorities.cardinality() && sourceAuthorities.cardinality() == contributions.cardinality() && integer_range(int!(0), sourceReferences.cardinality()).all(({
                    let mut sourceReferences = sourceReferences.clone();
                    Rc::new(move |__forall_var_0: DafnyInt| -> bool{
            let mut i: DafnyInt = __forall_var_0.clone();
            !(int!(0) <= i.clone() && i.clone() < sourceReferences.cardinality()) || crate::KipioAccountEffectiveAuthority::_default::ValidAuthoritySourceReference(&sourceReferences.get(&i))
        }) as Rc<dyn ::std::ops::Fn(_) -> _>
                }).as_ref()) && integer_range(int!(0), contributions.cardinality()).all(({
                    let mut contributions = contributions.clone();
                    let mut sourceAuthorities = sourceAuthorities.clone();
                    Rc::new(move |__forall_var_1: DafnyInt| -> bool{
            let mut i: DafnyInt = __forall_var_1.clone();
            !(int!(0) <= i.clone() && i.clone() < contributions.cardinality()) || contributions.get(&i) <= sourceAuthorities.get(&i)
        }) as Rc<dyn ::std::ops::Fn(_) -> _>
                }).as_ref()) && effectiveAuthority.iter().all(({
                    let mut contributions = contributions.clone();
                    let mut effectiveAuthority = effectiveAuthority.clone();
                    Rc::new(move |__forall_var_2: &Rc<Capability>| -> bool{
            let mut capability: Rc<Capability> = __forall_var_2.clone();
            !effectiveAuthority.contains(&capability) || integer_range(int!(0), contributions.cardinality()).any(({
                    let mut contributions = contributions.clone();
                    let mut capability = capability.clone();
                    Rc::new(move |__exists_var_0: DafnyInt| -> bool{
            let mut i: DafnyInt = __exists_var_0.clone();
            int!(0) <= i.clone() && i.clone() < contributions.cardinality() && contributions.get(&i).contains(&capability)
        }) as Rc<dyn ::std::ops::Fn(_) -> _>
                }).as_ref())
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref()) && integer_range(int!(0), contributions.cardinality()).all(({
                    let mut contributions = contributions.clone();
                    let mut effectiveAuthority = effectiveAuthority.clone();
                    Rc::new(move |__forall_var_3: DafnyInt| -> bool{
            let mut i: DafnyInt = __forall_var_3.clone();
            !(int!(0) <= i.clone() && i.clone() < contributions.cardinality()) || (&contributions.get(&i)).iter().all(({
                    let mut contributions = contributions.clone();
                    let mut effectiveAuthority = effectiveAuthority.clone();
                    let mut i = i.clone();
                    Rc::new(move |__forall_var_4: &Rc<Capability>| -> bool{
            let mut capability: Rc<Capability> = __forall_var_4.clone();
            !contributions.get(&i).contains(&capability) || effectiveAuthority.contains(&capability)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }) as Rc<dyn ::std::ops::Fn(_) -> _>
                }).as_ref())
        }
        /// authorization/AuthorizationAcceptanceWithProvenance.dfy(132,5)
        pub fn CurrentlyUsableAuthoritySourceReferencesForAcceptanceExec(state: &Rc<AuthorizationState>, account: &Rc<Account>, identities: &Set<Rc<Identity>>, now: &DafnyInt) -> Set<Rc<AuthoritySourceReference>> {
            set!{Rc::new(AuthoritySourceReference::AuthoritySourceReference {
                        kind: Rc::new(AuthoritySourceKind::AccountAuthoritySource {}),
                        id: crate::KipioAccountAccount::_default::AccountIdOf(account)
                    })}.merge(&(&({
                    let mut state = state.clone();
                    Rc::new(move || -> Set<Rc<AuthoritySourceReference>> {
            let mut _coll0: SetBuilder<Rc<AuthoritySourceReference>> = SetBuilder::<Rc<AuthoritySourceReference>>::new();
            for __compr_0 in (&crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state).keys()).iter().cloned() {
                let mut credentialId: Sequence<u8> = __compr_0.clone();
                if crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state).keys().contains(&credentialId) && crate::KipioAccountAuthorizationState::_default::ActiveCredentialRecognizedInState(&state, &credentialId) {
                    _coll0.add(&Rc::new(AuthoritySourceReference::AuthoritySourceReference {
                                kind: Rc::new(AuthoritySourceKind::CredentialAuthoritySource {}),
                                id: credentialId.clone()
                            }))
                }
            }
            _coll0.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Rc<AuthoritySourceReference>>>
                }))()).merge(&(&({
                    let mut now = now.clone();
                    let mut state = state.clone();
                    Rc::new(move || -> Set<Rc<AuthoritySourceReference>> {
            let mut _coll1: SetBuilder<Rc<AuthoritySourceReference>> = SetBuilder::<Rc<AuthoritySourceReference>>::new();
            for __compr_1 in (&crate::KipioAccountAuthorizationState::_default::StateSessions(&state)).iter().cloned() {
                let mut session: Rc<Session> = __compr_1.clone();
                if crate::KipioAccountAuthorizationState::_default::StateSessions(&state).contains(&session) && crate::KipioAccountAuthorizationState::_default::SessionCanContributeAuthorityAt(&state, &session, &now) {
                    _coll1.add(&Rc::new(AuthoritySourceReference::AuthoritySourceReference {
                                kind: Rc::new(AuthoritySourceKind::SessionAuthoritySource {}),
                                id: crate::KipioAccountSession::_default::SessionId(&session)
                            }))
                }
            }
            _coll1.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Rc<AuthoritySourceReference>>>
                }))()).merge(&(&({
                    let mut now = now.clone();
                    let mut state = state.clone();
                    let mut account = account.clone();
                    let mut identities = identities.clone();
                    Rc::new(move || -> Set<Rc<AuthoritySourceReference>> {
            let mut _coll2: SetBuilder<Rc<AuthoritySourceReference>> = SetBuilder::<Rc<AuthoritySourceReference>>::new();
            for __compr_2 in (&crate::KipioAccountAuthorizationState::_default::StateDelegations(&state)).iter().cloned() {
                let mut delegation: Rc<Delegation> = __compr_2.clone();
                if crate::KipioAccountAuthorizationState::_default::StateDelegations(&state).contains(&delegation) && crate::KipioAccountAuthorizationState::_default::DelegationCanContributeAuthorityAt(&state, &delegation, &now) && crate::KipioAccountEffectiveAuthorityComposition::_default::DelegationHasLegitimateSourceProvenanceForAccountExecutable(&account, &state, &identities, &delegation) {
                    _coll2.add(&Rc::new(AuthoritySourceReference::AuthoritySourceReference {
                                kind: Rc::new(AuthoritySourceKind::DelegationAuthoritySource {}),
                                id: crate::KipioAccountDelegation::_default::DelegationId(&delegation)
                            }))
                }
            }
            _coll2.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Rc<AuthoritySourceReference>>>
                }))())
        }
        /// authorization/AuthorizationAcceptanceWithProvenance.dfy(182,5)
        pub fn AuthoritySourceReferenceResolvesToAuthorityExec(state: &Rc<AuthorizationState>, account: &Rc<Account>, identities: &Set<Rc<Identity>>, now: &DafnyInt, sourceReference: &Rc<AuthoritySourceReference>, sourceAuthority: &Set<Rc<Capability>>) -> bool {
            let mut _source0: Rc<AuthoritySourceKind> = crate::KipioAccountEffectiveAuthority::_default::AuthoritySourceReferenceKind(sourceReference);
            if matches!((&_source0).as_ref(), AccountAuthoritySource{ .. }) {
                crate::KipioAccountEffectiveAuthority::_default::AuthoritySourceReferenceId(sourceReference) == crate::KipioAccountAccount::_default::AccountIdOf(account) && sourceAuthority.clone() == crate::KipioAccountAuthorizationState::_default::StateCapabilities(state)
            } else {
                if matches!((&_source0).as_ref(), CredentialAuthoritySource{ .. }) {
                    crate::KipioAccountAuthorizationState::_default::CredentialAuthorityDefinedForCredentialId(state, &crate::KipioAccountEffectiveAuthority::_default::AuthoritySourceReferenceId(sourceReference)) && crate::KipioAccountAuthorizationState::_default::ActiveCredentialRecognizedInState(state, &crate::KipioAccountEffectiveAuthority::_default::AuthoritySourceReferenceId(sourceReference)) && sourceAuthority.clone() == crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorityById(state, &crate::KipioAccountEffectiveAuthority::_default::AuthoritySourceReferenceId(sourceReference))
                } else {
                    if matches!((&_source0).as_ref(), SessionAuthoritySource{ .. }) {
                        (&crate::KipioAccountAuthorizationState::_default::StateSessions(state)).iter().any(({
                                let mut sourceAuthority = sourceAuthority.clone();
                                let mut sourceReference = sourceReference.clone();
                                let mut now = now.clone();
                                let mut state = state.clone();
                                Rc::new(move |__exists_var_0: &Rc<Session>| -> bool{
            let mut session: Rc<Session> = __exists_var_0.clone();
            crate::KipioAccountAuthorizationState::_default::StateSessions(&state).contains(&session) && crate::KipioAccountSession::_default::SessionId(&session) == crate::KipioAccountEffectiveAuthority::_default::AuthoritySourceReferenceId(&sourceReference) && crate::KipioAccountAuthorizationState::_default::SessionCanContributeAuthorityAt(&state, &session, &now) && sourceAuthority.clone() == crate::KipioAccountSession::_default::SessionCapabilities(&session)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                            }).as_ref())
                    } else {
                        (&crate::KipioAccountAuthorizationState::_default::StateDelegations(state)).iter().any(({
                                let mut sourceAuthority = sourceAuthority.clone();
                                let mut sourceReference = sourceReference.clone();
                                let mut now = now.clone();
                                let mut state = state.clone();
                                let mut account = account.clone();
                                let mut identities = identities.clone();
                                Rc::new(move |__exists_var_1: &Rc<Delegation>| -> bool{
            let mut delegation: Rc<Delegation> = __exists_var_1.clone();
            crate::KipioAccountAuthorizationState::_default::StateDelegations(&state).contains(&delegation) && crate::KipioAccountDelegation::_default::DelegationId(&delegation) == crate::KipioAccountEffectiveAuthority::_default::AuthoritySourceReferenceId(&sourceReference) && crate::KipioAccountAuthorizationState::_default::DelegationCanContributeAuthorityAt(&state, &delegation, &now) && crate::KipioAccountEffectiveAuthorityComposition::_default::DelegationHasLegitimateSourceProvenanceForAccountExecutable(&account, &state, &identities, &delegation) && sourceAuthority.clone() == crate::KipioAccountDelegation::_default::DelegationCapabilities(&delegation)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                            }).as_ref())
                    }
                }
            }
        }
        /// authorization/AuthorizationAcceptanceWithProvenance.dfy(336,5)
        pub fn EffectiveAuthoritySourceReferencesAreCurrentlyUsableExec(state: &Rc<AuthorizationState>, account: &Rc<Account>, identities: &Set<Rc<Identity>>, now: &DafnyInt, sourceReferences: &Sequence<Rc<AuthoritySourceReference>>) -> bool {
            integer_range(int!(0), sourceReferences.cardinality()).all(({
                    let mut sourceReferences = sourceReferences.clone();
                    let mut now = now.clone();
                    let mut state = state.clone();
                    let mut account = account.clone();
                    let mut identities = identities.clone();
                    Rc::new(move |__forall_var_0: DafnyInt| -> bool{
            let mut i: DafnyInt = __forall_var_0.clone();
            !(int!(0) <= i.clone() && i.clone() < sourceReferences.cardinality()) || _default::CurrentlyUsableAuthoritySourceReferencesForAcceptanceExec(&state, &account, &identities, &now).contains(&sourceReferences.get(&i))
        }) as Rc<dyn ::std::ops::Fn(_) -> _>
                }).as_ref())
        }
        /// authorization/AuthorizationAcceptanceWithProvenance.dfy(516,5)
        pub fn EffectiveAuthorityHasRecognizedAttributedProvenanceForAccountAtExec(effectiveAuthority: &Set<Rc<Capability>>, account: &Rc<Account>, identities: &Set<Rc<Identity>>, now: &DafnyInt, sourceReferences: &Sequence<Rc<AuthoritySourceReference>>, sourceAuthorities: &Sequence<Set<Rc<Capability>>>, contributions: &Sequence<Set<Rc<Capability>>>) -> bool {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            crate::KipioAccountEffectiveAuthorityComposition::_default::AuthorizationIdentityContextIsValidExecutable(identities) && _default::EffectiveAuthorityDerivedFromAttributedSourcesExec(effectiveAuthority, sourceReferences, sourceAuthorities, contributions) && _default::EffectiveAuthoritySourceReferencesAreCurrentlyUsableExec(&state, account, identities, now, sourceReferences) && integer_range(int!(0), sourceReferences.cardinality()).all(({
                    let mut sourceReferences = sourceReferences.clone();
                    let mut sourceAuthorities = sourceAuthorities.clone();
                    let mut now = now.clone();
                    let mut state = state.clone();
                    let mut account = account.clone();
                    let mut identities = identities.clone();
                    Rc::new(move |__forall_var_0: DafnyInt| -> bool{
            let mut i: DafnyInt = __forall_var_0.clone();
            !(int!(0) <= i.clone() && i.clone() < sourceReferences.cardinality()) || _default::AuthoritySourceReferenceResolvesToAuthorityExec(&state, &account, &identities, &now, &sourceReferences.get(&i), &sourceAuthorities.get(&i))
        }) as Rc<dyn ::std::ops::Fn(_) -> _>
                }).as_ref())
        }
        /// authorization/AuthorizationAcceptanceWithProvenance.dfy(555,5)
        pub fn EffectiveAuthorityProvenanceIsValidForAccountAtExec(effectiveAuthority: &Set<Rc<Capability>>, account: &Rc<Account>, identities: &Set<Rc<Identity>>, now: &DafnyInt, sourceReferences: &Sequence<Rc<AuthoritySourceReference>>, sourceAuthorities: &Sequence<Set<Rc<Capability>>>, contributions: &Sequence<Set<Rc<Capability>>>) -> bool {
            _default::EffectiveAuthorityHasRecognizedAttributedProvenanceForAccountAtExec(effectiveAuthority, account, identities, now, sourceReferences, sourceAuthorities, contributions)
        }
        /// authorization/AuthorizationAcceptanceWithProvenance.dfy(581,5)
        pub fn AuthorizationCanBeAcceptedWithProvenance(authorization: &Rc<Authorization>, account: &Rc<Account>, effectiveAuthority: &Set<Rc<Capability>>, identities: &Set<Rc<Identity>>, sourceReferences: &Sequence<Rc<AuthoritySourceReference>>, sourceAuthorities: &Sequence<Set<Rc<Capability>>>, contributions: &Sequence<Set<Rc<Capability>>>, now: &DafnyInt, proofVerified: bool) -> bool {
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            crate::KipioAccountAccount::_default::ValidAccount(account) && crate::KipioAccountEffectiveAuthorityComposition::_default::AuthorizationIdentityContextIsValidExecutable(identities) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationIsStructurallyValid(authorization) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationContextMatchesAccount(authorization, account) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationCredentialIsUsable(authorization, &state) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationIsTemporallyValidAt(authorization, now) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationReplayIsFresh(authorization, &state) && crate::KipioAccountAuthorizationValidation::_default::EffectiveAuthorityIsValid(effectiveAuthority) && _default::EffectiveAuthorityProvenanceIsValidForAccountAtExec(effectiveAuthority, account, identities, now, sourceReferences, sourceAuthorities, contributions) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationAuthorityIsEffective(authorization, effectiveAuthority) && crate::KipioAccountAuthorizationValidation::_default::AuthorizationProofIsVerified(proofVerified)
        }
        /// authorization/AuthorizationAcceptanceWithProvenance.dfy(683,5)
        pub fn AuthorizationCanBeAcceptedFull(authorization: &Rc<Authorization>, account: &Rc<Account>, identities: &Set<Rc<Identity>>, now: &DafnyInt, proofVerified: bool) -> bool {
            let mut accepted: bool = <bool as Default>::default();
            let mut effectiveAuthority: Set<Rc<Capability>>;
            let mut sourceReferences: Sequence<Rc<AuthoritySourceReference>>;
            let mut sourceAuthorities: Sequence<Set<Rc<Capability>>>;
            let mut contributions: Sequence<Set<Rc<Capability>>>;
            let mut _out0: Set<Rc<Capability>>;
            let mut _out1: Sequence<Rc<AuthoritySourceReference>>;
            let mut _out2: Sequence<Set<Rc<Capability>>>;
            let mut _out3: Sequence<Set<Rc<Capability>>>;
            let _x = crate::KipioAccountEffectiveAuthorityComposition::_default::ComputeEffectiveAuthorityWitness(account, identities, now);
            _out0 = _x.0;
            _out1 = _x.1;
            _out2 = _x.2;
            _out3 = _x.3;
            effectiveAuthority = _out0.clone();
            sourceReferences = _out1.clone();
            sourceAuthorities = _out2.clone();
            contributions = _out3.clone();
            accepted = _default::AuthorizationCanBeAcceptedWithProvenance(authorization, account, &effectiveAuthority, identities, &sourceReferences, &sourceAuthorities, &contributions, now, proofVerified);
            return accepted;
        }
    }
}
/// authority/AuthorizationState.dfy(144,1)
pub mod KipioAccountAuthorizationState {
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountCapability::Capability;
    pub use crate::KipioAccountPolicyEffect::OptionalPolicyScope;
    pub use crate::KipioAccountPolicyEffect::OptionalPolicyScope::NoScope;
    pub use crate::KipioAccountScope::Scope;
    pub use ::dafny_runtime::Set;
    pub use crate::KipioAccountCredential::Credential;
    pub use ::dafny_runtime::Map;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountSession::Session;
    pub use crate::KipioAccountDelegation::Delegation;
    pub use crate::KipioAccountRestriction::Restriction;
    pub use ::dafny_runtime::SetBuilder;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect;
    pub use ::std::iter;
    pub use ::dafny_runtime::int;
    pub use crate::KipioAccountIdentity::Identity;
    pub use crate::KipioAccountSubject::Subject;
    pub use ::dafny_runtime::DafnyInt;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// authority/AuthorizationState.dfy(209,3)
        pub fn RestrictionTargetCapability(target: &Rc<RestrictionTarget>) -> Rc<Capability> {
            target.capability().clone()
        }
        /// authority/AuthorizationState.dfy(217,3)
        pub fn RestrictionTargetScope(target: &Rc<RestrictionTarget>) -> Rc<OptionalPolicyScope> {
            target.scope().clone()
        }
        /// authority/AuthorizationState.dfy(238,3)
        pub fn ValidRestrictionTarget(target: &Rc<RestrictionTarget>) -> bool {
            crate::KipioAccountCapability::_default::ValidCapability(&_default::RestrictionTargetCapability(target)) && (&({
                Rc::new(move |_source0: &Rc<OptionalPolicyScope>| -> bool{
            if matches!(_source0.as_ref(), NoScope{ .. }) {
                true
            } else {
                let mut ___mcc_h0: Rc<Scope> = _source0.scope().clone();
                let mut scope: Rc<Scope> = ___mcc_h0.clone();
                crate::KipioAccountScope::_default::ValidScope(&scope)
            }
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
            }))(&_default::RestrictionTargetScope(target))
        }
        /// authority/AuthorizationState.dfy(285,3)
        pub fn StateCapabilities(state: &Rc<AuthorizationState>) -> Set<Rc<Capability>> {
            state.capabilities().clone()
        }
        /// authority/AuthorizationState.dfy(293,3)
        pub fn StateCredentials(state: &Rc<AuthorizationState>) -> Set<Rc<Credential>> {
            state.credentials().clone()
        }
        /// authority/AuthorizationState.dfy(301,3)
        pub fn StateCredentialAuthorities(state: &Rc<AuthorizationState>) -> Map<Sequence<u8>, Set<Rc<Capability>>> {
            state.credentialAuthorities().clone()
        }
        /// authority/AuthorizationState.dfy(309,3)
        pub fn StateSessions(state: &Rc<AuthorizationState>) -> Set<Rc<Session>> {
            state.sessions().clone()
        }
        /// authority/AuthorizationState.dfy(317,3)
        pub fn StateDelegations(state: &Rc<AuthorizationState>) -> Set<Rc<Delegation>> {
            state.delegations().clone()
        }
        /// authority/AuthorizationState.dfy(325,3)
        pub fn StateDelegationProvenance(state: &Rc<AuthorizationState>) -> Map<Sequence<u8>, Set<Sequence<u8>>> {
            state.delegationProvenance().clone()
        }
        /// authority/AuthorizationState.dfy(334,3)
        pub fn StateRestrictionMap(state: &Rc<AuthorizationState>) -> Map<Rc<RestrictionTarget>, Rc<Restriction>> {
            state.restrictionMap().clone()
        }
        /// authority/AuthorizationState.dfy(356,3)
        pub fn StateRestrictions(state: &Rc<AuthorizationState>) -> Set<Rc<Restriction>> {
            (&({
                let mut state = state.clone();
                Rc::new(move || -> Set<Rc<Restriction>> {
            let mut _coll0: SetBuilder<Rc<Restriction>> = SetBuilder::<Rc<Restriction>>::new();
            for __compr_0 in (&_default::StateRestrictionMap(&state).keys()).iter().cloned() {
                let mut target: Rc<RestrictionTarget> = __compr_0.clone();
                if _default::StateRestrictionMap(&state).keys().contains(&target) {
                    _coll0.add(&_default::StateRestrictionMap(&state).get(&target))
                }
            }
            _coll0.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Rc<Restriction>>>
            }))()
        }
        /// authority/AuthorizationState.dfy(366,3)
        pub fn StatePolicyEffects(state: &Rc<AuthorizationState>) -> Set<Rc<PolicyEffect>> {
            state.policyEffects().clone()
        }
        /// authority/AuthorizationState.dfy(374,3)
        pub fn StateConsumedReplayKeys(state: &Rc<AuthorizationState>) -> Set<Sequence<u8>> {
            state.consumedReplayKeys().clone()
        }
        /// authority/AuthorizationState.dfy(388,3)
        pub fn RestrictionDefinedForTarget(state: &Rc<AuthorizationState>, target: &Rc<RestrictionTarget>) -> bool {
            _default::StateRestrictionMap(state).keys().contains(target)
        }
        /// authority/AuthorizationState.dfy(400,3)
        pub fn RestrictionDefinedFor(state: &Rc<AuthorizationState>, capability: &Rc<Capability>, scope: &Rc<OptionalPolicyScope>) -> bool {
            _default::RestrictionDefinedForTarget(state, &Rc::new(RestrictionTarget::RestrictionTarget {
                        capability: capability.clone(),
                        scope: scope.clone()
                    }))
        }
        /// authority/AuthorizationState.dfy(418,3)
        pub fn StateRestrictionForTarget(state: &Rc<AuthorizationState>, target: &Rc<RestrictionTarget>) -> Rc<Restriction> {
            _default::StateRestrictionMap(state).get(target)
        }
        /// authority/AuthorizationState.dfy(434,3)
        pub fn StateRestrictionFor(state: &Rc<AuthorizationState>, capability: &Rc<Capability>, scope: &Rc<OptionalPolicyScope>) -> Rc<Restriction> {
            _default::StateRestrictionMap(state).get(&Rc::new(RestrictionTarget::RestrictionTarget {
                        capability: capability.clone(),
                        scope: scope.clone()
                    }))
        }
        /// authority/AuthorizationState.dfy(467,3)
        pub fn RestrictionTargetsAreUnique(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateRestrictionMap(state).keys()).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<RestrictionTarget>| -> bool{
            let mut left: Rc<RestrictionTarget> = __forall_var_0.clone();
            iter::once(left.clone()).all(({
                    let mut left = left.clone();
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_1: Rc<RestrictionTarget>| -> bool{
            let mut right: Rc<RestrictionTarget> = __forall_var_1.clone();
            !(_default::StateRestrictionMap(&state).keys().contains(&left) && _default::StateRestrictionMap(&state).keys().contains(&right) && left.clone() == right.clone()) || left.clone() == right.clone()
        }) as Rc<dyn ::std::ops::Fn(_) -> _>
                }).as_ref())
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(484,3)
        pub fn RestrictionsBelongToRecognizedCapabilities(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateRestrictionMap(state).keys()).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<RestrictionTarget>| -> bool{
            let mut target: Rc<RestrictionTarget> = __forall_var_0.clone();
            !_default::StateRestrictionMap(&state).keys().contains(&target) || _default::StateCapabilities(&state).contains(&_default::RestrictionTargetCapability(&target))
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(498,3)
        pub fn ValidRestrictionTargetsInState(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateRestrictionMap(state).keys()).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<RestrictionTarget>| -> bool{
            let mut target: Rc<RestrictionTarget> = __forall_var_0.clone();
            !_default::StateRestrictionMap(&state).keys().contains(&target) || _default::ValidRestrictionTarget(&target)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(511,3)
        pub fn ValidRestrictionValuesInState(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateRestrictionMap(state).keys()).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<RestrictionTarget>| -> bool{
            let mut target: Rc<RestrictionTarget> = __forall_var_0.clone();
            !_default::StateRestrictionMap(&state).keys().contains(&target) || crate::KipioAccountRestriction::_default::ValidRestriction(&_default::StateRestrictionMap(&state).get(&target))
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(529,3)
        pub fn NoOrphanRestrictionsInState(state: &Rc<AuthorizationState>) -> bool {
            _default::RestrictionsBelongToRecognizedCapabilities(state)
        }
        /// authority/AuthorizationState.dfy(539,3)
        pub fn ValidRestrictionMapInState(state: &Rc<AuthorizationState>) -> bool {
            _default::ValidRestrictionTargetsInState(state) && _default::ValidRestrictionValuesInState(state) && _default::NoOrphanRestrictionsInState(state)
        }
        /// authority/AuthorizationState.dfy(633,3)
        pub fn CredentialRecognizedInState(state: &Rc<AuthorizationState>, credential: &Rc<Credential>) -> bool {
            (&_default::StateCredentials(state)).iter().any(({
                    let mut state = state.clone();
                    let mut credential = credential.clone();
                    Rc::new(move |__exists_var_0: &Rc<Credential>| -> bool{
            let mut registeredCredential: Rc<Credential> = __exists_var_0.clone();
            _default::StateCredentials(&state).contains(&registeredCredential) && crate::KipioAccountCredential::_default::CredentialId(&registeredCredential) == crate::KipioAccountCredential::_default::CredentialId(&credential)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(645,3)
        pub fn CredentialIdRecognizedInState(state: &Rc<AuthorizationState>, credentialId: &Sequence<u8>) -> bool {
            (&_default::StateCredentials(state)).iter().any(({
                    let mut credentialId = credentialId.clone();
                    let mut state = state.clone();
                    Rc::new(move |__exists_var_0: &Rc<Credential>| -> bool{
            let mut credential: Rc<Credential> = __exists_var_0.clone();
            _default::StateCredentials(&state).contains(&credential) && crate::KipioAccountCredential::_default::CredentialId(&credential) == credentialId.clone()
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(660,3)
        pub fn SessionIdRecognizedInState(state: &Rc<AuthorizationState>, sessionId: &Sequence<u8>) -> bool {
            (&_default::StateSessions(state)).iter().any(({
                    let mut state = state.clone();
                    let mut sessionId = sessionId.clone();
                    Rc::new(move |__exists_var_0: &Rc<Session>| -> bool{
            let mut session: Rc<Session> = __exists_var_0.clone();
            _default::StateSessions(&state).contains(&session) && crate::KipioAccountSession::_default::SessionId(&session) == sessionId.clone()
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(675,3)
        pub fn DelegationIdRecognizedInState(state: &Rc<AuthorizationState>, delegationId: &Sequence<u8>) -> bool {
            (&_default::StateDelegations(state)).iter().any(({
                    let mut state = state.clone();
                    let mut delegationId = delegationId.clone();
                    Rc::new(move |__exists_var_0: &Rc<Delegation>| -> bool{
            let mut delegation: Rc<Delegation> = __exists_var_0.clone();
            _default::StateDelegations(&state).contains(&delegation) && crate::KipioAccountDelegation::_default::DelegationId(&delegation) == delegationId.clone()
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(690,3)
        pub fn CredentialAuthorityDefinedInState(state: &Rc<AuthorizationState>, credential: &Rc<Credential>) -> bool {
            _default::StateCredentialAuthorities(state).keys().contains(&crate::KipioAccountCredential::_default::CredentialId(credential))
        }
        /// authority/AuthorizationState.dfy(700,3)
        pub fn CredentialAuthorityDefinedForCredentialId(state: &Rc<AuthorizationState>, credentialId: &Sequence<u8>) -> bool {
            _default::StateCredentialAuthorities(state).keys().contains(credentialId)
        }
        /// authority/AuthorizationState.dfy(710,3)
        pub fn StateCredentialAuthority(state: &Rc<AuthorizationState>, credential: &Rc<Credential>) -> Set<Rc<Capability>> {
            _default::StateCredentialAuthorities(state).get(&crate::KipioAccountCredential::_default::CredentialId(credential))
        }
        /// authority/AuthorizationState.dfy(729,3)
        pub fn StateCredentialAuthorityById(state: &Rc<AuthorizationState>, credentialId: &Sequence<u8>) -> Set<Rc<Capability>> {
            _default::StateCredentialAuthorities(state).get(credentialId)
        }
        /// authority/AuthorizationState.dfy(750,3)
        pub fn DelegationProvenanceDefinedInState(state: &Rc<AuthorizationState>, delegation: &Rc<Delegation>) -> bool {
            _default::StateDelegationProvenance(state).keys().contains(&crate::KipioAccountDelegation::_default::DelegationId(delegation))
        }
        /// authority/AuthorizationState.dfy(760,3)
        pub fn DelegationProvenanceDefinedForDelegationId(state: &Rc<AuthorizationState>, delegationId: &Sequence<u8>) -> bool {
            _default::StateDelegationProvenance(state).keys().contains(delegationId)
        }
        /// authority/AuthorizationState.dfy(770,3)
        pub fn StateDelegationParents(state: &Rc<AuthorizationState>, delegation: &Rc<Delegation>) -> Set<Sequence<u8>> {
            _default::StateDelegationProvenance(state).get(&crate::KipioAccountDelegation::_default::DelegationId(delegation))
        }
        /// authority/AuthorizationState.dfy(785,3)
        pub fn DelegationHasParentProvenance(state: &Rc<AuthorizationState>, delegation: &Rc<Delegation>) -> bool {
            int!(0) < _default::StateDelegationParents(state, delegation).cardinality()
        }
        /// authority/AuthorizationState.dfy(798,3)
        pub fn DelegationProvenanceParentsBelongToState(state: &Rc<AuthorizationState>, delegation: &Rc<Delegation>) -> bool {
            (&_default::StateDelegationParents(state, delegation)).iter().all(({
                    let mut state = state.clone();
                    let mut delegation = delegation.clone();
                    Rc::new(move |__forall_var_0: &Sequence<u8>| -> bool{
            let mut parentId: Sequence<u8> = __forall_var_0.clone();
            !_default::StateDelegationParents(&state, &delegation).contains(&parentId) || _default::DelegationIdRecognizedInState(&state, &parentId)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(817,3)
        pub fn DelegationProvenanceHasNoSelfReference(state: &Rc<AuthorizationState>, delegation: &Rc<Delegation>) -> bool {
            !_default::StateDelegationParents(state, delegation).contains(&crate::KipioAccountDelegation::_default::DelegationId(delegation))
        }
        /// authority/AuthorizationState.dfy(831,3)
        pub fn DelegationProvenancePreservesAuthorityBoundary(state: &Rc<AuthorizationState>, delegation: &Rc<Delegation>) -> bool {
            _default::StateDelegationParents(state, delegation).cardinality() == int!(0) || (&crate::KipioAccountDelegation::_default::DelegationCapabilities(delegation)).iter().all(({
                    let mut state = state.clone();
                    let mut delegation = delegation.clone();
                    Rc::new(move |__forall_var_0: &Rc<Capability>| -> bool{
            let mut capability: Rc<Capability> = __forall_var_0.clone();
            !crate::KipioAccountDelegation::_default::DelegationCapabilities(&delegation).contains(&capability) || (&_default::StateDelegations(&state)).iter().any(({
                    let mut state = state.clone();
                    let mut delegation = delegation.clone();
                    let mut capability = capability.clone();
                    Rc::new(move |__exists_var_0: &Rc<Delegation>| -> bool{
            let mut parent: Rc<Delegation> = __exists_var_0.clone();
            _default::StateDelegations(&state).contains(&parent) && _default::StateDelegationParents(&state, &delegation).contains(&crate::KipioAccountDelegation::_default::DelegationId(&parent)) && crate::KipioAccountDelegation::_default::DelegationCapabilities(&parent).contains(&capability)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(856,3)
        pub fn ValidDelegationProvenanceForDelegation(state: &Rc<AuthorizationState>, delegation: &Rc<Delegation>) -> bool {
            if _default::DelegationProvenanceDefinedInState(state, delegation) {
                _default::DelegationProvenanceParentsBelongToState(state, delegation) && _default::DelegationProvenanceHasNoSelfReference(state, delegation) && _default::DelegationProvenancePreservesAuthorityBoundary(state, delegation)
            } else {
                false
            }
        }
        /// authority/AuthorizationState.dfy(889,3)
        pub fn IdentityIdIsAttributedToSubject(identities: &Set<Rc<Identity>>, identityId: &Sequence<u8>, subject: &Rc<Subject>) -> bool {
            identities.iter().any(({
                    let mut subject = subject.clone();
                    let mut identities = identities.clone();
                    let mut identityId = identityId.clone();
                    Rc::new(move |__exists_var_0: &Rc<Identity>| -> bool{
            let mut identity: Rc<Identity> = __exists_var_0.clone();
            identities.contains(&identity) && crate::KipioAccountIdentity::_default::IdentityId(&identity) == identityId.clone() && crate::KipioAccountIdentity::_default::IdentitySubject(&identity) == subject.clone()
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(902,3)
        pub fn ParentDelegationSupportsChildSource(identities: &Set<Rc<Identity>>, parent: &Rc<Delegation>, child: &Rc<Delegation>) -> bool {
            _default::IdentityIdIsAttributedToSubject(identities, &crate::KipioAccountDelegation::_default::DelegationSourceIdentityId(child), &crate::KipioAccountDelegation::_default::DelegationDelegatee(parent))
        }
        /// authority/AuthorizationState.dfy(916,3)
        pub fn DelegationProvenanceSupportsChildSource(state: &Rc<AuthorizationState>, identities: &Set<Rc<Identity>>, delegation: &Rc<Delegation>) -> bool {
            if _default::StateDelegationParents(state, delegation).cardinality() == int!(0) {
                true
            } else {
                (&_default::StateDelegationParents(state, delegation)).iter().all(({
                        let mut state = state.clone();
                        let mut delegation = delegation.clone();
                        let mut identities = identities.clone();
                        Rc::new(move |__forall_var_0: &Sequence<u8>| -> bool{
            let mut parentId: Sequence<u8> = __forall_var_0.clone();
            !_default::StateDelegationParents(&state, &delegation).contains(&parentId) || (&_default::StateDelegations(&state)).iter().any(({
                    let mut state = state.clone();
                    let mut parentId = parentId.clone();
                    let mut delegation = delegation.clone();
                    let mut identities = identities.clone();
                    Rc::new(move |__exists_var_0: &Rc<Delegation>| -> bool{
            let mut parent: Rc<Delegation> = __exists_var_0.clone();
            _default::StateDelegations(&state).contains(&parent) && crate::KipioAccountDelegation::_default::DelegationId(&parent) == parentId.clone() && _default::ParentDelegationSupportsChildSource(&identities, &parent, &delegation)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                    }).as_ref())
            }
        }
        /// authority/AuthorizationState.dfy(948,3)
        pub fn ValidDelegationProvenanceAgainstIdentityContext(state: &Rc<AuthorizationState>, identities: &Set<Rc<Identity>>, delegation: &Rc<Delegation>) -> bool {
            _default::DelegationProvenanceDefinedInState(state, delegation) && _default::ValidDelegationProvenanceForDelegation(state, delegation) && _default::DelegationProvenanceSupportsChildSource(state, identities, delegation)
        }
        /// authority/AuthorizationState.dfy(976,3)
        pub fn EveryRecognizedDelegationHasProvenance(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateDelegations(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Delegation>| -> bool{
            let mut delegation: Rc<Delegation> = __forall_var_0.clone();
            !_default::StateDelegations(&state).contains(&delegation) || _default::StateDelegationProvenance(&state).keys().contains(&crate::KipioAccountDelegation::_default::DelegationId(&delegation))
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(987,3)
        pub fn EveryDelegationProvenanceBelongsToRecognizedDelegation(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateDelegationProvenance(state).keys()).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Sequence<u8>| -> bool{
            let mut delegationId: Sequence<u8> = __forall_var_0.clone();
            !_default::StateDelegationProvenance(&state).keys().contains(&delegationId) || _default::DelegationIdRecognizedInState(&state, &delegationId)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1005,3)
        pub fn ValidAuthorizationState(state: &Rc<AuthorizationState>) -> bool {
            _default::ValidCapabilitiesInState(state) && _default::ValidCredentialsInState(state) && _default::CredentialIdsAreUnique(state) && _default::ValidCredentialAuthoritiesInState(state) && _default::ValidSessionsInState(state) && _default::SessionCredentialsBelongToState(state) && _default::SessionIdsAreUnique(state) && _default::SessionAuthoritiesRespectCredentialAuthorities(state) && _default::ValidDelegationsInState(state) && _default::DelegationIdsAreUnique(state) && _default::EveryRecognizedDelegationHasProvenance(state) && _default::EveryDelegationProvenanceBelongsToRecognizedDelegation(state) && _default::ValidDelegationProvenanceInState(state) && _default::ValidRestrictionMapInState(state) && _default::ValidPolicyEffectsInState(state) && _default::ValidReplayState(state)
        }
        /// authority/AuthorizationState.dfy(1051,3)
        pub fn ValidCapabilitiesInState(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateCapabilities(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Capability>| -> bool{
            let mut capability: Rc<Capability> = __forall_var_0.clone();
            !_default::StateCapabilities(&state).contains(&capability) || crate::KipioAccountCapability::_default::ValidCapability(&capability)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1065,3)
        pub fn ValidCredentialsInState(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateCredentials(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Credential>| -> bool{
            let mut credential: Rc<Credential> = __forall_var_0.clone();
            !_default::StateCredentials(&state).contains(&credential) || crate::KipioAccountCredential::_default::ValidCredential(&credential)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1075,3)
        pub fn CredentialIdsAreUnique(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateCredentials(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Credential>| -> bool{
            let mut left: Rc<Credential> = __forall_var_0.clone();
            (&_default::StateCredentials(&state)).iter().all(({
                    let mut left = left.clone();
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_1: &Rc<Credential>| -> bool{
            let mut right: Rc<Credential> = __forall_var_1.clone();
            !(_default::StateCredentials(&state).contains(&left) && _default::StateCredentials(&state).contains(&right) && left.clone() != right.clone()) || crate::KipioAccountCredential::_default::CredentialId(&left) != crate::KipioAccountCredential::_default::CredentialId(&right)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1091,3)
        pub fn EveryRecognizedCredentialHasAuthority(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateCredentials(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Credential>| -> bool{
            let mut credential: Rc<Credential> = __forall_var_0.clone();
            !_default::StateCredentials(&state).contains(&credential) || _default::StateCredentialAuthorities(&state).keys().contains(&crate::KipioAccountCredential::_default::CredentialId(&credential))
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1102,3)
        pub fn EveryAuthorityBelongsToRecognizedCredential(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateCredentialAuthorities(state).keys()).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Sequence<u8>| -> bool{
            let mut credentialId: Sequence<u8> = __forall_var_0.clone();
            !_default::StateCredentialAuthorities(&state).keys().contains(&credentialId) || _default::CredentialIdRecognizedInState(&state, &credentialId)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1117,3)
        pub fn CredentialAuthoritiesContainValidAuthorities(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateCredentialAuthorities(state).keys()).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Sequence<u8>| -> bool{
            let mut credentialId: Sequence<u8> = __forall_var_0.clone();
            !_default::StateCredentialAuthorities(&state).keys().contains(&credentialId) || crate::KipioAccountCredentialAuthority::_default::ValidCredentialAuthority(&_default::StateCredentialAuthorities(&state).get(&credentialId))
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1131,3)
        pub fn ValidCredentialAuthoritiesInState(state: &Rc<AuthorizationState>) -> bool {
            _default::EveryRecognizedCredentialHasAuthority(state) && _default::EveryAuthorityBelongsToRecognizedCredential(state) && _default::CredentialAuthoritiesContainValidAuthorities(state)
        }
        /// authority/AuthorizationState.dfy(1147,3)
        pub fn ActiveCredentialRecognizedInState(state: &Rc<AuthorizationState>, credentialId: &Sequence<u8>) -> bool {
            (&_default::StateCredentials(state)).iter().any(({
                    let mut credentialId = credentialId.clone();
                    let mut state = state.clone();
                    Rc::new(move |__exists_var_0: &Rc<Credential>| -> bool{
            let mut credential: Rc<Credential> = __exists_var_0.clone();
            _default::StateCredentials(&state).contains(&credential) && crate::KipioAccountCredential::_default::CredentialId(&credential) == credentialId.clone() && crate::KipioAccountCredential::_default::CredentialIsActive(&credential)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1159,3)
        pub fn RecognizedCredentialIsInactive(state: &Rc<AuthorizationState>, credentialId: &Sequence<u8>) -> bool {
            (&_default::StateCredentials(state)).iter().any(({
                    let mut credentialId = credentialId.clone();
                    let mut state = state.clone();
                    Rc::new(move |__exists_var_0: &Rc<Credential>| -> bool{
            let mut credential: Rc<Credential> = __exists_var_0.clone();
            _default::StateCredentials(&state).contains(&credential) && crate::KipioAccountCredential::_default::CredentialId(&credential) == credentialId.clone() && crate::KipioAccountCredential::_default::CredentialCannotCurrentlyExerciseAuthority(&credential)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1175,3)
        pub fn ValidSessionsInState(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateSessions(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Session>| -> bool{
            let mut session: Rc<Session> = __forall_var_0.clone();
            !_default::StateSessions(&state).contains(&session) || crate::KipioAccountSession::_default::ValidSession(&session)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1185,3)
        pub fn SessionCredentialsBelongToState(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateSessions(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Session>| -> bool{
            let mut session: Rc<Session> = __forall_var_0.clone();
            !_default::StateSessions(&state).contains(&session) || _default::CredentialIdRecognizedInState(&state, &crate::KipioAccountSession::_default::SessionCredentialId(&session))
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1199,3)
        pub fn SessionCredentialAuthorityIsDefinedInState(state: &Rc<AuthorizationState>, session: &Rc<Session>) -> bool {
            _default::StateSessions(state).contains(session) && _default::CredentialAuthorityDefinedForCredentialId(state, &crate::KipioAccountSession::_default::SessionCredentialId(session))
        }
        /// authority/AuthorizationState.dfy(1213,3)
        pub fn SessionAuthoritiesRespectCredentialAuthorities(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateSessions(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Session>| -> bool{
            let mut session: Rc<Session> = __forall_var_0.clone();
            !_default::StateSessions(&state).contains(&session) || (if _default::SessionCredentialAuthorityIsDefinedInState(&state, &session) {
                    crate::KipioAccountSession::_default::SessionAuthorityWithinCredentialAuthority(&session, &_default::StateCredentialAuthorities(&state).get(&crate::KipioAccountSession::_default::SessionCredentialId(&session)))
                } else {
                    false
                })
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1236,3)
        pub fn SessionCredentialIsRecognizedInState(state: &Rc<AuthorizationState>, session: &Rc<Session>) -> bool {
            _default::CredentialIdRecognizedInState(state, &crate::KipioAccountSession::_default::SessionCredentialId(session))
        }
        /// authority/AuthorizationState.dfy(1248,3)
        pub fn SessionCanContributeAuthorityAt(state: &Rc<AuthorizationState>, session: &Rc<Session>, now: &DafnyInt) -> bool {
            _default::StateSessions(state).contains(session) && crate::KipioAccountSession::_default::SessionCanCurrentlyExerciseAuthorityAt(session, now) && _default::ActiveCredentialRecognizedInState(state, &crate::KipioAccountSession::_default::SessionCredentialId(session)) && (if _default::SessionCredentialAuthorityIsDefinedInState(state, session) {
                    crate::KipioAccountSession::_default::SessionAuthorityWithinCredentialAuthority(session, &_default::StateCredentialAuthorities(state).get(&crate::KipioAccountSession::_default::SessionCredentialId(session)))
                } else {
                    false
                })
        }
        /// authority/AuthorizationState.dfy(1282,3)
        pub fn SessionIdsAreUnique(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateSessions(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Session>| -> bool{
            let mut left: Rc<Session> = __forall_var_0.clone();
            (&_default::StateSessions(&state)).iter().all(({
                    let mut left = left.clone();
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_1: &Rc<Session>| -> bool{
            let mut right: Rc<Session> = __forall_var_1.clone();
            !(_default::StateSessions(&state).contains(&left) && _default::StateSessions(&state).contains(&right) && left.clone() != right.clone()) || crate::KipioAccountSession::_default::SessionId(&left) != crate::KipioAccountSession::_default::SessionId(&right)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1298,3)
        pub fn ValidDelegationsInState(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateDelegations(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Delegation>| -> bool{
            let mut delegation: Rc<Delegation> = __forall_var_0.clone();
            !_default::StateDelegations(&state).contains(&delegation) || crate::KipioAccountDelegation::_default::ValidDelegation(&delegation)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1308,3)
        pub fn DelegationIdsAreUnique(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateDelegations(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Delegation>| -> bool{
            let mut left: Rc<Delegation> = __forall_var_0.clone();
            (&_default::StateDelegations(&state)).iter().all(({
                    let mut left = left.clone();
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_1: &Rc<Delegation>| -> bool{
            let mut right: Rc<Delegation> = __forall_var_1.clone();
            !(_default::StateDelegations(&state).contains(&left) && _default::StateDelegations(&state).contains(&right) && left.clone() != right.clone()) || crate::KipioAccountDelegation::_default::DelegationId(&left) != crate::KipioAccountDelegation::_default::DelegationId(&right)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1320,3)
        pub fn ValidDelegationProvenanceInState(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateDelegations(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<Delegation>| -> bool{
            let mut delegation: Rc<Delegation> = __forall_var_0.clone();
            !_default::StateDelegations(&state).contains(&delegation) || _default::ValidDelegationProvenanceForDelegation(&state, &delegation)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1334,3)
        pub fn DelegationCanContributeAuthorityAt(state: &Rc<AuthorizationState>, delegation: &Rc<Delegation>, now: &DafnyInt) -> bool {
            _default::StateDelegations(state).contains(delegation) && crate::KipioAccountDelegation::_default::DelegationCanCurrentlyExerciseAuthorityAt(delegation, now)
        }
        /// authority/AuthorizationState.dfy(1353,3)
        pub fn ValidPolicyEffectsInState(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StatePolicyEffects(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Rc<PolicyEffect>| -> bool{
            let mut effect: Rc<PolicyEffect> = __forall_var_0.clone();
            !_default::StatePolicyEffects(&state).contains(&effect) || crate::KipioAccountPolicyEffect::_default::ValidPolicyEffect(&effect)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/AuthorizationState.dfy(1363,3)
        pub fn PolicyEffectRecognizedInState(state: &Rc<AuthorizationState>, effect: &Rc<PolicyEffect>) -> bool {
            _default::StatePolicyEffects(state).contains(effect)
        }
        /// authority/AuthorizationState.dfy(1376,3)
        pub fn ValidReplayState(state: &Rc<AuthorizationState>) -> bool {
            (&_default::StateConsumedReplayKeys(state)).iter().all(({
                    let mut state = state.clone();
                    Rc::new(move |__forall_var_0: &Sequence<u8>| -> bool{
            let mut replayKey: Sequence<u8> = __forall_var_0.clone();
            !_default::StateConsumedReplayKeys(&state).contains(&replayKey) || crate::KipioAccountDomainPrimitives::_default::ValidId(&replayKey)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
    }

    /// authority/AuthorizationState.dfy(186,3)
    #[derive(Clone)]
    pub enum RestrictionTarget {
        RestrictionTarget {
            capability: Rc<Capability>,
            scope: Rc<OptionalPolicyScope>
        }
    }

    impl RestrictionTarget {
        /// Returns a borrow of the field capability
        pub fn capability(&self) -> &Rc<Capability> {
            match self {
                RestrictionTarget::RestrictionTarget{capability, scope, } => capability,
            }
        }
        /// Returns a borrow of the field scope
        pub fn scope(&self) -> &Rc<OptionalPolicyScope> {
            match self {
                RestrictionTarget::RestrictionTarget{capability, scope, } => scope,
            }
        }
    }

    impl Debug
        for RestrictionTarget {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for RestrictionTarget {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                RestrictionTarget::RestrictionTarget{capability, scope, } => {
                    write!(_formatter, "KipioAccountAuthorizationState.RestrictionTarget.RestrictionTarget(")?;
                    DafnyPrint::fmt_print(capability, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(scope, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for RestrictionTarget {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (RestrictionTarget::RestrictionTarget{capability, scope, }, RestrictionTarget::RestrictionTarget{capability: _2_capability, scope: _2_scope, }) => {
                    capability == _2_capability && scope == _2_scope
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for RestrictionTarget {}

    impl Hash
        for RestrictionTarget {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                RestrictionTarget::RestrictionTarget{capability, scope, } => {
                    Hash::hash(capability, _state);
                    Hash::hash(scope, _state)
                },
            }
        }
    }

    impl AsRef<RestrictionTarget>
        for RestrictionTarget {
        fn as_ref(&self) -> &Self {
            self
        }
    }

    /// authority/AuthorizationState.dfy(267,3)
    #[derive(Clone)]
    pub enum AuthorizationState {
        AuthorizationState {
            capabilities: Set<Rc<Capability>>,
            credentials: Set<Rc<Credential>>,
            credentialAuthorities: Map<Sequence<u8>, Set<Rc<Capability>>>,
            sessions: Set<Rc<Session>>,
            delegations: Set<Rc<Delegation>>,
            delegationProvenance: Map<Sequence<u8>, Set<Sequence<u8>>>,
            restrictionMap: Map<Rc<RestrictionTarget>, Rc<Restriction>>,
            policyEffects: Set<Rc<PolicyEffect>>,
            consumedReplayKeys: Set<Sequence<u8>>
        }
    }

    impl AuthorizationState {
        /// Returns a borrow of the field capabilities
        pub fn capabilities(&self) -> &Set<Rc<Capability>> {
            match self {
                AuthorizationState::AuthorizationState{capabilities, credentials, credentialAuthorities, sessions, delegations, delegationProvenance, restrictionMap, policyEffects, consumedReplayKeys, } => capabilities,
            }
        }
        /// Returns a borrow of the field credentials
        pub fn credentials(&self) -> &Set<Rc<Credential>> {
            match self {
                AuthorizationState::AuthorizationState{capabilities, credentials, credentialAuthorities, sessions, delegations, delegationProvenance, restrictionMap, policyEffects, consumedReplayKeys, } => credentials,
            }
        }
        /// Returns a borrow of the field credentialAuthorities
        pub fn credentialAuthorities(&self) -> &Map<Sequence<u8>, Set<Rc<Capability>>> {
            match self {
                AuthorizationState::AuthorizationState{capabilities, credentials, credentialAuthorities, sessions, delegations, delegationProvenance, restrictionMap, policyEffects, consumedReplayKeys, } => credentialAuthorities,
            }
        }
        /// Returns a borrow of the field sessions
        pub fn sessions(&self) -> &Set<Rc<Session>> {
            match self {
                AuthorizationState::AuthorizationState{capabilities, credentials, credentialAuthorities, sessions, delegations, delegationProvenance, restrictionMap, policyEffects, consumedReplayKeys, } => sessions,
            }
        }
        /// Returns a borrow of the field delegations
        pub fn delegations(&self) -> &Set<Rc<Delegation>> {
            match self {
                AuthorizationState::AuthorizationState{capabilities, credentials, credentialAuthorities, sessions, delegations, delegationProvenance, restrictionMap, policyEffects, consumedReplayKeys, } => delegations,
            }
        }
        /// Returns a borrow of the field delegationProvenance
        pub fn delegationProvenance(&self) -> &Map<Sequence<u8>, Set<Sequence<u8>>> {
            match self {
                AuthorizationState::AuthorizationState{capabilities, credentials, credentialAuthorities, sessions, delegations, delegationProvenance, restrictionMap, policyEffects, consumedReplayKeys, } => delegationProvenance,
            }
        }
        /// Returns a borrow of the field restrictionMap
        pub fn restrictionMap(&self) -> &Map<Rc<RestrictionTarget>, Rc<Restriction>> {
            match self {
                AuthorizationState::AuthorizationState{capabilities, credentials, credentialAuthorities, sessions, delegations, delegationProvenance, restrictionMap, policyEffects, consumedReplayKeys, } => restrictionMap,
            }
        }
        /// Returns a borrow of the field policyEffects
        pub fn policyEffects(&self) -> &Set<Rc<PolicyEffect>> {
            match self {
                AuthorizationState::AuthorizationState{capabilities, credentials, credentialAuthorities, sessions, delegations, delegationProvenance, restrictionMap, policyEffects, consumedReplayKeys, } => policyEffects,
            }
        }
        /// Returns a borrow of the field consumedReplayKeys
        pub fn consumedReplayKeys(&self) -> &Set<Sequence<u8>> {
            match self {
                AuthorizationState::AuthorizationState{capabilities, credentials, credentialAuthorities, sessions, delegations, delegationProvenance, restrictionMap, policyEffects, consumedReplayKeys, } => consumedReplayKeys,
            }
        }
    }

    impl Debug
        for AuthorizationState {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for AuthorizationState {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                AuthorizationState::AuthorizationState{capabilities, credentials, credentialAuthorities, sessions, delegations, delegationProvenance, restrictionMap, policyEffects, consumedReplayKeys, } => {
                    write!(_formatter, "KipioAccountAuthorizationState.AuthorizationState.AuthorizationState(")?;
                    DafnyPrint::fmt_print(capabilities, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(credentials, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(credentialAuthorities, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(sessions, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(delegations, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(delegationProvenance, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(restrictionMap, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(policyEffects, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(consumedReplayKeys, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for AuthorizationState {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (AuthorizationState::AuthorizationState{capabilities, credentials, credentialAuthorities, sessions, delegations, delegationProvenance, restrictionMap, policyEffects, consumedReplayKeys, }, AuthorizationState::AuthorizationState{capabilities: _2_capabilities, credentials: _2_credentials, credentialAuthorities: _2_credentialAuthorities, sessions: _2_sessions, delegations: _2_delegations, delegationProvenance: _2_delegationProvenance, restrictionMap: _2_restrictionMap, policyEffects: _2_policyEffects, consumedReplayKeys: _2_consumedReplayKeys, }) => {
                    capabilities == _2_capabilities && credentials == _2_credentials && credentialAuthorities == _2_credentialAuthorities && sessions == _2_sessions && delegations == _2_delegations && delegationProvenance == _2_delegationProvenance && restrictionMap == _2_restrictionMap && policyEffects == _2_policyEffects && consumedReplayKeys == _2_consumedReplayKeys
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for AuthorizationState {}

    impl Hash
        for AuthorizationState {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                AuthorizationState::AuthorizationState{capabilities, credentials, credentialAuthorities, sessions, delegations, delegationProvenance, restrictionMap, policyEffects, consumedReplayKeys, } => {
                    Hash::hash(capabilities, _state);
                    Hash::hash(credentials, _state);
                    Hash::hash(credentialAuthorities, _state);
                    Hash::hash(sessions, _state);
                    Hash::hash(delegations, _state);
                    Hash::hash(delegationProvenance, _state);
                    Hash::hash(restrictionMap, _state);
                    Hash::hash(policyEffects, _state);
                    Hash::hash(consumedReplayKeys, _state)
                },
            }
        }
    }

    impl AsRef<AuthorizationState>
        for AuthorizationState {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// policy/AuthorizationStateTransition.dfy(115,1)
pub mod KipioAccountAuthorizationStateTransition {
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountAuthorizationState::AuthorizationState;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::RevokeCredential;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::RevokeSession;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::RevokeDelegation;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::DisableCapability;
    pub use crate::KipioAccountCapability::Capability;
    pub use crate::KipioAccountPolicyEffect::OptionalPolicyScope;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::EnableCapability;
    pub use crate::KipioAccountRestriction::Restriction;
    pub use crate::KipioAccountCredential::Credential;
    pub use crate::KipioAccountSession::Session;
    pub use crate::KipioAccountDelegation::Delegation;
    pub use crate::KipioAccountPolicy::Policy;
    pub use ::dafny_runtime::integer_range;
    pub use ::dafny_runtime::int;
    pub use ::dafny_runtime::DafnyInt;

    pub struct _default {}

    impl _default {
        /// policy/AuthorizationStateTransition.dfy(139,3)
        pub fn PolicyEffectIsApplicable(state: &Rc<AuthorizationState>, effect: &Rc<PolicyEffect>) -> bool {
            let mut _source0: Rc<PolicyEffect> = effect.clone();
            if matches!((&_source0).as_ref(), RevokeCredential{ .. }) {
                let mut ___mcc_h0: Sequence<u8> = _source0.credentialId().clone();
                let mut credentialId: Sequence<u8> = ___mcc_h0.clone();
                crate::KipioAccountAuthorizationState::_default::CredentialIdRecognizedInState(state, &credentialId)
            } else {
                if matches!((&_source0).as_ref(), RevokeSession{ .. }) {
                    let mut ___mcc_h1: Sequence<u8> = _source0.sessionId().clone();
                    let mut sessionId: Sequence<u8> = ___mcc_h1.clone();
                    crate::KipioAccountAuthorizationState::_default::SessionIdRecognizedInState(state, &sessionId)
                } else {
                    if matches!((&_source0).as_ref(), RevokeDelegation{ .. }) {
                        let mut ___mcc_h2: Sequence<u8> = _source0.delegationId().clone();
                        let mut delegationId: Sequence<u8> = ___mcc_h2.clone();
                        crate::KipioAccountAuthorizationState::_default::DelegationIdRecognizedInState(state, &delegationId)
                    } else {
                        if matches!((&_source0).as_ref(), DisableCapability{ .. }) {
                            let mut ___mcc_h3: Rc<Capability> = _source0.capability().clone();
                            let mut ___mcc_h4: Rc<OptionalPolicyScope> = _source0.scope().clone();
                            let mut scope: Rc<OptionalPolicyScope> = ___mcc_h4.clone();
                            let mut capability: Rc<Capability> = ___mcc_h3.clone();
                            crate::KipioAccountCapability::_default::ValidCapability(&capability) && crate::KipioAccountPolicyEffect::_default::ValidOptionalPolicyScope(&scope) && crate::KipioAccountAuthorizationState::_default::StateCapabilities(state).contains(&capability)
                        } else {
                            if matches!((&_source0).as_ref(), EnableCapability{ .. }) {
                                let mut ___mcc_h5: Rc<Capability> = _source0.capability().clone();
                                let mut ___mcc_h6: Rc<OptionalPolicyScope> = _source0.scope().clone();
                                let mut scope: Rc<OptionalPolicyScope> = ___mcc_h6.clone();
                                let mut capability: Rc<Capability> = ___mcc_h5.clone();
                                crate::KipioAccountCapability::_default::ValidCapability(&capability) && crate::KipioAccountPolicyEffect::_default::ValidOptionalPolicyScope(&scope)
                            } else {
                                let mut ___mcc_h7: Rc<Capability> = _source0.capability().clone();
                                let mut ___mcc_h8: Rc<OptionalPolicyScope> = _source0.scope().clone();
                                let mut ___mcc_h9: Rc<Restriction> = _source0.restriction().clone();
                                let mut restriction: Rc<Restriction> = ___mcc_h9.clone();
                                let mut scope: Rc<OptionalPolicyScope> = ___mcc_h8.clone();
                                let mut capability: Rc<Capability> = ___mcc_h7.clone();
                                crate::KipioAccountCapability::_default::ValidCapability(&capability) && crate::KipioAccountPolicyEffect::_default::ValidOptionalPolicyScope(&scope) && crate::KipioAccountRestriction::_default::ValidRestriction(&restriction) && crate::KipioAccountAuthorizationState::_default::StateCapabilities(state).contains(&capability)
                            }
                        }
                    }
                }
            }
        }
        /// policy/AuthorizationStateTransition.dfy(191,3)
        pub fn PolicyEffectIsConsistent(effect: &Rc<PolicyEffect>) -> bool {
            crate::KipioAccountPolicyEffect::_default::ValidPolicyEffect(effect)
        }
        /// policy/AuthorizationStateTransition.dfy(203,3)
        pub fn RecognizedCredentialMatchesId(state: &Rc<AuthorizationState>, credential: &Rc<Credential>, credentialId: &Sequence<u8>) -> bool {
            crate::KipioAccountAuthorizationState::_default::StateCredentials(state).contains(credential) && crate::KipioAccountCredential::_default::CredentialId(credential) == credentialId.clone()
        }
        /// policy/AuthorizationStateTransition.dfy(215,3)
        pub fn RecognizedSessionMatchesId(state: &Rc<AuthorizationState>, session: &Rc<Session>, sessionId: &Sequence<u8>) -> bool {
            crate::KipioAccountAuthorizationState::_default::StateSessions(state).contains(session) && crate::KipioAccountSession::_default::SessionId(session) == sessionId.clone()
        }
        /// policy/AuthorizationStateTransition.dfy(227,3)
        pub fn RecognizedDelegationMatchesId(state: &Rc<AuthorizationState>, delegation: &Rc<Delegation>, delegationId: &Sequence<u8>) -> bool {
            crate::KipioAccountAuthorizationState::_default::StateDelegations(state).contains(delegation) && crate::KipioAccountDelegation::_default::DelegationId(delegation) == delegationId.clone()
        }
        /// policy/AuthorizationStateTransition.dfy(390,3)
        pub fn IsEnableCapabilityEffect(effect: &Rc<PolicyEffect>, capability: &Rc<Capability>, scope: &Rc<OptionalPolicyScope>) -> bool {
            let mut _source0: Rc<PolicyEffect> = effect.clone();
            if matches!((&_source0).as_ref(), RevokeCredential{ .. }) {
                let mut ___mcc_h0: Sequence<u8> = _source0.credentialId().clone();
                false
            } else {
                if matches!((&_source0).as_ref(), RevokeSession{ .. }) {
                    let mut ___mcc_h2: Sequence<u8> = _source0.sessionId().clone();
                    false
                } else {
                    if matches!((&_source0).as_ref(), RevokeDelegation{ .. }) {
                        let mut ___mcc_h4: Sequence<u8> = _source0.delegationId().clone();
                        false
                    } else {
                        if matches!((&_source0).as_ref(), DisableCapability{ .. }) {
                            let mut ___mcc_h6: Rc<Capability> = _source0.capability().clone();
                            let mut ___mcc_h7: Rc<OptionalPolicyScope> = _source0.scope().clone();
                            false
                        } else {
                            if matches!((&_source0).as_ref(), EnableCapability{ .. }) {
                                let mut ___mcc_h10: Rc<Capability> = _source0.capability().clone();
                                let mut ___mcc_h11: Rc<OptionalPolicyScope> = _source0.scope().clone();
                                let mut candidateScope: Rc<OptionalPolicyScope> = ___mcc_h11.clone();
                                let mut candidateCapability: Rc<Capability> = ___mcc_h10.clone();
                                candidateCapability.clone() == capability.clone() && candidateScope.clone() == scope.clone()
                            } else {
                                let mut ___mcc_h14: Rc<Capability> = _source0.capability().clone();
                                let mut ___mcc_h15: Rc<OptionalPolicyScope> = _source0.scope().clone();
                                let mut ___mcc_h16: Rc<Restriction> = _source0.restriction().clone();
                                false
                            }
                        }
                    }
                }
            }
        }
        /// policy/AuthorizationStateTransition.dfy(411,3)
        pub fn IsDisableCapabilityEffect(effect: &Rc<PolicyEffect>, capability: &Rc<Capability>, scope: &Rc<OptionalPolicyScope>) -> bool {
            let mut _source0: Rc<PolicyEffect> = effect.clone();
            if matches!((&_source0).as_ref(), RevokeCredential{ .. }) {
                let mut ___mcc_h0: Sequence<u8> = _source0.credentialId().clone();
                false
            } else {
                if matches!((&_source0).as_ref(), RevokeSession{ .. }) {
                    let mut ___mcc_h2: Sequence<u8> = _source0.sessionId().clone();
                    false
                } else {
                    if matches!((&_source0).as_ref(), RevokeDelegation{ .. }) {
                        let mut ___mcc_h4: Sequence<u8> = _source0.delegationId().clone();
                        false
                    } else {
                        if matches!((&_source0).as_ref(), DisableCapability{ .. }) {
                            let mut ___mcc_h6: Rc<Capability> = _source0.capability().clone();
                            let mut ___mcc_h7: Rc<OptionalPolicyScope> = _source0.scope().clone();
                            let mut candidateScope: Rc<OptionalPolicyScope> = ___mcc_h7.clone();
                            let mut candidateCapability: Rc<Capability> = ___mcc_h6.clone();
                            candidateCapability.clone() == capability.clone() && candidateScope.clone() == scope.clone()
                        } else {
                            if matches!((&_source0).as_ref(), EnableCapability{ .. }) {
                                let mut ___mcc_h10: Rc<Capability> = _source0.capability().clone();
                                let mut ___mcc_h11: Rc<OptionalPolicyScope> = _source0.scope().clone();
                                false
                            } else {
                                let mut ___mcc_h14: Rc<Capability> = _source0.capability().clone();
                                let mut ___mcc_h15: Rc<OptionalPolicyScope> = _source0.scope().clone();
                                let mut ___mcc_h16: Rc<Restriction> = _source0.restriction().clone();
                                false
                            }
                        }
                    }
                }
            }
        }
        /// policy/AuthorizationStateTransition.dfy(436,3)
        pub fn ContradictoryPolicyEffects(first: &Rc<PolicyEffect>, second: &Rc<PolicyEffect>) -> bool {
            let mut _source0: Rc<PolicyEffect> = first.clone();
            if matches!((&_source0).as_ref(), RevokeCredential{ .. }) {
                let mut ___mcc_h0: Sequence<u8> = _source0.credentialId().clone();
                false
            } else {
                if matches!((&_source0).as_ref(), RevokeSession{ .. }) {
                    let mut ___mcc_h1: Sequence<u8> = _source0.sessionId().clone();
                    false
                } else {
                    if matches!((&_source0).as_ref(), RevokeDelegation{ .. }) {
                        let mut ___mcc_h2: Sequence<u8> = _source0.delegationId().clone();
                        false
                    } else {
                        if matches!((&_source0).as_ref(), DisableCapability{ .. }) {
                            let mut ___mcc_h3: Rc<Capability> = _source0.capability().clone();
                            let mut ___mcc_h4: Rc<OptionalPolicyScope> = _source0.scope().clone();
                            let mut scopeA: Rc<OptionalPolicyScope> = ___mcc_h4.clone();
                            let mut capabilityA: Rc<Capability> = ___mcc_h3.clone();
                            _default::IsEnableCapabilityEffect(second, &capabilityA, &scopeA)
                        } else {
                            if matches!((&_source0).as_ref(), EnableCapability{ .. }) {
                                let mut ___mcc_h5: Rc<Capability> = _source0.capability().clone();
                                let mut ___mcc_h6: Rc<OptionalPolicyScope> = _source0.scope().clone();
                                let mut scopeA: Rc<OptionalPolicyScope> = ___mcc_h6.clone();
                                let mut capabilityA: Rc<Capability> = ___mcc_h5.clone();
                                _default::IsDisableCapabilityEffect(second, &capabilityA, &scopeA)
                            } else {
                                let mut ___mcc_h7: Rc<Capability> = _source0.capability().clone();
                                let mut ___mcc_h8: Rc<OptionalPolicyScope> = _source0.scope().clone();
                                let mut ___mcc_h9: Rc<Restriction> = _source0.restriction().clone();
                                false
                            }
                        }
                    }
                }
            }
        }
        /// policy/AuthorizationStateTransition.dfy(471,3)
        pub fn PolicyContainsContradiction(policy: &Rc<Policy>) -> bool {
            integer_range(int!(0), crate::KipioAccountPolicy::_default::PolicyEffects(policy).cardinality()).any(({
                    let mut policy = policy.clone();
                    Rc::new(move |__exists_var_0: DafnyInt| -> bool{
            let mut i: DafnyInt = __exists_var_0.clone();
            integer_range(i.clone() + int!(1), crate::KipioAccountPolicy::_default::PolicyEffects(&policy).cardinality()).any(({
                    let mut i = i.clone();
                    let mut policy = policy.clone();
                    Rc::new(move |__exists_var_1: DafnyInt| -> bool{
            let mut j: DafnyInt = __exists_var_1.clone();
            int!(0) <= i.clone() && i.clone() < j.clone() && j.clone() < crate::KipioAccountPolicy::_default::PolicyEffects(&policy).cardinality() && _default::ContradictoryPolicyEffects(&crate::KipioAccountPolicy::_default::PolicyEffects(&policy).get(&i), &crate::KipioAccountPolicy::_default::PolicyEffects(&policy).get(&j))
        }) as Rc<dyn ::std::ops::Fn(_) -> _>
                }).as_ref())
        }) as Rc<dyn ::std::ops::Fn(_) -> _>
                }).as_ref())
        }
    }
}
/// authorization/AuthorizationValidation.dfy(507,1)
pub mod KipioAccountAuthorizationValidation {
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountAuthorization::Authorization;
    pub use crate::KipioAccountAccount::Account;
    pub use crate::KipioAccountAuthorizationState::AuthorizationState;
    pub use crate::KipioAccountCapability::Capability;
    pub use crate::KipioAccountRestriction::Restriction;
    pub use ::dafny_runtime::DafnyInt;
    pub use ::dafny_runtime::Set;
    pub use ::dafny_runtime::set;
    pub use ::dafny_runtime::SetBuilder;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountSession::Session;
    pub use crate::KipioAccountDelegation::Delegation;

    pub struct _default {}

    impl _default {
        /// authorization/AuthorizationValidation.dfy(529,3)
        pub fn AuthorizationContextMatchesAccount(authorization: &Rc<Authorization>, account: &Rc<Account>) -> bool {
            crate::KipioAccountAuthorization::_default::AuthorizationAccountId(authorization) == crate::KipioAccountAccount::_default::AccountIdOf(account)
        }
        /// authorization/AuthorizationValidation.dfy(572,3)
        pub fn AuthorizationCredentialIsRecognized(authorization: &Rc<Authorization>, state: &Rc<AuthorizationState>) -> bool {
            crate::KipioAccountAuthorizationState::_default::CredentialIdRecognizedInState(state, &crate::KipioAccountAuthorization::_default::AuthorizationCredentialId(authorization))
        }
        /// authorization/AuthorizationValidation.dfy(584,3)
        pub fn AuthorizationCredentialIsActive(authorization: &Rc<Authorization>, state: &Rc<AuthorizationState>) -> bool {
            crate::KipioAccountAuthorizationState::_default::ActiveCredentialRecognizedInState(state, &crate::KipioAccountAuthorization::_default::AuthorizationCredentialId(authorization))
        }
        /// authorization/AuthorizationValidation.dfy(596,3)
        pub fn AuthorizationCredentialIsUsable(authorization: &Rc<Authorization>, state: &Rc<AuthorizationState>) -> bool {
            _default::AuthorizationCredentialIsRecognized(authorization, state) && _default::AuthorizationCredentialIsActive(authorization, state)
        }
        /// authorization/AuthorizationValidation.dfy(617,3)
        pub fn RequestedAuthorityIsValid(authorization: &Rc<Authorization>) -> bool {
            (&crate::KipioAccountAuthorization::_default::AuthorizationRequestedAuthority(authorization)).iter().all(({
                    let mut authorization = authorization.clone();
                    Rc::new(move |__forall_var_0: &Rc<Capability>| -> bool{
            let mut capability: Rc<Capability> = __forall_var_0.clone();
            !crate::KipioAccountAuthorization::_default::AuthorizationRequestedAuthority(&authorization).contains(&capability) || crate::KipioAccountCapability::_default::ValidCapability(&capability)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authorization/AuthorizationValidation.dfy(631,3)
        pub fn AuthorizationRestrictionsAreValid(authorization: &Rc<Authorization>) -> bool {
            (&crate::KipioAccountAuthorization::_default::AuthorizationRestrictions(authorization)).iter().all(({
                    let mut authorization = authorization.clone();
                    Rc::new(move |__forall_var_0: &Rc<Restriction>| -> bool{
            let mut restriction: Rc<Restriction> = __forall_var_0.clone();
            !crate::KipioAccountAuthorization::_default::AuthorizationRestrictions(&authorization).contains(&restriction) || crate::KipioAccountRestriction::_default::ValidRestriction(&restriction)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authorization/AuthorizationValidation.dfy(645,3)
        pub fn AuthorizationIntervalIsWellFormed(authorization: &Rc<Authorization>) -> bool {
            crate::KipioAccountAuthorization::_default::AuthorizationValidFrom(authorization) <= crate::KipioAccountAuthorization::_default::AuthorizationValidUntil(authorization)
        }
        /// authorization/AuthorizationValidation.dfy(654,3)
        pub fn AuthorizationIsTemporallyValidAt(authorization: &Rc<Authorization>, now: &DafnyInt) -> bool {
            crate::KipioAccountAuthorization::_default::AuthorizationValidFrom(authorization) <= now.clone() && now.clone() <= crate::KipioAccountAuthorization::_default::AuthorizationValidUntil(authorization)
        }
        /// authorization/AuthorizationValidation.dfy(671,3)
        pub fn AuthorizationReplayIsFresh(authorization: &Rc<Authorization>, state: &Rc<AuthorizationState>) -> bool {
            crate::KipioAccountReplay::_default::AuthorizationReplayKeyIsFresh(authorization, &crate::KipioAccountAuthorizationState::_default::StateConsumedReplayKeys(state))
        }
        /// authorization/AuthorizationValidation.dfy(687,3)
        pub fn RecognizedAuthoritySources(state: &Rc<AuthorizationState>) -> Set<Set<Rc<Capability>>> {
            set!{crate::KipioAccountAuthorizationState::_default::StateCapabilities(state)}.merge(&(&({
                    let mut state = state.clone();
                    Rc::new(move || -> Set<Set<Rc<Capability>>> {
            let mut _coll0: SetBuilder<Set<Rc<Capability>>> = SetBuilder::<Set<Rc<Capability>>>::new();
            for __compr_0 in (&crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state).keys()).iter().cloned() {
                let mut credentialId: Sequence<u8> = __compr_0.clone();
                if crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state).keys().contains(&credentialId) {
                    _coll0.add(&crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state).get(&credentialId))
                }
            }
            _coll0.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Set<Rc<Capability>>>>
                }))()).merge(&(&({
                    let mut state = state.clone();
                    Rc::new(move || -> Set<Set<Rc<Capability>>> {
            let mut _coll1: SetBuilder<Set<Rc<Capability>>> = SetBuilder::<Set<Rc<Capability>>>::new();
            for __compr_1 in (&crate::KipioAccountAuthorizationState::_default::StateSessions(&state)).iter().cloned() {
                let mut session: Rc<Session> = __compr_1.clone();
                if crate::KipioAccountAuthorizationState::_default::StateSessions(&state).contains(&session) {
                    _coll1.add(&crate::KipioAccountSession::_default::SessionCapabilities(&session))
                }
            }
            _coll1.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Set<Rc<Capability>>>>
                }))()).merge(&(&({
                    let mut state = state.clone();
                    Rc::new(move || -> Set<Set<Rc<Capability>>> {
            let mut _coll2: SetBuilder<Set<Rc<Capability>>> = SetBuilder::<Set<Rc<Capability>>>::new();
            for __compr_2 in (&crate::KipioAccountAuthorizationState::_default::StateDelegations(&state)).iter().cloned() {
                let mut delegation: Rc<Delegation> = __compr_2.clone();
                if crate::KipioAccountAuthorizationState::_default::StateDelegations(&state).contains(&delegation) {
                    _coll2.add(&crate::KipioAccountDelegation::_default::DelegationCapabilities(&delegation))
                }
            }
            _coll2.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Set<Rc<Capability>>>>
                }))())
        }
        /// authorization/AuthorizationValidation.dfy(714,3)
        pub fn CurrentlyUsableAuthoritySources(state: &Rc<AuthorizationState>, now: &DafnyInt) -> Set<Set<Rc<Capability>>> {
            set!{crate::KipioAccountAuthorizationState::_default::StateCapabilities(state)}.merge(&(&({
                    let mut state = state.clone();
                    Rc::new(move || -> Set<Set<Rc<Capability>>> {
            let mut _coll0: SetBuilder<Set<Rc<Capability>>> = SetBuilder::<Set<Rc<Capability>>>::new();
            for __compr_0 in (&crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state).keys()).iter().cloned() {
                let mut credentialId: Sequence<u8> = __compr_0.clone();
                if crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state).keys().contains(&credentialId) && crate::KipioAccountAuthorizationState::_default::ActiveCredentialRecognizedInState(&state, &credentialId) {
                    _coll0.add(&crate::KipioAccountAuthorizationState::_default::StateCredentialAuthorities(&state).get(&credentialId))
                }
            }
            _coll0.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Set<Rc<Capability>>>>
                }))()).merge(&(&({
                    let mut now = now.clone();
                    let mut state = state.clone();
                    Rc::new(move || -> Set<Set<Rc<Capability>>> {
            let mut _coll1: SetBuilder<Set<Rc<Capability>>> = SetBuilder::<Set<Rc<Capability>>>::new();
            for __compr_1 in (&crate::KipioAccountAuthorizationState::_default::StateSessions(&state)).iter().cloned() {
                let mut session: Rc<Session> = __compr_1.clone();
                if crate::KipioAccountAuthorizationState::_default::StateSessions(&state).contains(&session) && crate::KipioAccountAuthorizationState::_default::SessionCanContributeAuthorityAt(&state, &session, &now) {
                    _coll1.add(&crate::KipioAccountSession::_default::SessionCapabilities(&session))
                }
            }
            _coll1.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Set<Rc<Capability>>>>
                }))()).merge(&(&({
                    let mut now = now.clone();
                    let mut state = state.clone();
                    Rc::new(move || -> Set<Set<Rc<Capability>>> {
            let mut _coll2: SetBuilder<Set<Rc<Capability>>> = SetBuilder::<Set<Rc<Capability>>>::new();
            for __compr_2 in (&crate::KipioAccountAuthorizationState::_default::StateDelegations(&state)).iter().cloned() {
                let mut delegation: Rc<Delegation> = __compr_2.clone();
                if crate::KipioAccountAuthorizationState::_default::StateDelegations(&state).contains(&delegation) && crate::KipioAccountAuthorizationState::_default::DelegationCanContributeAuthorityAt(&state, &delegation, &now) {
                    _coll2.add(&crate::KipioAccountDelegation::_default::DelegationCapabilities(&delegation))
                }
            }
            _coll2.build()
        }) as Rc<dyn ::std::ops::Fn() -> Set<Set<Rc<Capability>>>>
                }))())
        }
        /// authorization/AuthorizationValidation.dfy(1536,3)
        pub fn AuthorizationAuthorityIsEffective(authorization: &Rc<Authorization>, effectiveAuthority: &Set<Rc<Capability>>) -> bool {
            crate::KipioAccountAuthorization::_default::RequestedAuthorityWithinEffectiveAuthority(authorization, effectiveAuthority)
        }
        /// authorization/AuthorizationValidation.dfy(1548,3)
        pub fn EffectiveAuthorityIsValid(effectiveAuthority: &Set<Rc<Capability>>) -> bool {
            crate::KipioAccountEffectiveAuthority::_default::ValidEffectiveAuthority(effectiveAuthority)
        }
        /// authorization/AuthorizationValidation.dfy(1612,3)
        pub fn AuthorizationIsStructurallyValid(authorization: &Rc<Authorization>) -> bool {
            crate::KipioAccountAuthorization::_default::ValidAuthorization(authorization) && _default::RequestedAuthorityIsValid(authorization) && _default::AuthorizationRestrictionsAreValid(authorization) && _default::AuthorizationIntervalIsWellFormed(authorization)
        }
        /// authorization/AuthorizationValidation.dfy(1630,3)
        pub fn AuthorizationProofIsVerified(proofVerified: bool) -> bool {
            proofVerified
        }
    }
}
/// foundation/Capability.dfy(63,1)
pub mod KipioAccountCapability {
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::int;
    pub use ::dafny_runtime::Sequence;
    pub use ::dafny_runtime::DafnyChar;
    pub use ::dafny_runtime::string_of;
    pub use crate::KipioAccountScope::Scope;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// foundation/Capability.dfy(109,3)
        pub fn ValidCapabilityKind(kind: &Rc<CapabilityKind>) -> bool {
            int!(0) < kind.name().cardinality()
        }
        /// foundation/Capability.dfy(118,3)
        pub fn CapabilityKindName(kind: &Rc<CapabilityKind>) -> Sequence<DafnyChar> {
            kind.name().clone()
        }
        /// foundation/Capability.dfy(148,3)
        pub fn DelegateCapabilityKind() -> Rc<CapabilityKind> {
            Rc::new(CapabilityKind::CapabilityKind {
                    name: string_of("Delegate")
                })
        }
        /// foundation/Capability.dfy(156,3)
        pub fn IsDelegateKind(kind: &Rc<CapabilityKind>) -> bool {
            kind.clone() == _default::DelegateCapabilityKind()
        }
        /// foundation/Capability.dfy(173,3)
        pub fn DelegateCapability(scope: &Rc<Scope>) -> Rc<Capability> {
            Rc::new(Capability::Capability {
                    kind: _default::DelegateCapabilityKind(),
                    scope: scope.clone()
                })
        }
        /// foundation/Capability.dfy(192,3)
        pub fn ValidDelegateCapability(capability: &Rc<Capability>) -> bool {
            _default::IsDelegateCapability(capability) && crate::KipioAccountScope::_default::ValidScope(&_default::CapabilityScope(capability))
        }
        /// foundation/Capability.dfy(257,3)
        pub fn CapabilityKindOf(capability: &Rc<Capability>) -> Rc<CapabilityKind> {
            capability.kind().clone()
        }
        /// foundation/Capability.dfy(267,3)
        pub fn CapabilityScope(capability: &Rc<Capability>) -> Rc<Scope> {
            capability.scope().clone()
        }
        /// foundation/Capability.dfy(289,3)
        pub fn IsDelegateCapability(capability: &Rc<Capability>) -> bool {
            _default::IsDelegateKind(&_default::CapabilityKindOf(capability))
        }
        /// foundation/Capability.dfy(313,3)
        pub fn SameCapability(left: &Rc<Capability>, right: &Rc<Capability>) -> bool {
            _default::CapabilityKindOf(left) == _default::CapabilityKindOf(right) && _default::CapabilityScope(left) == _default::CapabilityScope(right)
        }
        /// foundation/Capability.dfy(459,3)
        pub fn ValidCapability(capability: &Rc<Capability>) -> bool {
            _default::ValidCapabilityKind(&_default::CapabilityKindOf(capability)) && crate::KipioAccountScope::_default::ValidScope(&_default::CapabilityScope(capability))
        }
        /// foundation/Capability.dfy(517,3)
        pub fn SameAuthorityMeaning(left: &Rc<Capability>, right: &Rc<Capability>) -> bool {
            _default::SameCapability(left, right)
        }
    }

    /// foundation/Capability.dfy(96,3)
    #[derive(Clone)]
    pub enum CapabilityKind {
        CapabilityKind {
            name: Sequence<DafnyChar>
        }
    }

    impl CapabilityKind {
        /// Returns a borrow of the field name
        pub fn name(&self) -> &Sequence<DafnyChar> {
            match self {
                CapabilityKind::CapabilityKind{name, } => name,
            }
        }
    }

    impl Debug
        for CapabilityKind {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for CapabilityKind {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                CapabilityKind::CapabilityKind{name, } => {
                    write!(_formatter, "KipioAccountCapability.CapabilityKind.CapabilityKind(")?;
                    DafnyPrint::fmt_print(name, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for CapabilityKind {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (CapabilityKind::CapabilityKind{name, }, CapabilityKind::CapabilityKind{name: _2_name, }) => {
                    name == _2_name
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for CapabilityKind {}

    impl Hash
        for CapabilityKind {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                CapabilityKind::CapabilityKind{name, } => {
                    Hash::hash(name, _state)
                },
            }
        }
    }

    impl AsRef<CapabilityKind>
        for CapabilityKind {
        fn as_ref(&self) -> &Self {
            self
        }
    }

    /// foundation/Capability.dfy(245,3)
    #[derive(Clone)]
    pub enum Capability {
        Capability {
            kind: Rc<CapabilityKind>,
            scope: Rc<Scope>
        }
    }

    impl Capability {
        /// Returns a borrow of the field kind
        pub fn kind(&self) -> &Rc<CapabilityKind> {
            match self {
                Capability::Capability{kind, scope, } => kind,
            }
        }
        /// Returns a borrow of the field scope
        pub fn scope(&self) -> &Rc<Scope> {
            match self {
                Capability::Capability{kind, scope, } => scope,
            }
        }
    }

    impl Debug
        for Capability {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for Capability {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                Capability::Capability{kind, scope, } => {
                    write!(_formatter, "KipioAccountCapability.Capability.Capability(")?;
                    DafnyPrint::fmt_print(kind, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(scope, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for Capability {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (Capability::Capability{kind, scope, }, Capability::Capability{kind: _2_kind, scope: _2_scope, }) => {
                    kind == _2_kind && scope == _2_scope
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for Capability {}

    impl Hash
        for Capability {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                Capability::Capability{kind, scope, } => {
                    Hash::hash(kind, _state);
                    Hash::hash(scope, _state)
                },
            }
        }
    }

    impl AsRef<Capability>
        for Capability {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// foundation/Chain.dfy(103,1)
pub mod KipioAccountChain {
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Chain;
    impl ::dafny_runtime::DafnyPrint for Chain {
        fn fmt_print(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
            _in_seq: bool,
        ) -> ::std::fmt::Result {
            write!(f, "Chain")
        }
    }

    
}
/// authority/Credential.dfy(44,1)
pub mod KipioAccountCredential {
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::Sequence;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::dafny_runtime::SequenceIter;
    pub use ::dafny_runtime::seq;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// authority/Credential.dfy(102,3)
        pub fn CredentialId(credential: &Rc<Credential>) -> Sequence<u8> {
            credential.id().clone()
        }
        /// authority/Credential.dfy(111,3)
        pub fn CredentialStatusOf(credential: &Rc<Credential>) -> Rc<CredentialStatus> {
            credential.status().clone()
        }
        /// authority/Credential.dfy(125,3)
        pub fn CredentialIsActive(credential: &Rc<Credential>) -> bool {
            _default::CredentialStatusOf(credential) == Rc::new(CredentialStatus::Active {})
        }
        /// authority/Credential.dfy(135,3)
        pub fn CredentialIsSuspended(credential: &Rc<Credential>) -> bool {
            _default::CredentialStatusOf(credential) == Rc::new(CredentialStatus::Suspended {})
        }
        /// authority/Credential.dfy(145,3)
        pub fn CredentialIsRevoked(credential: &Rc<Credential>) -> bool {
            _default::CredentialStatusOf(credential) == Rc::new(CredentialStatus::Revoked {})
        }
        /// authority/Credential.dfy(158,3)
        pub fn CredentialCannotCurrentlyExerciseAuthority(credential: &Rc<Credential>) -> bool {
            !_default::CredentialIsActive(credential)
        }
        /// authority/Credential.dfy(177,3)
        pub fn ValidCredential(credential: &Rc<Credential>) -> bool {
            crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::CredentialId(credential))
        }
        /// authority/Credential.dfy(194,3)
        pub fn SameCredential(left: &Rc<Credential>, right: &Rc<Credential>) -> bool {
            _default::CredentialId(left) == _default::CredentialId(right)
        }
    }

    /// authority/Credential.dfy(69,3)
    #[derive(Clone)]
    pub enum CredentialStatus {
        Active {},
        Suspended {},
        Revoked {}
    }

    impl CredentialStatus {}

    impl Debug
        for CredentialStatus {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for CredentialStatus {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                CredentialStatus::Active{} => {
                    write!(_formatter, "KipioAccountCredential.CredentialStatus.Active")?;
                    Ok(())
                },
                CredentialStatus::Suspended{} => {
                    write!(_formatter, "KipioAccountCredential.CredentialStatus.Suspended")?;
                    Ok(())
                },
                CredentialStatus::Revoked{} => {
                    write!(_formatter, "KipioAccountCredential.CredentialStatus.Revoked")?;
                    Ok(())
                },
            }
        }
    }

    impl CredentialStatus {
        /// Enumerates all possible values of CredentialStatus
        pub fn _AllSingletonConstructors() -> SequenceIter<Rc<CredentialStatus>> {
            seq![Rc::new(CredentialStatus::Active {}), Rc::new(CredentialStatus::Suspended {}), Rc::new(CredentialStatus::Revoked {})].iter()
        }
    }

    impl PartialEq
        for CredentialStatus {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (CredentialStatus::Active{}, CredentialStatus::Active{}) => {
                    true
                },
                (CredentialStatus::Suspended{}, CredentialStatus::Suspended{}) => {
                    true
                },
                (CredentialStatus::Revoked{}, CredentialStatus::Revoked{}) => {
                    true
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for CredentialStatus {}

    impl Hash
        for CredentialStatus {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                CredentialStatus::Active{} => {
                    
                },
                CredentialStatus::Suspended{} => {
                    
                },
                CredentialStatus::Revoked{} => {
                    
                },
            }
        }
    }

    impl AsRef<CredentialStatus>
        for CredentialStatus {
        fn as_ref(&self) -> &Self {
            self
        }
    }

    /// authority/Credential.dfy(90,3)
    #[derive(Clone)]
    pub enum Credential {
        Credential {
            id: Sequence<u8>,
            status: Rc<CredentialStatus>
        }
    }

    impl Credential {
        /// Returns a borrow of the field id
        pub fn id(&self) -> &Sequence<u8> {
            match self {
                Credential::Credential{id, status, } => id,
            }
        }
        /// Returns a borrow of the field status
        pub fn status(&self) -> &Rc<CredentialStatus> {
            match self {
                Credential::Credential{id, status, } => status,
            }
        }
    }

    impl Debug
        for Credential {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for Credential {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                Credential::Credential{id, status, } => {
                    write!(_formatter, "KipioAccountCredential.Credential.Credential(")?;
                    DafnyPrint::fmt_print(id, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(status, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for Credential {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (Credential::Credential{id, status, }, Credential::Credential{id: _2_id, status: _2_status, }) => {
                    id == _2_id && status == _2_status
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for Credential {}

    impl Hash
        for Credential {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                Credential::Credential{id, status, } => {
                    Hash::hash(id, _state);
                    Hash::hash(status, _state)
                },
            }
        }
    }

    impl AsRef<Credential>
        for Credential {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// authority/CredentialAuthority.dfy(58,1)
pub mod KipioAccountCredentialAuthority {
    pub use ::dafny_runtime::Set;
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountCapability::Capability;

    pub struct _default {}

    impl _default {
        /// authority/CredentialAuthority.dfy(92,3)
        pub fn CredentialCanExercise(authority: &Set<Rc<Capability>>, capability: &Rc<Capability>) -> bool {
            authority.contains(capability)
        }
        /// authority/CredentialAuthority.dfy(110,3)
        pub fn CredentialAuthorityPermitsAttempt(authority: &Set<Rc<Capability>>, capability: &Rc<Capability>) -> bool {
            _default::CredentialCanExercise(authority, capability)
        }
        /// authority/CredentialAuthority.dfy(127,3)
        pub fn SameCredentialAuthority(left: &Set<Rc<Capability>>, right: &Set<Rc<Capability>>) -> bool {
            left.clone() == right.clone()
        }
        /// authority/CredentialAuthority.dfy(192,3)
        pub fn ValidCredentialAuthority(authority: &Set<Rc<Capability>>) -> bool {
            authority.iter().all(({
                    let mut authority = authority.clone();
                    Rc::new(move |__forall_var_0: &Rc<Capability>| -> bool{
            let mut capability: Rc<Capability> = __forall_var_0.clone();
            !authority.contains(&capability) || crate::KipioAccountCapability::_default::ValidCapability(&capability)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
    }
}
/// authority/Delegation.dfy(134,1)
pub mod KipioAccountDelegation {
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountSubject::Subject;
    pub use ::dafny_runtime::Set;
    pub use crate::KipioAccountCapability::Capability;
    pub use crate::KipioAccountRestriction::Restriction;
    pub use ::dafny_runtime::DafnyInt;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::dafny_runtime::SequenceIter;
    pub use ::dafny_runtime::seq;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// authority/Delegation.dfy(173,3)
        pub fn DelegationId(delegation: &Rc<Delegation>) -> Sequence<u8> {
            delegation.id().clone()
        }
        /// authority/Delegation.dfy(181,3)
        pub fn DelegationSourceIdentityId(delegation: &Rc<Delegation>) -> Sequence<u8> {
            delegation.delegatorIdentityId().clone()
        }
        /// authority/Delegation.dfy(189,3)
        pub fn DelegationDelegatee(delegation: &Rc<Delegation>) -> Rc<Subject> {
            delegation.delegatee().clone()
        }
        /// authority/Delegation.dfy(197,3)
        pub fn DelegationCapabilities(delegation: &Rc<Delegation>) -> Set<Rc<Capability>> {
            delegation.capabilities().clone()
        }
        /// authority/Delegation.dfy(205,3)
        pub fn DelegationRestrictions(delegation: &Rc<Delegation>) -> Set<Rc<Restriction>> {
            delegation.restrictions().clone()
        }
        /// authority/Delegation.dfy(213,3)
        pub fn DelegationValidFrom(delegation: &Rc<Delegation>) -> DafnyInt {
            delegation.validFrom().clone()
        }
        /// authority/Delegation.dfy(221,3)
        pub fn DelegationValidUntil(delegation: &Rc<Delegation>) -> DafnyInt {
            delegation.validUntil().clone()
        }
        /// authority/Delegation.dfy(229,3)
        pub fn DelegationMetadata(delegation: &Rc<Delegation>) -> Sequence<u8> {
            delegation.metadata().clone()
        }
        /// authority/Delegation.dfy(237,3)
        pub fn DelegationStatusOf(delegation: &Rc<Delegation>) -> Rc<DelegationStatus> {
            delegation.status().clone()
        }
        /// authority/Delegation.dfy(256,3)
        pub fn DelegationHasExactlyOneSource(delegation: &Rc<Delegation>) -> bool {
            crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::DelegationSourceIdentityId(delegation))
        }
        /// authority/Delegation.dfy(277,3)
        pub fn DelegationIsActive(delegation: &Rc<Delegation>) -> bool {
            _default::DelegationStatusOf(delegation) == Rc::new(DelegationStatus::Active {})
        }
        /// authority/Delegation.dfy(285,3)
        pub fn DelegationIsRevoked(delegation: &Rc<Delegation>) -> bool {
            _default::DelegationStatusOf(delegation) == Rc::new(DelegationStatus::Revoked {})
        }
        /// authority/Delegation.dfy(293,3)
        pub fn DelegationIsExpiredAt(delegation: &Rc<Delegation>, now: &DafnyInt) -> bool {
            _default::DelegationValidUntil(delegation) < now.clone()
        }
        /// authority/Delegation.dfy(302,3)
        pub fn DelegationIsWithinValidityIntervalAt(delegation: &Rc<Delegation>, now: &DafnyInt) -> bool {
            _default::DelegationValidFrom(delegation) <= now.clone() && now.clone() <= _default::DelegationValidUntil(delegation)
        }
        /// authority/Delegation.dfy(312,3)
        pub fn DelegationCanCurrentlyExerciseAuthorityAt(delegation: &Rc<Delegation>, now: &DafnyInt) -> bool {
            _default::DelegationIsActive(delegation) && _default::DelegationIsWithinValidityIntervalAt(delegation, now)
        }
        /// authority/Delegation.dfy(329,3)
        pub fn ValidDelegation(delegation: &Rc<Delegation>) -> bool {
            crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::DelegationId(delegation)) && crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::DelegationSourceIdentityId(delegation)) && crate::KipioAccountSubject::_default::ValidSubject(&_default::DelegationDelegatee(delegation)) && _default::DelegationValidFrom(delegation) <= _default::DelegationValidUntil(delegation) && (&_default::DelegationCapabilities(delegation)).iter().all(({
                    let mut delegation = delegation.clone();
                    Rc::new(move |__forall_var_0: &Rc<Capability>| -> bool{
            let mut capability: Rc<Capability> = __forall_var_0.clone();
            !_default::DelegationCapabilities(&delegation).contains(&capability) || crate::KipioAccountCapability::_default::ValidCapability(&capability)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref()) && (&_default::DelegationRestrictions(delegation)).iter().all(({
                    let mut delegation = delegation.clone();
                    Rc::new(move |__forall_var_1: &Rc<Restriction>| -> bool{
            let mut restriction: Rc<Restriction> = __forall_var_1.clone();
            !_default::DelegationRestrictions(&delegation).contains(&restriction) || crate::KipioAccountRestriction::_default::ValidRestriction(&restriction)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/Delegation.dfy(353,3)
        pub fn SameDelegation(left: &Rc<Delegation>, right: &Rc<Delegation>) -> bool {
            _default::DelegationId(left) == _default::DelegationId(right)
        }
        /// authority/Delegation.dfy(494,3)
        pub fn DelegatableAuthorityWithinEffectiveAuthority(delegatableAuthority: &Set<Rc<Capability>>, effectiveAuthority: &Set<Rc<Capability>>) -> bool {
            delegatableAuthority.clone() <= effectiveAuthority.clone()
        }
        /// authority/Delegation.dfy(533,3)
        pub fn SourceHasDelegationCapability(effectiveAuthority: &Set<Rc<Capability>>, delegateCapability: &Rc<Capability>) -> bool {
            crate::KipioAccountCapability::_default::IsDelegateCapability(delegateCapability) && effectiveAuthority.contains(delegateCapability)
        }
        /// authority/Delegation.dfy(568,3)
        pub fn DelegationAuthorityWithinDelegatableAuthority(delegation: &Rc<Delegation>, delegatableAuthority: &Set<Rc<Capability>>) -> bool {
            _default::DelegationCapabilities(delegation) <= delegatableAuthority.clone()
        }
        /// authority/Delegation.dfy(596,3)
        pub fn DelegationMayRestrictAuthority(delegation: &Rc<Delegation>, delegatableAuthority: &Set<Rc<Capability>>) -> bool {
            _default::DelegationAuthorityWithinDelegatableAuthority(delegation, delegatableAuthority)
        }
        /// authority/Delegation.dfy(628,3)
        pub fn DelegationAuthorityIsBounded(delegation: &Rc<Delegation>, delegateCapability: &Rc<Capability>, delegatableAuthority: &Set<Rc<Capability>>, effectiveAuthority: &Set<Rc<Capability>>) -> bool {
            _default::SourceHasDelegationCapability(effectiveAuthority, delegateCapability) && _default::DelegationAuthorityWithinDelegatableAuthority(delegation, delegatableAuthority) && _default::DelegatableAuthorityWithinEffectiveAuthority(delegatableAuthority, effectiveAuthority)
        }
        /// authority/Delegation.dfy(697,3)
        pub fn TransitiveDelegationAuthorityIsBounded(childAuthority: &Set<Rc<Capability>>, parentDelegatedAuthority: &Set<Rc<Capability>>) -> bool {
            childAuthority.clone() <= parentDelegatedAuthority.clone()
        }
    }

    /// authority/Delegation.dfy(146,3)
    #[derive(Clone)]
    pub enum DelegationStatus {
        Active {},
        Revoked {}
    }

    impl DelegationStatus {}

    impl Debug
        for DelegationStatus {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for DelegationStatus {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                DelegationStatus::Active{} => {
                    write!(_formatter, "KipioAccountDelegation.DelegationStatus.Active")?;
                    Ok(())
                },
                DelegationStatus::Revoked{} => {
                    write!(_formatter, "KipioAccountDelegation.DelegationStatus.Revoked")?;
                    Ok(())
                },
            }
        }
    }

    impl DelegationStatus {
        /// Enumerates all possible values of DelegationStatus
        pub fn _AllSingletonConstructors() -> SequenceIter<Rc<DelegationStatus>> {
            seq![Rc::new(DelegationStatus::Active {}), Rc::new(DelegationStatus::Revoked {})].iter()
        }
    }

    impl PartialEq
        for DelegationStatus {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (DelegationStatus::Active{}, DelegationStatus::Active{}) => {
                    true
                },
                (DelegationStatus::Revoked{}, DelegationStatus::Revoked{}) => {
                    true
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for DelegationStatus {}

    impl Hash
        for DelegationStatus {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                DelegationStatus::Active{} => {
                    
                },
                DelegationStatus::Revoked{} => {
                    
                },
            }
        }
    }

    impl AsRef<DelegationStatus>
        for DelegationStatus {
        fn as_ref(&self) -> &Self {
            self
        }
    }

    /// authority/Delegation.dfy(155,3)
    #[derive(Clone)]
    pub enum Delegation {
        Delegation {
            id: Sequence<u8>,
            delegatorIdentityId: Sequence<u8>,
            delegatee: Rc<Subject>,
            capabilities: Set<Rc<Capability>>,
            validFrom: DafnyInt,
            validUntil: DafnyInt,
            restrictions: Set<Rc<Restriction>>,
            metadata: Sequence<u8>,
            status: Rc<DelegationStatus>
        }
    }

    impl Delegation {
        /// Returns a borrow of the field id
        pub fn id(&self) -> &Sequence<u8> {
            match self {
                Delegation::Delegation{id, delegatorIdentityId, delegatee, capabilities, validFrom, validUntil, restrictions, metadata, status, } => id,
            }
        }
        /// Returns a borrow of the field delegatorIdentityId
        pub fn delegatorIdentityId(&self) -> &Sequence<u8> {
            match self {
                Delegation::Delegation{id, delegatorIdentityId, delegatee, capabilities, validFrom, validUntil, restrictions, metadata, status, } => delegatorIdentityId,
            }
        }
        /// Returns a borrow of the field delegatee
        pub fn delegatee(&self) -> &Rc<Subject> {
            match self {
                Delegation::Delegation{id, delegatorIdentityId, delegatee, capabilities, validFrom, validUntil, restrictions, metadata, status, } => delegatee,
            }
        }
        /// Returns a borrow of the field capabilities
        pub fn capabilities(&self) -> &Set<Rc<Capability>> {
            match self {
                Delegation::Delegation{id, delegatorIdentityId, delegatee, capabilities, validFrom, validUntil, restrictions, metadata, status, } => capabilities,
            }
        }
        /// Returns a borrow of the field validFrom
        pub fn validFrom(&self) -> &DafnyInt {
            match self {
                Delegation::Delegation{id, delegatorIdentityId, delegatee, capabilities, validFrom, validUntil, restrictions, metadata, status, } => validFrom,
            }
        }
        /// Returns a borrow of the field validUntil
        pub fn validUntil(&self) -> &DafnyInt {
            match self {
                Delegation::Delegation{id, delegatorIdentityId, delegatee, capabilities, validFrom, validUntil, restrictions, metadata, status, } => validUntil,
            }
        }
        /// Returns a borrow of the field restrictions
        pub fn restrictions(&self) -> &Set<Rc<Restriction>> {
            match self {
                Delegation::Delegation{id, delegatorIdentityId, delegatee, capabilities, validFrom, validUntil, restrictions, metadata, status, } => restrictions,
            }
        }
        /// Returns a borrow of the field metadata
        pub fn metadata(&self) -> &Sequence<u8> {
            match self {
                Delegation::Delegation{id, delegatorIdentityId, delegatee, capabilities, validFrom, validUntil, restrictions, metadata, status, } => metadata,
            }
        }
        /// Returns a borrow of the field status
        pub fn status(&self) -> &Rc<DelegationStatus> {
            match self {
                Delegation::Delegation{id, delegatorIdentityId, delegatee, capabilities, validFrom, validUntil, restrictions, metadata, status, } => status,
            }
        }
    }

    impl Debug
        for Delegation {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for Delegation {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                Delegation::Delegation{id, delegatorIdentityId, delegatee, capabilities, validFrom, validUntil, restrictions, metadata, status, } => {
                    write!(_formatter, "KipioAccountDelegation.Delegation.Delegation(")?;
                    DafnyPrint::fmt_print(id, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(delegatorIdentityId, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(delegatee, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(capabilities, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(validFrom, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(validUntil, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(restrictions, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(metadata, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(status, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for Delegation {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (Delegation::Delegation{id, delegatorIdentityId, delegatee, capabilities, validFrom, validUntil, restrictions, metadata, status, }, Delegation::Delegation{id: _2_id, delegatorIdentityId: _2_delegatorIdentityId, delegatee: _2_delegatee, capabilities: _2_capabilities, validFrom: _2_validFrom, validUntil: _2_validUntil, restrictions: _2_restrictions, metadata: _2_metadata, status: _2_status, }) => {
                    id == _2_id && delegatorIdentityId == _2_delegatorIdentityId && delegatee == _2_delegatee && capabilities == _2_capabilities && validFrom == _2_validFrom && validUntil == _2_validUntil && restrictions == _2_restrictions && metadata == _2_metadata && status == _2_status
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for Delegation {}

    impl Hash
        for Delegation {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                Delegation::Delegation{id, delegatorIdentityId, delegatee, capabilities, validFrom, validUntil, restrictions, metadata, status, } => {
                    Hash::hash(id, _state);
                    Hash::hash(delegatorIdentityId, _state);
                    Hash::hash(delegatee, _state);
                    Hash::hash(capabilities, _state);
                    Hash::hash(validFrom, _state);
                    Hash::hash(validUntil, _state);
                    Hash::hash(restrictions, _state);
                    Hash::hash(metadata, _state);
                    Hash::hash(status, _state)
                },
            }
        }
    }

    impl AsRef<Delegation>
        for Delegation {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// foundation/DomainAction.dfy(115,1)
pub mod KipioAccountDomainAction {
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct DomainAction;
    impl ::dafny_runtime::DafnyPrint for DomainAction {
        fn fmt_print(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
            _in_seq: bool,
        ) -> ::std::fmt::Result {
            write!(f, "DomainAction")
        }
    }

    
}
/// foundation/DomainPrimitives.dfy(49,1)
pub mod KipioAccountDomainPrimitives {
    pub use ::dafny_runtime::Sequence;
    pub use ::dafny_runtime::int;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// foundation/DomainPrimitives.dfy(86,3)
        pub fn ValidId(id: &Sequence<u8>) -> bool {
            int!(0) < id.cardinality()
        }
    }

    /// foundation/DomainPrimitives.dfy(99,3)
    #[derive(Clone)]
    pub enum OptionalId {
        None {},
        Some {
            value: Sequence<u8>
        }
    }

    impl OptionalId {
        /// Gets the field value for all enum members which have it
        pub fn value(&self) -> &Sequence<u8> {
            match self {
                OptionalId::None{} => panic!("field does not exist on this variant"),
                OptionalId::Some{value, } => value,
            }
        }
    }

    impl Debug
        for OptionalId {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for OptionalId {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                OptionalId::None{} => {
                    write!(_formatter, "KipioAccountDomainPrimitives.OptionalId.None")?;
                    Ok(())
                },
                OptionalId::Some{value, } => {
                    write!(_formatter, "KipioAccountDomainPrimitives.OptionalId.Some(")?;
                    DafnyPrint::fmt_print(value, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for OptionalId {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (OptionalId::None{}, OptionalId::None{}) => {
                    true
                },
                (OptionalId::Some{value, }, OptionalId::Some{value: _2_value, }) => {
                    value == _2_value
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for OptionalId {}

    impl Hash
        for OptionalId {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                OptionalId::None{} => {
                    
                },
                OptionalId::Some{value, } => {
                    Hash::hash(value, _state)
                },
            }
        }
    }

    impl AsRef<OptionalId>
        for OptionalId {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// authority/EffectiveAuthority.dfy(211,1)
pub mod KipioAccountEffectiveAuthority {
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::Sequence;
    pub use ::dafny_runtime::Set;
    pub use crate::KipioAccountCapability::Capability;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::dafny_runtime::SequenceIter;
    pub use ::dafny_runtime::seq;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// authority/EffectiveAuthority.dfy(265,3)
        pub fn AuthoritySourceReferenceKind(source: &Rc<AuthoritySourceReference>) -> Rc<AuthoritySourceKind> {
            source.kind().clone()
        }
        /// authority/EffectiveAuthority.dfy(273,3)
        pub fn AuthoritySourceReferenceId(source: &Rc<AuthoritySourceReference>) -> Sequence<u8> {
            source.id().clone()
        }
        /// authority/EffectiveAuthority.dfy(281,3)
        pub fn ValidAuthoritySourceReference(source: &Rc<AuthoritySourceReference>) -> bool {
            crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::AuthoritySourceReferenceId(source))
        }
        /// authority/EffectiveAuthority.dfy(346,3)
        pub fn SameEffectiveAuthority(left: &Set<Rc<Capability>>, right: &Set<Rc<Capability>>) -> bool {
            left.clone() == right.clone()
        }
        /// authority/EffectiveAuthority.dfy(377,3)
        pub fn CapabilityIsEffective(authority: &Set<Rc<Capability>>, capability: &Rc<Capability>) -> bool {
            authority.contains(capability)
        }
        /// authority/EffectiveAuthority.dfy(410,3)
        pub fn ValidEffectiveAuthority(authority: &Set<Rc<Capability>>) -> bool {
            authority.iter().all(({
                    let mut authority = authority.clone();
                    Rc::new(move |__forall_var_0: &Rc<Capability>| -> bool{
            let mut capability: Rc<Capability> = __forall_var_0.clone();
            !authority.contains(&capability) || crate::KipioAccountCapability::_default::ValidCapability(&capability)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/EffectiveAuthority.dfy(435,3)
        pub fn EffectiveAuthorityWithinSourceAuthority(effectiveAuthority: &Set<Rc<Capability>>, sourceAuthority: &Set<Rc<Capability>>) -> bool {
            effectiveAuthority.clone() <= sourceAuthority.clone()
        }
        /// authority/EffectiveAuthority.dfy(600,3)
        pub fn DerivedAuthorityWithinSource(derivedAuthority: &Set<Rc<Capability>>, sourceAuthority: &Set<Rc<Capability>>) -> bool {
            derivedAuthority.clone() <= sourceAuthority.clone()
        }
    }

    /// authority/EffectiveAuthority.dfy(251,3)
    #[derive(Clone)]
    pub enum AuthoritySourceKind {
        AccountAuthoritySource {},
        CredentialAuthoritySource {},
        SessionAuthoritySource {},
        DelegationAuthoritySource {}
    }

    impl AuthoritySourceKind {}

    impl Debug
        for AuthoritySourceKind {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for AuthoritySourceKind {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                AuthoritySourceKind::AccountAuthoritySource{} => {
                    write!(_formatter, "KipioAccountEffectiveAuthority.AuthoritySourceKind.AccountAuthoritySource")?;
                    Ok(())
                },
                AuthoritySourceKind::CredentialAuthoritySource{} => {
                    write!(_formatter, "KipioAccountEffectiveAuthority.AuthoritySourceKind.CredentialAuthoritySource")?;
                    Ok(())
                },
                AuthoritySourceKind::SessionAuthoritySource{} => {
                    write!(_formatter, "KipioAccountEffectiveAuthority.AuthoritySourceKind.SessionAuthoritySource")?;
                    Ok(())
                },
                AuthoritySourceKind::DelegationAuthoritySource{} => {
                    write!(_formatter, "KipioAccountEffectiveAuthority.AuthoritySourceKind.DelegationAuthoritySource")?;
                    Ok(())
                },
            }
        }
    }

    impl AuthoritySourceKind {
        /// Enumerates all possible values of AuthoritySourceKind
        pub fn _AllSingletonConstructors() -> SequenceIter<Rc<AuthoritySourceKind>> {
            seq![Rc::new(AuthoritySourceKind::AccountAuthoritySource {}), Rc::new(AuthoritySourceKind::CredentialAuthoritySource {}), Rc::new(AuthoritySourceKind::SessionAuthoritySource {}), Rc::new(AuthoritySourceKind::DelegationAuthoritySource {})].iter()
        }
    }

    impl PartialEq
        for AuthoritySourceKind {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (AuthoritySourceKind::AccountAuthoritySource{}, AuthoritySourceKind::AccountAuthoritySource{}) => {
                    true
                },
                (AuthoritySourceKind::CredentialAuthoritySource{}, AuthoritySourceKind::CredentialAuthoritySource{}) => {
                    true
                },
                (AuthoritySourceKind::SessionAuthoritySource{}, AuthoritySourceKind::SessionAuthoritySource{}) => {
                    true
                },
                (AuthoritySourceKind::DelegationAuthoritySource{}, AuthoritySourceKind::DelegationAuthoritySource{}) => {
                    true
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for AuthoritySourceKind {}

    impl Hash
        for AuthoritySourceKind {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                AuthoritySourceKind::AccountAuthoritySource{} => {
                    
                },
                AuthoritySourceKind::CredentialAuthoritySource{} => {
                    
                },
                AuthoritySourceKind::SessionAuthoritySource{} => {
                    
                },
                AuthoritySourceKind::DelegationAuthoritySource{} => {
                    
                },
            }
        }
    }

    impl AsRef<AuthoritySourceKind>
        for AuthoritySourceKind {
        fn as_ref(&self) -> &Self {
            self
        }
    }

    /// authority/EffectiveAuthority.dfy(258,3)
    #[derive(Clone)]
    pub enum AuthoritySourceReference {
        AuthoritySourceReference {
            kind: Rc<AuthoritySourceKind>,
            id: Sequence<u8>
        }
    }

    impl AuthoritySourceReference {
        /// Returns a borrow of the field kind
        pub fn kind(&self) -> &Rc<AuthoritySourceKind> {
            match self {
                AuthoritySourceReference::AuthoritySourceReference{kind, id, } => kind,
            }
        }
        /// Returns a borrow of the field id
        pub fn id(&self) -> &Sequence<u8> {
            match self {
                AuthoritySourceReference::AuthoritySourceReference{kind, id, } => id,
            }
        }
    }

    impl Debug
        for AuthoritySourceReference {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for AuthoritySourceReference {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                AuthoritySourceReference::AuthoritySourceReference{kind, id, } => {
                    write!(_formatter, "KipioAccountEffectiveAuthority.AuthoritySourceReference.AuthoritySourceReference(")?;
                    DafnyPrint::fmt_print(kind, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(id, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for AuthoritySourceReference {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (AuthoritySourceReference::AuthoritySourceReference{kind, id, }, AuthoritySourceReference::AuthoritySourceReference{kind: _2_kind, id: _2_id, }) => {
                    kind == _2_kind && id == _2_id
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for AuthoritySourceReference {}

    impl Hash
        for AuthoritySourceReference {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                AuthoritySourceReference::AuthoritySourceReference{kind, id, } => {
                    Hash::hash(kind, _state);
                    Hash::hash(id, _state)
                },
            }
        }
    }

    impl AsRef<AuthoritySourceReference>
        for AuthoritySourceReference {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// authority/EffectiveAuthorityComposition.dfy(60,1)
pub mod KipioAccountEffectiveAuthorityComposition {
    pub use ::dafny_runtime::Set;
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountIdentity::Identity;
    pub use ::dafny_runtime::Sequence;
    pub use ::dafny_runtime::int;
    pub use crate::KipioAccountCredential::Credential;
    pub use crate::KipioAccountSession::Session;
    pub use crate::KipioAccountDelegation::Delegation;
    pub use crate::KipioAccountAuthorizationState::AuthorizationState;
    pub use ::dafny_runtime::DafnyInt;
    pub use ::dafny_runtime::MaybePlacebo;
    pub use crate::KipioAccountAccount::Account;
    pub use ::dafny_runtime::set;
    pub use crate::KipioAccountCapability::Capability;
    pub use crate::KipioAccountEffectiveAuthority::AuthoritySourceReference;
    pub use ::dafny_runtime::seq;
    pub use crate::KipioAccountEffectiveAuthority::AuthoritySourceKind;

    pub struct _default {}

    impl _default {
        /// authority/EffectiveAuthorityComposition.dfy(83,5)
        pub fn AuthorizationIdentityContextIsValidExecutable(identities: &Set<Rc<Identity>>) -> bool {
            identities.iter().all(({
                    let mut identities = identities.clone();
                    Rc::new(move |__forall_var_0: &Rc<Identity>| -> bool{
            let mut identity: Rc<Identity> = __forall_var_0.clone();
            !identities.contains(&identity) || crate::KipioAccountIdentity::_default::ValidIdentity(&identity)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref()) && identities.iter().all(({
                    let mut identities = identities.clone();
                    Rc::new(move |__forall_var_1: &Rc<Identity>| -> bool{
            let mut first: Rc<Identity> = __forall_var_1.clone();
            (&identities).iter().all(({
                    let mut first = first.clone();
                    let mut identities = identities.clone();
                    Rc::new(move |__forall_var_2: &Rc<Identity>| -> bool{
            let mut second: Rc<Identity> = __forall_var_2.clone();
            !(identities.contains(&first) && identities.contains(&second) && crate::KipioAccountIdentity::_default::IdentityId(&first) == crate::KipioAccountIdentity::_default::IdentityId(&second)) || first.clone() == second.clone()
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/EffectiveAuthorityComposition.dfy(117,5)
        pub fn IdLtLexicographic(left: &Sequence<u8>, right: &Sequence<u8>) -> bool {
            let mut _r0 = left.clone();
            let mut _r1 = right.clone();
            'TAIL_CALL_START: loop {
                let left = _r0;
                let right = _r1;
                if left.cardinality() == int!(0) {
                    return int!(0) < right.cardinality();
                } else {
                    if right.cardinality() == int!(0) {
                        return false;
                    } else {
                        if left.get(&int!(0)) < right.get(&int!(0)) {
                            return true;
                        } else {
                            if left.get(&int!(0)) == right.get(&int!(0)) {
                                let mut _in0: Sequence<u8> = left.drop(&int!(1));
                                let mut _in1: Sequence<u8> = right.drop(&int!(1));
                                _r0 = _in0.clone();
                                _r1 = _in1.clone();
                                continue 'TAIL_CALL_START;
                            } else {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        /// authority/EffectiveAuthorityComposition.dfy(133,5)
        pub fn IdLessEq(left: &Sequence<u8>, right: &Sequence<u8>) -> bool {
            left.clone() == right.clone() || _default::IdLtLexicographic(left, right)
        }
        /// authority/EffectiveAuthorityComposition.dfy(292,5)
        pub fn CredentialLessEq(left: &Rc<Credential>, right: &Rc<Credential>) -> bool {
            _default::IdLessEq(&crate::KipioAccountCredential::_default::CredentialId(left), &crate::KipioAccountCredential::_default::CredentialId(right))
        }
        /// authority/EffectiveAuthorityComposition.dfy(372,5)
        pub fn SessionLessEq(left: &Rc<Session>, right: &Rc<Session>) -> bool {
            _default::IdLessEq(&crate::KipioAccountSession::_default::SessionId(left), &crate::KipioAccountSession::_default::SessionId(right))
        }
        /// authority/EffectiveAuthorityComposition.dfy(452,5)
        pub fn DelegationLessEq(left: &Rc<Delegation>, right: &Rc<Delegation>) -> bool {
            _default::IdLessEq(&crate::KipioAccountDelegation::_default::DelegationId(left), &crate::KipioAccountDelegation::_default::DelegationId(right))
        }
        /// authority/EffectiveAuthorityComposition.dfy(532,5)
        pub fn IsMinCredential(c: &Rc<Credential>, s: &Set<Rc<Credential>>) -> bool {
            s.contains(c) && s.iter().all(({
                    let mut c = c.clone();
                    let mut s = s.clone();
                    Rc::new(move |__forall_var_0: &Rc<Credential>| -> bool{
            let mut other: Rc<Credential> = __forall_var_0.clone();
            !s.contains(&other) || _default::CredentialLessEq(&c, &other)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/EffectiveAuthorityComposition.dfy(541,5)
        pub fn IsMinSession(sess: &Rc<Session>, s: &Set<Rc<Session>>) -> bool {
            s.contains(sess) && s.iter().all(({
                    let mut s = s.clone();
                    let mut sess = sess.clone();
                    Rc::new(move |__forall_var_0: &Rc<Session>| -> bool{
            let mut other: Rc<Session> = __forall_var_0.clone();
            !s.contains(&other) || _default::SessionLessEq(&sess, &other)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/EffectiveAuthorityComposition.dfy(550,5)
        pub fn IsMinDelegation(d: &Rc<Delegation>, s: &Set<Rc<Delegation>>) -> bool {
            s.contains(d) && s.iter().all(({
                    let mut d = d.clone();
                    let mut s = s.clone();
                    Rc::new(move |__forall_var_0: &Rc<Delegation>| -> bool{
            let mut other: Rc<Delegation> = __forall_var_0.clone();
            !s.contains(&other) || _default::DelegationLessEq(&d, &other)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/EffectiveAuthorityComposition.dfy(739,5)
        pub fn SelectMinCredential(s: &Set<Rc<Credential>>, state: &Rc<AuthorizationState>) -> Rc<Credential> {
            (&({
                let mut s = s.clone();
                Rc::new(move |__let_dummy_0: &DafnyInt| -> Rc<Credential>{
            let mut c = MaybePlacebo::<Rc<Credential>>::new();
            'label_goto__ASSIGN_SUCH_THAT_0: loop {
                for __assign_such_that_0 in (&s).iter().cloned() {
                    c = MaybePlacebo::from(__assign_such_that_0.clone());
                    if s.contains(&c.read()) && (&s).iter().all(({
                                let mut c = c.clone();
                                let mut s = s.clone();
                                Rc::new(move |__forall_var_0: &Rc<Credential>| -> bool{
            let mut other: Rc<Credential> = __forall_var_0.clone();
            !s.contains(&other) || _default::CredentialLessEq(&c.read(), &other)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                            }).as_ref()) {
                        break 'label_goto__ASSIGN_SUCH_THAT_0;
                    }
                }
                panic!("Halt");
                break;
            };
            c.read()
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
            }))(&int!(0))
        }
        /// authority/EffectiveAuthorityComposition.dfy(757,5)
        pub fn SelectMinSession(s: &Set<Rc<Session>>, state: &Rc<AuthorizationState>) -> Rc<Session> {
            (&({
                let mut s = s.clone();
                Rc::new(move |__let_dummy_1: &DafnyInt| -> Rc<Session>{
            let mut sess = MaybePlacebo::<Rc<Session>>::new();
            'label_goto__ASSIGN_SUCH_THAT_0: loop {
                for __assign_such_that_0 in (&s).iter().cloned() {
                    sess = MaybePlacebo::from(__assign_such_that_0.clone());
                    if s.contains(&sess.read()) && (&s).iter().all(({
                                let mut s = s.clone();
                                let mut sess = sess.clone();
                                Rc::new(move |__forall_var_0: &Rc<Session>| -> bool{
            let mut other: Rc<Session> = __forall_var_0.clone();
            !s.contains(&other) || _default::SessionLessEq(&sess.read(), &other)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                            }).as_ref()) {
                        break 'label_goto__ASSIGN_SUCH_THAT_0;
                    }
                }
                panic!("Halt");
                break;
            };
            sess.read()
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
            }))(&int!(0))
        }
        /// authority/EffectiveAuthorityComposition.dfy(775,5)
        pub fn SelectMinDelegation(s: &Set<Rc<Delegation>>, state: &Rc<AuthorizationState>) -> Rc<Delegation> {
            (&({
                let mut s = s.clone();
                Rc::new(move |__let_dummy_2: &DafnyInt| -> Rc<Delegation>{
            let mut d = MaybePlacebo::<Rc<Delegation>>::new();
            'label_goto__ASSIGN_SUCH_THAT_0: loop {
                for __assign_such_that_0 in (&s).iter().cloned() {
                    d = MaybePlacebo::from(__assign_such_that_0.clone());
                    if s.contains(&d.read()) && (&s).iter().all(({
                                let mut d = d.clone();
                                let mut s = s.clone();
                                Rc::new(move |__forall_var_0: &Rc<Delegation>| -> bool{
            let mut other: Rc<Delegation> = __forall_var_0.clone();
            !s.contains(&other) || _default::DelegationLessEq(&d.read(), &other)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                            }).as_ref()) {
                        break 'label_goto__ASSIGN_SUCH_THAT_0;
                    }
                }
                panic!("Halt");
                break;
            };
            d.read()
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
            }))(&int!(0))
        }
        /// authority/EffectiveAuthorityComposition.dfy(797,5)
        pub fn DelegationProvenanceIsLegitimateForAccountExecutable(account: &Rc<Account>, state: &Rc<AuthorizationState>, identities: &Set<Rc<Identity>>, delegation: &Rc<Delegation>, visited: &Set<Sequence<u8>>) -> bool {
            crate::KipioAccountAuthorizationState::_default::StateDelegations(state).contains(delegation) && _default::AuthorizationIdentityContextIsValidExecutable(identities) && !visited.contains(&crate::KipioAccountDelegation::_default::DelegationId(delegation)) && crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(state).keys().contains(&crate::KipioAccountDelegation::_default::DelegationId(delegation)) && (crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(state).get(&crate::KipioAccountDelegation::_default::DelegationId(delegation)) == set!{} && crate::KipioAccountDelegation::_default::DelegationSourceIdentityId(delegation) == crate::KipioAccountIdentity::_default::IdentityId(&crate::KipioAccountAccount::_default::AccountSovereignIdentity(account)) || crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(state).get(&crate::KipioAccountDelegation::_default::DelegationId(delegation)) != set!{} && (&crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(state).get(&crate::KipioAccountDelegation::_default::DelegationId(delegation))).iter().all(({
                        let mut state = state.clone();
                        let mut account = account.clone();
                        let mut visited = visited.clone();
                        let mut delegation = delegation.clone();
                        let mut identities = identities.clone();
                        Rc::new(move |__forall_var_0: &Sequence<u8>| -> bool{
            let mut parentId: Sequence<u8> = __forall_var_0.clone();
            !crate::KipioAccountAuthorizationState::_default::StateDelegationProvenance(&state).get(&crate::KipioAccountDelegation::_default::DelegationId(&delegation)).contains(&parentId) || (&crate::KipioAccountAuthorizationState::_default::StateDelegations(&state)).iter().any(({
                    let mut state = state.clone();
                    let mut account = account.clone();
                    let mut visited = visited.clone();
                    let mut parentId = parentId.clone();
                    let mut delegation = delegation.clone();
                    let mut identities = identities.clone();
                    Rc::new(move |__exists_var_0: &Rc<Delegation>| -> bool{
            let mut parent: Rc<Delegation> = __exists_var_0.clone();
            crate::KipioAccountAuthorizationState::_default::StateDelegations(&state).contains(&parent) && crate::KipioAccountDelegation::_default::DelegationId(&parent) == parentId.clone() && crate::KipioAccountAuthorizationState::_default::ParentDelegationSupportsChildSource(&identities, &parent, &delegation) && _default::DelegationProvenanceIsLegitimateForAccountExecutable(&account, &state, &identities, &parent, &visited.merge(&set!{crate::KipioAccountDelegation::_default::DelegationId(&delegation)}))
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                    }).as_ref()))
        }
        /// authority/EffectiveAuthorityComposition.dfy(841,5)
        pub fn DelegationHasLegitimateSourceProvenanceForAccountExecutable(account: &Rc<Account>, state: &Rc<AuthorizationState>, identities: &Set<Rc<Identity>>, delegation: &Rc<Delegation>) -> bool {
            _default::DelegationProvenanceIsLegitimateForAccountExecutable(account, state, identities, delegation, &set!{})
        }
        /// authority/EffectiveAuthorityComposition.dfy(906,5)
        pub fn UnionOfContributions(contributions: &Sequence<Set<Rc<Capability>>>) -> Set<Rc<Capability>> {
            let mut _accumulator: Set<Rc<Capability>> = set!{};
            let mut _r0 = contributions.clone();
            'TAIL_CALL_START: loop {
                let contributions = _r0;
                if contributions.cardinality() == int!(0) {
                    return set!{}.merge(&_accumulator);
                } else {
                    _accumulator = contributions.get(&(contributions.cardinality() - int!(1))).merge(&_accumulator);
                    let mut _in0: Sequence<Set<Rc<Capability>>> = contributions.take(&(contributions.cardinality() - int!(1)));
                    _r0 = _in0.clone();
                    continue 'TAIL_CALL_START;
                }
            }
        }
        /// authority/EffectiveAuthorityComposition.dfy(942,5)
        pub fn ComputeEffectiveAuthorityWitness(account: &Rc<Account>, identities: &Set<Rc<Identity>>, now: &DafnyInt) -> (Set<Rc<Capability>>, Sequence<Rc<AuthoritySourceReference>>, Sequence<Set<Rc<Capability>>>, Sequence<Set<Rc<Capability>>>) {
            let mut effectiveAuthority = MaybePlacebo::<Set<Rc<Capability>>>::new();
            let mut sourceReferences: Sequence<Rc<AuthoritySourceReference>>;
            let mut sourceAuthorities: Sequence<Set<Rc<Capability>>>;
            let mut contributions: Sequence<Set<Rc<Capability>>>;
            let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
            sourceReferences = seq![] as Sequence<Rc<AuthoritySourceReference>>;
            sourceAuthorities = seq![] as Sequence<Set<Rc<Capability>>>;
            contributions = seq![] as Sequence<Set<Rc<Capability>>>;
            let mut accountRef: Rc<AuthoritySourceReference> = Rc::new(AuthoritySourceReference::AuthoritySourceReference {
                        kind: Rc::new(AuthoritySourceKind::AccountAuthoritySource {}),
                        id: crate::KipioAccountAccount::_default::AccountIdOf(account)
                    });
            let mut accountAuth: Set<Rc<Capability>> = crate::KipioAccountAuthorizationState::_default::StateCapabilities(&state);
            sourceReferences = sourceReferences.concat(&seq![accountRef.clone()]);
            sourceAuthorities = sourceAuthorities.concat(&seq![accountAuth.clone()]);
            contributions = contributions.concat(&seq![accountAuth.clone()]);
            let mut remainingCredentials: Set<Rc<Credential>> = crate::KipioAccountAuthorizationState::_default::StateCredentials(&state);
            while remainingCredentials.clone() != set!{} {
                let mut credential: Rc<Credential> = _default::SelectMinCredential(&remainingCredentials, &state);
                remainingCredentials = remainingCredentials.subtract(&set!{credential.clone()});
                if crate::KipioAccountAuthorizationState::_default::ActiveCredentialRecognizedInState(&state, &crate::KipioAccountCredential::_default::CredentialId(&credential)) {
                    let mut credRef: Rc<AuthoritySourceReference> = Rc::new(AuthoritySourceReference::AuthoritySourceReference {
                                kind: Rc::new(AuthoritySourceKind::CredentialAuthoritySource {}),
                                id: crate::KipioAccountCredential::_default::CredentialId(&credential)
                            });
                    let mut credAuth: Set<Rc<Capability>> = crate::KipioAccountAuthorizationState::_default::StateCredentialAuthority(&state, &credential);
                    sourceReferences = sourceReferences.concat(&seq![credRef.clone()]);
                    sourceAuthorities = sourceAuthorities.concat(&seq![credAuth.clone()]);
                    contributions = contributions.concat(&seq![credAuth.clone()]);
                }
            };
            let mut remainingSessions: Set<Rc<Session>> = crate::KipioAccountAuthorizationState::_default::StateSessions(&state);
            while remainingSessions.clone() != set!{} {
                let mut session: Rc<Session> = _default::SelectMinSession(&remainingSessions, &state);
                remainingSessions = remainingSessions.subtract(&set!{session.clone()});
                if crate::KipioAccountAuthorizationState::_default::SessionCanContributeAuthorityAt(&state, &session, now) {
                    let mut sessionRef: Rc<AuthoritySourceReference> = Rc::new(AuthoritySourceReference::AuthoritySourceReference {
                                kind: Rc::new(AuthoritySourceKind::SessionAuthoritySource {}),
                                id: crate::KipioAccountSession::_default::SessionId(&session)
                            });
                    let mut sessionAuth: Set<Rc<Capability>> = crate::KipioAccountSession::_default::SessionCapabilities(&session);
                    sourceReferences = sourceReferences.concat(&seq![sessionRef.clone()]);
                    sourceAuthorities = sourceAuthorities.concat(&seq![sessionAuth.clone()]);
                    contributions = contributions.concat(&seq![sessionAuth.clone()]);
                }
            };
            let mut remainingDelegations: Set<Rc<Delegation>> = crate::KipioAccountAuthorizationState::_default::StateDelegations(&state);
            while remainingDelegations.clone() != set!{} {
                let mut delegation: Rc<Delegation> = _default::SelectMinDelegation(&remainingDelegations, &state);
                remainingDelegations = remainingDelegations.subtract(&set!{delegation.clone()});
                if crate::KipioAccountAuthorizationState::_default::DelegationCanContributeAuthorityAt(&state, &delegation, now) && _default::DelegationHasLegitimateSourceProvenanceForAccountExecutable(account, &state, identities, &delegation) {
                    let mut delegRef: Rc<AuthoritySourceReference> = Rc::new(AuthoritySourceReference::AuthoritySourceReference {
                                kind: Rc::new(AuthoritySourceKind::DelegationAuthoritySource {}),
                                id: crate::KipioAccountDelegation::_default::DelegationId(&delegation)
                            });
                    let mut delegAuth: Set<Rc<Capability>> = crate::KipioAccountDelegation::_default::DelegationCapabilities(&delegation);
                    sourceReferences = sourceReferences.concat(&seq![delegRef.clone()]);
                    sourceAuthorities = sourceAuthorities.concat(&seq![delegAuth.clone()]);
                    contributions = contributions.concat(&seq![delegAuth.clone()]);
                }
            };
            effectiveAuthority = MaybePlacebo::from(_default::UnionOfContributions(&contributions));
            return (
                    effectiveAuthority.read(),
                    sourceReferences.clone(),
                    sourceAuthorities.clone(),
                    contributions.clone()
                );
        }
    }
}
/// execution/ExecutionConstraints.dfy(119,1)
pub mod KipioAccountExecutionConstraints {
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct ExecutionConstraints;
    impl ::dafny_runtime::DafnyPrint for ExecutionConstraints {
        fn fmt_print(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
            _in_seq: bool,
        ) -> ::std::fmt::Result {
            write!(f, "ExecutionConstraints")
        }
    }

    
}
/// execution/ExecutionContext.dfy(198,1)
pub mod KipioAccountExecutionContext {
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountExecutionRequest::ExecutionRequest;
    pub use crate::KipioAccountAuthorizationState::AuthorizationState;
    pub use ::dafny_runtime::Set;
    pub use crate::KipioAccountCapability::Capability;
    pub use crate::KipioAccountExecutionConstraints::ExecutionConstraints;
    pub use ::dafny_runtime::DafnyInt;
    pub use crate::KipioAccountAuthorization::Authorization;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// execution/ExecutionContext.dfy(233,3)
        pub fn ExecutionContextRequest(context: &Rc<ExecutionContext>) -> Rc<ExecutionRequest> {
            context.request().clone()
        }
        /// execution/ExecutionContext.dfy(241,3)
        pub fn ExecutionContextAuthorizationState(context: &Rc<ExecutionContext>) -> Rc<AuthorizationState> {
            context.authorizationState().clone()
        }
        /// execution/ExecutionContext.dfy(249,3)
        pub fn ExecutionContextEffectiveAuthority(context: &Rc<ExecutionContext>) -> Set<Rc<Capability>> {
            context.effectiveAuthority().clone()
        }
        /// execution/ExecutionContext.dfy(257,3)
        pub fn ExecutionContextConstraints(context: &Rc<ExecutionContext>) -> ExecutionConstraints {
            crate::KipioAccountExecutionRequest::_default::ExecutionRequestConstraints(&_default::ExecutionContextRequest(context))
        }
        /// execution/ExecutionContext.dfy(267,3)
        pub fn ExecutionContextEvaluationTime(context: &Rc<ExecutionContext>) -> DafnyInt {
            context.evaluationTime().clone()
        }
        /// execution/ExecutionContext.dfy(279,3)
        pub fn ExecutionContextAuthorization(context: &Rc<ExecutionContext>) -> Rc<Authorization> {
            crate::KipioAccountExecutionRequest::_default::ExecutionRequestAuthorization(&_default::ExecutionContextRequest(context))
        }
        /// execution/ExecutionContext.dfy(293,3)
        pub fn ExecutionContextCredentialIsRecognized(context: &Rc<ExecutionContext>) -> bool {
            crate::KipioAccountAuthorizationValidation::_default::AuthorizationCredentialIsRecognized(&_default::ExecutionContextAuthorization(context), &_default::ExecutionContextAuthorizationState(context))
        }
        /// execution/ExecutionContext.dfy(304,3)
        pub fn ExecutionContextCredentialIsUsable(context: &Rc<ExecutionContext>) -> bool {
            crate::KipioAccountAuthorizationValidation::_default::AuthorizationCredentialIsUsable(&_default::ExecutionContextAuthorization(context), &_default::ExecutionContextAuthorizationState(context))
        }
        /// execution/ExecutionContext.dfy(319,3)
        pub fn ExecutionContextRequestIsConsistent(context: &Rc<ExecutionContext>) -> bool {
            crate::KipioAccountExecutionRequest::_default::ExecutionRequestAuthorizationContextIsConsistent(&_default::ExecutionContextRequest(context))
        }
        /// execution/ExecutionContext.dfy(520,3)
        pub fn StructurallyValidExecutionContext(context: &Rc<ExecutionContext>) -> bool {
            crate::KipioAccountExecutionRequest::_default::ValidExecutionRequest(&_default::ExecutionContextRequest(context)) && crate::KipioAccountAuthorizationState::_default::ValidAuthorizationState(&_default::ExecutionContextAuthorizationState(context)) && crate::KipioAccountEffectiveAuthority::_default::ValidEffectiveAuthority(&_default::ExecutionContextEffectiveAuthority(context)) && crate::KipioAccountAuthorization::_default::RequestedAuthorityWithinEffectiveAuthority(&_default::ExecutionContextAuthorization(context), &_default::ExecutionContextEffectiveAuthority(context)) && _default::ExecutionContextCredentialIsRecognized(context) && _default::ExecutionContextCredentialIsUsable(context)
        }
    }

    /// execution/ExecutionContext.dfy(220,3)
    #[derive(Clone)]
    pub enum ExecutionContext {
        ExecutionContext {
            request: Rc<ExecutionRequest>,
            authorizationState: Rc<AuthorizationState>,
            effectiveAuthority: Set<Rc<Capability>>,
            evaluationTime: DafnyInt
        }
    }

    impl ExecutionContext {
        /// Returns a borrow of the field request
        pub fn request(&self) -> &Rc<ExecutionRequest> {
            match self {
                ExecutionContext::ExecutionContext{request, authorizationState, effectiveAuthority, evaluationTime, } => request,
            }
        }
        /// Returns a borrow of the field authorizationState
        pub fn authorizationState(&self) -> &Rc<AuthorizationState> {
            match self {
                ExecutionContext::ExecutionContext{request, authorizationState, effectiveAuthority, evaluationTime, } => authorizationState,
            }
        }
        /// Returns a borrow of the field effectiveAuthority
        pub fn effectiveAuthority(&self) -> &Set<Rc<Capability>> {
            match self {
                ExecutionContext::ExecutionContext{request, authorizationState, effectiveAuthority, evaluationTime, } => effectiveAuthority,
            }
        }
        /// Returns a borrow of the field evaluationTime
        pub fn evaluationTime(&self) -> &DafnyInt {
            match self {
                ExecutionContext::ExecutionContext{request, authorizationState, effectiveAuthority, evaluationTime, } => evaluationTime,
            }
        }
    }

    impl Debug
        for ExecutionContext {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for ExecutionContext {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                ExecutionContext::ExecutionContext{request, authorizationState, effectiveAuthority, evaluationTime, } => {
                    write!(_formatter, "KipioAccountExecutionContext.ExecutionContext.ExecutionContext(")?;
                    DafnyPrint::fmt_print(request, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(authorizationState, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(effectiveAuthority, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(evaluationTime, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for ExecutionContext {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (ExecutionContext::ExecutionContext{request, authorizationState, effectiveAuthority, evaluationTime, }, ExecutionContext::ExecutionContext{request: _2_request, authorizationState: _2_authorizationState, effectiveAuthority: _2_effectiveAuthority, evaluationTime: _2_evaluationTime, }) => {
                    request == _2_request && authorizationState == _2_authorizationState && effectiveAuthority == _2_effectiveAuthority && evaluationTime == _2_evaluationTime
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for ExecutionContext {}

    impl Hash
        for ExecutionContext {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                ExecutionContext::ExecutionContext{request, authorizationState, effectiveAuthority, evaluationTime, } => {
                    Hash::hash(request, _state);
                    Hash::hash(authorizationState, _state);
                    Hash::hash(effectiveAuthority, _state);
                    Hash::hash(evaluationTime, _state)
                },
            }
        }
    }

    impl AsRef<ExecutionContext>
        for ExecutionContext {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// execution/ExecutionContextComposition.dfy(80,1)
pub mod KipioAccountExecutionContextComposition {
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountExecutionContext::ExecutionContext;
    pub use crate::KipioAccountAccount::Account;
    pub use ::dafny_runtime::Set;
    pub use crate::KipioAccountIdentity::Identity;
    pub use ::std::default::Default;
    pub use crate::KipioAccountCapability::Capability;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountEffectiveAuthority::AuthoritySourceReference;

    pub struct _default {}

    impl _default {
        /// execution/ExecutionContextComposition.dfy(106,5)
        pub fn ExecutionContextUsesAccountAuthorizationStateExec(context: &Rc<ExecutionContext>, account: &Rc<Account>) -> bool {
            crate::KipioAccountExecutionContext::_default::ExecutionContextAuthorizationState(context) == crate::KipioAccountAccount::_default::AccountAuthorizationState(account)
        }
        /// execution/ExecutionContextComposition.dfy(133,5)
        pub fn StructurallyValidExecutionContextExec(context: &Rc<ExecutionContext>) -> bool {
            crate::KipioAccountExecutionContext::_default::StructurallyValidExecutionContext(context)
        }
        /// execution/ExecutionContextComposition.dfy(145,5)
        pub fn ValidExecutionContextForAccountFull(context: &Rc<ExecutionContext>, account: &Rc<Account>, identities: &Set<Rc<Identity>>, proofVerified: bool) -> bool {
            let mut valid: bool = <bool as Default>::default();
            if !crate::KipioAccountExecutionContext::_default::StructurallyValidExecutionContext(context) {
                valid = false;
                return valid;
            };
            if !_default::ExecutionContextUsesAccountAuthorizationStateExec(context, account) {
                valid = false;
                return valid;
            };
            if !crate::KipioAccountAccount::_default::ValidAccount(account) {
                valid = false;
                return valid;
            };
            if !crate::KipioAccountEffectiveAuthorityComposition::_default::AuthorizationIdentityContextIsValidExecutable(identities) {
                valid = false;
                return valid;
            };
            let mut accepted: bool;
            let mut _out0: bool = crate::KipioAccountAuthorizationAcceptanceWithProvenance::_default::AuthorizationCanBeAcceptedFull(&crate::KipioAccountExecutionContext::_default::ExecutionContextAuthorization(context), account, identities, &crate::KipioAccountExecutionContext::_default::ExecutionContextEvaluationTime(context), proofVerified);
            accepted = _out0;
            if !accepted {
                valid = false;
                return valid;
            };
            let mut derivedEA: Set<Rc<Capability>>;
            let mut _v0: Sequence<Rc<AuthoritySourceReference>>;
            let mut _v1: Sequence<Set<Rc<Capability>>>;
            let mut _v2: Sequence<Set<Rc<Capability>>>;
            let mut _out1: Set<Rc<Capability>>;
            let mut _out2: Sequence<Rc<AuthoritySourceReference>>;
            let mut _out3: Sequence<Set<Rc<Capability>>>;
            let mut _out4: Sequence<Set<Rc<Capability>>>;
            let _x = crate::KipioAccountEffectiveAuthorityComposition::_default::ComputeEffectiveAuthorityWitness(account, identities, &crate::KipioAccountExecutionContext::_default::ExecutionContextEvaluationTime(context));
            _out1 = _x.0;
            _out2 = _x.1;
            _out3 = _x.2;
            _out4 = _x.3;
            derivedEA = _out1.clone();
            _v0 = _out2.clone();
            _v1 = _out3.clone();
            _v2 = _out4.clone();
            valid = crate::KipioAccountExecutionContext::_default::ExecutionContextEffectiveAuthority(context) == derivedEA.clone();
            return valid;
        }
    }
}
/// execution/ExecutionRequest.dfy(217,1)
pub mod KipioAccountExecutionRequest {
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountDomainAction::DomainAction;
    pub use crate::KipioAccountAuthorization::Authorization;
    pub use crate::KipioAccountExecutionTarget::ExecutionTarget;
    pub use crate::KipioAccountExecutionConstraints::ExecutionConstraints;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// execution/ExecutionRequest.dfy(253,3)
        pub fn ExecutionRequestAction(request: &Rc<ExecutionRequest>) -> DomainAction {
            request.action().clone()
        }
        /// execution/ExecutionRequest.dfy(264,3)
        pub fn ExecutionRequestAuthorization(request: &Rc<ExecutionRequest>) -> Rc<Authorization> {
            request.authorization().clone()
        }
        /// execution/ExecutionRequest.dfy(276,3)
        pub fn ExecutionRequestTarget(request: &Rc<ExecutionRequest>) -> ExecutionTarget {
            request.target().clone()
        }
        /// execution/ExecutionRequest.dfy(286,3)
        pub fn ExecutionRequestConstraints(request: &Rc<ExecutionRequest>) -> ExecutionConstraints {
            request.constraints().clone()
        }
        /// execution/ExecutionRequest.dfy(306,3)
        pub fn ExecutionRequestActionMatchesAuthorization(request: &Rc<ExecutionRequest>) -> bool {
            _default::ExecutionRequestAction(request) == crate::KipioAccountAuthorization::_default::AuthorizationDomainAction(&_default::ExecutionRequestAuthorization(request))
        }
        /// execution/ExecutionRequest.dfy(323,3)
        pub fn ExecutionRequestTargetMatchesAuthorization(request: &Rc<ExecutionRequest>) -> bool {
            _default::ExecutionRequestTarget(request) == crate::KipioAccountAuthorization::_default::AuthorizationExecutionTarget(&_default::ExecutionRequestAuthorization(request))
        }
        /// execution/ExecutionRequest.dfy(342,3)
        pub fn ExecutionRequestAuthorizationContextIsConsistent(request: &Rc<ExecutionRequest>) -> bool {
            _default::ExecutionRequestActionMatchesAuthorization(request) && _default::ExecutionRequestTargetMatchesAuthorization(request)
        }
        /// execution/ExecutionRequest.dfy(368,3)
        pub fn ValidExecutionRequest(request: &Rc<ExecutionRequest>) -> bool {
            crate::KipioAccountAuthorization::_default::ValidAuthorization(&_default::ExecutionRequestAuthorization(request)) && _default::ExecutionRequestAuthorizationContextIsConsistent(request)
        }
    }

    /// execution/ExecutionRequest.dfy(238,3)
    #[derive(Clone)]
    pub enum ExecutionRequest {
        ExecutionRequest {
            action: DomainAction,
            authorization: Rc<Authorization>,
            target: ExecutionTarget,
            constraints: ExecutionConstraints
        }
    }

    impl ExecutionRequest {
        /// Returns a borrow of the field action
        pub fn action(&self) -> &DomainAction {
            match self {
                ExecutionRequest::ExecutionRequest{action, authorization, target, constraints, } => action,
            }
        }
        /// Returns a borrow of the field authorization
        pub fn authorization(&self) -> &Rc<Authorization> {
            match self {
                ExecutionRequest::ExecutionRequest{action, authorization, target, constraints, } => authorization,
            }
        }
        /// Returns a borrow of the field target
        pub fn target(&self) -> &ExecutionTarget {
            match self {
                ExecutionRequest::ExecutionRequest{action, authorization, target, constraints, } => target,
            }
        }
        /// Returns a borrow of the field constraints
        pub fn constraints(&self) -> &ExecutionConstraints {
            match self {
                ExecutionRequest::ExecutionRequest{action, authorization, target, constraints, } => constraints,
            }
        }
    }

    impl Debug
        for ExecutionRequest {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for ExecutionRequest {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                ExecutionRequest::ExecutionRequest{action, authorization, target, constraints, } => {
                    write!(_formatter, "KipioAccountExecutionRequest.ExecutionRequest.ExecutionRequest(")?;
                    DafnyPrint::fmt_print(action, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(authorization, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(target, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(constraints, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for ExecutionRequest {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (ExecutionRequest::ExecutionRequest{action, authorization, target, constraints, }, ExecutionRequest::ExecutionRequest{action: _2_action, authorization: _2_authorization, target: _2_target, constraints: _2_constraints, }) => {
                    action == _2_action && authorization == _2_authorization && target == _2_target && constraints == _2_constraints
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for ExecutionRequest {}

    impl Hash
        for ExecutionRequest {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                ExecutionRequest::ExecutionRequest{action, authorization, target, constraints, } => {
                    Hash::hash(action, _state);
                    Hash::hash(authorization, _state);
                    Hash::hash(target, _state);
                    Hash::hash(constraints, _state)
                },
            }
        }
    }

    impl AsRef<ExecutionRequest>
        for ExecutionRequest {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// execution/ExecutionSemantics.dfy(261,1)
pub mod KipioAccountExecutionSemantics {
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct ExecutionEnvironment;
    impl ::dafny_runtime::DafnyPrint for ExecutionEnvironment {
        fn fmt_print(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
            _in_seq: bool,
        ) -> ::std::fmt::Result {
            write!(f, "ExecutionEnvironment")
        }
    }

    pub use ::std::rc::Rc;
    pub use crate::KipioAccountExecutionContext::ExecutionContext;
    pub use crate::KipioAccountAuthorizationState::AuthorizationState;
    pub use ::dafny_runtime::DafnyInt;

    pub struct _default {}

    impl _default {
        /// execution/ExecutionSemantics.dfy(1332,3)
        pub fn ExecutionEngineAuthorizationState(context: &Rc<ExecutionContext>) -> Rc<AuthorizationState> {
            crate::KipioAccountExecutionContext::_default::ExecutionContextAuthorizationState(context)
        }
        /// execution/ExecutionSemantics.dfy(1664,3)
        pub fn ExecutionEngineEvaluationTime(context: &Rc<ExecutionContext>) -> DafnyInt {
            crate::KipioAccountExecutionContext::_default::ExecutionContextEvaluationTime(context)
        }
    }
}
/// foundation/ExecutionTarget.dfy(97,1)
pub mod KipioAccountExecutionTarget {
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct ExecutionTarget;
    impl ::dafny_runtime::DafnyPrint for ExecutionTarget {
        fn fmt_print(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
            _in_seq: bool,
        ) -> ::std::fmt::Result {
            write!(f, "ExecutionTarget")
        }
    }

    
}
/// foundation/Identity.dfy(53,1)
pub mod KipioAccountIdentity {
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountSubject::Subject;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// foundation/Identity.dfy(105,3)
        pub fn IdentityId(identity: &Rc<Identity>) -> Sequence<u8> {
            identity.id().clone()
        }
        /// foundation/Identity.dfy(128,3)
        pub fn IdentitySubject(identity: &Rc<Identity>) -> Rc<Subject> {
            identity.subject().clone()
        }
        /// foundation/Identity.dfy(148,3)
        pub fn SameIdentity(left: &Rc<Identity>, right: &Rc<Identity>) -> bool {
            _default::IdentityId(left) == _default::IdentityId(right)
        }
        /// foundation/Identity.dfy(172,3)
        pub fn ValidIdentity(identity: &Rc<Identity>) -> bool {
            crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::IdentityId(identity)) && crate::KipioAccountSubject::_default::ValidSubject(&_default::IdentitySubject(identity))
        }
    }

    /// foundation/Identity.dfy(78,3)
    #[derive(Clone)]
    pub enum Identity {
        Identity {
            id: Sequence<u8>,
            subject: Rc<Subject>
        }
    }

    impl Identity {
        /// Returns a borrow of the field id
        pub fn id(&self) -> &Sequence<u8> {
            match self {
                Identity::Identity{id, subject, } => id,
            }
        }
        /// Returns a borrow of the field subject
        pub fn subject(&self) -> &Rc<Subject> {
            match self {
                Identity::Identity{id, subject, } => subject,
            }
        }
    }

    impl Debug
        for Identity {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for Identity {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                Identity::Identity{id, subject, } => {
                    write!(_formatter, "KipioAccountIdentity.Identity.Identity(")?;
                    DafnyPrint::fmt_print(id, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(subject, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for Identity {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (Identity::Identity{id, subject, }, Identity::Identity{id: _2_id, subject: _2_subject, }) => {
                    id == _2_id && subject == _2_subject
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for Identity {}

    impl Hash
        for Identity {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                Identity::Identity{id, subject, } => {
                    Hash::hash(id, _state);
                    Hash::hash(subject, _state)
                },
            }
        }
    }

    impl AsRef<Identity>
        for Identity {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// policy/Policy.dfy(88,1)
pub mod KipioAccountPolicy {
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect;
    pub use ::dafny_runtime::_System::nat;
    pub use ::dafny_runtime::integer_range;
    pub use ::dafny_runtime::int;
    pub use ::dafny_runtime::DafnyInt;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// policy/Policy.dfy(115,3)
        pub fn PolicyEffects(policy: &Rc<Policy>) -> Sequence<Rc<PolicyEffect>> {
            policy.effects().clone()
        }
        /// policy/Policy.dfy(124,3)
        pub fn PolicyEffectCount(policy: &Rc<Policy>) -> nat {
            _default::PolicyEffects(policy).cardinality()
        }
        /// policy/Policy.dfy(137,3)
        pub fn PolicyContainsEffect(policy: &Rc<Policy>, effect: &Rc<PolicyEffect>) -> bool {
            integer_range(int!(0), _default::PolicyEffects(policy).cardinality()).any(({
                    let mut effect = effect.clone();
                    let mut policy = policy.clone();
                    Rc::new(move |__exists_var_0: DafnyInt| -> bool{
            let mut index: DafnyInt = __exists_var_0.clone();
            int!(0) <= index.clone() && index.clone() < _default::PolicyEffects(&policy).cardinality() && _default::PolicyEffects(&policy).get(&index) == effect.clone()
        }) as Rc<dyn ::std::ops::Fn(_) -> _>
                }).as_ref())
        }
        /// policy/Policy.dfy(151,3)
        pub fn PolicyIsEmpty(policy: &Rc<Policy>) -> bool {
            _default::PolicyEffects(policy).cardinality() == int!(0)
        }
        /// policy/Policy.dfy(176,3)
        pub fn PolicyContainsOnlyValidEffects(policy: &Rc<Policy>) -> bool {
            integer_range(int!(0), _default::PolicyEffects(policy).cardinality()).all(({
                    let mut policy = policy.clone();
                    Rc::new(move |__forall_var_0: DafnyInt| -> bool{
            let mut index: DafnyInt = __forall_var_0.clone();
            !(int!(0) <= index.clone() && index.clone() < _default::PolicyEffects(&policy).cardinality()) || crate::KipioAccountPolicyEffect::_default::ValidPolicyEffect(&_default::PolicyEffects(&policy).get(&index))
        }) as Rc<dyn ::std::ops::Fn(_) -> _>
                }).as_ref())
        }
        /// policy/Policy.dfy(201,3)
        pub fn ValidPolicy(policy: &Rc<Policy>) -> bool {
            _default::PolicyContainsOnlyValidEffects(policy)
        }
        /// policy/Policy.dfy(222,3)
        pub fn SamePolicy(left: &Rc<Policy>, right: &Rc<Policy>) -> bool {
            _default::PolicyEffects(left) == _default::PolicyEffects(right)
        }
    }

    /// policy/Policy.dfy(104,3)
    #[derive(Clone)]
    pub enum Policy {
        Policy {
            effects: Sequence<Rc<PolicyEffect>>
        }
    }

    impl Policy {
        /// Returns a borrow of the field effects
        pub fn effects(&self) -> &Sequence<Rc<PolicyEffect>> {
            match self {
                Policy::Policy{effects, } => effects,
            }
        }
    }

    impl Debug
        for Policy {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for Policy {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                Policy::Policy{effects, } => {
                    write!(_formatter, "KipioAccountPolicy.Policy.Policy(")?;
                    DafnyPrint::fmt_print(effects, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for Policy {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (Policy::Policy{effects, }, Policy::Policy{effects: _2_effects, }) => {
                    effects == _2_effects
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for Policy {}

    impl Hash
        for Policy {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                Policy::Policy{effects, } => {
                    Hash::hash(effects, _state)
                },
            }
        }
    }

    impl AsRef<Policy>
        for Policy {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// policy/PolicyApplicationComposition.dfy(64,1)
pub mod KipioAccountPolicyApplicationComposition {
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountAuthorizationState::AuthorizationState;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountCredential::Credential;
    pub use ::dafny_runtime::DafnyInt;
    pub use ::dafny_runtime::MaybePlacebo;
    pub use ::dafny_runtime::int;
    pub use crate::KipioAccountSession::Session;
    pub use crate::KipioAccountDelegation::Delegation;
    pub use crate::KipioAccountAccount::Account;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::RevokeCredential;
    pub use crate::KipioAccountCredential::CredentialStatus;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::RevokeSession;
    pub use crate::KipioAccountSession::SessionStatus;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::RevokeDelegation;
    pub use crate::KipioAccountDelegation::DelegationStatus;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::DisableCapability;
    pub use crate::KipioAccountCapability::Capability;
    pub use crate::KipioAccountPolicyEffect::OptionalPolicyScope;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::EnableCapability;
    pub use crate::KipioAccountRestriction::Restriction;
    pub use crate::KipioAccountPolicyConsumption::PolicyConsumption;

    pub struct _default {}

    impl _default {
        /// policy/PolicyApplicationComposition.dfy(173,3)
        pub fn FindCredentialById(state: &Rc<AuthorizationState>, credentialId: &Sequence<u8>) -> Rc<Credential> {
            (&({
                let mut credentialId = credentialId.clone();
                let mut state = state.clone();
                Rc::new(move |__let_dummy_3: &DafnyInt| -> Rc<Credential>{
            let mut c = MaybePlacebo::<Rc<Credential>>::new();
            'label_goto__ASSIGN_SUCH_THAT_0: loop {
                for __assign_such_that_0 in (&crate::KipioAccountAuthorizationState::_default::StateCredentials(&state)).iter().cloned() {
                    c = MaybePlacebo::from(__assign_such_that_0.clone());
                    if crate::KipioAccountAuthorizationState::_default::StateCredentials(&state).contains(&c.read()) && crate::KipioAccountCredential::_default::CredentialId(&c.read()) == credentialId.clone() {
                        break 'label_goto__ASSIGN_SUCH_THAT_0;
                    }
                }
                panic!("Halt");
                break;
            };
            c.read()
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
            }))(&int!(0))
        }
        /// policy/PolicyApplicationComposition.dfy(189,3)
        pub fn FindSessionById(state: &Rc<AuthorizationState>, sessionId: &Sequence<u8>) -> Rc<Session> {
            (&({
                let mut state = state.clone();
                let mut sessionId = sessionId.clone();
                Rc::new(move |__let_dummy_4: &DafnyInt| -> Rc<Session>{
            let mut s = MaybePlacebo::<Rc<Session>>::new();
            'label_goto__ASSIGN_SUCH_THAT_0: loop {
                for __assign_such_that_0 in (&crate::KipioAccountAuthorizationState::_default::StateSessions(&state)).iter().cloned() {
                    s = MaybePlacebo::from(__assign_such_that_0.clone());
                    if crate::KipioAccountAuthorizationState::_default::StateSessions(&state).contains(&s.read()) && crate::KipioAccountSession::_default::SessionId(&s.read()) == sessionId.clone() {
                        break 'label_goto__ASSIGN_SUCH_THAT_0;
                    }
                }
                panic!("Halt");
                break;
            };
            s.read()
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
            }))(&int!(0))
        }
        /// policy/PolicyApplicationComposition.dfy(205,3)
        pub fn FindDelegationById(state: &Rc<AuthorizationState>, delegationId: &Sequence<u8>) -> Rc<Delegation> {
            (&({
                let mut state = state.clone();
                let mut delegationId = delegationId.clone();
                Rc::new(move |__let_dummy_5: &DafnyInt| -> Rc<Delegation>{
            let mut d = MaybePlacebo::<Rc<Delegation>>::new();
            'label_goto__ASSIGN_SUCH_THAT_0: loop {
                for __assign_such_that_0 in (&crate::KipioAccountAuthorizationState::_default::StateDelegations(&state)).iter().cloned() {
                    d = MaybePlacebo::from(__assign_such_that_0.clone());
                    if crate::KipioAccountAuthorizationState::_default::StateDelegations(&state).contains(&d.read()) && crate::KipioAccountDelegation::_default::DelegationId(&d.read()) == delegationId.clone() {
                        break 'label_goto__ASSIGN_SUCH_THAT_0;
                    }
                }
                panic!("Halt");
                break;
            };
            d.read()
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
            }))(&int!(0))
        }
        /// policy/PolicyApplicationComposition.dfy(257,3)
        pub fn PolicyEffectTransitionIsWellFormedExec(account: &Rc<Account>, effect: &Rc<PolicyEffect>) -> bool {
            crate::KipioAccountAccount::_default::ValidAccount(account) && crate::KipioAccountAuthorizationStateTransition::_default::PolicyEffectIsConsistent(effect) && crate::KipioAccountAuthorizationStateTransition::_default::PolicyEffectIsApplicable(&crate::KipioAccountAccount::_default::AccountAuthorizationState(account), effect)
        }
        /// policy/PolicyApplicationComposition.dfy(292,3)
        pub fn ApplyPolicyEffect(account: &Rc<Account>, effect: &Rc<PolicyEffect>) -> Rc<Account> {
            let mut _source0: Rc<PolicyEffect> = effect.clone();
            if matches!((&_source0).as_ref(), RevokeCredential{ .. }) {
                let mut ___mcc_h0: Sequence<u8> = _source0.credentialId().clone();
                let mut credentialId: Sequence<u8> = ___mcc_h0.clone();
                let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
                let mut credential: Rc<Credential> = _default::FindCredentialById(&state, &credentialId);
                crate::KipioAccountAccountTransitions::_default::UpdateCredentialStatus(account, &credential, &Rc::new(CredentialStatus::Revoked {}))
            } else {
                if matches!((&_source0).as_ref(), RevokeSession{ .. }) {
                    let mut ___mcc_h1: Sequence<u8> = _source0.sessionId().clone();
                    let mut sessionId: Sequence<u8> = ___mcc_h1.clone();
                    let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
                    let mut session: Rc<Session> = _default::FindSessionById(&state, &sessionId);
                    crate::KipioAccountAccountTransitions::_default::UpdateSessionStatus(account, &session, &Rc::new(SessionStatus::Revoked {}))
                } else {
                    if matches!((&_source0).as_ref(), RevokeDelegation{ .. }) {
                        let mut ___mcc_h2: Sequence<u8> = _source0.delegationId().clone();
                        let mut delegationId: Sequence<u8> = ___mcc_h2.clone();
                        let mut state: Rc<AuthorizationState> = crate::KipioAccountAccount::_default::AccountAuthorizationState(account);
                        let mut delegation: Rc<Delegation> = _default::FindDelegationById(&state, &delegationId);
                        crate::KipioAccountAccountTransitions::_default::UpdateDelegationStatus(account, &delegation, &Rc::new(DelegationStatus::Revoked {}))
                    } else {
                        if matches!((&_source0).as_ref(), DisableCapability{ .. }) {
                            let mut ___mcc_h3: Rc<Capability> = _source0.capability().clone();
                            let mut ___mcc_h4: Rc<OptionalPolicyScope> = _source0.scope().clone();
                            let mut scope: Rc<OptionalPolicyScope> = ___mcc_h4.clone();
                            let mut capability: Rc<Capability> = ___mcc_h3.clone();
                            crate::KipioAccountAccountTransitions::_default::RemoveCapability(account, &capability)
                        } else {
                            if matches!((&_source0).as_ref(), EnableCapability{ .. }) {
                                let mut ___mcc_h5: Rc<Capability> = _source0.capability().clone();
                                let mut ___mcc_h6: Rc<OptionalPolicyScope> = _source0.scope().clone();
                                let mut scope: Rc<OptionalPolicyScope> = ___mcc_h6.clone();
                                let mut capability: Rc<Capability> = ___mcc_h5.clone();
                                crate::KipioAccountAccountTransitions::_default::AddCapability(account, &capability)
                            } else {
                                let mut ___mcc_h7: Rc<Capability> = _source0.capability().clone();
                                let mut ___mcc_h8: Rc<OptionalPolicyScope> = _source0.scope().clone();
                                let mut ___mcc_h9: Rc<Restriction> = _source0.restriction().clone();
                                let mut restriction: Rc<Restriction> = ___mcc_h9.clone();
                                let mut scope: Rc<OptionalPolicyScope> = ___mcc_h8.clone();
                                let mut capability: Rc<Capability> = ___mcc_h7.clone();
                                crate::KipioAccountAccountTransitions::_default::SetRestriction(account, &capability, &scope, &restriction)
                            }
                        }
                    }
                }
            }
        }
        /// policy/PolicyApplicationComposition.dfy(348,3)
        pub fn ApplyPolicyEffectOrIdentity(account: &Rc<Account>, effect: &Rc<PolicyEffect>) -> Rc<Account> {
            if _default::PolicyEffectTransitionIsWellFormedExec(account, effect) {
                _default::ApplyPolicyEffect(account, effect)
            } else {
                account.clone()
            }
        }
        /// policy/PolicyApplicationComposition.dfy(365,3)
        pub fn ApplyPolicySequence(account: &Rc<Account>, effects: &Sequence<Rc<PolicyEffect>>) -> Rc<Account> {
            if effects.cardinality() == int!(0) {
                account.clone()
            } else {
                let mut prefixResult: Rc<Account> = _default::ApplyPolicySequence(account, &effects.take(&(effects.cardinality() - int!(1))));
                _default::ApplyPolicyEffectOrIdentity(&prefixResult, &effects.get(&(effects.cardinality() - int!(1))))
            }
        }
        /// policy/PolicyApplicationComposition.dfy(390,3)
        pub fn PolicyPrefixIsApplicableExec(account: &Rc<Account>, effects: &Sequence<Rc<PolicyEffect>>) -> bool {
            if effects.cardinality() == int!(0) {
                true
            } else {
                let mut prefixAccount: Rc<Account> = _default::ApplyPolicySequence(account, &effects.take(&(effects.cardinality() - int!(1))));
                _default::PolicyPrefixIsApplicableExec(account, &effects.take(&(effects.cardinality() - int!(1)))) && crate::KipioAccountAuthorizationStateTransition::_default::PolicyEffectIsApplicable(&crate::KipioAccountAccount::_default::AccountAuthorizationState(&prefixAccount), &effects.get(&(effects.cardinality() - int!(1))))
            }
        }
        /// policy/PolicyApplicationComposition.dfy(419,3)
        pub fn PolicyIsApplicableAndConsistentExec(account: &Rc<Account>, consumption: &Rc<PolicyConsumption>) -> bool {
            crate::KipioAccountAccount::_default::ValidAccount(account) && crate::KipioAccountPolicyConsumption::_default::ValidPolicyConsumption(consumption) && crate::KipioAccountPolicy::_default::PolicyContainsOnlyValidEffects(&crate::KipioAccountPolicyConsumption::_default::ConsumptionPolicy(consumption)) && !crate::KipioAccountAuthorizationStateTransition::_default::PolicyContainsContradiction(&crate::KipioAccountPolicyConsumption::_default::ConsumptionPolicy(consumption)) && _default::PolicyPrefixIsApplicableExec(account, &crate::KipioAccountPolicyConsumption::_default::ConsumptionPolicyEffects(consumption))
        }
        /// policy/PolicyApplicationComposition.dfy(450,3)
        pub fn ApplyPolicyConsumption(account: &Rc<Account>, consumption: &Rc<PolicyConsumption>) -> Rc<Account> {
            if _default::PolicyIsApplicableAndConsistentExec(account, consumption) {
                _default::ApplyPolicySequence(account, &crate::KipioAccountPolicyConsumption::_default::ConsumptionPolicyEffects(consumption))
            } else {
                account.clone()
            }
        }
    }
}
/// policy/PolicyConsumption.dfy(115,1)
pub mod KipioAccountPolicyConsumption {
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountPolicy::Policy;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect;
    pub use ::dafny_runtime::_System::nat;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// policy/PolicyConsumption.dfy(147,3)
        pub fn ConsumptionPolicy(consumption: &Rc<PolicyConsumption>) -> Rc<Policy> {
            consumption.policy().clone()
        }
        /// policy/PolicyConsumption.dfy(164,3)
        pub fn ConsumptionPolicyEffects(consumption: &Rc<PolicyConsumption>) -> Sequence<Rc<PolicyEffect>> {
            crate::KipioAccountPolicy::_default::PolicyEffects(&_default::ConsumptionPolicy(consumption))
        }
        /// policy/PolicyConsumption.dfy(176,3)
        pub fn ConsumptionEffectCount(consumption: &Rc<PolicyConsumption>) -> nat {
            _default::ConsumptionPolicyEffects(consumption).cardinality()
        }
        /// policy/PolicyConsumption.dfy(193,3)
        pub fn ConsumptionContainsEffect(consumption: &Rc<PolicyConsumption>, effect: &Rc<PolicyEffect>) -> bool {
            crate::KipioAccountPolicy::_default::PolicyContainsEffect(&_default::ConsumptionPolicy(consumption), effect)
        }
        /// policy/PolicyConsumption.dfy(209,3)
        pub fn ConsumptionEffectAt(consumption: &Rc<PolicyConsumption>, index: &nat) -> Rc<PolicyEffect> {
            _default::ConsumptionPolicyEffects(consumption).get(index)
        }
        /// policy/PolicyConsumption.dfy(228,3)
        pub fn ValidPolicyConsumption(consumption: &Rc<PolicyConsumption>) -> bool {
            crate::KipioAccountPolicy::_default::ValidPolicy(&_default::ConsumptionPolicy(consumption))
        }
        /// policy/PolicyConsumption.dfy(246,3)
        pub fn SamePolicyConsumption(left: &Rc<PolicyConsumption>, right: &Rc<PolicyConsumption>) -> bool {
            _default::ConsumptionPolicy(left) == _default::ConsumptionPolicy(right)
        }
    }

    /// policy/PolicyConsumption.dfy(133,3)
    #[derive(Clone)]
    pub enum PolicyConsumption {
        PolicyConsumption {
            policy: Rc<Policy>
        }
    }

    impl PolicyConsumption {
        /// Returns a borrow of the field policy
        pub fn policy(&self) -> &Rc<Policy> {
            match self {
                PolicyConsumption::PolicyConsumption{policy, } => policy,
            }
        }
    }

    impl Debug
        for PolicyConsumption {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for PolicyConsumption {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                PolicyConsumption::PolicyConsumption{policy, } => {
                    write!(_formatter, "KipioAccountPolicyConsumption.PolicyConsumption.PolicyConsumption(")?;
                    DafnyPrint::fmt_print(policy, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for PolicyConsumption {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (PolicyConsumption::PolicyConsumption{policy, }, PolicyConsumption::PolicyConsumption{policy: _2_policy, }) => {
                    policy == _2_policy
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for PolicyConsumption {}

    impl Hash
        for PolicyConsumption {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                PolicyConsumption::PolicyConsumption{policy, } => {
                    Hash::hash(policy, _state)
                },
            }
        }
    }

    impl AsRef<PolicyConsumption>
        for PolicyConsumption {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// policy/PolicyEffect.dfy(112,1)
pub mod KipioAccountPolicyEffect {
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountPolicyEffect::OptionalPolicyScope::NoScope;
    pub use crate::KipioAccountScope::Scope;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::RevokeCredential;
    pub use ::dafny_runtime::Sequence;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::RevokeSession;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::RevokeDelegation;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::DisableCapability;
    pub use crate::KipioAccountCapability::Capability;
    pub use crate::KipioAccountPolicyEffect::PolicyEffect::EnableCapability;
    pub use crate::KipioAccountRestriction::Restriction;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// policy/PolicyEffect.dfy(186,3)
        pub fn PolicyEffectScope(scope: &Rc<OptionalPolicyScope>) -> Rc<OptionalPolicyScope> {
            scope.clone()
        }
        /// policy/PolicyEffect.dfy(207,3)
        pub fn SamePolicyEffect(left: &Rc<PolicyEffect>, right: &Rc<PolicyEffect>) -> bool {
            left.clone() == right.clone()
        }
        /// policy/PolicyEffect.dfy(259,3)
        pub fn ValidOptionalPolicyScope(scope: &Rc<OptionalPolicyScope>) -> bool {
            let mut _source0: Rc<OptionalPolicyScope> = scope.clone();
            if matches!((&_source0).as_ref(), NoScope{ .. }) {
                true
            } else {
                let mut ___mcc_h0: Rc<Scope> = _source0.scope().clone();
                let mut value: Rc<Scope> = ___mcc_h0.clone();
                crate::KipioAccountScope::_default::ValidScope(&value)
            }
        }
        /// policy/PolicyEffect.dfy(274,3)
        pub fn ValidPolicyEffect(effect: &Rc<PolicyEffect>) -> bool {
            let mut _source0: Rc<PolicyEffect> = effect.clone();
            if matches!((&_source0).as_ref(), RevokeCredential{ .. }) {
                let mut ___mcc_h0: Sequence<u8> = _source0.credentialId().clone();
                let mut credentialId: Sequence<u8> = ___mcc_h0.clone();
                crate::KipioAccountDomainPrimitives::_default::ValidId(&credentialId)
            } else {
                if matches!((&_source0).as_ref(), RevokeSession{ .. }) {
                    let mut ___mcc_h1: Sequence<u8> = _source0.sessionId().clone();
                    let mut sessionId: Sequence<u8> = ___mcc_h1.clone();
                    crate::KipioAccountDomainPrimitives::_default::ValidId(&sessionId)
                } else {
                    if matches!((&_source0).as_ref(), RevokeDelegation{ .. }) {
                        let mut ___mcc_h2: Sequence<u8> = _source0.delegationId().clone();
                        let mut delegationId: Sequence<u8> = ___mcc_h2.clone();
                        crate::KipioAccountDomainPrimitives::_default::ValidId(&delegationId)
                    } else {
                        if matches!((&_source0).as_ref(), DisableCapability{ .. }) {
                            let mut ___mcc_h3: Rc<Capability> = _source0.capability().clone();
                            let mut ___mcc_h4: Rc<OptionalPolicyScope> = _source0.scope().clone();
                            let mut scope: Rc<OptionalPolicyScope> = ___mcc_h4.clone();
                            let mut capability: Rc<Capability> = ___mcc_h3.clone();
                            crate::KipioAccountCapability::_default::ValidCapability(&capability) && _default::ValidOptionalPolicyScope(&scope)
                        } else {
                            if matches!((&_source0).as_ref(), EnableCapability{ .. }) {
                                let mut ___mcc_h5: Rc<Capability> = _source0.capability().clone();
                                let mut ___mcc_h6: Rc<OptionalPolicyScope> = _source0.scope().clone();
                                let mut scope: Rc<OptionalPolicyScope> = ___mcc_h6.clone();
                                let mut capability: Rc<Capability> = ___mcc_h5.clone();
                                crate::KipioAccountCapability::_default::ValidCapability(&capability) && _default::ValidOptionalPolicyScope(&scope)
                            } else {
                                let mut ___mcc_h7: Rc<Capability> = _source0.capability().clone();
                                let mut ___mcc_h8: Rc<OptionalPolicyScope> = _source0.scope().clone();
                                let mut ___mcc_h9: Rc<Restriction> = _source0.restriction().clone();
                                let mut restriction: Rc<Restriction> = ___mcc_h9.clone();
                                let mut scope: Rc<OptionalPolicyScope> = ___mcc_h8.clone();
                                let mut capability: Rc<Capability> = ___mcc_h7.clone();
                                crate::KipioAccountCapability::_default::ValidCapability(&capability) && _default::ValidOptionalPolicyScope(&scope) && crate::KipioAccountRestriction::_default::ValidRestriction(&restriction)
                            }
                        }
                    }
                }
            }
        }
    }

    /// policy/PolicyEffect.dfy(129,3)
    #[derive(Clone)]
    pub enum OptionalPolicyScope {
        NoScope {},
        Scoped {
            scope: Rc<Scope>
        }
    }

    impl OptionalPolicyScope {
        /// Gets the field scope for all enum members which have it
        pub fn scope(&self) -> &Rc<Scope> {
            match self {
                OptionalPolicyScope::NoScope{} => panic!("field does not exist on this variant"),
                OptionalPolicyScope::Scoped{scope, } => scope,
            }
        }
    }

    impl Debug
        for OptionalPolicyScope {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for OptionalPolicyScope {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                OptionalPolicyScope::NoScope{} => {
                    write!(_formatter, "KipioAccountPolicyEffect.OptionalPolicyScope.NoScope")?;
                    Ok(())
                },
                OptionalPolicyScope::Scoped{scope, } => {
                    write!(_formatter, "KipioAccountPolicyEffect.OptionalPolicyScope.Scoped(")?;
                    DafnyPrint::fmt_print(scope, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for OptionalPolicyScope {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (OptionalPolicyScope::NoScope{}, OptionalPolicyScope::NoScope{}) => {
                    true
                },
                (OptionalPolicyScope::Scoped{scope, }, OptionalPolicyScope::Scoped{scope: _2_scope, }) => {
                    scope == _2_scope
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for OptionalPolicyScope {}

    impl Hash
        for OptionalPolicyScope {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                OptionalPolicyScope::NoScope{} => {
                    
                },
                OptionalPolicyScope::Scoped{scope, } => {
                    Hash::hash(scope, _state)
                },
            }
        }
    }

    impl AsRef<OptionalPolicyScope>
        for OptionalPolicyScope {
        fn as_ref(&self) -> &Self {
            self
        }
    }

    /// policy/PolicyEffect.dfy(153,3)
    #[derive(Clone)]
    pub enum PolicyEffect {
        RevokeCredential {
            credentialId: Sequence<u8>
        },
        RevokeSession {
            sessionId: Sequence<u8>
        },
        RevokeDelegation {
            delegationId: Sequence<u8>
        },
        DisableCapability {
            capability: Rc<Capability>,
            scope: Rc<OptionalPolicyScope>
        },
        EnableCapability {
            capability: Rc<Capability>,
            scope: Rc<OptionalPolicyScope>
        },
        ModifyRestriction {
            capability: Rc<Capability>,
            scope: Rc<OptionalPolicyScope>,
            restriction: Rc<Restriction>
        }
    }

    impl PolicyEffect {
        /// Gets the field credentialId for all enum members which have it
        pub fn credentialId(&self) -> &Sequence<u8> {
            match self {
                PolicyEffect::RevokeCredential{credentialId, } => credentialId,
                PolicyEffect::RevokeSession{sessionId, } => panic!("field does not exist on this variant"),
                PolicyEffect::RevokeDelegation{delegationId, } => panic!("field does not exist on this variant"),
                PolicyEffect::DisableCapability{capability, scope, } => panic!("field does not exist on this variant"),
                PolicyEffect::EnableCapability{capability, scope, } => panic!("field does not exist on this variant"),
                PolicyEffect::ModifyRestriction{capability, scope, restriction, } => panic!("field does not exist on this variant"),
            }
        }
        /// Gets the field sessionId for all enum members which have it
        pub fn sessionId(&self) -> &Sequence<u8> {
            match self {
                PolicyEffect::RevokeCredential{credentialId, } => panic!("field does not exist on this variant"),
                PolicyEffect::RevokeSession{sessionId, } => sessionId,
                PolicyEffect::RevokeDelegation{delegationId, } => panic!("field does not exist on this variant"),
                PolicyEffect::DisableCapability{capability, scope, } => panic!("field does not exist on this variant"),
                PolicyEffect::EnableCapability{capability, scope, } => panic!("field does not exist on this variant"),
                PolicyEffect::ModifyRestriction{capability, scope, restriction, } => panic!("field does not exist on this variant"),
            }
        }
        /// Gets the field delegationId for all enum members which have it
        pub fn delegationId(&self) -> &Sequence<u8> {
            match self {
                PolicyEffect::RevokeCredential{credentialId, } => panic!("field does not exist on this variant"),
                PolicyEffect::RevokeSession{sessionId, } => panic!("field does not exist on this variant"),
                PolicyEffect::RevokeDelegation{delegationId, } => delegationId,
                PolicyEffect::DisableCapability{capability, scope, } => panic!("field does not exist on this variant"),
                PolicyEffect::EnableCapability{capability, scope, } => panic!("field does not exist on this variant"),
                PolicyEffect::ModifyRestriction{capability, scope, restriction, } => panic!("field does not exist on this variant"),
            }
        }
        /// Gets the field capability for all enum members which have it
        pub fn capability(&self) -> &Rc<Capability> {
            match self {
                PolicyEffect::RevokeCredential{credentialId, } => panic!("field does not exist on this variant"),
                PolicyEffect::RevokeSession{sessionId, } => panic!("field does not exist on this variant"),
                PolicyEffect::RevokeDelegation{delegationId, } => panic!("field does not exist on this variant"),
                PolicyEffect::DisableCapability{capability, scope, } => capability,
                PolicyEffect::EnableCapability{capability, scope, } => capability,
                PolicyEffect::ModifyRestriction{capability, scope, restriction, } => capability,
            }
        }
        /// Gets the field scope for all enum members which have it
        pub fn scope(&self) -> &Rc<OptionalPolicyScope> {
            match self {
                PolicyEffect::RevokeCredential{credentialId, } => panic!("field does not exist on this variant"),
                PolicyEffect::RevokeSession{sessionId, } => panic!("field does not exist on this variant"),
                PolicyEffect::RevokeDelegation{delegationId, } => panic!("field does not exist on this variant"),
                PolicyEffect::DisableCapability{capability, scope, } => scope,
                PolicyEffect::EnableCapability{capability, scope, } => scope,
                PolicyEffect::ModifyRestriction{capability, scope, restriction, } => scope,
            }
        }
        /// Gets the field restriction for all enum members which have it
        pub fn restriction(&self) -> &Rc<Restriction> {
            match self {
                PolicyEffect::RevokeCredential{credentialId, } => panic!("field does not exist on this variant"),
                PolicyEffect::RevokeSession{sessionId, } => panic!("field does not exist on this variant"),
                PolicyEffect::RevokeDelegation{delegationId, } => panic!("field does not exist on this variant"),
                PolicyEffect::DisableCapability{capability, scope, } => panic!("field does not exist on this variant"),
                PolicyEffect::EnableCapability{capability, scope, } => panic!("field does not exist on this variant"),
                PolicyEffect::ModifyRestriction{capability, scope, restriction, } => restriction,
            }
        }
    }

    impl Debug
        for PolicyEffect {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for PolicyEffect {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                PolicyEffect::RevokeCredential{credentialId, } => {
                    write!(_formatter, "KipioAccountPolicyEffect.PolicyEffect.RevokeCredential(")?;
                    DafnyPrint::fmt_print(credentialId, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
                PolicyEffect::RevokeSession{sessionId, } => {
                    write!(_formatter, "KipioAccountPolicyEffect.PolicyEffect.RevokeSession(")?;
                    DafnyPrint::fmt_print(sessionId, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
                PolicyEffect::RevokeDelegation{delegationId, } => {
                    write!(_formatter, "KipioAccountPolicyEffect.PolicyEffect.RevokeDelegation(")?;
                    DafnyPrint::fmt_print(delegationId, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
                PolicyEffect::DisableCapability{capability, scope, } => {
                    write!(_formatter, "KipioAccountPolicyEffect.PolicyEffect.DisableCapability(")?;
                    DafnyPrint::fmt_print(capability, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(scope, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
                PolicyEffect::EnableCapability{capability, scope, } => {
                    write!(_formatter, "KipioAccountPolicyEffect.PolicyEffect.EnableCapability(")?;
                    DafnyPrint::fmt_print(capability, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(scope, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
                PolicyEffect::ModifyRestriction{capability, scope, restriction, } => {
                    write!(_formatter, "KipioAccountPolicyEffect.PolicyEffect.ModifyRestriction(")?;
                    DafnyPrint::fmt_print(capability, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(scope, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(restriction, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for PolicyEffect {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (PolicyEffect::RevokeCredential{credentialId, }, PolicyEffect::RevokeCredential{credentialId: _2_credentialId, }) => {
                    credentialId == _2_credentialId
                },
                (PolicyEffect::RevokeSession{sessionId, }, PolicyEffect::RevokeSession{sessionId: _2_sessionId, }) => {
                    sessionId == _2_sessionId
                },
                (PolicyEffect::RevokeDelegation{delegationId, }, PolicyEffect::RevokeDelegation{delegationId: _2_delegationId, }) => {
                    delegationId == _2_delegationId
                },
                (PolicyEffect::DisableCapability{capability, scope, }, PolicyEffect::DisableCapability{capability: _2_capability, scope: _2_scope, }) => {
                    capability == _2_capability && scope == _2_scope
                },
                (PolicyEffect::EnableCapability{capability, scope, }, PolicyEffect::EnableCapability{capability: _2_capability, scope: _2_scope, }) => {
                    capability == _2_capability && scope == _2_scope
                },
                (PolicyEffect::ModifyRestriction{capability, scope, restriction, }, PolicyEffect::ModifyRestriction{capability: _2_capability, scope: _2_scope, restriction: _2_restriction, }) => {
                    capability == _2_capability && scope == _2_scope && restriction == _2_restriction
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for PolicyEffect {}

    impl Hash
        for PolicyEffect {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                PolicyEffect::RevokeCredential{credentialId, } => {
                    Hash::hash(credentialId, _state)
                },
                PolicyEffect::RevokeSession{sessionId, } => {
                    Hash::hash(sessionId, _state)
                },
                PolicyEffect::RevokeDelegation{delegationId, } => {
                    Hash::hash(delegationId, _state)
                },
                PolicyEffect::DisableCapability{capability, scope, } => {
                    Hash::hash(capability, _state);
                    Hash::hash(scope, _state)
                },
                PolicyEffect::EnableCapability{capability, scope, } => {
                    Hash::hash(capability, _state);
                    Hash::hash(scope, _state)
                },
                PolicyEffect::ModifyRestriction{capability, scope, restriction, } => {
                    Hash::hash(capability, _state);
                    Hash::hash(scope, _state);
                    Hash::hash(restriction, _state)
                },
            }
        }
    }

    impl AsRef<PolicyEffect>
        for PolicyEffect {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// authorization/Replay.dfy(343,1)
pub mod KipioAccountReplay {
    pub use ::std::rc::Rc;
    pub use crate::KipioAccountAuthorization::Authorization;
    pub use ::dafny_runtime::Sequence;
    pub use ::dafny_runtime::Set;
    pub use ::dafny_runtime::set;

    pub struct _default {}

    impl _default {
        /// authorization/Replay.dfy(385,3)
        pub fn ReplayKeyOfAuthorization(authorization: &Rc<Authorization>) -> Sequence<u8> {
            crate::KipioAccountAuthorization::_default::AuthorizationReplayKey(authorization)
        }
        /// authorization/Replay.dfy(402,3)
        pub fn AuthorizationReplayKeyIsFresh(authorization: &Rc<Authorization>, replayState: &Set<Sequence<u8>>) -> bool {
            !replayState.contains(&_default::ReplayKeyOfAuthorization(authorization))
        }
        /// authorization/Replay.dfy(426,3)
        pub fn ConsumeAuthorizationReplayKey(replayState: &Set<Sequence<u8>>, authorization: &Rc<Authorization>) -> Set<Sequence<u8>> {
            replayState.merge(&set!{_default::ReplayKeyOfAuthorization(authorization)})
        }
    }
}
/// foundation/Restriction.dfy(63,1)
pub mod KipioAccountRestriction {
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::int;
    pub use ::dafny_runtime::Sequence;
    pub use ::dafny_runtime::DafnyChar;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// foundation/Restriction.dfy(98,3)
        pub fn ValidRestrictionKind(kind: &Rc<RestrictionKind>) -> bool {
            int!(0) < kind.name().cardinality()
        }
        /// foundation/Restriction.dfy(107,3)
        pub fn RestrictionKindName(kind: &Rc<RestrictionKind>) -> Sequence<DafnyChar> {
            kind.name().clone()
        }
        /// foundation/Restriction.dfy(157,3)
        pub fn RestrictionKindOf(restriction: &Rc<Restriction>) -> Rc<RestrictionKind> {
            restriction.kind().clone()
        }
        /// foundation/Restriction.dfy(166,3)
        pub fn RestrictionValueOf(restriction: &Rc<Restriction>) -> Sequence<u8> {
            restriction.value().clone()
        }
        /// foundation/Restriction.dfy(180,3)
        pub fn SameRestriction(left: &Rc<Restriction>, right: &Rc<Restriction>) -> bool {
            _default::RestrictionKindOf(left) == _default::RestrictionKindOf(right) && _default::RestrictionValueOf(left) == _default::RestrictionValueOf(right)
        }
        /// foundation/Restriction.dfy(258,3)
        pub fn ValidRestriction(restriction: &Rc<Restriction>) -> bool {
            _default::ValidRestrictionKind(&_default::RestrictionKindOf(restriction))
        }
        /// foundation/Restriction.dfy(290,3)
        pub fn RestrictionDoesNotCreateAuthority(restriction: &Rc<Restriction>) -> bool {
            true
        }
    }

    /// foundation/Restriction.dfy(90,3)
    #[derive(Clone)]
    pub enum RestrictionKind {
        RestrictionKind {
            name: Sequence<DafnyChar>
        }
    }

    impl RestrictionKind {
        /// Returns a borrow of the field name
        pub fn name(&self) -> &Sequence<DafnyChar> {
            match self {
                RestrictionKind::RestrictionKind{name, } => name,
            }
        }
    }

    impl Debug
        for RestrictionKind {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for RestrictionKind {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                RestrictionKind::RestrictionKind{name, } => {
                    write!(_formatter, "KipioAccountRestriction.RestrictionKind.RestrictionKind(")?;
                    DafnyPrint::fmt_print(name, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for RestrictionKind {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (RestrictionKind::RestrictionKind{name, }, RestrictionKind::RestrictionKind{name: _2_name, }) => {
                    name == _2_name
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for RestrictionKind {}

    impl Hash
        for RestrictionKind {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                RestrictionKind::RestrictionKind{name, } => {
                    Hash::hash(name, _state)
                },
            }
        }
    }

    impl AsRef<RestrictionKind>
        for RestrictionKind {
        fn as_ref(&self) -> &Self {
            self
        }
    }

    /// foundation/Restriction.dfy(145,3)
    #[derive(Clone)]
    pub enum Restriction {
        Restriction {
            kind: Rc<RestrictionKind>,
            value: Sequence<u8>
        }
    }

    impl Restriction {
        /// Returns a borrow of the field kind
        pub fn kind(&self) -> &Rc<RestrictionKind> {
            match self {
                Restriction::Restriction{kind, value, } => kind,
            }
        }
        /// Returns a borrow of the field value
        pub fn value(&self) -> &Sequence<u8> {
            match self {
                Restriction::Restriction{kind, value, } => value,
            }
        }
    }

    impl Debug
        for Restriction {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for Restriction {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                Restriction::Restriction{kind, value, } => {
                    write!(_formatter, "KipioAccountRestriction.Restriction.Restriction(")?;
                    DafnyPrint::fmt_print(kind, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(value, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for Restriction {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (Restriction::Restriction{kind, value, }, Restriction::Restriction{kind: _2_kind, value: _2_value, }) => {
                    kind == _2_kind && value == _2_value
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for Restriction {}

    impl Hash
        for Restriction {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                Restriction::Restriction{kind, value, } => {
                    Hash::hash(kind, _state);
                    Hash::hash(value, _state)
                },
            }
        }
    }

    impl AsRef<Restriction>
        for Restriction {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// foundation/Scope.dfy(40,1)
pub mod KipioAccountScope {
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::Sequence;
    pub use ::dafny_runtime::Set;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// foundation/Scope.dfy(67,3)
        pub fn ScopeAtomValue(atom: &Rc<ScopeAtom>) -> Sequence<u8> {
            atom.id().clone()
        }
        /// foundation/Scope.dfy(76,3)
        pub fn ValidScopeAtom(atom: &Rc<ScopeAtom>) -> bool {
            crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::ScopeAtomValue(atom))
        }
        /// foundation/Scope.dfy(130,3)
        pub fn ScopeAtoms(scope: &Rc<Scope>) -> Set<Rc<ScopeAtom>> {
            scope.atoms().clone()
        }
        /// foundation/Scope.dfy(139,3)
        pub fn ScopeContains(scope: &Rc<Scope>, atom: &Rc<ScopeAtom>) -> bool {
            _default::ScopeAtoms(scope).contains(atom)
        }
        /// foundation/Scope.dfy(154,3)
        pub fn ScopesEqualByValue(left: &Rc<Scope>, right: &Rc<Scope>) -> bool {
            _default::ScopeAtoms(left) == _default::ScopeAtoms(right)
        }
        /// foundation/Scope.dfy(236,3)
        pub fn ScopeWithin(inner: &Rc<Scope>, outer: &Rc<Scope>) -> bool {
            _default::ScopeAtoms(inner) <= _default::ScopeAtoms(outer)
        }
        /// foundation/Scope.dfy(287,3)
        pub fn ValidScope(scope: &Rc<Scope>) -> bool {
            (&_default::ScopeAtoms(scope)).iter().all(({
                    let mut scope = scope.clone();
                    Rc::new(move |__forall_var_0: &Rc<ScopeAtom>| -> bool{
            let mut atom: Rc<ScopeAtom> = __forall_var_0.clone();
            !_default::ScopeAtoms(&scope).contains(&atom) || _default::ValidScopeAtom(&atom)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// foundation/Scope.dfy(330,3)
        pub fn SameScopeMeaning(left: &Rc<Scope>, right: &Rc<Scope>) -> bool {
            _default::ScopesEqualByValue(left, right)
        }
    }

    /// foundation/Scope.dfy(60,3)
    #[derive(Clone)]
    pub enum ScopeAtom {
        ScopeAtom {
            id: Sequence<u8>
        }
    }

    impl ScopeAtom {
        /// Returns a borrow of the field id
        pub fn id(&self) -> &Sequence<u8> {
            match self {
                ScopeAtom::ScopeAtom{id, } => id,
            }
        }
    }

    impl Debug
        for ScopeAtom {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for ScopeAtom {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                ScopeAtom::ScopeAtom{id, } => {
                    write!(_formatter, "KipioAccountScope.ScopeAtom.ScopeAtom(")?;
                    DafnyPrint::fmt_print(id, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for ScopeAtom {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (ScopeAtom::ScopeAtom{id, }, ScopeAtom::ScopeAtom{id: _2_id, }) => {
                    id == _2_id
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for ScopeAtom {}

    impl Hash
        for ScopeAtom {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                ScopeAtom::ScopeAtom{id, } => {
                    Hash::hash(id, _state)
                },
            }
        }
    }

    impl AsRef<ScopeAtom>
        for ScopeAtom {
        fn as_ref(&self) -> &Self {
            self
        }
    }

    /// foundation/Scope.dfy(123,3)
    #[derive(Clone)]
    pub enum Scope {
        Scope {
            atoms: Set<Rc<ScopeAtom>>
        }
    }

    impl Scope {
        /// Returns a borrow of the field atoms
        pub fn atoms(&self) -> &Set<Rc<ScopeAtom>> {
            match self {
                Scope::Scope{atoms, } => atoms,
            }
        }
    }

    impl Debug
        for Scope {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for Scope {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                Scope::Scope{atoms, } => {
                    write!(_formatter, "KipioAccountScope.Scope.Scope(")?;
                    DafnyPrint::fmt_print(atoms, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for Scope {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (Scope::Scope{atoms, }, Scope::Scope{atoms: _2_atoms, }) => {
                    atoms == _2_atoms
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for Scope {}

    impl Hash
        for Scope {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                Scope::Scope{atoms, } => {
                    Hash::hash(atoms, _state)
                },
            }
        }
    }

    impl AsRef<Scope>
        for Scope {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// authority/Session.dfy(67,1)
pub mod KipioAccountSession {
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::Sequence;
    pub use ::dafny_runtime::Set;
    pub use crate::KipioAccountCapability::Capability;
    pub use ::dafny_runtime::DafnyInt;
    pub use crate::KipioAccountRestriction::Restriction;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::dafny_runtime::SequenceIter;
    pub use ::dafny_runtime::seq;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// authority/Session.dfy(134,3)
        pub fn SessionId(session: &Rc<Session>) -> Sequence<u8> {
            session.id().clone()
        }
        /// authority/Session.dfy(144,3)
        pub fn SessionCredentialId(session: &Rc<Session>) -> Sequence<u8> {
            session.credentialId().clone()
        }
        /// authority/Session.dfy(154,3)
        pub fn SessionCapabilities(session: &Rc<Session>) -> Set<Rc<Capability>> {
            session.capabilities().clone()
        }
        /// authority/Session.dfy(163,3)
        pub fn SessionValidFrom(session: &Rc<Session>) -> DafnyInt {
            session.validFrom().clone()
        }
        /// authority/Session.dfy(172,3)
        pub fn SessionValidUntil(session: &Rc<Session>) -> DafnyInt {
            session.validUntil().clone()
        }
        /// authority/Session.dfy(181,3)
        pub fn SessionRestrictions(session: &Rc<Session>) -> Set<Rc<Restriction>> {
            session.restrictions().clone()
        }
        /// authority/Session.dfy(190,3)
        pub fn SessionStatusOf(session: &Rc<Session>) -> Rc<SessionStatus> {
            session.status().clone()
        }
        /// authority/Session.dfy(204,3)
        pub fn SessionIsActive(session: &Rc<Session>) -> bool {
            _default::SessionStatusOf(session) == Rc::new(SessionStatus::Active {})
        }
        /// authority/Session.dfy(214,3)
        pub fn SessionIsRevoked(session: &Rc<Session>) -> bool {
            _default::SessionStatusOf(session) == Rc::new(SessionStatus::Revoked {})
        }
        /// authority/Session.dfy(226,3)
        pub fn SessionIsExpiredAt(session: &Rc<Session>, now: &DafnyInt) -> bool {
            _default::SessionValidUntil(session) < now.clone()
        }
        /// authority/Session.dfy(243,3)
        pub fn SessionIsWithinValidityIntervalAt(session: &Rc<Session>, now: &DafnyInt) -> bool {
            _default::SessionValidFrom(session) <= now.clone() && now.clone() <= _default::SessionValidUntil(session)
        }
        /// authority/Session.dfy(264,3)
        pub fn SessionCanCurrentlyExerciseAuthorityAt(session: &Rc<Session>, now: &DafnyInt) -> bool {
            _default::ValidSession(session) && _default::SessionIsActive(session) && _default::SessionIsWithinValidityIntervalAt(session, now)
        }
        /// authority/Session.dfy(297,3)
        pub fn ValidSession(session: &Rc<Session>) -> bool {
            crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::SessionId(session)) && crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::SessionCredentialId(session)) && _default::SessionValidFrom(session) <= _default::SessionValidUntil(session) && (&_default::SessionCapabilities(session)).iter().all(({
                    let mut session = session.clone();
                    Rc::new(move |__forall_var_0: &Rc<Capability>| -> bool{
            let mut capability: Rc<Capability> = __forall_var_0.clone();
            !_default::SessionCapabilities(&session).contains(&capability) || crate::KipioAccountCapability::_default::ValidCapability(&capability)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref()) && (&_default::SessionRestrictions(session)).iter().all(({
                    let mut session = session.clone();
                    Rc::new(move |__forall_var_1: &Rc<Restriction>| -> bool{
            let mut restriction: Rc<Restriction> = __forall_var_1.clone();
            !_default::SessionRestrictions(&session).contains(&restriction) || crate::KipioAccountRestriction::_default::ValidRestriction(&restriction)
        }) as Rc<dyn ::std::ops::Fn(&_) -> _>
                }).as_ref())
        }
        /// authority/Session.dfy(331,3)
        pub fn SameSession(left: &Rc<Session>, right: &Rc<Session>) -> bool {
            _default::SessionId(left) == _default::SessionId(right)
        }
        /// authority/Session.dfy(459,3)
        pub fn SessionAuthorityWithinCredentialAuthority(session: &Rc<Session>, credentialAuthority: &Set<Rc<Capability>>) -> bool {
            _default::SessionCapabilities(session) <= credentialAuthority.clone()
        }
    }

    /// authority/Session.dfy(91,3)
    #[derive(Clone)]
    pub enum SessionStatus {
        Active {},
        Revoked {}
    }

    impl SessionStatus {}

    impl Debug
        for SessionStatus {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for SessionStatus {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                SessionStatus::Active{} => {
                    write!(_formatter, "KipioAccountSession.SessionStatus.Active")?;
                    Ok(())
                },
                SessionStatus::Revoked{} => {
                    write!(_formatter, "KipioAccountSession.SessionStatus.Revoked")?;
                    Ok(())
                },
            }
        }
    }

    impl SessionStatus {
        /// Enumerates all possible values of SessionStatus
        pub fn _AllSingletonConstructors() -> SequenceIter<Rc<SessionStatus>> {
            seq![Rc::new(SessionStatus::Active {}), Rc::new(SessionStatus::Revoked {})].iter()
        }
    }

    impl PartialEq
        for SessionStatus {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (SessionStatus::Active{}, SessionStatus::Active{}) => {
                    true
                },
                (SessionStatus::Revoked{}, SessionStatus::Revoked{}) => {
                    true
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for SessionStatus {}

    impl Hash
        for SessionStatus {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                SessionStatus::Active{} => {
                    
                },
                SessionStatus::Revoked{} => {
                    
                },
            }
        }
    }

    impl AsRef<SessionStatus>
        for SessionStatus {
        fn as_ref(&self) -> &Self {
            self
        }
    }

    /// authority/Session.dfy(117,3)
    #[derive(Clone)]
    pub enum Session {
        Session {
            id: Sequence<u8>,
            credentialId: Sequence<u8>,
            capabilities: Set<Rc<Capability>>,
            validFrom: DafnyInt,
            validUntil: DafnyInt,
            restrictions: Set<Rc<Restriction>>,
            status: Rc<SessionStatus>
        }
    }

    impl Session {
        /// Returns a borrow of the field id
        pub fn id(&self) -> &Sequence<u8> {
            match self {
                Session::Session{id, credentialId, capabilities, validFrom, validUntil, restrictions, status, } => id,
            }
        }
        /// Returns a borrow of the field credentialId
        pub fn credentialId(&self) -> &Sequence<u8> {
            match self {
                Session::Session{id, credentialId, capabilities, validFrom, validUntil, restrictions, status, } => credentialId,
            }
        }
        /// Returns a borrow of the field capabilities
        pub fn capabilities(&self) -> &Set<Rc<Capability>> {
            match self {
                Session::Session{id, credentialId, capabilities, validFrom, validUntil, restrictions, status, } => capabilities,
            }
        }
        /// Returns a borrow of the field validFrom
        pub fn validFrom(&self) -> &DafnyInt {
            match self {
                Session::Session{id, credentialId, capabilities, validFrom, validUntil, restrictions, status, } => validFrom,
            }
        }
        /// Returns a borrow of the field validUntil
        pub fn validUntil(&self) -> &DafnyInt {
            match self {
                Session::Session{id, credentialId, capabilities, validFrom, validUntil, restrictions, status, } => validUntil,
            }
        }
        /// Returns a borrow of the field restrictions
        pub fn restrictions(&self) -> &Set<Rc<Restriction>> {
            match self {
                Session::Session{id, credentialId, capabilities, validFrom, validUntil, restrictions, status, } => restrictions,
            }
        }
        /// Returns a borrow of the field status
        pub fn status(&self) -> &Rc<SessionStatus> {
            match self {
                Session::Session{id, credentialId, capabilities, validFrom, validUntil, restrictions, status, } => status,
            }
        }
    }

    impl Debug
        for Session {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for Session {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                Session::Session{id, credentialId, capabilities, validFrom, validUntil, restrictions, status, } => {
                    write!(_formatter, "KipioAccountSession.Session.Session(")?;
                    DafnyPrint::fmt_print(id, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(credentialId, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(capabilities, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(validFrom, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(validUntil, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(restrictions, _formatter, false)?;
                    write!(_formatter, ", ")?;
                    DafnyPrint::fmt_print(status, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for Session {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (Session::Session{id, credentialId, capabilities, validFrom, validUntil, restrictions, status, }, Session::Session{id: _2_id, credentialId: _2_credentialId, capabilities: _2_capabilities, validFrom: _2_validFrom, validUntil: _2_validUntil, restrictions: _2_restrictions, status: _2_status, }) => {
                    id == _2_id && credentialId == _2_credentialId && capabilities == _2_capabilities && validFrom == _2_validFrom && validUntil == _2_validUntil && restrictions == _2_restrictions && status == _2_status
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for Session {}

    impl Hash
        for Session {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                Session::Session{id, credentialId, capabilities, validFrom, validUntil, restrictions, status, } => {
                    Hash::hash(id, _state);
                    Hash::hash(credentialId, _state);
                    Hash::hash(capabilities, _state);
                    Hash::hash(validFrom, _state);
                    Hash::hash(validUntil, _state);
                    Hash::hash(restrictions, _state);
                    Hash::hash(status, _state)
                },
            }
        }
    }

    impl AsRef<Session>
        for Session {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}
/// foundation/Subject.dfy(38,1)
pub mod KipioAccountSubject {
    pub use ::std::rc::Rc;
    pub use ::dafny_runtime::Sequence;
    pub use ::std::fmt::Debug;
    pub use ::std::fmt::Formatter;
    pub use ::std::fmt::Result;
    pub use ::dafny_runtime::DafnyPrint;
    pub use ::std::cmp::PartialEq;
    pub use ::std::cmp::Eq;
    pub use ::std::hash::Hash;
    pub use ::std::hash::Hasher;
    pub use ::std::convert::AsRef;

    pub struct _default {}

    impl _default {
        /// foundation/Subject.dfy(75,3)
        pub fn SubjectReference(subject: &Rc<Subject>) -> Sequence<u8> {
            subject.reference().clone()
        }
        /// foundation/Subject.dfy(97,3)
        pub fn ValidSubject(subject: &Rc<Subject>) -> bool {
            crate::KipioAccountDomainPrimitives::_default::ValidId(&_default::SubjectReference(subject))
        }
    }

    /// foundation/Subject.dfy(60,3)
    #[derive(Clone)]
    pub enum Subject {
        Subject {
            reference: Sequence<u8>
        }
    }

    impl Subject {
        /// Returns a borrow of the field reference
        pub fn reference(&self) -> &Sequence<u8> {
            match self {
                Subject::Subject{reference, } => reference,
            }
        }
    }

    impl Debug
        for Subject {
        fn fmt(&self, f: &mut Formatter) -> Result {
            DafnyPrint::fmt_print(self, f, true)
        }
    }

    impl DafnyPrint
        for Subject {
        fn fmt_print(&self, _formatter: &mut Formatter, _in_seq: bool) -> std::fmt::Result {
            match self {
                Subject::Subject{reference, } => {
                    write!(_formatter, "KipioAccountSubject.Subject.Subject(")?;
                    DafnyPrint::fmt_print(reference, _formatter, false)?;
                    write!(_formatter, ")")?;
                    Ok(())
                },
            }
        }
    }

    impl PartialEq
        for Subject {
        fn eq(&self, other: &Self) -> bool {
            match (
                    self,
                    other
                ) {
                (Subject::Subject{reference, }, Subject::Subject{reference: _2_reference, }) => {
                    reference == _2_reference
                },
                _ => {
                    false
                },
            }
        }
    }

    impl Eq
        for Subject {}

    impl Hash
        for Subject {
        fn hash<_H: Hasher>(&self, _state: &mut _H) {
            match self {
                Subject::Subject{reference, } => {
                    Hash::hash(reference, _state)
                },
            }
        }
    }

    impl AsRef<Subject>
        for Subject {
        fn as_ref(&self) -> &Self {
            self
        }
    }
}