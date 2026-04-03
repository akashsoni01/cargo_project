use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enterprise {
    pub name: Option<String>,
    pub founded: Option<u16>,
    pub is_active: Option<bool>,
    pub tax_id: Option<String>,
    pub headquarters: Option<Headquarters>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headquarters {
    pub address: Option<Address>,
    pub facilities: Option<Facilities>,
    pub logistics: Option<Logistics>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Facilities {
    pub warehouses: Option<Vec<Warehouse>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Warehouse {
    pub id: Option<String>,
    pub capacity: Option<u32>,
    pub is_automated: Option<bool>,
    pub manager: Option<Manager>,
    pub inventory: Option<Inventory>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manager {
    pub name: Option<String>,
    pub employee_id: Option<String>,
    pub contacts: Option<Contacts>,
    pub permissions: Option<Vec<String>>,
    pub active_since: Option<String>,
    pub last_promotion: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contacts {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub emergency_contact: Option<EmergencyContact>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmergencyContact {
    pub name: Option<String>,
    pub relation: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub categories: Option<Categories>,
    pub last_audit: Option<String>,
    pub is_audited: Option<bool>,
    pub audit_score: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Categories {
    pub electronics: Option<ElectronicsCategory>,
    pub clothing: Option<ClothingCategory>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectronicsCategory {
    pub total_units: Option<u32>,
    pub last_restocked: Option<String>,
    pub items: Option<Vec<ElectronicItem>>,
    pub seasonal_demand: Option<HashMap<String, Option<f64>>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectronicItem {
    pub sku: Option<String>,
    pub price: Option<f64>,
    pub in_stock: Option<bool>,
    pub details: Option<ItemDetails>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetails {
    pub specs: Option<HashMap<String, String>>,
    pub reviews: Option<Vec<Review>>,
    pub discount: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub rating: Option<u8>,
    pub comment: Option<String>,
    pub verified_purchase: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClothingCategory {
    pub total_units: Option<u32>,
    pub last_restocked: Option<String>,
    pub items: Option<Vec<ClothingItem>>,
    pub seasonal_demand: Option<HashMap<String, Option<f64>>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClothingItem {
    pub sku: Option<String>,
    pub size: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logistics {
    pub shipping_zones: Option<HashMap<String, ShippingZone>>,
    pub carriers: Option<Vec<Carrier>>,
    pub handling_fee: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingZone {
    pub code: Option<String>,
    pub rates: Option<HashMap<String, Option<f64>>>,
    pub estimated_days: Option<HashMap<String, Vec<u8>>>,
    pub restrictions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Carrier {
    pub name: Option<String>,
    pub is_preferred: Option<bool>,
    pub tracking_url_template: Option<String>,
    pub account_number: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    use serde_json;

    #[test]
    fn test_deserialize_complete_json() {
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
                        "warehouses": [
                            {
                                "id": "WH-NY-01",
                                "capacity": 50000,
                                "isAutomated": false,
                                "manager": {
                                    "name": "Jane Smith",
                                    "employeeId": "E8923",
                                    "contacts": {
                                        "email": "jane@globalmart.com",
                                        "phone": null,
                                        "emergencyContact": {
                                            "name": "",
                                            "relation": "spouse",
                                            "phone": "555-1234"
                                        }
                                    },
                                    "permissions": ["manage_inventory", "view_reports"],
                                    "activeSince": "2018-06-01",
                                    "lastPromotion": null
                                },
                                "inventory": {
                                    "categories": {
                                        "electronics": {
                                            "totalUnits": 12450,
                                            "lastRestocked": "2025-03-20",
                                            "items": [
                                                {
                                                    "sku": "LAP-1001",
                                                    "price": 899.99,
                                                    "inStock": true,
                                                    "details": {
                                                        "specs": {
                                                            "ram": "16GB",
                                                            "storage": "512GB SSD"
                                                        },
                                                        "reviews": [],
                                                        "discount": null
                                                    }
                                                }
                                            ],
                                            "seasonalDemand": {
                                                "q1": 0.15,
                                                "q2": null
                                            }
                                        },
                                        "clothing": {
                                            "totalUnits": 0,
                                            "lastRestocked": "",
                                            "items": [],
                                            "seasonalDemand": {}
                                        }
                                    },
                                    "lastAudit": "2025-03-28T14:30:00Z",
                                    "isAudited": true,
                                    "auditScore": null
                                }
                            }
                        ]
                    },
                    "logistics": {
                        "shippingZones": {
                            "domestic": {
                                "code": "DOM",
                                "rates": {
                                    "standard": 5.99,
                                    "express": 14.99
                                },
                                "estimatedDays": {
                                    "standard": [3, 7],
                                    "express": [1, 3]
                                },
                                "restrictions": []
                            }
                        },
                        "carriers": [
                            {
                                "name": "FastShip",
                                "isPreferred": true,
                                "trackingUrlTemplate": "https://fastship.com/track/{trackingId}",
                                "accountNumber": ""
                            }
                        ],
                        "handlingFee": 0
                    }
                },
                "metadata": {
                    "version": "2.1.0",
                    "lastDeployed": "2025-03-30T08:00:00Z",
                    "environment": "production",
                    "config": {
                        "features": {
                            "chatSupport": true,
                            "recommendations": false
                        },
                        "limits": {
                            "maxCartItems": 99,
                            "maxDiscountPercent": 0.25
                        },
                        "placeholders": {}
                    },
                    "tags": ["ecommerce", "fulfillment"],
                    "isTestMode": false,
                    "notes": ""
                }
            }
        }"#;

        let root: HashMap<String, Enterprise> = serde_json::from_str(json_data).unwrap();
        let enterprise = root.get("enterprise").unwrap();
        
        assert_eq!(enterprise.get_name(), "GlobalMart");
        assert_eq!(enterprise.get_founded(), 2001);
        
        // Safe navigation using Option
        if let Some(headquarters) = &enterprise.headquarters {
            if let Some(address) = &headquarters.address {
                assert_eq!(address.street.as_deref(), Some("100 Commerce Drive"));
                assert_eq!(address.contact_phone.as_deref(), Some(""));
            }
        }
    }

    #[test]
    fn test_deserialize_minimal_json() {
        let json_data = r#"
        {
            "enterprise": {
                "name": "MinimalMart"
            }
        }"#;

        let root: HashMap<String, Enterprise> = serde_json::from_str(json_data).unwrap();
        let enterprise = root.get("enterprise").unwrap();
        
        assert_eq!(enterprise.get_name(), "MinimalMart");
        assert_eq!(enterprise.get_founded(), 0); // Default value
        assert!(enterprise.headquarters.is_none());
        assert!(enterprise.metadata.is_none());
    }

    #[test]
    fn test_deserialize_empty_json() {
        let json_data = r#"{}"#;
        let root: HashMap<String, Enterprise> = serde_json::from_str(json_data).unwrap();
        assert!(root.is_empty());
    }

    #[test]
    fn test_serialize_with_defaults() {
        let enterprise = Enterprise::default();
        let serialized = serde_json::to_string(&enterprise).unwrap();
        
        // All fields should be present as null or omitted based on serde defaults
        println!("Serialized: {}", serialized);
        assert!(serialized.contains("null") || serialized == "{}");
    }

    #[test]
    fn test_partial_update() {
        let json_data = r#"
        {
            "enterprise": {
                "name": "PartialMart",
                "headquarters": {
                    "address": {
                        "city": "Chicago"
                    }
                }
            }
        }"#;

        let root: HashMap<String, Enterprise> = serde_json::from_str(json_data).unwrap();
        let enterprise = root.get("enterprise").unwrap();
        
        assert_eq!(enterprise.get_name(), "PartialMart");
        assert!(enterprise.founded.is_none());
        
        if let Some(headquarters) = &enterprise.headquarters {
            if let Some(address) = &headquarters.address {
                assert_eq!(address.city.as_deref(), Some("Chicago"));
                assert!(address.street.is_none());
            }
        }
    }

    #[test]
    fn test_handle_null_values() {
        let json_data = r#"
        {
            "enterprise": {
                "name": null,
                "founded": null,
                "headquarters": null
            }
        }"#;

        let root: HashMap<String, Enterprise> = serde_json::from_str(json_data).unwrap();
        let enterprise = root.get("enterprise").unwrap();
        
        assert_eq!(enterprise.get_name(), "Unknown"); // Default from helper
        assert!(enterprise.name.is_none());
        assert!(enterprise.headquarters.is_none());
    }

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

        let root: HashMap<String, Enterprise> = serde_json::from_str(json_data).unwrap();
        
        // Using if let chain (Rust 1.65+)
        let emergency_phone = root
            .get("enterprise")
            .and_then(|e| e.headquarters.as_ref())
            .and_then(|h| h.facilities.as_ref())
            .and_then(|f| f.warehouses.as_ref())
            .and_then(|w| w.first())
            .and_then(|w| w.manager.as_ref())
            .and_then(|m| m.contacts.as_ref())
            .and_then(|c| c.emergency_contact.as_ref())
            .and_then(|ec| ec.phone.as_ref());
        
        assert_eq!(emergency_phone, Some(&"555-0000".to_string()));
    }
}

// Convenience macro for building nested Option structs
#[macro_export]
macro_rules! some_vec {
    ($($x:expr),* $(,)?) => {
        Some(vec![$($x),*])
    };
}

// Helper trait for converting to Option
pub trait Optional {
    fn optional(self) -> Option<Self> where Self: Sized;
}

impl<T> Optional for T {
    fn optional(self) -> Option<Self> {
        Some(self)
    }
}