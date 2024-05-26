import sqlite3

OLD_DATABASE = "old-db/data.db"
NEW_DATABASE = "backend/data.db"

old_conn = sqlite3.connect(OLD_DATABASE)
new_conn = sqlite3.connect(NEW_DATABASE)

old_cursor = old_conn.cursor()
old_cursor.execute(
    "SELECT a.name, (SELECT COUNT(*) FROM article_views WHERE name = a.name) FROM article_views a GROUP BY name"
)

new_cursor = new_conn.cursor()
for row in old_cursor.fetchall():
    new_cursor.execute(
        "INSERT INTO legacy_views (path, views) VALUES (?, ?)",
        (f"writing/{row[0]}", row[1]),
    )

new_conn.commit()
