using SQLite

OLD_DATABASE = "old-db/data.db"
NEW_DATABASE = "backend/data.db"

old_db = SQLite.DB(OLD_DATABASE)
new_db = SQLite.DB(NEW_DATABASE)

# get view counts from old database
old_views = DBInterface.execute(old_db, "SELECT a.name, (SELECT COUNT(*) FROM article_views WHERE name = a.name) FROM article_views a GROUP BY name")

# transfer view counts to new database
stmt = SQLite.Stmt(new_db, "INSERT INTO legacy_views (path, views) VALUES (?, ?)")
for row in old_views
	DBInterface.execute(stmt, ["writing/$(row[1])", row[2]])
end
