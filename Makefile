.PHONY: duckdump uploadpost

duckdump:
	@duckdb -c "\
	INSTALL sqlite; \
	LOAD sqlite; \
	COPY ( \
	    SELECT a.id,a.slug,a.title,a.category,a.published_at,a.author,b.body,STRING_AGG(c.tag,',') AS tags \
		FROM \
	        sqlite_scan('../database/mp_database.db','posts_metadata') a, \
		    sqlite_scan('../database/mp_database.db','posts_content') b, \
		    sqlite_scan('../database/mp_database.db','posts_tags') c \
		WHERE a.id = b.post_id AND b.post_id = c.post_id \
		GROUP BY a.id,a.slug,a.title,a.category,a.published_at,a.author,b.body) \
	        TO '~/GO/PORTOF/gzipportof/post.parquet' (format 'parquet', compression 'snappy');"

uploadpost:
	make duckdump
	scp ~/GO/PORTOF/gzipportof/* sabang:PORTOF/gzipportof/.
