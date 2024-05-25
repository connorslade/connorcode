WITH views(x) as (
    SELECT COUNT(*)
    FROM (
            SELECT DISTINCT timestamp / ?1,
                ip
            FROM analytics
            WHERE path = ?2
        )
    UNION ALL
    SELECT views
    from legacy_views
    where path = ?2
)
SELECT SUM(x)
FROM views;