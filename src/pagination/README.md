## Different Pagination Types

The package implements several pagination strategies, each with its own advantages:

### 1. Page-based Pagination (`pagepagination/`)

- Uses simple page numbers: `?page=2&limit=10`
- Easy for users to understand
- Good for UIs with explicit page navigation

### 2. Token-based Pagination (`tokenpagination/`)

- Uses opaque tokens to track position: `?page_token=abc123&limit=10`
- Encapsulates pagination state without exposing implementation details
- Better for forward-only pagination in APIs

### 3. Keyset Pagination (`keysetpagination/`)

- Uses values from the last record: `?after=last_id&limit=10`
- More efficient for large datasets
- Maintains consistency when records are added/removed

### 4. Migration Pagination (`migrationpagination/`)

- Likely specialized for data migration scenarios
- May handle larger batch sizes or have different consistency guarantees

## Why Multiple Pagination Types?

Different pagination strategies are needed because:

- **Performance requirements**: Some methods (like keyset) perform better with large datasets
- **Consistency needs**: When data changes between requests, some methods maintain consistency better
- **API use cases**: Different clients have different capabilities and expectations
- **Resource characteristics**: Some collections work better with specific pagination approaches

Each strategy modifies response headers differently to communicate to clients how to navigate through paginated data.
