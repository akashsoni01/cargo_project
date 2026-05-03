# Cassandra POC - Multi-Repository Demo

A comprehensive Proof of Concept demonstrating resilient Cassandra database operations using Rust with ScyllaDB driver, featuring multiple repositories with separate keyspaces, CRUD operations, and fault-tolerant connection handling.

## Features

### 🚀 Core Features

1. **Resilient Connection Management**
   - Application starts even if Cassandra is unavailable
   - Automatic connection polling every 5 seconds
   - Graceful error handling without crashes
   - Automatic reconnection when database becomes available
   - Connection isolation: problems in one repository don't affect others

2. **Repository Pattern with Trait-Based Design**
   - **Repository Trait**: Common interface with default method implementations
   - **UserRepository**: Manages user data in `user_keyspace`
   - **ProductRepository**: Manages product data in `product_keyspace`
   - Each repository uses its own dedicated keyspace
   - Independent connection management per repository
   - Shared common functionality through trait defaults

3. **Complete CRUD Operations**
   - **CREATE**: Insert new records with type-safe models
   - **READ**: Query by ID, email/name, or retrieve all records
   - **UPDATE**: Update specific fields with optional parameters
   - **DELETE**: Remove records by ID or email/name
   - All operations include automatic connection checking

4. **Prepared Statements**
   - All queries use prepared statements for optimal performance
   - Automatic statement caching and reuse
   - Graceful failure handling for statement preparation

5. **Keyspace Management**
   - Automatic keyspace creation if it doesn't exist
   - Table creation with proper schema definitions
   - Initialization methods for repository setup
   - Configuration validation before initialization

6. **Type-Safe Models**
   - **User Model**: `id`, `name`, `email`, `password`
   - **Product Model**: `id`, `name`, `description`, `price`, `stock`
   - UUID-based primary keys
   - Serialization support (Serialize/Deserialize)

7. **Common Repository Methods (Trait Defaults)**
   - Connection management: `wait_for_connection()`, `ensure_connected()`
   - Retry logic: `retry_operation()` with exponential backoff
   - Health checks: `health_check()`, `is_ready()`, `status()`
   - Configuration validation: `validate_config()`
   - Initialization variants: `initialize_with_validation()`, `initialize_with_retry()`

### 📦 Project Structure

```
src/
├── main.rs                 # Application entry point with CRUD examples
├── cassandra_manager.rs    # Core Cassandra connection and query manager
├── repository.rs            # Repository trait with default implementations
├── user.rs                 # User model definition
├── product.rs              # Product model definition
├── user_repository.rs      # User CRUD repository (user_keyspace)
└── product_repository.rs   # Product CRUD repository (product_keyspace)
```

### 🗄️ Database Schema

#### User Keyspace (`user_keyspace`)
```cql
CREATE KEYSPACE IF NOT EXISTS user_keyspace WITH replication = {
    'class': 'SimpleStrategy',
    'replication_factor': 1
};

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    name TEXT,
    email TEXT,
    password TEXT
);
```

#### Product Keyspace (`product_keyspace`)
```cql
CREATE KEYSPACE IF NOT EXISTS product_keyspace WITH replication = {
    'class': 'SimpleStrategy',
    'replication_factor': 1
};

CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY,
    name TEXT,
    description TEXT,
    price DOUBLE,
    stock INT
);
```

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable version)
- Docker and Docker Compose
- Cassandra/ScyllaDB (via Docker)

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd poc_cassandra
   ```

2. **Start Cassandra using Docker Compose**
   ```bash
   docker-compose up -d cassandra
   ```

3. **Wait for Cassandra to be ready** (usually 30-60 seconds)
   ```bash
   docker-compose logs -f cassandra
   # Wait for "Starting listening for CQL clients" message
   ```

4. **Build and run the application**
   ```bash
   cargo build
   cargo run
   ```

### Configuration

The application connects to Cassandra at `127.0.0.1:9042` by default. To change this, modify the connection string in `src/main.rs`:

```rust
let user_repo = UserRepository::new("your-host:9042".to_string());
let product_repo = ProductRepository::new("your-host:9042".to_string());
```

## 📖 Usage Examples

### User Repository

```rust
use repository::Repository;
use user_repository::UserRepository;
use user::User;

// Create repository
let user_repo = UserRepository::new("127.0.0.1:9042".to_string());

// Validate configuration (using trait method)
user_repo.validate_config()?;

// Initialize with validation (using trait method)
user_repo.initialize_with_validation().await?;

