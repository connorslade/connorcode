UPDATE analytics
SET path = SUBSTR(path, 2)
WHERE path LIKE '/%';