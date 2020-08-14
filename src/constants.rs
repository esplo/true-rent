use strum_macros::{Display, EnumIter};
use wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use strum::IntoEnumIterator;


#[derive(Display, Debug)]
pub enum InputId {
    Rent,
    RentUnit,
    ManagementFee,
    ManagementFeeUnit,
    FreeRentPeriod,
    FreeRentPeriodUnit,
    SupportFee,
    SupportFeeUnit,
    GuaranteeFee,
    GuaranteeFeeUnit,
    AssociationMembershipFee,
    AssociationMembershipFeeUnit,
    KeyMoney,
    KeyMoneyUnit,
    BrokerageFee,
    BrokerageFeeUnit,
    GuaranteeAdministrativeFee,
    GuaranteeAdministrativeFeeUnit,
    InsuranceFee,
    InsuranceFeeUnit,
    BicycleSpaceFee,
    BicycleSpaceFeeUnit,
    CarSpaceFee,
    CarSpaceFeeUnit,
    KeyChangeFee,
    KeyChangeFeeUnit,
    CleaningFee,
    CleaningFeeUnit,
    ContractUpdateFee,
    ContractUpdateFeeUnit,

    ContractPeriod,
    ContractPeriodUnit,
    LeasePeriod,
    LeasePeriodUnit,

    CalcResultTitle,
    CalcResultBody,
    JsonExportTextArea,
    JsonImportTextArea,
}


#[derive(Serialize, Deserialize, Display, EnumIter, Debug, Copy, Clone)]
pub enum UnitId {
    MonthlyYen = 0,
    OneShotYen = 1,
    EveryContractYen = 2,
    EveryContractUpdateYen = 3,
    Month = 4,
}

impl UnitId {
    pub fn to_string(&self) -> &str {
        match self {
            UnitId::MonthlyYen => "円 / 月",
            UnitId::OneShotYen => "円（初回のみ）",
            UnitId::EveryContractYen => "円 / 契約",
            UnitId::EveryContractUpdateYen => "円 / 契約更新",
            UnitId::Month => "か月",
        }
    }
}

impl TryFrom<i32> for UnitId {
    type Error = JsValue;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let mut mt = UnitId::iter().filter(|x| *x as i32 == value);
        let first_one = mt.next();

        first_one.and_then(|e| if mt.next().is_some() { None } else { Some(e) })
            .ok_or_else(|| JsValue::from_str("Invalid integer for UnitId"))
    }
}