// Check health status (using trait method)
let (healthy, message) = user_repo.health_check().await;
user_repo.log_status().await;

// CREATE (automatically checks connection)
let user = User::new(
    "John Doe".to_string(),
    "john@example.com".to_string(),
    "password123".to_string(),
);
user_repo.create(&user).await?;

// READ by ID (automatically checks connection)
let user = user_repo.get_by_id(user.id).await?;

// READ by email (automatically checks connection)
let user = user_repo.get_by_email("john@example.com").await?;

// READ all (automatically checks connection)
let users = user_repo.get_all().await?;

// UPDATE (automatically checks connection)
user_repo.update(user.id, None, Some("newemail@example.com"), None).await?;

// DELETE (automatically checks connection)
user_repo.delete(user.id).await?;
```

### Product Repository

```rust
use repository::Repository;
use product_repository::ProductRepository;
use product::Product;

// Create repository
let product_repo = ProductRepository::new("127.0.0.1:9042".to_string());

// Validate configuration (using trait method)
product_repo.validate_config()?;

// Initialize with validation (using trait method)
product_repo.initialize_with_validation().await?;

// Check health status (using trait method)
let (healthy, message) = product_repo.health_check().await;
product_repo.log_status().await;

// CREATE (automatically checks connection)
let product = Product::new(
    "Laptop".to_string(),
    "High-performance laptop".to_string(),
    1299.99,
    10,
);
product_repo.create(&product).await?;

// READ by ID (automatically checks connection)
let product = product_repo.get_by_id(product.id).await?;

// READ by name (automatically checks connection)
let product = product_repo.get_by_name("Laptop").await?;

// READ all (automatically checks connection)
let products = product_repo.get_all().await?;

// UPDATE (automatically checks connection)
product_repo.update(product.id, None, None, Some(1199.99), Some(15)).await?;

// DELETE (automatically checks connection)
product_repo.delete(product.id).await?;
```

### Using Repository Trait Methods

```rust
use repository::Repository;

// Wait for connection with custom timeout
if repo.wait_for_connection(60, 2).await {
    println!("Connected!");
}

// Retry an operation with exponential backoff
let result = Repository::retry_operation(
    || async { repo.create(&item).await },
    3,  // max retries
    1000, // initial delay in ms
).await?;

// Check repository health
let (healthy, message) = repo.health_check().await;
if healthy {
    println!("Repository is healthy: {}", message);
}

// Get formatted status
let status = repo.status().await;
println!("{}", status);

// Initialize with retry logic
repo.initialize_with_retry(3, 1000).await?;
```

## 🔧 Architecture Details

### Repository Trait

The `Repository` trait provides common functionality through default method implementations:

**Connection Management:**
- `wait_for_connection()` - Wait for connection with custom timeout
- `wait_for_connection_default()` - Wait with default timeout (30 attempts, 1 second)
- `ensure_connected()` - Ensure connection before operations

**Retry Logic:**
- `retry_operation()` - Retry with exponential backoff
- `retry_operation_fixed()` - Retry with fixed delay

**Health Checks:**
- `is_ready()` - Check if repository is ready
- `health_check()` - Detailed health status
- `status()` - Formatted status string
- `log_status()` - Log current status
- `log_health()` - Log health check results

**Configuration Validation:**
- `validate_config()` - Validate keyspace name format
- `initialize_with_validation()` - Validate then initialize
- `initialize_with_retry()` - Initialize with retry logic
- `initialize_with_validation_and_retry()` - Combined approach

### CassandraManager

The `CassandraManager` provides:
- Resilient connection handling with automatic retry
- Prepared statement management and caching
- Keyspace and table creation utilities
- Generic CRUD operations (insert, select, update, delete)
- Connection status monitoring

### Repository Pattern

Each repository:
- Implements the `Repository` trait for common functionality
- Encapsulates domain-specific operations
- Uses its own keyspace for data isolation
- Provides type-safe CRUD methods
- Handles initialization and schema setup
- Manages its own independent CassandraManager instance
- All CRUD operations automatically check connection before execution

### Connection Resilience & Isolation

The application demonstrates:
- **Startup Resilience**: Application starts even if Cassandra is down
- **Runtime Resilience**: Operations fail gracefully without crashing
- **Automatic Recovery**: Automatic reconnection when database becomes available
- **Connection Polling**: Checks connection status every 5 seconds
- **Repository Isolation**: Connection problems in one repository don't affect others
- **Independent Managers**: Each repository has its own CassandraManager instance

## 🧪 Testing Connection Resilience

The application includes built-in resilience testing:

1. **Start the application** - It will start even if Cassandra is down
2. **Stop Cassandra** - `docker-compose stop cassandra`
3. **Observe** - Operations fail gracefully, connection status updates
4. **Restart Cassandra** - `docker-compose start cassandra`
5. **Verify** - Automatic reconnection within 5 seconds, operations resume

## 📊 Performance Features

- **Prepared Statements**: All queries use prepared statements for optimal performance
- **Connection Pooling**: Managed by ScyllaDB driver
- **Statement Caching**: Prepared statements are cached and reused
- **Efficient Queries**: Uses appropriate CQL operations for each use case
- **Connection Checking**: Lightweight connection checks before operations
- **Retry Logic**: Built-in retry mechanisms for transient failures
- **Isolated Repositories**: Independent connection management prevents cascading failures

## 🛠️ Dependencies

- `scylla = "1.4.1"` - ScyllaDB/Cassandra driver
- `tokio = "1.0"` - Async runtime
- `uuid = "1.8.0"` - UUID generation and handling
- `serde = "1.0"` - Serialization support
- `log = "0.4"` - Logging framework
- `env_logger = "0.10.0"` - Logging implementation

## 📝 Logging

Control logging levels with the `RUST_LOG` environment variable:

```bash
# Show info and above
RUST_LOG=info cargo run

