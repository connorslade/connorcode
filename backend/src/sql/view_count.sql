SELECT COUNT(*)
FROM (
        SELECT DISTINCT timestamp / ?,
            ip
        FROM analytics
        WHERE path = ?
    );