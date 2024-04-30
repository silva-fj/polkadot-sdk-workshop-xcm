use crate::parachain::{constants::RelayNetwork, AccountId};
use xcm_builder::{AccountId32Aliases, DescribeAllTerminal, DescribeFamily, HashedDescription};

type LocationToAccountId = (
    HashedDescription<AccountId, DescribeFamily<DescribeAllTerminal>>,
    AccountId32Aliases<RelayNetwork, AccountId>,
);

pub type LocationConverter = LocationToAccountId;