# Show debug and above
RUST_LOG=debug cargo run

# Suppress scylla internal errors
RUST_LOG=info,scylla=warn cargo run
```

## 🔍 Key Features Summary

| Feature | Description |
|---------|-------------|
| **Resilient Startup** | Application starts even if database is unavailable |
| **Auto Reconnection** | Polls every 5 seconds for database availability |
| **Multi-Repository** | Separate repositories with different keyspaces |
| **Repository Trait** | Common interface with default method implementations |
| **Connection Isolation** | Problems in one repository don't affect others |
| **Type-Safe Models** | User and Product models with UUID primary keys |
| **Full CRUD** | Create, Read, Update, Delete operations |
| **Auto Connection Check** | All CRUD operations check connection before execution |
| **Prepared Statements** | All queries use prepared statements |
| **Graceful Failures** | Operations fail gracefully without crashes |
| **Keyspace Management** | Automatic keyspace and table creation |
| **Configuration Validation** | Validates keyspace names before initialization |
| **Health Checks** | Built-in health check and status reporting |
| **Retry Logic** | Exponential backoff and fixed delay retry mechanisms |
| **Initialization Variants** | Validation, retry, and combined initialization methods |

## 🐛 Troubleshooting

### Connection Issues

If the application can't connect:
1. Verify Cassandra is running: `docker-compose ps`
2. Check Cassandra logs: `docker-compose logs cassandra`
3. Verify port 9042 is accessible
4. Wait for Cassandra to fully start (30-60 seconds)

### Compilation Errors

If you encounter compilation errors:
1. Run `cargo clean`
2. Update dependencies: `cargo update`
3. Check Rust version: `rustc --version`

## 📄 License

This is a Proof of Concept project for demonstration purposes.

## 🤝 Contributing

This is a POC project. Feel free to use it as a reference for your own projects.

## 📚 Additional Resources

- [ScyllaDB Rust Driver Documentation](https://docs.rs/scylla/)
- [Cassandra CQL Documentation](https://cassandra.apache.org/doc/latest/cql/)
- [Tokio Async Runtime](https://tokio.rs/)

## 🎯 Repository Trait Benefits

The `Repository` trait provides several advantages:

1. **Code Reusability**: Common functionality is defined once in the trait
2. **Consistency**: All repositories follow the same patterns and conventions
3. **Maintainability**: Changes to common logic only need to be made in one place
4. **Extensibility**: Easy to add new repositories by implementing the trait
5. **Testability**: Trait methods can be tested independently
6. **Isolation**: Each repository maintains independent connection management

### Trait Method Categories

- **Connection Management**: Wait for connections, ensure connectivity
- **Retry Logic**: Handle transient failures with exponential backoff
- **Health Monitoring**: Check repository health and status
- **Configuration**: Validate and manage repository configuration
- **Initialization**: Multiple initialization strategies (validation, retry, combined)

---

**Note**: This POC demonstrates best practices for resilient database operations with Cassandra/ScyllaDB in Rust. It's designed to be production-ready with proper error handling, connection management, fault tolerance, and a clean trait-based architecture for code reuse and maintainability.

