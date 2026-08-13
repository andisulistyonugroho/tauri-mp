import { sql } from "kysely";
export default defineEventHandler(async () => {
	try {
		const posts = await db
			.selectFrom("posts_metadata as pm")
			.select([
				"pm.id",
				"pm.title",
				"pm.slug",
				"pm.category",
				"pm.published_at",
				sql<
					string | null
				>`(SELECT GROUP_CONCAT(pt.tag) FROM posts_tags pt WHERE pt.post_id = pm.id)`.as(
					"tags_string",
				),
			])
			.where("pm.status", "=", "published")
			.orderBy("pm.published_at", "desc")
			.execute();

		return posts.map((p) => ({
			...p,
			tags: p.tags_string ? p.tags_string.split(",") : [],
		}));
	} catch (error: any) {
		throw createError({
			statusCode: 500,
			statusMessage: error.message,
		});
	}
});
