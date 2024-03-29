# Development

## Authenticator

TODO

## Web

### Application

```bash
# Run application in development mode
cargo leptos watch
```

### Database

**Migration**

```bash
# Generate - generate a new, empty migration
sea-orm-cli migrate -d web/migration generate

# Fresh - drop all tables from the database, then reapply all migrations
sea-orm-cli migrate -d web/migration fresh

# Refresh - rollback all applied migrations, then reapply all migrations
sea-orm-cli migrate -d web/migration refresh

# Reset - rollback all applied migrations
sea-orm-cli migrate -d web/migration reset

# Status - check the status of all migrations
sea-orm-cli migrate -d web/migration status

# Up - apply pending migrations
sea-orm-cli migrate -d web/migration up

# Down - rollback applied migrations
sea-orm-cli migrate -d web/migration down
```

**Entities**

```bash
# Generate entities (apply migrations first)
sea-orm-cli generate entity -o web/database/src/entities --with-serde both
```
