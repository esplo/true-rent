use std::cmp;

use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::prelude::*;
use web_sys::Document;
use std::convert::TryInto;

use crate::constants::{InputId, UnitId};
use crate::utils::fetch_value;

pub struct RentResult {
    pub lease_period: i32,
    pub total: i32,
    pub monthly: i32,
    pub original_total: i32,
    pub original_monthly: i32,
}


#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct RentItem {
    pub value: i32,
    pub unit: UnitId,
}

impl RentItem {
    fn get(&self, lease_period: i32, contract_period: i32) -> i32 {
        match self.unit {
            UnitId::MonthlyYen => self.value * lease_period,
            UnitId::OneShotYen => self.value,
            UnitId::EveryContractYen => self.value * (1 + (lease_period - 1) / contract_period),
            UnitId::EveryContractUpdateYen => cmp::max(0, self.value * ((lease_period - 1) / contract_period)),
            UnitId::Month => self.value,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RentElements {
    pub rent: RentItem,
    pub management_fee: RentItem,
    pub free_rent_period: RentItem,
    pub guarantee_fee: RentItem,
    pub support_fee: RentItem,
    pub association_membership_fee: RentItem,
    pub key_money: RentItem,
    pub brokerage_fee: RentItem,
    pub guarantee_administrative_fee: RentItem,
    pub insurance_fee: RentItem,
    pub bicycle_space_fee: RentItem,
    pub car_space_fee: RentItem,
    pub key_change_fee: RentItem,
    pub cleaning_fee: RentItem,
    pub contract_update_fee: RentItem,
    pub contract_period: RentItem,
    pub lease_period: RentItem,
}

impl RentElements {
    pub fn to_json_string(&self) -> String {
        format!("{}", json!(self))
    }

    pub fn build(document: &Document) -> Result<Self, JsValue> {
        Ok(Self {
            rent: RentItem {
                value: fetch_value::<i32>(&document, InputId::Rent)?,
                unit: fetch_value::<i32>(&document, InputId::RentUnit)?.try_into()?,
            },
            management_fee: RentItem {
                value: fetch_value::<i32>(&document, InputId::ManagementFee)?,
                unit: fetch_value::<i32>(&document, InputId::ManagementFeeUnit)?.try_into()?,
            },
            free_rent_period: RentItem {
                value: fetch_value::<i32>(&document, InputId::FreeRentPeriod)?,
                unit: fetch_value::<i32>(&document, InputId::FreeRentPeriodUnit)?.try_into()?,
            },
            guarantee_fee: RentItem {
                value: fetch_value::<i32>(&document, InputId::GuaranteeFee)?,
                unit: fetch_value::<i32>(&document, InputId::GuaranteeFeeUnit)?.try_into()?,
            },
            support_fee: RentItem {
                value: fetch_value::<i32>(&document, InputId::SupportFee)?,
                unit: fetch_value::<i32>(&document, InputId::SupportFeeUnit)?.try_into()?,
            },
            association_membership_fee: RentItem {
                value: fetch_value::<i32>(&document, InputId::AssociationMembershipFee)?,
                unit: fetch_value::<i32>(&document, InputId::AssociationMembershipFeeUnit)?.try_into()?,
            },
            key_money: RentItem {
                value: fetch_value::<i32>(&document, InputId::KeyMoney)?,
                unit: fetch_value::<i32>(&document, InputId::KeyMoneyUnit)?.try_into()?,
            },
            brokerage_fee: RentItem {
                value: fetch_value::<i32>(&document, InputId::BrokerageFee)?,
                unit: fetch_value::<i32>(&document, InputId::BrokerageFeeUnit)?.try_into()?,
            },
            guarantee_administrative_fee: RentItem {
                value: fetch_value::<i32>(&document, InputId::GuaranteeAdministrativeFee)?,
                unit: fetch_value::<i32>(&document, InputId::GuaranteeAdministrativeFeeUnit)?.try_into()?,
            },
            insurance_fee: RentItem {
                value: fetch_value::<i32>(&document, InputId::InsuranceFee)?,
                unit: fetch_value::<i32>(&document, InputId::InsuranceFeeUnit)?.try_into()?,
            },
            bicycle_space_fee: RentItem {
                value: fetch_value::<i32>(&document, InputId::BicycleSpaceFee)?,
                unit: fetch_value::<i32>(&document, InputId::BicycleSpaceFeeUnit)?.try_into()?,
            },
            car_space_fee: RentItem {
                value: fetch_value::<i32>(&document, InputId::CarSpaceFee)?,
                unit: fetch_value::<i32>(&document, InputId::CarSpaceFeeUnit)?.try_into()?,
            },
            key_change_fee: RentItem {
                value: fetch_value::<i32>(&document, InputId::KeyChangeFee)?,
                unit: fetch_value::<i32>(&document, InputId::KeyChangeFeeUnit)?.try_into()?,
            },
            cleaning_fee: RentItem {
                value: fetch_value::<i32>(&document, InputId::CleaningFee)?,
                unit: fetch_value::<i32>(&document, InputId::CleaningFeeUnit)?.try_into()?,
            },
            contract_update_fee: RentItem {
                value: fetch_value::<i32>(&document, InputId::ContractUpdateFee)?,
                unit: fetch_value::<i32>(&document, InputId::ContractUpdateFeeUnit)?.try_into()?,
            },
            contract_period: RentItem {
                value: fetch_value::<i32>(&document, InputId::ContractPeriod)?,
                unit: fetch_value::<i32>(&document, InputId::ContractPeriodUnit)?.try_into()?,
            },
            lease_period: RentItem {
                value: fetch_value::<i32>(&document, InputId::LeasePeriod)?,
                unit: fetch_value::<i32>(&document, InputId::LeasePeriodUnit)?.try_into()?,
            },
        })
    }
}

pub struct RentCalculator {}

impl RentCalculator {
    fn total(&self, e: &RentElements) -> i32 {
        let lease_period = e.lease_period.get(0, 0);
        let contract_period = e.contract_period.get(0, 0);

        // for simplicity
        let cget = |lease_period: i32, contract_period: i32| -> Box<dyn Fn(&RentItem) -> i32> {
            Box::new(move |x| x.get(lease_period, contract_period))
        };

        let f = cget(lease_period, contract_period);
        let rent_sum = f(&e.rent);
        let rent = rent_sum / lease_period;

        rent_sum
            + f(&e.management_fee)
            + f(&e.guarantee_fee)
            + f(&e.association_membership_fee)
            - (rent * f(&e.free_rent_period))
            + f(&e.key_money)
            + f(&e.brokerage_fee)
            + f(&e.guarantee_administrative_fee)
            + f(&e.insurance_fee)
            + f(&e.bicycle_space_fee)
            + f(&e.car_space_fee)
            + f(&e.key_change_fee)
            + f(&e.cleaning_fee)
            + f(&e.contract_update_fee)
    }
    pub fn calc(&self, e: RentElements) -> Result<RentResult, JsValue> {
        let total = self.total(&e);

        let lease_period = e.lease_period.get(0, 0);
        let contract_period = e.contract_period.get(0, 0);

        // for simplicity
        let cget = |lease_period: i32, contract_period: i32| -> Box<dyn Fn(&RentItem) -> i32> {
            Box::new(move |x| x.get(lease_period, contract_period))
        };
        let f = cget(lease_period, contract_period);

        let original_total = f(&e.rent) + f(&e.management_fee);
        let result = RentResult {
            lease_period,
            total,
            monthly: total / lease_period,
            original_total,
            original_monthly: original_total / lease_period,
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::calculator::RentItem;
    use crate::constants::UnitId;

    #[test]
    fn every_contract_update() {
        let e = RentItem { value: 111, unit: UnitId::EveryContractUpdateYen };
        assert_eq!(e.get(1, 1), 0);
        assert_eq!(e.get(10, 11), 0);
        assert_eq!(e.get(11, 11), 0);
        assert_eq!(e.get(12, 11), 111);
        assert_eq!(e.get(22, 11), 111);
        assert_eq!(e.get(23, 11), 222);
    }

    #[test]
    fn one_shot() {
        let e = RentItem { value: 111, unit: UnitId::OneShotYen };
        assert_eq!(e.get(1, 1), 111);
        assert_eq!(e.get(10, 11), 111);
        assert_eq!(e.get(11, 11), 111);
        assert_eq!(e.get(12, 11), 111);
        assert_eq!(e.get(22, 11), 111);
        assert_eq!(e.get(23, 11), 111);
    }

    #[test]
    fn every_contract() {
        let e = RentItem { value: 111, unit: UnitId::EveryContractYen };
        assert_eq!(e.get(1, 1), 111);
        assert_eq!(e.get(10, 11), 111);
        assert_eq!(e.get(11, 11), 111);
        assert_eq!(e.get(12, 11), 222);
        assert_eq!(e.get(22, 11), 222);
        assert_eq!(e.get(23, 11), 333);
    }
}