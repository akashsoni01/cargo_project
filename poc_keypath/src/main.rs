use key_paths_derive::Kp;
use rust_key_paths::KpDynamic;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BigPayload {
    pub enterprise: Option<Enterprise>,
    // Add other top-level fields if needed
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enterprise {
    pub name: Option<String>,
    pub founded: Option<u16>,
    pub is_active: Option<bool>,
    pub tax_id: Option<String>,
    pub headquarters: Option<Headquarters>,
    pub metadata: Option<Metadata>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headquarters {
    pub address: Option<Address>,
    pub facilities: Option<Facilities>,
    pub logistics: Option<Logistics>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: Option<String>,
    pub city: Option<String>,
    pub zip_code: Option<String>,
    pub country: Option<String>,
    pub timezone: Option<String>,
    pub is_main_office: Option<bool>,
    pub contact_phone: Option<String>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Facilities {
    pub warehouses: Option<Vec<Warehouse>>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Warehouse {
    pub id: Option<String>,
    pub capacity: Option<u32>,
    pub is_automated: Option<bool>,
    pub manager: Option<Manager>,
    pub inventory: Option<Inventory>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manager {
    pub name: Option<String>,
    pub employee_id: Option<String>,
    pub contacts: Option<Contacts>,
    pub permissions: Option<Vec<String>>,
    pub active_since: Option<String>,
    pub last_promotion: Option<String>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contacts {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub emergency_contact: Option<EmergencyContact>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmergencyContact {
    pub name: Option<String>,
    pub relation: Option<String>,
    pub phone: Option<Box<String>>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub categories: Option<Categories>,
    pub last_audit: Option<String>,
    pub is_audited: Option<bool>,
    pub audit_score: Option<f64>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Categories {
    pub electronics: Option<ElectronicsCategory>,
    pub clothing: Option<ClothingCategory>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectronicsCategory {
    pub total_units: Option<u32>,
    pub last_restocked: Option<String>,
    pub items: Option<Vec<ElectronicItem>>,
    pub seasonal_demand: Option<HashMap<String, Option<f64>>>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectronicItem {
    pub sku: Option<String>,
    pub price: Option<f64>,
    pub in_stock: Option<bool>,
    pub details: Option<ItemDetails>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetails {
    pub specs: Option<HashMap<String, String>>,
    pub reviews: Option<Vec<Review>>,
    pub discount: Option<f64>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub rating: Option<u8>,
    pub comment: Option<String>,
    pub verified_purchase: Option<bool>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClothingCategory {
    pub total_units: Option<u32>,
    pub last_restocked: Option<String>,
    pub items: Option<Vec<ClothingItem>>,
    pub seasonal_demand: Option<HashMap<String, Option<f64>>>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClothingItem {
    pub sku: Option<String>,
    pub size: Option<String>,
    pub color: Option<String>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logistics {
    pub shipping_zones: Option<HashMap<String, ShippingZone>>,
    pub carriers: Option<Vec<Carrier>>,
    pub handling_fee: Option<f64>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingZone {
    pub code: Option<String>,
    pub rates: Option<HashMap<String, Option<f64>>>,
    pub estimated_days: Option<HashMap<String, Vec<u8>>>,
    pub restrictions: Option<Vec<String>>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Carrier {
    pub name: Option<String>,
    pub is_preferred: Option<bool>,
    pub tracking_url_template: Option<String>,
    pub account_number: Option<String>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub version: Option<String>,
    pub last_deployed: Option<String>,
    pub environment: Option<String>,
    pub config: Option<Config>,
    pub tags: Option<Vec<String>>,
    pub is_test_mode: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Kp, Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub features: Option<HashMap<String, Option<bool>>>,
    pub limits: Option<HashMap<String, serde_json::Value>>,
    pub placeholders: Option<HashMap<String, String>>,
}

// Helper methods for ergonomic access
impl Enterprise {
    pub fn new() -> Self {
        Self::default()
    }

    // Example of safe field access
    pub fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown")
    }

    pub fn get_founded(&self) -> u16 {
        self.founded.unwrap_or(0)
    }
}

impl Warehouse {
    pub fn get_capacity(&self) -> u32 {
        self.capacity.unwrap_or(0)
    }
}

impl ElectronicItem {
    pub fn get_price(&self) -> f64 {
        self.price.unwrap_or(0.0)
    }

    pub fn is_in_stock(&self) -> bool {
        self.in_stock.unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_key_paths::{AccessorTrait, HofTrait, KpTrait};
    use serde_json;

    #[test]
    fn test_nested_option_access_patterns() {
        let json_data = r#"
        {
            "enterprise": {
                "headquarters": {
                    "facilities": {
                        "warehouses": [
                            {
                                "id": "TEST-01",
                                "manager": {
                                    "contacts": {
                                        "emergencyContact": {
                                            "phone": "555-0000"
                                        }
                                    }
                                }
                            }
                        ]
                    }
                }
            }
        }"#;

        let root: BigPayload = serde_json::from_str(json_data).unwrap();

        // BigPayload::enterprise()
        //     .then(Enterprise::headquarters())
        //     .then(Headquarters::facilities())
        //     .then(Facilities::warehouses_at(0))
        //     .then(Warehouse::manager())
        //     .then(Manager::contacts())
        //     .then(Contacts::emergency_contact())
        //     .then(EmergencyContact::phone())
        //     .get(& root).map( |emergency_phone| {
        //         assert_eq!(emergency_phone, &"555-0000".to_string());
        //     });
            
        // root
        //     .enterprise.as_ref()
        //     .and_then(|e| e.headquarters.as_ref())
        //     .and_then(|h| h.facilities.as_ref())
        //     .and_then(|f| f.warehouses.as_ref())
        //     .and_then(|w| w.first())
        //     .and_then(|w| w.manager.as_ref())
        //     .and_then(|m| m.contacts.as_ref())
        //     .and_then(|c| c.emergency_contact.as_ref())
        //     .and_then(|ec| ec.phone.as_ref())
        //     .map(|emergency_phone| {
        //         assert_eq!(emergency_phone, &"555-0000".to_string());
        //     });



            // BigPayload -> enterprise -> headquarters

            // op of a fn became in of anothe fn. we can combine or compose thse two to make a chain or pipeline
            BigPayload::enterprise()
            .then(Enterprise::headquarters())
            .then(Headquarters::facilities())
            .then(Facilities::warehouses_at(0))
            .then(Warehouse::manager())
            .then(Manager::contacts())
            .then(Contacts::emergency_contact())
            .then(EmergencyContact::phone())
            .get(&root).map(|name| {
                println!("name = {}", name);
            });


            // kp.get(& instance1);
            // kp.get(& instance2);
            // kp.get(& instance3);
            // kp.get(& instance4);
            // kp.get(& instance5);
            // kp.get(& instance5);
            // let x = to_string(multi_two(add_one(1)));

            // add_one.then(multi_two).then(to_string).get

            // add_one(3);
            // add_one(5);
            // add_one(6);


             let kp = BigPayload::enterprise()
            .then(Enterprise::headquarters())
            .then(Headquarters::facilities())
            .then(Facilities::warehouses_at(0))
            .then(Warehouse::manager())
            .then(Manager::name());
            println!("size of kp = {:?}", size_of_val(&kp));

            (kp).get(&root).map(|name:&String | {
                println!("name = {}", name);
            }) ;

            println!("size of kp = {:?}", size_of_val(&kp));

        let kp = BigPayload::enterprise()
            .then(Enterprise::headquarters())
            .then(Headquarters::facilities());

    println!("size of kp2 = {:?}", size_of_val(&kp));


    let service = SomeService::new();
    println!("size of dynamic kp ======= {:?}", size_of_val(&service.hot_path_to_emergency_contact));

    let result = service.hot_path_to_emergency_contact.get(& root);
    println!("result from service = {:?}", result);
    }
}

fn add_one(i: i32) -> i32 {
    i + 1
}

fn multi_two(i: i32) -> i32 {
    i * 2
}

fn to_string(i: i32) -> String {
    format!("{}", i)
}

fn main() {}



pub struct SomeService {
    hot_path_to_emergency_contact: KpDynamic<BigPayload, String>
}




impl SomeService {
    fn new() -> Self {
        Self { hot_path_to_emergency_contact:  BigPayload::enterprise()
            .then(Enterprise::headquarters())
            .then(Headquarters::facilities())
            .then(Facilities::warehouses_at(0))
            .then(Warehouse::manager())
            .then(Manager::contacts())
            .then(Contacts::emergency_contact())
            .then(EmergencyContact::phone()).into_dynamic() }
    }

    // fn use_hotpaths(&self) {
    //     self.hot_path_to_emergency_contact.get(todo!());
    //     self.hot_path_to_emergency_contact.get(todo!());
    //     self.hot_path_to_emergency_contact.get(todo!());
    //     self.hot_path_to_emergency_contact.get(todo!());
    // }
}
