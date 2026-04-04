use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enterprise {
    pub name: String,
    pub founded: u16,
    pub is_active: bool,
    pub tax_id: Option<String>,
    pub headquarters: Headquarters,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headquarters {
    pub address: Address,
    pub facilities: Facilities,
    pub logistics: Logistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zip_code: String,
    pub country: String,
    pub timezone: String,
    pub is_main_office: bool,
    pub contact_phone: String, // Empty string allowed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Facilities {
    pub warehouses: Vec<Warehouse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Warehouse {
    pub id: String,
    pub capacity: u32,
    pub is_automated: bool,
    pub manager: Manager,
    pub inventory: Inventory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manager {
    pub name: String,
    pub employee_id: String,
    pub contacts: Contacts,
    pub permissions: Vec<String>,
    pub active_since: String, // Using String for date; could use chrono::NaiveDate
    pub last_promotion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contacts {
    pub email: String,
    pub phone: Option<String>,
    pub emergency_contact: EmergencyContact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmergencyContact {
    pub name: String, // Empty string allowed
    pub relation: String,
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub categories: Categories,
    pub last_audit: String,
    pub is_audited: bool,
    pub audit_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Categories {
    pub electronics: ElectronicsCategory,
    pub clothing: ClothingCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectronicsCategory {
    pub total_units: u32,
    pub last_restocked: String,
    pub items: Vec<ElectronicItem>,
    pub seasonal_demand: HashMap<String, Option<f64>>, // Dictionary with nullable values
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectronicItem {
    pub sku: String,
    pub price: f64,
    pub in_stock: bool,
    pub details: ItemDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetails {
    pub specs: HashMap<String, String>, // Dictionary for specs
    pub reviews: Vec<Review>,
    pub discount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub rating: u8,
    pub comment: String,
    pub verified_purchase: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClothingCategory {
    pub total_units: u32,
    pub last_restocked: String, // Empty string allowed
    pub items: Vec<ClothingItem>,
    pub seasonal_demand: HashMap<String, Option<f64>>, // Empty dict allowed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClothingItem {
    // Placeholder - can be expanded
    pub sku: String,
    pub size: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logistics {
    pub shipping_zones: HashMap<String, ShippingZone>, // Dictionary of zones
    pub carriers: Vec<Carrier>,
    pub handling_fee: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingZone {
    pub code: String,
    pub rates: HashMap<String, Option<f64>>, // Dictionary with nullable values
    pub estimated_days: HashMap<String, Vec<u8>>, // Dictionary with arrays
    pub restrictions: Vec<String>, // Empty array allowed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Carrier {
    pub name: String, // Empty string allowed
    pub is_preferred: bool,
    pub tracking_url_template: Option<String>,
    pub account_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub version: String,
    pub last_deployed: String,
    pub environment: String,
    pub config: Config,
    pub tags: Vec<String>,
    pub is_test_mode: bool,
    pub notes: String, // Empty string allowed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub features: HashMap<String, Option<bool>>, // Dictionary with nullable booleans
    pub limits: HashMap<String, serde_json::Value>, // Mixed types: int, float, bool
    pub placeholders: HashMap<String, String>, // Empty dict allowed
}

// Optional: Implement Default for empty values
impl Default for ClothingCategory {
    fn default() -> Self {
        Self {
            total_units: 0,
            last_restocked: String::new(),
            items: Vec::new(),
            seasonal_demand: HashMap::new(),
        }
    }
}

impl Default for EmergencyContact {
    fn default() -> Self {
        Self {
            name: String::new(),
            relation: String::new(),
            phone: String::new(),
        }
    }
}

// Example usage with serde_json
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_example() {
        let json_data = r#"
        {
          "enterprise": {
            "name": "GlobalMart",
            "founded": 2001,
            "isActive": true,
            "taxId": null,
            "headquarters": {
              "address": {
                "street": "100 Commerce Drive",
                "city": "New York",
                "zipCode": "10001",
                "country": "USA",
                "timezone": "EST",
                "isMainOffice": true,
                "contactPhone": ""
              },
              "facilities": {
                "warehouses": []
              },
              "logistics": {
                "shippingZones": {},
                "carriers": [],
                "handlingFee": 0
              }
            },
            "metadata": {
              "version": "2.1.0",
              "lastDeployed": "2025-03-30T08:00:00Z",
              "environment": "production",
              "config": {
                "features": {},
                "limits": {},
                "placeholders": {}
              },
              "tags": [],
              "isTestMode": false,
              "notes": ""
            }
          }
        }"#;

        let root: HashMap<String, Enterprise> = serde_json::from_str(json_data).unwrap();
        let enterprise = root.get("enterprise").unwrap();
        
        assert_eq!(enterprise.name, "GlobalMart");
        assert_eq!(enterprise.founded, 2001);
        assert!(enterprise.tax_id.is_none());
        assert_eq!(enterprise.headquarters.address.contact_phone, "");
    }

    #[test]
    fn test_serialize_with_nulls() {
        let enterprise = Enterprise {
            name: "TestMart".to_string(),
            founded: 2020,
            is_active: true,
            tax_id: None,
            headquarters: Headquarters {
                address: Address {
                    street: "123 Test St".to_string(),
                    city: "Test City".to_string(),
                    zip_code: "12345".to_string(),
                    country: "USA".to_string(),
                    timezone: "PST".to_string(),
                    is_main_office: true,
                    contact_phone: String::new(),
                },
                facilities: Facilities {
                    warehouses: vec![],
                },
                logistics: Logistics {
                    shipping_zones: HashMap::new(),
                    carriers: vec![],
                    handling_fee: 0.0,
                },
            },
            metadata: Metadata {
                version: "1.0".to_string(),
                last_deployed: "2025-04-03T00:00:00Z".to_string(),
                environment: "dev".to_string(),
                config: Config {
                    features: HashMap::new(),
                    limits: HashMap::new(),
                    placeholders: HashMap::new(),
                },
                tags: vec![],
                is_test_mode: true,
                notes: String::new(),
            },
        };

        let serialized = serde_json::to_string_pretty(&enterprise).unwrap();
        assert!(serialized.contains("null"));
        assert!(serialized.contains("\"\""));
    }
}