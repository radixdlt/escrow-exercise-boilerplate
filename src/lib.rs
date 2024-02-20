use scrypto::prelude::*;



#[derive(ScryptoSbor, Clone)]
pub enum ResourceSpecifier {
    Fungible {
        resource_address: ResourceAddress,
        amount: Decimal
    },
    NonFungible {
        resource_address: ResourceAddress,
        non_fungible_local_id: NonFungibleLocalId
    }
}

impl ResourceSpecifier {

    pub fn get_resource_address(&self) -> ResourceAddress {
        match self {
            Self::Fungible {
                resource_address, ..
            }
            | Self::NonFungible {
                resource_address, ..
            } => *resource_address,
        }
    }
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct EscrowBadge {
    offered_resource: ResourceAddress
}

#[blueprint]
mod escrow {
    struct Escrow {
        requested_resource: ResourceSpecifier,
        offered_resource: Vault,
        requested_resource_vault: Vault,
        escrow_nft: ResourceAddress,
    }

    impl Escrow {

        pub fn instantiate_escrow(
            requested_resource: ResourceSpecifier,
            offered_resource: Bucket
        ) -> (Global<Escrow>, NonFungibleBucket) {

            to_do!();

        }

        pub fn exchange(&mut self, bucket_of_resource: Bucket) -> Bucket {

            to_do!();
        }

        pub fn withdraw_resource(&mut self, escrow_nft: NonFungibleBucket) -> Bucket {

            to_do!();

        }

        pub fn cancel_escrow(&mut self, escrow_nft: NonFungibleBucket) -> Bucket {

            to_do!();

        }
    }
}